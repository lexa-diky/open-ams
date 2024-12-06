use std::usize;

#[derive(Debug)]
pub(crate) enum IrVersion {
    Semver {
        major: usize,
        minor: usize,
        patch: usize,
    },
    Latest,
    Bundled
}