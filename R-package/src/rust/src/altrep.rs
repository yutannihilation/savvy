use savvy::altrep::{register_altinteger_class, AltInteger};
use savvy::savvy;

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
fn init_altrep_class(dll_info: *mut savvy::ffi::DllInfo) -> savvy::Result<()> {
    register_altinteger_class::<MyAltInt>(dll_info)?;
    Ok(())
}

#[savvy]
fn altint() -> savvy::Result<savvy::Sexp> {
    let v = MyAltInt::new(vec![1, 2, 3]);
    let v_altrep = savvy::altrep::create_altrep_instance(v)?;
    Ok(savvy::Sexp(v_altrep))
}
