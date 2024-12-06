use std::{
    f64::NAN,
    fs::{read, File},
    io::Error,
    path::{Path, PathBuf},
};

use crate::json::{AmsJson, AmsJsonParserError};

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
    group: String,
    directory: PathBuf,
    config: PathBuf,
    root_module: ProjectModuleLayout,
}

impl ProjectLayout {
    pub(crate) fn auto_detect<T: AsRef<Path>>(directory: T) -> Result<ProjectLayout, Error> {
        let mut directory_buf = directory.as_ref().to_path_buf();
        directory_buf.push("ams.json");
        let json = AmsJson::read(directory_buf).map_err(|e| match e {
            AmsJsonParserError::Other { message } => Error::other(message),
            AmsJsonParserError::Io { cause } => cause,
        })?;

        let json_body = json.as_body();
        let group = json_body
            .string("group")
            .ok_or(Error::other("group is missing in ams.json"))?;
        let name = json_body
            .string("name")
            .ok_or(Error::other("name is missing in ams.json"))?;

        Self::scan(directory, name, group)
    }

    pub(crate) fn scan<T: AsRef<Path>>(
        path: T,
        name: &str,
        group: &str,
    ) -> Result<ProjectLayout, Error> {
        let root = std::path::absolute(path)?;
        let mut layout = ProjectLayout {
            name: name.to_string(),
            directory: root.clone(),
            config: root.clone().join("ams.json"),
            group: group.to_string(),
            root_module: ProjectModuleLayout::scan(&root)?,
        };

        layout.root_module.name = name.to_string();

        Ok(layout)
    }

    pub(crate) fn module(&self, name: &str) -> Option<&ProjectModuleLayout> {
        if (self.root_module.name == name) {
            return Some(&self.root_module);
        }

        self.root_module.module(name)
    }

    pub(crate) fn module_by_path(&self, path: &str) -> Option<&ProjectModuleLayout> {
        let parts = path
            .split("/")
            .filter(|p| !p.is_empty())
            .collect::<Vec<&str>>();

        if (parts.is_empty()) {
            return None;
        }

        let mut root_q: &ProjectModuleLayout = self.module(parts[0])?;
        for part in &parts[1..] {
            root_q = root_q.module(part)?;
        }

        Some(root_q)
    }

    pub(crate) fn manifest_path(&self) -> PathBuf {
        let mut buf = self.directory.to_path_buf();
        buf.push("ams.json");
        buf
    }
}

impl ProjectModuleLayout {
    pub(crate) fn scan<T: AsRef<Path>>(path: T) -> Result<ProjectModuleLayout, Error> {
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

        let name = abs_path.file_name().unwrap().to_str().unwrap();
        Ok(ProjectModuleLayout {
            directory: abs_path.to_path_buf(),
            defs: defs_buf,
            children: child_buf,
            name: name.to_string(),
        })
    }

    pub(crate) fn module(&self, name: &str) -> Option<&ProjectModuleLayout> {
        if (self.name == name) {
            return Some(&self);
        }
        for child in &self.children {
            if child.name == name {
                return Some(child);
            }
        }

        None
    }

    pub(crate) fn definitions(&self) -> &Vec<PathBuf> {
        return &self.defs;
    }
}
