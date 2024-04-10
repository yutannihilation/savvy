use std::{fs::File, io::Read, path::Path};

use syn::parse_quote;

use crate::{extract_docs, ParsedResult, SavvyEnum, SavvyFn, SavvyImpl, SavvyStruct};

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

    let module_level_docs: Vec<&str> = file_content
        .lines()
        .filter(|x| x.trim().starts_with("//!"))
        .map(|x| x.split_at(3).1.trim())
        .collect();

    let tests = parse_doctests(&module_level_docs);
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
        result.parse_item(&item)
    }

    result
}

impl ParsedResult {
    fn parse_item(&mut self, item: &syn::Item) {
        match item {
            syn::Item::Fn(item_fn) => {
                if is_marked(item_fn.attrs.as_slice()) {
                    self.bare_fns
                        .push(SavvyFn::from_fn(item_fn).expect("Failed to parse function"))
                }

                self.tests
                    .append(&mut parse_doctests(&extract_docs(&item_fn.attrs)))
            }

            syn::Item::Impl(item_impl) => {
                if is_marked(item_impl.attrs.as_slice()) {
                    self.impls
                        .push(SavvyImpl::new(item_impl).expect("Failed to parse impl"))
                }

                self.tests
                    .append(&mut parse_doctests(&extract_docs(&item_impl.attrs)))
            }

            syn::Item::Struct(item_struct) => {
                if is_marked(item_struct.attrs.as_slice()) {
                    self.structs
                        .push(SavvyStruct::new(item_struct).expect("Failed to parse struct"))
                }

                self.tests
                    .append(&mut parse_doctests(&extract_docs(&item_struct.attrs)))
            }

            syn::Item::Enum(item_enum) => {
                if is_marked(item_enum.attrs.as_slice()) {
                    self.enums
                        .push(SavvyEnum::new(item_enum).expect("Failed to parse enum"))
                }

                self.tests
                    .append(&mut parse_doctests(&extract_docs(&item_enum.attrs)))
            }

            syn::Item::Mod(item_mod) => {
                // ignore mod inside the file (e.g. mod test { .. })
                if item_mod.content.is_none() {
                    self.mods.push(item_mod.ident.to_string())
                }
            }
            _ => {}
        };
    }
}

fn parse_doctests<T: AsRef<str>>(lines: &[T]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();

    let mut in_code_block = false;
    let mut ignore = false;
    let mut code_block: Vec<String> = Vec::new();
    for line in lines {
        let line = line.as_ref().trim_start_matches(['#', ' ']);

        eprintln!("{}", line);

        if line.starts_with("```") {
            if !in_code_block {
                // start of the code block

                in_code_block = true;
                let code_attr = line.strip_prefix("```").unwrap().trim();
                ignore = match code_attr {
                    "ignore" | "no_run" | "text" => true,
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
