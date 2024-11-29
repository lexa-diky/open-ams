#[derive(Debug)]
pub enum Error {
    Other { message: String },
    Delegate { error: Box<dyn std::error::Error> },
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}

pub type CliResult<T> = Result<T, Error>;
