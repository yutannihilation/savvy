use savvy_ffi::SEXP;

use super::Sexp;

/// An external SEXP of an environment.
pub struct EnvironmentSexp(pub SEXP);

impl EnvironmentSexp {
    #[inline]
    pub fn inner(&self) -> savvy_ffi::SEXP {
        self.0
    }
}

// conversions from/to EnvironmentSexp ***************

impl TryFrom<Sexp> for EnvironmentSexp {
    type Error = crate::error::Error;

    fn try_from(value: Sexp) -> crate::error::Result<Self> {
        if !value.is_environment() {
            let type_name = value.get_human_readable_type_name();
            let msg = format!("Expected an environment, got {type_name}s");
            return Err(crate::error::Error::UnexpectedType(msg));
        }
        Ok(Self(value.0))
    }
}

impl From<EnvironmentSexp> for Sexp {
    fn from(value: EnvironmentSexp) -> Self {
        Self(value.inner())
    }
}

impl From<EnvironmentSexp> for crate::error::Result<Sexp> {
    fn from(value: EnvironmentSexp) -> Self {
        Ok(<Sexp>::from(value))
    }
}
