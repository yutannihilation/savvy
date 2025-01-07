use std::collections::HashMap;

use syn::Ident;

use crate::{SavvyEnum, SavvyFn, SavvyImpl, SavvyStruct};

pub mod savvy_enum;
pub mod savvy_fn;
pub mod savvy_impl;
pub mod savvy_struct;

pub struct ParsedTestCase {
    pub label: String,
    pub orig_code: String,
    pub location: String,
    pub code: String,
}

// For main.rs
pub struct ParsedResult {
    pub base_path: std::path::PathBuf,
    pub bare_fns: Vec<SavvyFn>,
    pub impls: Vec<SavvyImpl>,
    pub structs: Vec<SavvyStruct>,
    pub enums: Vec<SavvyEnum>,
    pub mod_path: Vec<String>,
    pub child_mods: Vec<String>,
    pub tests: Vec<ParsedTestCase>,
}

pub struct SavvyMergedImpl {
    /// Doc comments
    pub docs: Vec<String>,
    /// Original type name
    pub ty: syn::Ident,
    /// Methods and accociated functions
    pub fns: Vec<SavvyFn>,
}

pub struct MergedResult {
    pub bare_fns: Vec<SavvyFn>,
    pub impls: Vec<(Ident, SavvyMergedImpl)>,
    pub enums: Vec<SavvyEnum>,
}

#[derive(Debug, Clone)]
pub enum SavvyParseError {
    ConflictingDefinitions(syn::Ident),
}

impl std::fmt::Display for SavvyParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SavvyParseError::ConflictingDefinitions(ident) => {
                write!(f, "Different definitions are found for fn `{ident}`")
            }
        }
    }
}

pub fn merge_parsed_results(
    results: Vec<ParsedResult>,
) -> Result<MergedResult, Vec<SavvyParseError>> {
    let mut bare_fns: Vec<SavvyFn> = Vec::new();
    let mut impl_map: HashMap<Ident, SavvyMergedImpl> = HashMap::new();
    let mut enums: Vec<SavvyEnum> = Vec::new();

    for result in results {
        let mut fns = result.bare_fns;
        bare_fns.append(&mut fns);

        for i in result.impls {
            let key = i.ty.clone();
            match impl_map.get_mut(&key) {
                Some(merged) => {
                    let mut fns = i.fns;
                    merged.fns.append(&mut fns);
                }
                None => {
                    impl_map.insert(
                        key,
                        SavvyMergedImpl {
                            docs: Vec::new(),
                            ty: i.ty,
                            fns: i.fns,
                        },
                    );
                }
            }
        }

        // get documents from struct
        for s in result.structs {
            let key = s.ty.clone();
            match impl_map.get_mut(&key) {
                Some(merged) => {
                    merged.docs = s.docs;
                }
                None => {
                    impl_map.insert(
                        key,
                        SavvyMergedImpl {
                            docs: Vec::new(),
                            ty: s.ty,
                            fns: Vec::new(),
                        },
                    );
                }
            }
        }

        for e in result.enums {
            let key = e.ty.clone();
            match impl_map.get_mut(&key) {
                Some(merged) => {
                    merged.docs.clone_from(&e.docs);
                }
                None => {
                    impl_map.insert(
                        key,
                        SavvyMergedImpl {
                            docs: e.docs.clone(),
                            ty: e.ty.clone(),
                            fns: Vec::new(),
                        },
                    );
                }
            }
            enums.push(e);
        }
    }

    let mut impls = impl_map.into_iter().collect::<Vec<_>>();

    // in order to make the wrapper generation deterministic, sort by the type
    impls.sort_by_key(|(k, _)| k.clone());

    let mut parse_errors: Vec<SavvyParseError> = Vec::new();

    // validate bare functions
    let bare_fns = match remove_duplicated_fns(&bare_fns) {
        Ok(fns) => fns,
        Err(mut e) => {
            parse_errors.append(&mut e);
            Vec::new()
        }
    };

    // validate impl functions
    for (_, merged) in impls.iter_mut() {
        merged.fns = match remove_duplicated_fns(&merged.fns) {
            Ok(fns) => fns,
            Err(mut e) => {
                parse_errors.append(&mut e);
                Vec::new()
            }
        };
    }

    if parse_errors.is_empty() {
        Ok(MergedResult {
            bare_fns,
            impls,
            enums,
        })
    } else {
        Err(parse_errors)
    }
}

fn remove_duplicated_fns(fns: &[SavvyFn]) -> Result<Vec<SavvyFn>, Vec<SavvyParseError>> {
    let mut fn_map: HashMap<syn::Ident, SavvyFn> = HashMap::new();
    let mut parse_errors: Vec<SavvyParseError> = Vec::new();

    for f in fns {
        match fn_map.get(&f.fn_name) {
            // If there's already the same name of the function, check if the signatures match.
            Some(f_entry) => {
                let args1 = &f.args;
                let args2 = &f_entry.args;
                if args1.len() == args2.len()
                    && args1.iter().zip(args2.iter()).all(|(a1, a2)| a1 == a2)
                {
                    // do nothing if these share the same signature
                } else {
                    // raise an error if the signatures differ
                    parse_errors.push(SavvyParseError::ConflictingDefinitions(f.fn_name.clone()));
                }
            }
            None => {
                fn_map.insert(f.fn_name.clone(), f.clone());
            }
        }
    }

    if parse_errors.is_empty() {
        Ok(fn_map.into_values().collect())
    } else {
        Err(parse_errors)
    }
}
