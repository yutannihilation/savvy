use std::cell::Cell;

use savvy_ffi::{R_NilValue, Rf_eval, Rf_xlength, SEXP, VECTOR_ELT};

use crate::{protect, sexp::utils::str_to_charsxp, unwind_protect, Sexp};

/// A result of a function call. Since the result does not yet belong to any
/// environemnt or object, so it needs protection and unprotection. This struct
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

/// Parse R code. This is equivalent to `parse(text = )`.
pub fn eval_parse_text<T: AsRef<str>>(text: T) -> crate::error::Result<EvalResult> {
    let parse_status: Cell<savvy_ffi::ParseStatus> = Cell::new(savvy_ffi::ParseStatus_PARSE_NULL);

    unsafe {
        let charsxp = str_to_charsxp(text.as_ref())?;
        savvy_ffi::Rf_protect(charsxp);
        let text_sexp = crate::unwind_protect(|| savvy_ffi::Rf_ScalarString(charsxp))?;

        // According to WRE (https://cran.r-project.org/doc/manuals/r-release/R-exts.html#Parsing-R-code-from-C),
        //
        // - R_ParseVector is essentially the code used to implement
        //   parse(text=) at R level.
        //   - The first argument is a character vector (corresponding to text).
        //   - The second the maximal number of expressions to parse
        //     (corresponding to n).
        //   - The third argument is a pointer to a variable of an enumeration
        //     type.
        //     - It is normal (as parse does) to regard all values other than
        //       PARSE_OK as an error.
        //     - Other values which might be returned are PARSE_INCOMPLETE (an
        //       incomplete expression was found) and PARSE_ERROR (a syntax
        //       error), in both cases the value returned being R_NilValue.
        //   - The fourth argument is a length one character vector to be used
        //     as a filename in error messages, a srcfile object or the R NULL
        //     object (as in the example above).
        //     - If a srcfile object was used, a srcref attribute would be
        //       attached to the result, containing a list of srcref objects of
        //       the same length as the expression, to allow it to be echoed
        //       with its original formatting.
        let parsed = unwind_protect(|| {
            savvy_ffi::R_ParseVector(text_sexp, -1, parse_status.as_ptr(), R_NilValue)
        })?;
        savvy_ffi::Rf_protect(parsed);

        if parse_status.get() != savvy_ffi::ParseStatus_PARSE_OK {
            return Err(crate::error::Error::InvalidRCode(text.as_ref().to_string()));
        }

        // For simplicity, accept only a single line of R code.
        if Rf_xlength(parsed) != 1 {
            return Err(crate::error::Error::GeneralError(format!(
                "eval_parse_text() accepts only a single expression, but got: {}",
                text.as_ref(),
            )));
        }

        let eval_result =
            unwind_protect(|| Rf_eval(VECTOR_ELT(parsed, 0), savvy_ffi::R_GlobalEnv))?;
        let token = protect::insert_to_preserved_list(eval_result);
        let out = EvalResult {
            inner: eval_result,
            token,
        };

        savvy_ffi::Rf_unprotect(2);

        Ok(out)
    }
}

#[cfg(test)]
mod test {
    use crate::{IntegerSexp, RealSexp};

    use super::eval_parse_text;

    fn assert_invalid_r_code(code: &str) {
        assert!(matches!(
            eval_parse_text(code),
            Err(crate::error::Error::InvalidRCode(_))
        ));
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

        assert!(eval_parse_text("1; 2; 3").is_err());

        Ok(())
    }
}
