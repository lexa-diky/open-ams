use std::{fs::File, io::BufReader, path::Path};
use std::collections::HashMap;
use crate::Assets;
use serde::de::{DeserializeOwned, Error};
use thiserror::Error;
use walkdir::WalkDir;
use crate::entity::epath::EPath;
use crate::entity::ProjectIdentifier;
use super::{SourceManifest, SourceModuleFragment};

#[derive(Debug)]
pub struct SourceProject {
    manifest: SourceManifest,
    module_fragments: Vec<SourceModuleFragment>,
}

#[derive(Debug, Error)]
pub enum ProjectLoadingError {
    #[error("Manifest file 'ams' not found in the project files")]
    ManifestNotFound,
}

impl SourceProject {
    pub fn new(manifest: SourceManifest, modules: Vec<SourceModuleFragment>) -> Self {
        SourceProject { manifest, module_fragments: modules }
    }

    pub fn identifier(&self) -> ProjectIdentifier {
        self.manifest.identifier()
    }
    
    pub fn modules(&self) -> &[SourceModuleFragment] {
        self.module_fragments.as_slice()
    }

    pub fn compress_module_fragments(&mut self) {
        let mut buf: HashMap<EPath, SourceModuleFragment> = HashMap::new();

        for module_fragment in self.module_fragments.drain(..) {
            if let Some(target) = buf.get_mut(module_fragment.path()) {
                target.merge_with(module_fragment);
            } else {
                buf.insert(module_fragment.path().clone(), module_fragment);
            }
        }

        self.module_fragments = buf.into_values().collect();
    }

    pub fn from_asset(folder_path: &str) -> Result<SourceProject, ProjectLoadingError> {
        let mut project_files = Assets::iter().filter(|path| path.starts_with(folder_path));

        let manifest = project_files
            .find(|path| path.ends_with("ams.yaml"))
            .map((|path| Self::read_from_assets::<SourceManifest>(path.as_ref())))
            .ok_or(ProjectLoadingError::ManifestNotFound)?
            .unwrap();

        let modules: Vec<SourceModuleFragment> = project_files
            .filter(|path| !path.ends_with("ams.yaml"))
            .map((|path| Self::read_from_assets::<SourceModuleFragment>(path.as_ref()).unwrap()))
            .collect();

        let mut project = SourceProject::new(manifest, modules);
        
        project.compress_module_fragments();
        Ok(project)
    }

    pub fn from_path<T: AsRef<Path>>(path: T) -> Result<SourceProject, ProjectLoadingError> {
        let entries: Vec<walkdir::DirEntry> = WalkDir::new(path)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| entry.path().extension().unwrap() == "yaml")
            .collect();

        let manifest = entries
            .iter()
            .find(|entry| entry.path().ends_with("ams.yaml"))
            .map((|entry| Self::read_from_files::<SourceManifest>(entry.path().as_ref())))
            .ok_or(ProjectLoadingError::ManifestNotFound)
            .unwrap();

        let mut project = SourceProject::new(
            manifest.unwrap(),
            entries
                .iter()
                .filter(|entry| !entry.path().ends_with("ams.yaml"))
                .map(
                    (|entry| {
                        Self::read_from_files::<SourceModuleFragment>(entry.path().as_ref()).unwrap()
                    }),
                )
                .collect(),
        );
        
        project.compress_module_fragments();
        Ok(project)
    }

    fn read_from_assets<T: DeserializeOwned>(path: &str) -> Result<T, serde_yaml::Error> {
        let asset: rust_embed::EmbeddedFile =
            Assets::get(path).ok_or(serde_yaml::Error::custom("Asset not found"))?;
        let buf = BufReader::new(asset.data.as_ref());
        let module: T = serde_yaml::from_reader(buf)?;

        Ok(module)
    }

    fn read_from_files<T: DeserializeOwned>(path: &Path) -> Result<T, serde_yaml::Error> {
        let asset = File::open(path).map_err(|e| serde_yaml::Error::custom(e));
        let buf = BufReader::new(asset.unwrap());
        let module: T = serde_yaml::from_reader(buf)?;

        Ok(module)
    }
}
