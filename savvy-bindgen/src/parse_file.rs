use std::{fs::File, io::Read, path::Path};

use syn::parse_quote;

use crate::{
    extract_docs, ir::ParsedTestCase, ParsedResult, SavvyEnum, SavvyFn, SavvyImpl, SavvyStruct,
};

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

pub fn parse_file(path: &Path, cur_mod: &str) -> ParsedResult {
    let file_content = read_file(path);

    let module_level_docs: Vec<&str> = file_content
        .lines()
        .filter(|x| x.trim().starts_with("//!"))
        .map(|x| x.split_at(3).1.trim())
        .collect();

    let tests = parse_doctests(&module_level_docs, cur_mod);
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
        cur_mod: cur_mod.to_string(),
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

                let label = format!("{}::{}", self.cur_mod, item_fn.sig.ident);

                self.tests
                    .append(&mut parse_doctests(&extract_docs(&item_fn.attrs), &label))
            }

            syn::Item::Impl(item_impl) => {
                if is_marked(item_impl.attrs.as_slice()) {
                    self.impls
                        .push(SavvyImpl::new(item_impl).expect("Failed to parse impl"))
                }

                let self_ty = match item_impl.self_ty.as_ref() {
                    syn::Type::Path(p) => p.path.segments.last().unwrap().ident.to_string(),
                    _ => "(unknown)".to_string(),
                };
                let label = format!("{}::{}", self.cur_mod, self_ty);

                item_impl
                    .items
                    .iter()
                    .for_each(|x| self.parse_impl_item(x, &label));

                self.tests
                    .append(&mut parse_doctests(&extract_docs(&item_impl.attrs), &label))
            }

            syn::Item::Struct(item_struct) => {
                if is_marked(item_struct.attrs.as_slice()) {
                    self.structs
                        .push(SavvyStruct::new(item_struct).expect("Failed to parse struct"))
                }

                let label = format!("{}::{}", self.cur_mod, item_struct.ident);

                self.tests.append(&mut parse_doctests(
                    &extract_docs(&item_struct.attrs),
                    &label,
                ))
            }

            syn::Item::Enum(item_enum) => {
                if is_marked(item_enum.attrs.as_slice()) {
                    self.enums
                        .push(SavvyEnum::new(item_enum).expect("Failed to parse enum"))
                }

                let label = format!("{}::{}", self.cur_mod, item_enum.ident);

                self.tests
                    .append(&mut parse_doctests(&extract_docs(&item_enum.attrs), &label))
            }

            syn::Item::Mod(item_mod) => {
                if let Some((_, items)) = &item_mod.content {
                    items.iter().for_each(|i| self.parse_item(i));
                } else {
                    self.mods.push(item_mod.ident.to_string());
                }
            }

            syn::Item::Macro(item_macro) => {
                let ident = match &item_macro.ident {
                    Some(i) => i.to_string(),
                    None => "unknown".to_string(),
                };
                let label = format!("{}::{}", self.cur_mod, ident);

                self.tests.append(&mut parse_doctests(
                    &extract_docs(&item_macro.attrs),
                    &label,
                ))
            }

            _ => {}
        };
    }

    fn parse_impl_item(&mut self, item: &syn::ImplItem, label: &str) {
        let attrs = match item {
            syn::ImplItem::Const(c) => &c.attrs,
            syn::ImplItem::Fn(f) => &f.attrs,
            syn::ImplItem::Type(t) => &t.attrs,
            syn::ImplItem::Macro(m) => &m.attrs,
            syn::ImplItem::Verbatim(_) => return,
            _ => return,
        };

        self.tests
            .append(&mut parse_doctests(&extract_docs(attrs), label))
    }
}

fn parse_doctests<T: AsRef<str>>(lines: &[T], label: &str) -> Vec<ParsedTestCase> {
    let mut out: Vec<ParsedTestCase> = Vec::new();

    let mut in_code_block = false;
    let mut ignore = false;
    let mut code_block: Vec<String> = Vec::new();
    let mut spaces = 0;
    for line_orig in lines {
        let line = line_orig.as_ref();

        if line.trim().starts_with("```") {
            if !in_code_block {
                // start of the code block

                spaces = line.len() - line.trim().len();

                in_code_block = true;
                let code_attr = line.trim().strip_prefix("```").unwrap().trim();
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
                    out.push(ParsedTestCase {
                        code: code_block.join("\n"),
                        label: label.to_string(),
                    });
                }

                code_block.truncate(0);

                // reset
                in_code_block = false;
                ignore = false;
                spaces = 0;
            }
            continue;
        }

        if in_code_block {
            let line = if line.len() <= spaces {
                ""
            } else {
                line.split_at(spaces).1
            };
            code_block.push(line.to_string());
        }
    }

    out
}
