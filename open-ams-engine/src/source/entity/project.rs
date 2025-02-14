use std::{fs::File, io::BufReader, path::Path};

use crate::Assets;
use serde::de::{DeserializeOwned, Error};
use thiserror::Error;
use walkdir::WalkDir;
use crate::entity::ProjectIdentifier;
use super::{Manifest, ModuleFragment};

#[derive(Debug)]
pub struct SourceProject {
    manifest: Manifest,
    modules: Vec<ModuleFragment>,
}

#[derive(Debug, Error)]
pub enum ProjectLoadingError {
    #[error("Manifest file 'ams' not found in the project files")]
    ManifestNotFound,
}

impl SourceProject {
    pub fn new(manifest: Manifest, modules: Vec<ModuleFragment>) -> Self {
        SourceProject { manifest, modules }
    }

    pub fn identifier(&self) -> ProjectIdentifier {
        self.manifest.identifier()
    }

    pub fn from_asset(folder_path: &str) -> Result<SourceProject, ProjectLoadingError> {
        let mut project_filess = Assets::iter().filter(|path| path.starts_with(folder_path));

        let manifest = project_filess
            .find(|path| path.ends_with("ams.yaml"))
            .map((|path| Self::read_from_assets::<Manifest>(path.as_ref())))
            .ok_or(ProjectLoadingError::ManifestNotFound)?
            .unwrap();

        let modules: Vec<ModuleFragment> = project_filess
            .filter(|path| !path.ends_with("ams.yaml"))
            .map((|path| Self::read_from_assets::<ModuleFragment>(path.as_ref()).unwrap()))
            .collect();

        Ok(SourceProject::new(manifest, modules))
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
            .map((|entry| Self::read_from_files::<Manifest>(entry.path().as_ref())))
            .ok_or(ProjectLoadingError::ManifestNotFound)
            .unwrap();

        Ok(SourceProject::new(
            manifest.unwrap(),
            entries
                .iter()
                .filter(|entry| !entry.path().ends_with("ams.yaml"))
                .map(
                    (|entry| {
                        Self::read_from_files::<ModuleFragment>(entry.path().as_ref()).unwrap()
                    }),
                )
                .collect(),
        ))
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
