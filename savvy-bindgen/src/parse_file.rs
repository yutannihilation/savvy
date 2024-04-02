use std::{fs::File, io::Read, path::Path};

use syn::parse_quote;

use crate::{ParsedResult, SavvyFn, SavvyImpl, SavvyStruct};

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
        base_path: path
            .parent()
            .expect("Should have a parent dir")
            .to_path_buf(),
        bare_fns: Vec::new(),
        impls: Vec::new(),
        structs: Vec::new(),
        mods: Vec::new(),
    };

    for item in ast.items {
        match item {
            syn::Item::Fn(item_fn) => {
                if is_marked(item_fn.attrs.as_slice()) {
                    result
                        .bare_fns
                        .push(SavvyFn::from_fn(&item_fn).expect("Failed to parse function"))
                }
            }

            syn::Item::Impl(item_impl) => {
                if is_marked(item_impl.attrs.as_slice()) {
                    result
                        .impls
                        .push(SavvyImpl::new(&item_impl).expect("Failed to parse impl"))
                }
            }

            syn::Item::Struct(item_struct) => {
                if is_marked(item_struct.attrs.as_slice()) {
                    result
                        .structs
                        .push(SavvyStruct::new(&item_struct).expect("Failed to parse struct"))
                }
            }

            syn::Item::Mod(item_mod) => {
                // ignore mod inside the file (e.g. mod test { .. })
                if item_mod.content.is_none() {
                    result.mods.push(item_mod.ident.to_string())
                }
            }
            _ => continue,
        };
    }

    result
}
