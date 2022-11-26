pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NotImplemented,
    ActionError(String),
    FileError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotImplemented => write!(f, "Not yet implemented"),
            Error::ActionError(err) => write!(f, "Error calling that action. {err}"),
            Error::FileError(err) => write!(f, "{err}"),
        }
    }
}
