use syn::parse_quote;

use crate::{savvy_fn::SavvyFnReturnType, SavvyFn, SavvyFnType};

impl SavvyFn {
    pub fn generate_inner_fn(&self) -> syn::ItemFn {
        let fn_name_orig = &self.fn_name;
        let fn_name_inner = self.fn_name_inner();

        let args_pat: Vec<syn::Ident> = self.args.iter().map(|arg| arg.pat()).collect();
        let args_ty: Vec<syn::Type> = self
            .args
            .iter()
            .map(|arg| arg.to_rust_type_inner())
            .collect();

        let stmts_additional = &self.stmts_additional;
        let stmts_orig = &self.stmts_orig;
        let attrs = &self.attrs;

        let ret_ty = self.return_type.inner();
        let out: syn::ItemFn = match &self.fn_type {
            // A bare function
            SavvyFnType::BareFunction => parse_quote!(
                #(#attrs)*
                unsafe fn #fn_name_inner( #(#args_pat: #args_ty),* ) #ret_ty {
                    #(#stmts_additional)*
                    #(#stmts_orig)*
                }
            ),
            // A method
            SavvyFnType::Method(ty) => parse_quote!(
                #(#attrs)*
                #[allow(non_snake_case)]
                unsafe fn #fn_name_inner(self__: savvy::SEXP, #(#args_pat: #args_ty),* ) #ret_ty {
                    let self__ = savvy::get_external_pointer_addr(self__) as *mut #ty;
                    #(#stmts_additional)*

                    (*self__).#fn_name_orig(#(#args_pat),*)
                }
            ),
            // An associated function
            SavvyFnType::AssociatedFunction(ty) => {
                parse_quote!(
                    #(#attrs)*
                    #[allow(non_snake_case)]
                    unsafe fn #fn_name_inner(#(#args_pat: #args_ty),* ) #ret_ty {
                        #(#stmts_additional)*

                        #ty::#fn_name_orig(#(#args_pat),*)
                    }
                )
            }
            SavvyFnType::Constructor(ty) => parse_quote!(
                #(#attrs)*
                #[allow(non_snake_case)]
                unsafe fn #fn_name_inner(#(#args_pat: #args_ty),* ) #ret_ty {
                    use savvy::IntoExtPtrSxp;

                    #(#stmts_additional)*
                    let x = #ty::#fn_name_orig(#(#args_pat),*);
                    Ok(x.into_external_pointer())
                }
            ),
        };

        out
    }

    pub fn generate_outer_fn(&self) -> syn::ItemFn {
        let fn_name_inner = self.fn_name_inner();
        let fn_name_outer = self.fn_name_outer();

        let args_pat: Vec<syn::Ident> = self.args.iter().map(|arg| arg.pat()).collect();
        let args_ty: Vec<syn::Type> = self
            .args
            .iter()
            .map(|arg| arg.to_rust_type_inner())
            .collect();

        let (ok_lhs, ok_rhs): (syn::Expr, syn::Expr) = match &self.return_type {
            SavvyFnReturnType::ResultSexp(_) => (parse_quote!(result), parse_quote!(result)),
            SavvyFnReturnType::ResultUnit(_) => {
                (parse_quote!(_), parse_quote!(savvy::NullSxp.into()))
            }
        };
        let out: syn::ItemFn = match &self.fn_type {
            // if the function is a method, add `self__` to the first argument
            SavvyFnType::Method(_) => parse_quote!(
                #[allow(clippy::missing_safety_doc)]
                #[no_mangle]
                pub unsafe extern "C" fn #fn_name_outer(self__: savvy::SEXP, #(#args_pat: #args_ty),* ) -> savvy::SEXP {
                    match #fn_name_inner(self__, #(#args_pat),*) {
                        Ok(#ok_lhs) => #ok_rhs,
                        Err(e) => savvy::handle_error(e),
                    }
                }
            ),
            _ => parse_quote!(
                #[allow(clippy::missing_safety_doc)]
                #[no_mangle]
                pub unsafe extern "C" fn #fn_name_outer( #(#args_pat: #args_ty),* ) -> savvy::SEXP {
                    match #fn_name_inner(#(#args_pat),*) {
                        Ok(#ok_lhs) => #ok_rhs,
                        Err(e) => savvy::handle_error(e),
                    }
                }
            ),
        };
        out
    }
}
