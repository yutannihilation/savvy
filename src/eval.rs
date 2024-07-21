use std::ffi::CString;

use savvy_ffi::{R_ParseEvalString, R_compute_identical, Rboolean_TRUE, SEXP};

use crate::{
    protect::{self},
    unwind_protect, Sexp,
};

/// A result of a function call. Since the result does not yet belong to any
/// environment or object, so it needs protection and unprotection. This struct
/// is solely for handling the unprotection in `Drop`.
pub struct EvalResult {
    pub(crate) inner: SEXP,
    pub(crate) token: SEXP,
}

impl EvalResult {
    pub fn inner(&self) -> SEXP {
        self.inner
    }
}

impl Drop for EvalResult {
    fn drop(&mut self) {
        protect::release_from_preserved_list(self.token);
    }
}

impl From<EvalResult> for Sexp {
    fn from(value: EvalResult) -> Self {
        Self(value.inner())
    }
}

impl From<EvalResult> for crate::error::Result<Sexp> {
    fn from(value: EvalResult) -> Self {
        Ok(<Sexp>::from(value))
    }
}

/// Parse and evaluate an R code. This is equivalent to `eval(parse(text = ))`.
///
/// For simplicity, this function accept only a single line of R code.
///
pub fn eval_parse_text<T: AsRef<str>>(text: T) -> crate::error::Result<EvalResult> {
    unsafe {
        let text_cstr = CString::new(text.as_ref()).unwrap();
        let eval_result =
            unwind_protect(|| R_ParseEvalString(text_cstr.as_ptr(), savvy_ffi::R_GlobalEnv))?;
        let token = protect::insert_to_preserved_list(eval_result);
        let out = EvalResult {
            inner: eval_result,
            token,
        };

        Ok(out)
    }
}

/// Check if the two SEXPs are identical in the sense that the R function
/// `identical()` returns `TRUE`.
pub fn is_r_identical<T1: Into<Sexp>, T2: Into<Sexp>>(x: T1, y: T2) -> bool {
    let x_sexp: Sexp = x.into();
    let y_sexp: Sexp = y.into();
    // They say 16 is the same as identical()'s default
    unsafe { R_compute_identical(x_sexp.0, y_sexp.0, 16) == Rboolean_TRUE }
}

/// Assert that the SEXPs have the same data inside. The second argument is a
/// string of R code.
///
/// ```
/// use savvy::assert_eq_r_code;
///
/// let mut x = savvy::OwnedRealSexp::new(3)?;
/// x[1] = 1.0;
/// x[2] = 2.0;
/// assert_eq_r_code(x, "c(0.0, 1.0, 2.0)");
/// ```
pub fn assert_eq_r_code<T1: Into<Sexp>, T2: AsRef<str>>(actual: T1, expected: T2) {
    let parsed = eval_parse_text(expected).expect("Failed to parse R code");
    assert!(is_r_identical(actual, parsed));
}

#[cfg(feature = "savvy-test")]
mod test {
    use crate::{IntegerSexp, RealSexp};

    use super::eval_parse_text;

    fn assert_invalid_r_code(code: &str) {
        assert!(eval_parse_text(code).is_err());
    }

    #[test]
    fn test_eval() -> crate::Result<()> {
        let parse_int = eval_parse_text("1L")?;
        let x = crate::Sexp(parse_int.inner());
        assert!(x.is_integer());
        assert_eq!(IntegerSexp::try_from(x)?.as_slice(), &[1]);

        let parse_real = eval_parse_text("1.0")?;
        let x = crate::Sexp(parse_real.inner());
        assert!(x.is_real());
        assert_eq!(RealSexp::try_from(x)?.as_slice(), &[1.0]);

        let parse_vec = eval_parse_text("c(1, 2, 3)")?;
        let x = crate::Sexp(parse_vec.inner());
        assert!(x.is_real());
        assert_eq!(RealSexp::try_from(x)?.as_slice(), &[1.0, 2.0, 3.0]);

        // error cases
        assert_invalid_r_code("foo(");
        assert_invalid_r_code("<- a");
        assert_invalid_r_code("1; 2; 3");

        Ok(())
    }
}
