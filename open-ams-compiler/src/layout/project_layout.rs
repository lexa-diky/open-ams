use std::{
    f64::NAN,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub(crate) struct ProjectModuleLayout {
    directory: PathBuf,
    name: String,
    defs: Vec<PathBuf>,
    children: Vec<ProjectModuleLayout>,
}

#[derive(Debug)]
pub(crate) struct ProjectLayout {
    name: String,
    directory: PathBuf,
    config: PathBuf,
    root_module: ProjectModuleLayout,
}

impl ProjectLayout {
    pub(crate) fn scan<T: AsRef<Path>>(
        path: T,
        name: &str,
    ) -> Result<ProjectLayout, std::io::Error> {
        let root = std::path::absolute(path)?;
        let mut layout = ProjectLayout {
            name: name.to_string(),
            directory: root.clone(),
            config: root.clone().join("ams.json"),
            root_module: ProjectModuleLayout::scan(&root)?,
        };

        layout.root_module.name = name.to_string();

        Ok(layout)
    }
}

impl ProjectModuleLayout {
    pub(crate) fn scan<T: AsRef<Path>>(path: T) -> Result<ProjectModuleLayout, std::io::Error> {
        let abs_path = std::path::absolute(path)?;

        let walk = walkdir::WalkDir::new(abs_path.clone())
            .max_depth(1)
            .min_depth(1);
        let mut defs_buf = Vec::new();
        let mut child_dirs = Vec::new();

        for dir_res in walk {
            match dir_res {
                Ok(dir) => {
                    if dir.file_type().is_file() {
                        if dir.file_name() == "ams.json" {
                            continue;
                        }
                        defs_buf.push(dir.into_path())
                    } else if dir.file_type().is_dir() {
                        child_dirs.push(dir.into_path());
                    }
                }
                Err(err) => return Err(err.into_io_error().unwrap()),
            }
        }

        let mut child_buf = Vec::new();
        for ch in child_dirs {
            child_buf.push(Self::scan(ch)?);
        }

        Ok(ProjectModuleLayout {
            directory: abs_path.to_path_buf(),
            defs: defs_buf,
            children: child_buf,
            name: String::from(abs_path.file_name().unwrap().to_str().unwrap()),
        })
    }
}
