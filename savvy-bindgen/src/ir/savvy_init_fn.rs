use quote::ToTokens;

pub struct SavvyInitFn {
    // The function name
    pub fn_name: syn::Ident,
    // The original function
    pub orig: syn::ItemFn,
    // If the function takes `DLLInfo` as an argument
    pub use_dll_info: bool,
}

impl SavvyInitFn {
    pub fn new(orig: &syn::ItemFn) -> syn::Result<Self> {
        if !matches!(orig.sig.output, syn::ReturnType::Default) {
            return Err(syn::Error::new_spanned(
                orig.sig.output.to_token_stream(),
                "An initialization function cannot have the return value",
            ));
        }

        let e = Err(syn::Error::new_spanned(
            orig.sig.inputs.clone().into_token_stream(),
            "An initialization function can only accept `*mut DllInfo`",
        ));

        let use_dll_info = match orig.sig.inputs.len() {
            0 => false,
            1 => match orig.sig.inputs.first().unwrap() {
                syn::FnArg::Typed(syn::PatType { ty, .. }) => match ty.as_ref() {
                    syn::Type::Ptr(ptr) => {
                        // p.path.segments.last().unwrap().ident != "DllInfo" => {
                        if ptr.mutability.is_none() {
                            return e;
                        }
                        match ptr.elem.as_ref() {
                            syn::Type::Path(p)
                                if p.path.segments.last().unwrap().ident == "DllInfo" =>
                            {
                                true
                            }
                            _ => {
                                return e;
                            }
                        }
                    }
                    _ => return e,
                },
                _ => {
                    return e;
                }
            },
            _ => {
                return e;
            }
        };

        Ok(Self {
            fn_name: orig.sig.ident.clone(),
            orig: orig.clone(),
            use_dll_info,
        })
    }
}
