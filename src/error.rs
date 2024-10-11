use savvy_ffi::SEXP;

#[derive(Debug)]
pub enum Error {
    UnexpectedType { expected: String, actual: String },
    NotScalar,
    Aborted(SEXP),
    InvalidPointer,
    InvalidRCode(String),
    GeneralError(String),
}

impl Error {
    pub fn new<T: ToString>(msg: T) -> Self {
        Self::GeneralError(msg.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnexpectedType { expected, actual } => {
                write!(f, "Must be {expected}, not {actual}")
            }
            Error::NotScalar => write!(f, "Must be length 1 of non-missing value"),
            Error::Aborted(_) => write!(f, "Aborted due to some error"),
            Error::InvalidPointer => {
                write!(f, "This external pointer is already consumed or deleted")
            }
            Error::InvalidRCode(code) => write!(f, "Failed to parse R code: {code}"),
            Error::GeneralError(msg) => write!(f, "{msg}"),
        }
    }
}

impl crate::error::Error {
    pub fn with_arg_name(self, arg_name: &str) -> Self {
        match self {
            Error::UnexpectedType { expected, actual } => Error::GeneralError(format!(
                "Argument `{arg_name}` must be {expected}, not {actual}"
            )),
            Error::NotScalar => Error::GeneralError(format!(
                "Argument `{arg_name}` must be be length 1 of non-missing value"
            )),
            Error::InvalidPointer => Error::GeneralError(format!(
                "Argument `{arg_name}` is already consumed or deleted"
            )),
            _ => self,
        }
    }
}

// Note: Unlike anyhow::Error, this doesn't require Send and Sync. This is
// because,
//
// - anyhow preserves the original implementation for std::error::Error by
//   accessing vtable directly.
// - anyhow needs to be async-aware (cf.
//   https://github.com/dtolnay/anyhow/issues/81)
//
// However, savvy creates a string immediately here (because only a string can
// be propagated to R session), so both won't be a problem.
impl<E> From<E> for Error
where
    E: std::error::Error + 'static,
{
    fn from(value: E) -> Self {
        Self::new(value)
    }
}
