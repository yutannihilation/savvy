use savvy::altrep::{register_altinteger_class, register_altreal_class, AltInteger, AltReal};
use savvy::savvy;

// integer

struct MyAltInt(Vec<i32>);
impl savvy::IntoExtPtrSexp for MyAltInt {}

impl MyAltInt {
    fn new(x: Vec<i32>) -> Self {
        Self(x)
    }
}

impl AltInteger for MyAltInt {
    const CLASS_NAME: &'static str = "MyAltInt";
    const PACKAGE_NAME: &'static str = "TestPackage";

    fn length(&mut self) -> usize {
        self.0.len()
    }

    fn elt(&mut self, i: usize) -> i32 {
        self.0[i]
    }
}

#[savvy]
fn altint() -> savvy::Result<savvy::Sexp> {
    let v = MyAltInt::new(vec![1, 2, 3]);
    let v_altrep = v.into_altrep()?;
    Ok(savvy::Sexp(v_altrep))
}

// real

struct MyAltReal(Vec<f64>);
impl savvy::IntoExtPtrSexp for MyAltReal {}

impl MyAltReal {
    fn new(x: Vec<f64>) -> Self {
        Self(x)
    }
}

impl AltReal for MyAltReal {
    const CLASS_NAME: &'static str = "MyAltReal";
    const PACKAGE_NAME: &'static str = "TestPackage";

    fn length(&mut self) -> usize {
        self.0.len()
    }

    fn elt(&mut self, i: usize) -> f64 {
        self.0[i]
    }
}

#[savvy]
fn altreal() -> savvy::Result<savvy::Sexp> {
    let v = MyAltReal::new(vec![1.0, 2.0, 3.0]);
    let v_altrep = v.into_altrep()?;
    Ok(savvy::Sexp(v_altrep))
}

// initialization

#[savvy]
fn init_altrep_class(dll_info: *mut savvy::ffi::DllInfo) -> savvy::Result<()> {
    register_altinteger_class::<MyAltInt>(dll_info)?;
    register_altreal_class::<MyAltReal>(dll_info)?;
    Ok(())
}
