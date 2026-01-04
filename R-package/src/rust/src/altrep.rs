use savvy::altrep::{
    get_altrep_body_ref_unchecked, register_altinteger_class, register_altlist_class,
    register_altlogical_class, register_altraw_class, register_altreal_class,
    register_altstring_class, AltInteger, AltList, AltLogical, AltRaw, AltReal, AltString,
};
use savvy::{
    r_println, savvy, savvy_err, savvy_init, IntegerSexp, ListSexp, LogicalSexp, NotAvailableValue,
    NullSexp, RawSexp, RealSexp, StringSexp,
};

// integer

#[derive(Debug, Clone)]
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
    v.into_altrep()
}

#[savvy]
fn altint_empty() -> savvy::Result<savvy::Sexp> {
    let v = MyAltInt::new(vec![]);
    v.into_altrep()
}

#[savvy]
fn altint_na_only() -> savvy::Result<savvy::Sexp> {
    let v = MyAltInt::new(vec![i32::na(), i32::na()]);
    v.into_altrep()
}

#[savvy]
fn altint_toobig() -> savvy::Result<savvy::Sexp> {
    let v = MyAltInt::new(vec![i32::MAX, i32::MAX]);
    v.into_altrep()
}

#[savvy]
fn print_altint(x: IntegerSexp) -> savvy::Result<()> {
    if let Ok(x) = MyAltInt::try_from_altrep_ref(&x) {
        r_println!("{x:?}");
        return Ok(());
    };

    Err(savvy_err!("Not a known ALTREP"))
}

#[savvy]
fn tweak_altint(mut x: IntegerSexp) -> savvy::Result<()> {
    if let Ok(x) = MyAltInt::try_from_altrep_mut(&mut x, true) {
        for i in x.0.iter_mut() {
            *i *= 2;
        }
        x.0.push(0);
        return Ok(());
    };

    Err(savvy_err!("Not a known ALTREP"))
}

#[derive(Debug, Clone)]
struct MyAltInt2;

impl MyAltInt2 {
    fn new() -> Self {
        Self
    }
}

impl savvy::IntoExtPtrSexp for MyAltInt2 {}

impl AltInteger for MyAltInt2 {
    const CLASS_NAME: &'static str = "MyAltInt2";
    const PACKAGE_NAME: &'static str = "TestPackage";

    fn length(&mut self) -> usize {
        1
    }

    fn elt(&mut self, i: usize) -> i32 {
        10
    }

    fn sum(&mut self, _na_rm: bool) -> Option<f64> {
        Some(20.0)
    }

    fn min(&mut self, _na_rm: bool) -> Option<f64> {
        Some(30.0)
    }

    fn max(&mut self, _na_rm: bool) -> Option<f64> {
        Some(40.0)
    }
}

#[savvy]
fn altint2() -> savvy::Result<savvy::Sexp> {
    let v = MyAltInt2::new();
    v.into_altrep()
}

// real

#[derive(Debug, Clone)]
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
    v.into_altrep()
}

#[savvy]
fn altreal_empty() -> savvy::Result<savvy::Sexp> {
    let v = MyAltReal::new(vec![]);
    v.into_altrep()
}

#[savvy]
fn altreal_na_only() -> savvy::Result<savvy::Sexp> {
    let v = MyAltReal::new(vec![f64::na(), f64::na()]);
    v.into_altrep()
}

#[savvy]
fn print_altreal(x: RealSexp) -> savvy::Result<()> {
    if let Ok(x) = MyAltReal::try_from_altrep_ref(&x) {
        r_println!("{x:?}");
        return Ok(());
    };

    Err(savvy_err!("Not a known ALTREP"))
}

#[savvy]
fn tweak_altreal(mut x: RealSexp) -> savvy::Result<()> {
    if let Ok(x) = MyAltReal::try_from_altrep_mut(&mut x, true) {
        for i in x.0.iter_mut() {
            *i *= 2.0;
        }
        x.0.push(0.0);
        return Ok(());
    };

    Err(savvy_err!("Not a known ALTREP"))
}

#[derive(Debug, Clone)]
struct MyAltReal2;

impl MyAltReal2 {
    fn new() -> Self {
        Self
    }
}

impl savvy::IntoExtPtrSexp for MyAltReal2 {}

impl AltReal for MyAltReal2 {
    const CLASS_NAME: &'static str = "MyAltReal2";
    const PACKAGE_NAME: &'static str = "TestPackage";

    fn length(&mut self) -> usize {
        1
    }

    fn elt(&mut self, i: usize) -> f64 {
        10.0
    }

    fn sum(&mut self, _na_rm: bool) -> Option<f64> {
        Some(20.0)
    }

    fn min(&mut self, _na_rm: bool) -> Option<f64> {
        Some(30.0)
    }

    fn max(&mut self, _na_rm: bool) -> Option<f64> {
        Some(40.0)
    }
}

#[savvy]
fn altreal2() -> savvy::Result<savvy::Sexp> {
    let v = MyAltReal2::new();
    v.into_altrep()
}

// logical

#[derive(Debug, Clone)]
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
    v.into_altrep()
}

#[savvy]
fn print_altlogical(x: LogicalSexp) -> savvy::Result<()> {
    if let Ok(x) = MyAltLogical::try_from_altrep_ref(&x) {
        r_println!("{x:?}");
        return Ok(());
    };

    Err(savvy_err!("Not a known ALTREP"))
}

#[savvy]
fn tweak_altlogical(mut x: LogicalSexp) -> savvy::Result<()> {
    if let Ok(x) = MyAltLogical::try_from_altrep_mut(&mut x, true) {
        for i in x.0.iter_mut() {
            *i ^= true;
        }
        x.0.push(false);
        return Ok(());
    };

    Err(savvy_err!("Not a known ALTREP"))
}

// raw

#[derive(Debug, Clone)]
struct MyAltRaw(Vec<u8>);

impl MyAltRaw {
    fn new(x: Vec<u8>) -> Self {
        Self(x)
    }
}

impl savvy::IntoExtPtrSexp for MyAltRaw {}

impl AltRaw for MyAltRaw {
    const CLASS_NAME: &'static str = "MyAltRaw";
    const PACKAGE_NAME: &'static str = "TestPackage";

    fn length(&mut self) -> usize {
        self.0.len()
    }

    fn elt(&mut self, i: usize) -> u8 {
        self.0[i]
    }
}

#[savvy]
fn altraw() -> savvy::Result<savvy::Sexp> {
    let v = MyAltRaw::new(vec![1u8, 2, 3]);
    v.into_altrep()
}

#[savvy]
fn print_altraw(x: RawSexp) -> savvy::Result<()> {
    if let Ok(x) = MyAltRaw::try_from_altrep_ref(&x) {
        r_println!("{x:?}");
        return Ok(());
    };

    Err(savvy_err!("Not a known ALTREP"))
}

#[savvy]
fn tweak_altraw(mut x: RawSexp) -> savvy::Result<()> {
    if let Ok(x) = MyAltRaw::try_from_altrep_mut(&mut x, true) {
        for i in x.0.iter_mut() {
            *i += 1;
        }
        x.0.push(2);
        return Ok(());
    };

    Err(savvy_err!("Not a known ALTREP"))
}

// string

#[derive(Debug, Clone)]
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
    v.into_altrep()
}

#[savvy]
fn print_altstring(x: StringSexp) -> savvy::Result<()> {
    if let Ok(x) = MyAltString::try_from_altrep_ref(&x) {
        r_println!("{x:?}");
        return Ok(());
    };

    Err(savvy_err!("Not a known ALTREP"))
}

#[savvy]
fn tweak_altstring(mut x: StringSexp) -> savvy::Result<()> {
    if let Ok(x) = MyAltString::try_from_altrep_mut(&mut x, true) {
        for s in x.0.iter_mut() {
            s.push('0');
        }
        x.0.push("-1".to_string());
        return Ok(());
    };

    Err(savvy_err!("Not a known ALTREP"))
}

// list

#[derive(Debug, Clone)]
struct MyAltList {
    one: MyAltInt,
    two: MyAltString,
}

impl MyAltList {
    fn new(one: Vec<i32>, two: Vec<String>) -> Self {
        Self {
            one: MyAltInt::new(one),
            two: MyAltString::new(two),
        }
    }
}

impl savvy::IntoExtPtrSexp for MyAltList {}

impl AltList for MyAltList {
    const CLASS_NAME: &'static str = "MyAltList";
    const PACKAGE_NAME: &'static str = "TestPackage";

    fn length(&mut self) -> usize {
        2
    }

    fn elt(&mut self, i: usize) -> savvy::Sexp {
        match i {
            0 => self.one.clone().into_altrep().unwrap_or(NullSexp.into()),
            1 => self.two.clone().into_altrep().unwrap_or(NullSexp.into()),
            _ => NullSexp.into(),
        }
    }
}

#[savvy]
fn altlist() -> savvy::Result<savvy::Sexp> {
    let v = MyAltList::new(
        vec![1, 2, 3],
        vec!["a".to_string(), "b".to_string(), "c".to_string()],
    );
    let mut out = v.into_altrep()?;
    out.set_names(["one", "two"])?;
    Ok(out)
}

#[savvy]
fn print_altlist(x: ListSexp) -> savvy::Result<()> {
    if let Ok(x) = MyAltList::try_from_altrep_ref(&x) {
        r_println!("{x:?}");
        return Ok(());
    };

    Err(savvy_err!("Not a known ALTREP"))
}

#[savvy]
fn tweak_altlist(mut x: ListSexp) -> savvy::Result<()> {
    if let Ok(x) = MyAltList::try_from_altrep_mut(&mut x, true) {
        for i in x.one.0.iter_mut() {
            *i *= 2;
        }
        x.one.0.push(0);

        for s in x.two.0.iter_mut() {
            s.push('0');
        }
        x.two.0.push("-1".to_string());

        return Ok(());
    };

    Err(savvy_err!("Not a known ALTREP"))
}

// initialization

#[savvy_init]
fn init_altrep_class(dll_info: *mut savvy::ffi::DllInfo) -> savvy::Result<()> {
    register_altinteger_class::<MyAltInt>(dll_info)?;
    register_altinteger_class::<MyAltInt2>(dll_info)?;
    register_altreal_class::<MyAltReal>(dll_info)?;
    register_altreal_class::<MyAltReal2>(dll_info)?;
    register_altlogical_class::<MyAltLogical>(dll_info)?;
    register_altraw_class::<MyAltRaw>(dll_info)?;
    register_altstring_class::<MyAltString>(dll_info)?;
    register_altlist_class::<MyAltList>(dll_info)?;
    Ok(())
}

// misc

#[savvy]
fn get_altrep_class_name(x: IntegerSexp) -> savvy::Result<()> {
    let c = unsafe { savvy::altrep::get_altrep_class_name(x.0)? };
    r_println!("{c}");
    Ok(())
}

#[savvy]
fn get_altrep_package_name(x: IntegerSexp) -> savvy::Result<()> {
    let p = unsafe { savvy::altrep::get_altrep_package_name(x.0)? };
    r_println!("{p}");
    Ok(())
}

// Note: this is just for testing. cast_altrep_unchecked() is usually for the
// class defined in an external package. If it's an owned class, we can just use
// `try_from_altrep_*()` as shown above.
#[savvy]
fn print_altint_by_weird_way(x: IntegerSexp) -> savvy::Result<()> {
    let cls = unsafe { savvy::altrep::get_altrep_class_name(x.0)? };
    let pkg = unsafe { savvy::altrep::get_altrep_package_name(x.0)? };
    if cls == MyAltInt::CLASS_NAME && pkg == MyAltInt::PACKAGE_NAME {
        let out = unsafe { get_altrep_body_ref_unchecked::<MyAltInt>(&x.0)? };
        r_println!("{out:?}");
        Ok(())
    } else {
        Err(savvy_err!("Not an altint"))
    }
}
