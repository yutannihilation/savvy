use libR_sys::SEXP;

#[derive(Debug)]
pub enum Error {
    UnexpectedType(String),
    Aborted(SEXP),
    Unknown,
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnexpectedType(type_name) => write!(f, "Unexpected type: {}", type_name),
            Error::Aborted(_) => write!(f, "Aborted due to some error"),
            Error::Unknown => write!(f, "Unknown error"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
