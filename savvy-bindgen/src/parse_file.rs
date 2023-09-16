use std::{fs::File, io::Read, path::Path};

use syn::parse_quote;

use crate::{ParsedResult, SavvyFn, SavvyImpl};

pub fn parse_savvy_fn(item: &syn::Item) -> Option<SavvyFn> {
    let func = match item {
        syn::Item::Fn(func) => func,
        _ => {
            return None;
        }
    };

    // Generate bindings only when the function is marked by #[savvy]
    if func
        .attrs
        .iter()
        .any(|attr| attr == &parse_quote!(#[savvy]))
    {
        Some(SavvyFn::from_fn(func))
    } else {
        None
    }
}

pub fn parse_savvy_impl(item: &syn::Item) -> Vec<SavvyFn> {
    let item_impl = match item {
        syn::Item::Impl(item_impl) => item_impl,
        _ => {
            return Vec::new();
        }
    };

    // Generate bindings only when the function is marked by #[savvy]
    if item_impl
        .attrs
        .iter()
        .any(|attr| attr == &parse_quote!(#[savvy]))
    {
        SavvyImpl::new(item_impl).fns
    } else {
        Vec::new()
    }
}

fn is_marked(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| attr == &parse_quote!(#[savvy]))
}

pub fn read_file(path: &Path) -> String {
    if !path.exists() {
        eprintln!("{} does not exist", path.to_string_lossy());
        std::process::exit(1);
    }

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Failed to read the specified file");
            std::process::exit(2);
        }
    };

    let mut content = String::new();
    if file.read_to_string(&mut content).is_err() {
        eprintln!("Failed to read the specified file");
        std::process::exit(2);
    };

    content
}

pub fn parse_file(path: &Path) -> ParsedResult {
    let ast = match syn::parse_str::<syn::File>(&read_file(path)) {
        Ok(ast) => ast,
        Err(_) => {
            eprintln!("Failed to parse the specified file");
            std::process::exit(3);
        }
    };

    let mut result = ParsedResult {
        bare_fns: Vec::new(),
        impls: Vec::new(),
    };

    for item in ast.items {
        match item {
            syn::Item::Fn(item_fn) => {
                if is_marked(item_fn.attrs.as_slice()) {
                    result.bare_fns.push(SavvyFn::from_fn(&item_fn))
                }
            }

            syn::Item::Impl(item_impl) => {
                if is_marked(item_impl.attrs.as_slice()) {
                    result.impls.push(SavvyImpl::new(&item_impl))
                }
            }
            _ => continue,
        };
    }

    result
}
