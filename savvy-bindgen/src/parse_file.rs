use std::{fs::File, io::Read, path::Path};

use syn::parse_quote;

use crate::{ParsedResult, SavvyEnum, SavvyFn, SavvyImpl, SavvyStruct};

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
    let file_content = read_file(path);

    let tests = parse_doctests(&file_content);
    // TODO
    // tests.append(parse_test_mods(&file_content));

    let ast = match syn::parse_str::<syn::File>(&file_content) {
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
        enums: Vec::new(),
        mods: Vec::new(),
        tests,
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

            syn::Item::Enum(item_enum) => {
                if is_marked(item_enum.attrs.as_slice()) {
                    result
                        .enums
                        .push(SavvyEnum::new(&item_enum).expect("Failed to parse enum"))
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

fn parse_doctests(file_content: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();

    let mut in_code_block = false;
    let mut ignore = false;
    let mut code_block: Vec<String> = Vec::new();
    for line in file_content.lines() {
        if !line.starts_with("///") && !line.starts_with("//!") {
            continue;
        }

        let (_, line) = line.split_at(3);
        let line = line.trim_start();

        if line.starts_with("```") {
            if !in_code_block {
                // start of the code block

                in_code_block = true;
                let code_attr = line.strip_prefix("```").unwrap().trim();
                ignore = match code_attr {
                    "ignore" => true,
                    "no_run" => true,
                    "" => false,
                    _ => {
                        eprintln!(
                            "[WARN] Ignoring unsupported code block attribute: {}",
                            code_attr
                        );
                        true
                    }
                }
            } else {
                // end of the code block

                if !ignore {
                    out.push(code_block.join("\n"));
                }

                code_block.truncate(0);

                // reset
                in_code_block = false;
                ignore = false;
            }
            continue;
        }

        if in_code_block {
            code_block.push(line.to_string());
        }
    }

    out
}
