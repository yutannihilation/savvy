use syn::parse_quote;

use crate::{
    savvy_fn::{SavvyFnReturnType, UserDefinedStructReturnType},
    SavvyFn, SavvyFnType,
};

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

        // If the original return type is not wrapped with Result, it needs to be wrapped with Ok()
        let wrapped_with_result = match &self.return_type {
            SavvyFnReturnType::UserDefinedStruct(UserDefinedStructReturnType {
                wrapped_with_result,
                ..
            }) => *wrapped_with_result,
            _ => true,
        };

        let out: syn::ItemFn = match &self.fn_type {
            // A bare function
            SavvyFnType::BareFunction => parse_quote!(
                #(#attrs)*
                unsafe fn #fn_name_inner( #(#args_pat: #args_ty),* ) #ret_ty {
                    #(#stmts_additional)*
                    #(#stmts_orig)*
                }
            ),
            // A method with &self or &mut self
            SavvyFnType::Method(ty) => {
                if wrapped_with_result {
                    parse_quote!(
                        #(#attrs)*
                        #[allow(non_snake_case)]
                        unsafe fn #fn_name_inner(self__: savvy::ffi::SEXP, #(#args_pat: #args_ty),* ) #ret_ty {
                            let self__ = savvy::get_external_pointer_addr(self__)? as *mut #ty;
                            #(#stmts_additional)*

                            (*self__).#fn_name_orig(#(#args_pat),*)
                        }
                    )
                } else {
                    parse_quote!(
                        #(#attrs)*
                        #[allow(non_snake_case)]
                        unsafe fn #fn_name_inner(self__: savvy::ffi::SEXP, #(#args_pat: #args_ty),* ) #ret_ty {
                            let self__ = savvy::get_external_pointer_addr(self__)? as *mut #ty;
                            #(#stmts_additional)*

                            Ok((*self__).#fn_name_orig(#(#args_pat),*))
                        }
                    )
                }
            }
            // A method with self
            SavvyFnType::ConsumingMethod(ty) => {
                if wrapped_with_result {
                    parse_quote!(
                        #(#attrs)*
                        #[allow(non_snake_case)]
                        unsafe fn #fn_name_inner(self__: savvy::ffi::SEXP, #(#args_pat: #args_ty),* ) #ret_ty {
                            let self__ = <#ty>::try_from(savvy::Sexp(self__))?;
                            #(#stmts_additional)*

                            self__.#fn_name_orig(#(#args_pat),*)
                        }
                    )
                } else {
                    parse_quote!(
                        #(#attrs)*
                        #[allow(non_snake_case)]
                        unsafe fn #fn_name_inner(self__: savvy::ffi::SEXP, #(#args_pat: #args_ty),* ) #ret_ty {
                            let self__ = <#ty>::try_from(savvy::Sexp(self__))?;
                            #(#stmts_additional)*

                            Ok(self__.#fn_name_orig(#(#args_pat),*))
                        }
                    )
                }
            }

            // An associated function
            SavvyFnType::AssociatedFunction(ty) => {
                if wrapped_with_result {
                    parse_quote!(
                        #(#attrs)*
                        #[allow(non_snake_case)]
                        unsafe fn #fn_name_inner(#(#args_pat: #args_ty),* ) #ret_ty {
                            #(#stmts_additional)*

                            #ty::#fn_name_orig(#(#args_pat),*)
                        }
                    )
                } else {
                    parse_quote!(
                        #(#attrs)*
                        #[allow(non_snake_case)]
                        unsafe fn #fn_name_inner(#(#args_pat: #args_ty),* ) #ret_ty {
                            #(#stmts_additional)*

                            Ok(#ty::#fn_name_orig(#(#args_pat),*))
                        }
                    )
                }
            }
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

        let (ok_lhs, ok_rhs, wrapped_with_result): (syn::Expr, syn::Expr, bool) =
            match &self.return_type {
                SavvyFnReturnType::Unit(_) => (
                    parse_quote!(_),
                    parse_quote!(savvy::sexp::null::null()),
                    true,
                ),
                SavvyFnReturnType::Sexp(_) => (parse_quote!(result), parse_quote!(result.0), true),
                SavvyFnReturnType::UserDefinedStruct(UserDefinedStructReturnType {
                    wrapped_with_result,
                    ..
                }) => (
                    parse_quote!(result),
                    parse_quote!({
                        use savvy::IntoExtPtrSexp;
                        result.into_external_pointer().0
                    }),
                    *wrapped_with_result,
                ),
            };

        let out: syn::ItemFn = match &self.fn_type {
            // if the function is a method, add `self__` to the first argument
            SavvyFnType::Method(_) | SavvyFnType::ConsumingMethod(_) => {
                // `-> Self` is allowed
                if wrapped_with_result {
                    parse_quote!(
                        #[allow(clippy::missing_safety_doc)]
                        #[no_mangle]
                        pub unsafe extern "C" fn #fn_name_outer(self__: savvy::ffi::SEXP, #(#args_pat: #args_ty),* ) -> savvy::ffi::SEXP {
                            match #fn_name_inner(self__, #(#args_pat),*) {
                                Ok(#ok_lhs) => #ok_rhs,
                                Err(e) => savvy::handle_error(e),
                            }
                        }
                    )
                } else {
                    parse_quote!(
                        #[allow(clippy::missing_safety_doc)]
                        #[no_mangle]
                        pub unsafe extern "C" fn #fn_name_outer(self__: savvy::ffi::SEXP, #(#args_pat: #args_ty),* ) -> savvy::ffi::SEXP {
                            #fn_name_inner(self__, #(#args_pat),*)
                        }
                    )
                }
            }
            _ => parse_quote!(
                #[allow(clippy::missing_safety_doc)]
                #[no_mangle]
                pub unsafe extern "C" fn #fn_name_outer( #(#args_pat: #args_ty),* ) -> savvy::ffi::SEXP {
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
