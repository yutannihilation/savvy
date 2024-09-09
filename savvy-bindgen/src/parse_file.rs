use std::{fs::File, io::Read, path::Path};

use proc_macro2::Span;
use quote::format_ident;
use syn::{ext::IdentExt, parse_quote};

use crate::{
    extract_docs, ir::ParsedTestCase, utils::add_indent, ParsedResult, SavvyEnum, SavvyFn,
    SavvyImpl, SavvyStruct,
};

fn is_savvified(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| attr == &parse_quote!(#[savvy]))
}

fn is_savvified_init(attrs: &[syn::Attribute]) -> bool {
    attrs
        .iter()
        .any(|attr| attr == &parse_quote!(#[savvy_init]))
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

pub fn parse_file(path: &Path, mod_path: &[String]) -> ParsedResult {
    let location = &path.to_string_lossy();
    let file_content = read_file(path);

    // First, parse doctests in the module level document because they are not
    // the part of the AST tree (at least I cannot find the way to treat it as a
    // AST). So, this cannot catch the documents in the form `#[doc = include_str!("path/to/README.md")]`.

    let module_level_docs: Vec<&str> = file_content
        .lines()
        .filter(|x| x.trim().starts_with("//!"))
        .map(|x| x.split_at(3).1.trim())
        .collect();

    let tests = parse_doctests(&module_level_docs, "module-level doc", location);

    let mut result = ParsedResult {
        base_path: path
            .parent()
            .expect("Should have a parent dir")
            .to_path_buf(),
        bare_fns: Vec::new(),
        impls: Vec::new(),
        structs: Vec::new(),
        enums: Vec::new(),
        mod_path: mod_path.to_vec(),
        child_mods: Vec::new(),
        tests,
    };

    match syn::parse_str::<syn::File>(&file_content) {
        Ok(file) => {
            for item in file.items {
                result.parse_item(&item, location)
            }
        }
        Err(e) => {
            eprintln!("Failed to parse the specified file: {location}\n");
            eprintln!("Error:\n{e}\n");
            eprintln!("Code:\n{file_content}\n");
            std::process::exit(3);
        }
    };

    result
}

impl ParsedResult {
    fn parse_item(&mut self, item: &syn::Item, location: &str) {
        match item {
            syn::Item::Fn(item_fn) => {
                if is_savvified(item_fn.attrs.as_slice()) {
                    self.bare_fns
                        .push(SavvyFn::from_fn(item_fn, false).expect("Failed to parse function"))
                }

                if is_savvified_init(item_fn.attrs.as_slice()) {
                    self.bare_fns
                        .push(SavvyFn::from_fn(item_fn, true).expect("Failed to parse function"))
                }

                let label = format!("fn {}", item_fn.sig.ident);

                self.tests.append(&mut parse_doctests(
                    &extract_docs(&item_fn.attrs),
                    &label,
                    location,
                ))
            }

            syn::Item::Impl(item_impl) => {
                if is_savvified(item_impl.attrs.as_slice()) {
                    self.impls
                        .push(SavvyImpl::new(item_impl).expect("Failed to parse impl"))
                }

                let self_ty = match item_impl.self_ty.as_ref() {
                    syn::Type::Path(p) => p.path.segments.last().unwrap().ident.to_string(),
                    _ => "(unknown)".to_string(),
                };
                let label = format!("impl {}", self_ty);

                item_impl
                    .items
                    .iter()
                    .for_each(|x| self.parse_impl_item(x, &label, location));

                self.tests.append(&mut parse_doctests(
                    &extract_docs(&item_impl.attrs),
                    &label,
                    location,
                ))
            }

            syn::Item::Struct(item_struct) => {
                if is_savvified(item_struct.attrs.as_slice()) {
                    self.structs
                        .push(SavvyStruct::new(item_struct).expect("Failed to parse struct"))
                }

                let label = format!("struct {}", item_struct.ident);

                self.tests.append(&mut parse_doctests(
                    &extract_docs(&item_struct.attrs),
                    &label,
                    location,
                ))
            }

            syn::Item::Enum(item_enum) => {
                if is_savvified(item_enum.attrs.as_slice()) {
                    self.enums
                        .push(SavvyEnum::new(item_enum).expect("Failed to parse enum"))
                }

                let label = format!("enum {}", item_enum.ident);

                self.tests.append(&mut parse_doctests(
                    &extract_docs(&item_enum.attrs),
                    &label,
                    location,
                ))
            }

            syn::Item::Mod(item_mod) => {
                let is_test_mod = item_mod
                    .attrs
                    .iter()
                    .any(|attr| attr == &parse_quote!(#[cfg(feature = "savvy-test")]));

                match (&item_mod.content, is_test_mod) {
                    (None, false) => {
                        self.child_mods.push(item_mod.ident.unraw().to_string());
                    }
                    (None, true) => {}
                    (Some((_, items)), false) => {
                        items.iter().for_each(|i| self.parse_item(i, location));
                    }
                    (Some(_), true) => {
                        let label = self.mod_path.join("::");
                        let mut cur_mod_path = self.mod_path.clone();
                        cur_mod_path.push(item_mod.ident.unraw().to_string());

                        self.tests.push(transform_test_mod(
                            item_mod,
                            &label,
                            location,
                            &cur_mod_path,
                        ))
                    }
                }
            }

            syn::Item::Macro(item_macro) => {
                let ident = match &item_macro.ident {
                    Some(i) => i.to_string(),
                    None => "unknown".to_string(),
                };
                let label = format!("macro {}", ident);

                self.tests.append(&mut parse_doctests(
                    &extract_docs(&item_macro.attrs),
                    &label,
                    location,
                ))
            }

            _ => {}
        };
    }

    fn parse_impl_item(&mut self, item: &syn::ImplItem, label: &str, location: &str) {
        let (attrs, label) = match item {
            syn::ImplItem::Const(c) => (&c.attrs, format!("{}::{}", label, c.ident)),
            syn::ImplItem::Fn(f) => (&f.attrs, format!("{}::{}", label, f.sig.ident)),
            syn::ImplItem::Type(t) => (&t.attrs, format!("{}::{}", label, t.ident)),
            syn::ImplItem::Macro(m) => (
                &m.attrs,
                format!("{}::{}", label, m.mac.path.segments.last().unwrap().ident),
            ),
            syn::ImplItem::Verbatim(_) => return,
            _ => return,
        };

        self.tests
            .append(&mut parse_doctests(&extract_docs(attrs), &label, location))
    }
}

fn parse_doctests<T: AsRef<str>>(lines: &[T], label: &str, location: &str) -> Vec<ParsedTestCase> {
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
                    let orig_code = code_block.join("\n");
                    let code_parsed =
                        match syn::parse_str::<syn::Block>(&format!("{{ {orig_code} }}")) {
                            Ok(block) => block.stmts,
                            Err(e) => {
                                eprintln!("Failed to parse the specified file: {location}\n");
                                eprintln!("Error:\n{e}\n");
                                eprintln!("Code:\n{orig_code}\n");
                                std::process::exit(3);
                            }
                        };

                    let test_fn = wrap_with_test_function(
                        &orig_code,
                        &code_parsed,
                        &format_ident!("doctest"),
                        label,
                        location,
                        true,
                    );

                    out.push(ParsedTestCase {
                        orig_code,
                        label: label.to_string(),
                        location: location.to_string(),
                        code: unparse(&test_fn),
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

            // doctest can use # to the line from the document. But, it still
            // needs to be evaluated as a complete Rust code.
            //
            // cf. https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html#hiding-portions-of-the-example
            let line = if line.trim_start().starts_with('#') {
                line.trim_start_matches(|c: char| c.is_whitespace() || c == '#')
            } else {
                line
            };

            code_block.push(line.to_string());
        }
    }

    out
}

#[cfg(feature = "use_formatter")]
fn unparse<T: quote::ToTokens>(item: &T) -> String {
    let code_parsed: syn::File = parse_quote!(#item);
    // replace() is needed for replacing the linebreaks inside string literals
    prettyplease::unparse(&code_parsed).replace(r#"\n"#, "\n")
}

#[cfg(not(feature = "use_formatter"))]
fn unparse<T: quote::ToTokens>(item: &T) -> String {
    quote::quote!(#item).to_string()
}

fn transform_test_mod(
    item_mod: &syn::ItemMod,
    label: &str,
    location: &str,
    mod_path: &[String],
) -> ParsedTestCase {
    let mut item_mod = item_mod.clone();

    // Remove #[cfg(feature = "savvy-test")]
    item_mod
        .attrs
        .retain(|attr| attr != &parse_quote!(#[cfg(feature = "savvy-test")]));

    item_mod.ident = format_ident!("__UNIQUE_PREFIX__mod_{}", item_mod.ident);

    if let Some((_, items)) = &mut item_mod.content {
        items.insert(
            0,
            parse_quote!(
                use savvy::savvy;
            ),
        );

        for item in items {
            if let syn::Item::Fn(item_fn) = item {
                let orig_code = unparse(&item_fn);
                let orig_len = item_fn.attrs.len();

                item_fn.attrs.retain(|attr| attr != &parse_quote!(#[test]));

                // if it's marked with #[test], add tweaks to the function.
                if item_fn.attrs.len() < orig_len {
                    item_fn.attrs.push(parse_quote!(#[savvy]));
                    item_fn.sig.ident = format_ident!("__UNIQUE_PREFIX__fn_{}", item_fn.sig.ident);

                    *item_fn = wrap_with_test_function(
                        &orig_code,
                        &item_fn.block.stmts,
                        &item_fn.sig.ident,
                        label,
                        location,
                        false,
                    );
                }
            }
        }
    }

    let (_last, rest) = mod_path.split_last().unwrap();
    let code = unparse(&item_mod)
        // Replace super and crate with the actual crate name
        .replace("super::", &format!("{}::", rest.join("::")))
        .replace("crate::", &format!("{}::", mod_path.first().unwrap()))
        // TODO: for some reason, prettyplease adds a space before ::
        .replace("super ::", &format!("{}::", rest.join("::")))
        .replace("crate ::", &format!("{}::", mod_path.first().unwrap()))
        // since savvy_show_error is defined in the parent space, add crate::
        .replace("savvy_show_error", "crate::savvy_show_error");

    ParsedTestCase {
        label: label.to_string(),
        orig_code: "".to_string(),
        location: location.to_string(),
        code,
    }
}

pub fn generate_test_code(parsed_results: &Vec<ParsedResult>) -> String {
    let header: syn::File = parse_quote! {
        #[allow(unused_imports)]
        use savvy::savvy;

        pub(crate) fn savvy_show_error(code: &str, label: &str, location: &str, panic_info: &std::panic::PanicHookInfo) {
            let mut msg: Vec<String> = Vec::new();
            let orig_msg = panic_info.to_string();
            let mut lines = orig_msg.lines();

            lines.next(); // remove location

            for line in lines {
                msg.push(format!("    {}", line));
            }

            let error = msg.join("\n");

            savvy::r_eprintln!(
                "

Location:
    {label} (file: {location})
    
Code:
{code}
    
Error:
{error}
            ");
        }
    };

    let mut out = unparse(&header);
    out.push_str("\n\n");

    let mut i = 0;
    for result in parsed_results {
        for test in &result.tests {
            i += 1;
            out.push_str(
                &test
                    .code
                    .replace("__UNIQUE_PREFIX__", &format!("test_{i}_")),
            );
            out.push_str("\n\n");
        }
    }

    out
}

fn wrap_with_test_function(
    orig_code: &str,
    code_parsed: &[syn::Stmt],
    orig_ident: &syn::Ident,
    label: &str,
    location: &str,
    is_doctest: bool,
) -> syn::ItemFn {
    let test_type = if is_doctest { "doctest" } else { "test" };
    let msg_lit = syn::LitStr::new(
        &format!("running {test_type} of {label} (file: {location}) ..."),
        Span::call_site(),
    );

    let label_lit = syn::LitStr::new(label, Span::call_site());
    let location_lit = syn::LitStr::new(location, Span::call_site());
    let code_lit = syn::LitStr::new(&add_indent(orig_code, 4), Span::call_site());
    let ident = format_ident!("__UNIQUE_PREFIX__{}", orig_ident);

    let mut code = code_parsed.to_vec();
    if !code.is_empty() {
        // Add return value Ok(()) unless the original statement has return value.
        match code.last().unwrap() {
            syn::Stmt::Expr(_, None) => {}
            _ => {
                let last_line: syn::Expr = parse_quote!(Ok(()));
                code.push(syn::Stmt::Expr(last_line, None));
            }
        }
    }

    // Note: it's hard to determine the unique function name at this point.
    //       So, put a placeholder here and replace it in the parent function.
    parse_quote! {
        #[savvy]
        fn #ident() -> savvy::Result<()> {
            eprint!(#msg_lit);

            std::panic::set_hook(Box::new(|panic_info| savvy_show_error(#code_lit, #label_lit, #location_lit, panic_info)));

            let test = || -> savvy::Result<()> {
                #(#code)*
            };
            let result = std::panic::catch_unwind(|| test().expect("some error"));

            match result {
                Ok(_) => {
                    eprintln!("ok");
                    Ok(())
                }
                Err(_) => Err("test failed".into()),
            }
        }
    }
}
