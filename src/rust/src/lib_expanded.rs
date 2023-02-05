#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use extendr_api::prelude::*;
/// Return a static string.
///
/// @export
fn static_string() -> &'static str {
    "Hello world!"
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn wrap__static_string() -> extendr_api::SEXP {
    unsafe {
        use extendr_api::robj::*;
        extendr_api::handle_panic("static_string panicked.\0", || {
            extendr_api::Robj::from(static_string()).get()
        })
    }
}
#[allow(non_snake_case)]
fn meta__static_string(metadata: &mut Vec<extendr_api::metadata::Func>) {
    let mut args = ::alloc::vec::Vec::new();
    metadata.push(extendr_api::metadata::Func {
        doc: " Return a static string.\n\n @export",
        rust_name: "static_string",
        r_name: "static_string",
        mod_name: "static_string",
        args: args,
        return_type: "str",
        func_ptr: wrap__static_string as *const u8,
        hidden: false,
    })
}
/// Return a dynamic string.
///
/// @export
fn string(input: &str) -> String {
    input.to_string()
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn wrap__string(input: extendr_api::SEXP) -> extendr_api::SEXP {
    unsafe {
        use extendr_api::robj::*;
        let _input_robj = extendr_api::new_owned(input);
        extendr_api::handle_panic("string panicked.\0", || {
            extendr_api::Robj::from(string(extendr_api::unwrap_or_throw_error(
                _input_robj
                    .try_into()
                    .map_err(|e| extendr_api::Error::from(e)),
            )))
            .get()
        })
    }
}
#[allow(non_snake_case)]
fn meta__string(metadata: &mut Vec<extendr_api::metadata::Func>) {
    let mut args = <[_]>::into_vec(
        #[rustc_box]
        ::alloc::boxed::Box::new([extendr_api::metadata::Arg {
            name: "input",
            arg_type: "str",
            default: None,
        }]),
    );
    metadata.push(extendr_api::metadata::Func {
        doc: " Return a dynamic string.\n\n @export",
        rust_name: "string",
        r_name: "string",
        mod_name: "string",
        args: args,
        return_type: "String",
        func_ptr: wrap__string as *const u8,
        hidden: false,
    })
}
#[no_mangle]
#[allow(non_snake_case)]
pub fn get_unextendr_metadata() -> extendr_api::metadata::Metadata {
    let mut functions = Vec::new();
    let mut impls = Vec::new();
    meta__static_string(&mut functions);
    meta__string(&mut functions);
    functions.push(extendr_api::metadata::Func {
        doc: "Metadata access function.",
        rust_name: "get_unextendr_metadata",
        mod_name: "get_unextendr_metadata",
        r_name: "get_unextendr_metadata",
        args: Vec::new(),
        return_type: "Metadata",
        func_ptr: wrap__get_unextendr_metadata as *const u8,
        hidden: true,
    });
    functions.push(extendr_api::metadata::Func {
        doc: "Wrapper generator.",
        rust_name: "make_unextendr_wrappers",
        mod_name: "make_unextendr_wrappers",
        r_name: "make_unextendr_wrappers",
        args: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                extendr_api::metadata::Arg {
                    name: "use_symbols",
                    arg_type: "bool",
                    default: None,
                },
                extendr_api::metadata::Arg {
                    name: "package_name",
                    arg_type: "&str",
                    default: None,
                },
            ]),
        ),
        return_type: "String",
        func_ptr: wrap__make_unextendr_wrappers as *const u8,
        hidden: true,
    });
    extendr_api::metadata::Metadata {
        name: "unextendr",
        functions,
        impls,
    }
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn wrap__get_unextendr_metadata() -> extendr_api::SEXP {
    unsafe { extendr_api::Robj::from(get_unextendr_metadata()).get() }
}
#[no_mangle]
#[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn wrap__make_unextendr_wrappers(
    use_symbols_sexp: extendr_api::SEXP,
    package_name_sexp: extendr_api::SEXP,
) -> extendr_api::SEXP {
    unsafe {
        use extendr_api::robj::*;
        let robj = new_owned(use_symbols_sexp);
        let use_symbols: bool = <bool>::from_robj(&robj).unwrap();
        let robj = new_owned(package_name_sexp);
        let package_name: &str = <&str>::from_robj(&robj).unwrap();
        extendr_api::Robj::from(
            get_unextendr_metadata()
                .make_r_wrappers(use_symbols, package_name)
                .unwrap(),
        )
        .get()
    }
}
#[no_mangle]
#[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn R_init_unextendr_extendr(info: *mut extendr_api::DllInfo) {
    unsafe { extendr_api::register_call_methods(info, get_unextendr_metadata()) };
}
