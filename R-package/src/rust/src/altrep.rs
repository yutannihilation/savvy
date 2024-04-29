use savvy::altrep::{
    register_altinteger_class, register_altlogical_class, register_altreal_class,
    register_altstring_class, AltInteger, AltLogical, AltReal, AltString,
};
use savvy::savvy;

// integer

struct MyAltInt(Vec<i32>);

impl MyAltInt {
    fn new(x: Vec<i32>) -> Self {
        Self(x)
    }
}

impl savvy::IntoExtPtrSexp for MyAltInt {}

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

impl MyAltReal {
    fn new(x: Vec<f64>) -> Self {
        Self(x)
    }
}

impl savvy::IntoExtPtrSexp for MyAltReal {}

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

// logical

struct MyAltLogical(Vec<bool>);

impl MyAltLogical {
    fn new(x: Vec<bool>) -> Self {
        Self(x)
    }
}

impl savvy::IntoExtPtrSexp for MyAltLogical {}

impl AltLogical for MyAltLogical {
    const CLASS_NAME: &'static str = "MyAltLogical";
    const PACKAGE_NAME: &'static str = "TestPackage";

    fn length(&mut self) -> usize {
        self.0.len()
    }

    fn elt(&mut self, i: usize) -> bool {
        self.0[i]
    }
}

#[savvy]
fn altlogical() -> savvy::Result<savvy::Sexp> {
    let v = MyAltLogical::new(vec![true, false, true]);
    let v_altrep = v.into_altrep()?;
    Ok(savvy::Sexp(v_altrep))
}

// string

struct MyAltString(Vec<String>);

impl MyAltString {
    fn new(x: Vec<String>) -> Self {
        Self(x)
    }
}

impl savvy::IntoExtPtrSexp for MyAltString {}

impl AltString for MyAltString {
    const CLASS_NAME: &'static str = "MyAltString";
    const PACKAGE_NAME: &'static str = "TestPackage";

    fn length(&mut self) -> usize {
        self.0.len()
    }

    fn elt(&mut self, i: usize) -> &str {
        self.0[i].as_str()
    }
}

#[savvy]
fn altstring() -> savvy::Result<savvy::Sexp> {
    let v = MyAltString::new(vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    let v_altrep = v.into_altrep()?;
    Ok(savvy::Sexp(v_altrep))
}

// initialization

#[savvy]
fn init_altrep_class(dll_info: *mut savvy::ffi::DllInfo) -> savvy::Result<()> {
    register_altinteger_class::<MyAltInt>(dll_info)?;
    register_altreal_class::<MyAltReal>(dll_info)?;
    register_altlogical_class::<MyAltLogical>(dll_info)?;
    register_altstring_class::<MyAltString>(dll_info)?;
    Ok(())
}
