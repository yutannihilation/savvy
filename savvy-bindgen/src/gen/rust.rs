use syn::parse_quote;

use crate::{SavvyFn, SavvyFnType};

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

        let out: syn::ItemFn = match (&self.fn_type, self.has_result) {
            // A bare function with result
            (SavvyFnType::BareFunction, true) => parse_quote!(
                #(#attrs)*
                unsafe fn #fn_name_inner( #(#args_pat: #args_ty),* ) -> savvy::Result<savvy::SEXP> {
                    #(#stmts_additional)*
                    #(#stmts_orig)*
                }
            ),
            // A bare function without result; return a dummy value
            (SavvyFnType::BareFunction, false) => parse_quote!(
                #(#attrs)*
                unsafe fn #fn_name_inner( #(#args_pat: #args_ty),* ) -> savvy::Result<savvy::SEXP> {
                    #(#stmts_additional)*
                    #(#stmts_orig)*

                    // Dummy return value
                    Ok(savvy::NullSxp.into())
                }
            ),
            // A method with result
            (SavvyFnType::Method(ty), true) => parse_quote!(
                #(#attrs)*
                #[allow(non_snake_case)]
                unsafe fn #fn_name_inner(self__: savvy::SEXP, #(#args_pat: #args_ty),* ) -> savvy::Result<savvy::SEXP> {
                    let self__ = savvy::get_external_pointer_addr(self__) as *mut #ty;
                    #(#stmts_additional)*

                    (*self__).#fn_name_orig(#(#args_pat),*)
                }
            ),
            // A method without result; return a dummy value
            (SavvyFnType::Method(ty), false) => parse_quote!(
                #(#attrs)*
                #[allow(non_snake_case)]
                unsafe fn #fn_name_inner(self__: savvy::SEXP, #(#args_pat: #args_ty),* ) -> savvy::Result<savvy::SEXP> {
                    let self__ = savvy::get_external_pointer_addr(self__) as *mut #ty;
                    #(#stmts_additional)*

                    (*self__).#fn_name_orig(#(#args_pat),*);

                    // Dummy return value
                    Ok(savvy::NullSxp.into())
                }
            ),
            // An associated function with a result
            (SavvyFnType::AssociatedFunction(ty), true) => parse_quote!(
                #(#attrs)*
                #[allow(non_snake_case)]
                unsafe fn #fn_name_inner(#(#args_pat: #args_ty),* ) -> savvy::Result<savvy::SEXP> {
                    #(#stmts_additional)*

                    #ty::#fn_name_orig(#(#args_pat),*)
                }
            ),
            // An associated function without result; return a dummy value
            (SavvyFnType::AssociatedFunction(ty), false) => parse_quote!(
                #(#attrs)*
                #[allow(non_snake_case)]
                unsafe fn #fn_name_inner(#(#args_pat: #args_ty),* ) -> savvy::Result<savvy::SEXP> {
                    #(#stmts_additional)*

                    #ty::#fn_name_orig(#(#args_pat),*);

                    // Dummy return value
                    Ok(savvy::NullSxp.into())
                }
            ),
            (SavvyFnType::Constructor(ty), _) => parse_quote!(
                #(#attrs)*
                #[allow(non_snake_case)]
                unsafe fn #fn_name_inner(#(#args_pat: #args_ty),* ) -> savvy::Result<savvy::SEXP> {
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

        let out: syn::ItemFn = match &self.fn_type {
            // if the function is a method, add `self__` to the first argument
            SavvyFnType::Method(_) => parse_quote!(
                #[allow(clippy::missing_safety_doc)]
                #[no_mangle]
                pub unsafe extern "C" fn #fn_name_outer(self__: savvy::SEXP, #(#args_pat: #args_ty),* ) -> savvy::SEXP {
                    savvy::handle_result(#fn_name_inner(self__, #(#args_pat),*))
                }
            ),
            _ => parse_quote!(
                #[allow(clippy::missing_safety_doc)]
                #[no_mangle]
                pub unsafe extern "C" fn #fn_name_outer( #(#args_pat: #args_ty),* ) -> savvy::SEXP {
                    savvy::handle_result(#fn_name_inner(#(#args_pat),*))
                }
            ),
        };
        out
    }
}
