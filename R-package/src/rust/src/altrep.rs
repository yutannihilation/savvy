use savvy::altrep::{
    register_altinteger_class, register_altlogical_class, register_altreal_class,
    register_altstring_class, AltInteger, AltLogical, AltReal, AltString,
};
use savvy::{r_println, savvy, savvy_init, IntegerSexp, LogicalSexp, RealSexp, StringSexp};

// integer

#[derive(Debug)]
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

    fn dataptr(&mut self) -> Option<*mut i32> {
        Some(self.0.as_mut_ptr())
    }
}

#[derive(Debug)]
struct MyAltIntMutable(Vec<i32>);

impl MyAltIntMutable {
    fn new(x: Vec<i32>) -> Self {
        Self(x)
    }
}

impl savvy::IntoExtPtrSexp for MyAltIntMutable {}

impl AltInteger for MyAltIntMutable {
    const CLASS_NAME: &'static str = "MyAltIntMutable";
    const PACKAGE_NAME: &'static str = "TestPackage";
    const CACHE_MATERIALIZED_SEXP: bool = false;

    fn length(&mut self) -> usize {
        self.0.len()
    }

    fn elt(&mut self, i: usize) -> i32 {
        self.0[i]
    }

    fn dataptr(&mut self) -> Option<*mut i32> {
        Some(self.0.as_mut_ptr())
    }
}

#[savvy]
fn altint() -> savvy::Result<savvy::Sexp> {
    let v = MyAltInt::new(vec![1, 2, 3]);
    let v_altrep = v.into_altrep()?;
    Ok(savvy::Sexp(v_altrep))
}

#[savvy]
fn altint_mutable() -> savvy::Result<savvy::Sexp> {
    let v = MyAltIntMutable::new(vec![1, 2, 3]);
    let v_altrep = v.into_altrep()?;
    Ok(savvy::Sexp(v_altrep))
}

#[savvy]
fn print_altint(x: IntegerSexp) -> savvy::Result<()> {
    if let Ok(x) = MyAltInt::try_from_altrep_ref(&x) {
        r_println!("{x:?}");
        return Ok(());
    };

    if let Ok(x) = MyAltIntMutable::try_from_altrep_ref(&x) {
        r_println!("{x:?}");
        return Ok(());
    };

    Err("Not a known ALTREP".into())
}

#[savvy]
fn tweak_altint(mut x: IntegerSexp) -> savvy::Result<()> {
    if let Ok(x) = MyAltInt::try_from_altrep_mut(&mut x) {
        for i in x.0.iter_mut() {
            *i *= 2;
        }
        x.0.push(0);
        return Ok(());
    };

    if let Ok(x) = MyAltIntMutable::try_from_altrep_mut(&mut x) {
        for i in x.0.iter_mut() {
            *i *= 2;
        }
        x.0.push(0);
        return Ok(());
    };

    Err("Not a known ALTREP".into())
}

// real

#[derive(Debug)]
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

    fn dataptr(&mut self) -> Option<*mut f64> {
        Some(self.0.as_mut_ptr())
    }
}

#[savvy]
fn altreal() -> savvy::Result<savvy::Sexp> {
    let v = MyAltReal::new(vec![1.0, 2.0, 3.0]);
    let v_altrep = v.into_altrep()?;
    Ok(savvy::Sexp(v_altrep))
}

#[derive(Debug)]
struct MyAltRealMutable(Vec<f64>);

impl MyAltRealMutable {
    fn new(x: Vec<f64>) -> Self {
        Self(x)
    }
}

impl savvy::IntoExtPtrSexp for MyAltRealMutable {}

impl AltReal for MyAltRealMutable {
    const CLASS_NAME: &'static str = "MyAltRealMutable";
    const PACKAGE_NAME: &'static str = "TestPackage";
    const CACHE_MATERIALIZED_SEXP: bool = false;

    fn length(&mut self) -> usize {
        self.0.len()
    }

    fn elt(&mut self, i: usize) -> f64 {
        self.0[i]
    }

    fn dataptr(&mut self) -> Option<*mut f64> {
        Some(self.0.as_mut_ptr())
    }
}

#[savvy]
fn altreal_mutable() -> savvy::Result<savvy::Sexp> {
    let v = MyAltRealMutable::new(vec![1.0, 2.0, 3.0]);
    let v_altrep = v.into_altrep()?;
    Ok(savvy::Sexp(v_altrep))
}

#[savvy]
fn print_altreal(x: RealSexp) -> savvy::Result<()> {
    if let Ok(x) = MyAltReal::try_from_altrep_ref(&x) {
        r_println!("{x:?}");
        return Ok(());
    };

    if let Ok(x) = MyAltRealMutable::try_from_altrep_ref(&x) {
        r_println!("{x:?}");
        return Ok(());
    };

    Err("Not a known ALTREP".into())
}

#[savvy]
fn tweak_altreal(mut x: RealSexp) -> savvy::Result<()> {
    if let Ok(x) = MyAltReal::try_from_altrep_mut(&mut x) {
        for i in x.0.iter_mut() {
            *i *= 2.0;
        }
        x.0.push(0.0);
        return Ok(());
    };

    if let Ok(x) = MyAltRealMutable::try_from_altrep_mut(&mut x) {
        for i in x.0.iter_mut() {
            *i *= 2.0;
        }
        x.0.push(0.0);
        return Ok(());
    };

    Err("Not a known ALTREP".into())
}

// logical

#[derive(Debug)]
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

#[derive(Debug)]
struct MyAltLogicalMutable(Vec<bool>);

impl MyAltLogicalMutable {
    fn new(x: Vec<bool>) -> Self {
        Self(x)
    }
}

impl savvy::IntoExtPtrSexp for MyAltLogicalMutable {}

impl AltLogical for MyAltLogicalMutable {
    const CLASS_NAME: &'static str = "MyAltLogicalMutable";
    const PACKAGE_NAME: &'static str = "TestPackage";
    const CACHE_MATERIALIZED_SEXP: bool = false;

    fn length(&mut self) -> usize {
        self.0.len()
    }

    fn elt(&mut self, i: usize) -> bool {
        self.0[i]
    }
}

#[savvy]
fn altlogical_mutable() -> savvy::Result<savvy::Sexp> {
    let v = MyAltLogicalMutable::new(vec![true, false, true]);
    let v_altrep = v.into_altrep()?;
    Ok(savvy::Sexp(v_altrep))
}

#[savvy]
fn print_altlogical(x: LogicalSexp) -> savvy::Result<()> {
    if let Ok(x) = MyAltLogical::try_from_altrep_ref(&x) {
        r_println!("{x:?}");
        return Ok(());
    };

    if let Ok(x) = MyAltLogicalMutable::try_from_altrep_ref(&x) {
        r_println!("{x:?}");
        return Ok(());
    };

    Err("Not a known ALTREP".into())
}

#[savvy]
fn tweak_altlogical(mut x: LogicalSexp) -> savvy::Result<()> {
    if let Ok(x) = MyAltLogical::try_from_altrep_mut(&mut x) {
        for i in x.0.iter_mut() {
            *i ^= true;
        }
        x.0.push(false);
        return Ok(());
    };

    if let Ok(x) = MyAltLogicalMutable::try_from_altrep_mut(&mut x) {
        for i in x.0.iter_mut() {
            *i ^= true;
        }
        x.0.push(false);
        return Ok(());
    };

    Err("Not a known ALTREP".into())
}

// string

#[derive(Debug)]
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

#[derive(Debug)]
struct MyAltStringMutable(Vec<String>);

impl MyAltStringMutable {
    fn new(x: Vec<String>) -> Self {
        Self(x)
    }
}

impl savvy::IntoExtPtrSexp for MyAltStringMutable {}

impl AltString for MyAltStringMutable {
    const CLASS_NAME: &'static str = "MyAltStringMutable";
    const PACKAGE_NAME: &'static str = "TestPackage";
    const CACHE_MATERIALIZED_SEXP: bool = false;

    fn length(&mut self) -> usize {
        self.0.len()
    }

    fn elt(&mut self, i: usize) -> &str {
        self.0[i].as_str()
    }
}

#[savvy]
fn altstring_mutable() -> savvy::Result<savvy::Sexp> {
    let v = MyAltStringMutable::new(vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    let v_altrep = v.into_altrep()?;
    Ok(savvy::Sexp(v_altrep))
}

#[savvy]
fn print_altstring(x: StringSexp) -> savvy::Result<()> {
    if let Ok(x) = MyAltString::try_from_altrep_ref(&x) {
        r_println!("{x:?}");
        return Ok(());
    };

    if let Ok(x) = MyAltStringMutable::try_from_altrep_ref(&x) {
        r_println!("{x:?}");
        return Ok(());
    };

    Err("Not a known ALTREP".into())
}

#[savvy]
fn tweak_altstring(mut x: StringSexp) -> savvy::Result<()> {
    if let Ok(x) = MyAltString::try_from_altrep_mut(&mut x) {
        for s in x.0.iter_mut() {
            s.push('0');
        }
        x.0.push("-1".to_string());
        return Ok(());
    };

    if let Ok(x) = MyAltStringMutable::try_from_altrep_mut(&mut x) {
        for s in x.0.iter_mut() {
            s.push('0');
        }
        x.0.push("-1".to_string());
        return Ok(());
    };

    Err("Not a known ALTREP".into())
}

// initialization

#[savvy_init]
fn init_altrep_class(dll_info: *mut savvy::ffi::DllInfo) -> savvy::Result<()> {
    register_altinteger_class::<MyAltInt>(dll_info)?;
    register_altinteger_class::<MyAltIntMutable>(dll_info)?;

    register_altreal_class::<MyAltReal>(dll_info)?;
    register_altreal_class::<MyAltRealMutable>(dll_info)?;

    register_altlogical_class::<MyAltLogical>(dll_info)?;
    register_altlogical_class::<MyAltLogicalMutable>(dll_info)?;

    register_altstring_class::<MyAltString>(dll_info)?;
    register_altstring_class::<MyAltStringMutable>(dll_info)?;

    Ok(())
}
