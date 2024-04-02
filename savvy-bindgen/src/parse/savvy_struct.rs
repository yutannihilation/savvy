use syn::{parse_quote, ItemStruct};

use crate::extract_docs;

pub struct SavvyStruct {
    /// Doc comments
    pub docs: Vec<String>,
    /// Attributes except for `#[savvy]`
    pub attrs: Vec<syn::Attribute>,
    /// Original struct name
    pub ty: syn::Ident,
    /// Original code
    pub orig: ItemStruct,
}

impl SavvyStruct {
    pub fn new(orig: &syn::ItemStruct) -> syn::Result<Self> {
        let mut attrs = orig.attrs.clone();
        // Remove #[savvy]
        attrs.retain(|attr| attr != &parse_quote!(#[savvy]));
        // Extract doc comments
        let docs = extract_docs(attrs.as_slice());
        let ty = orig.ident.clone();

        Ok(Self {
            docs,
            attrs,
            ty,
            orig: orig.clone(),
        })
    }

    pub fn generate_try_from_impls(&self) -> Vec<syn::ItemImpl> {
        let ty = self.ty.clone();

        let impl_into_external_pointer: syn::ItemImpl =
            parse_quote!(impl savvy::IntoExtPtrSexp for #ty {});

        let impl_try_from_ty_to_sexp: syn::ItemImpl = parse_quote!(
            impl TryFrom<#ty> for savvy::Sexp {
                type Error = savvy::Error;

                fn try_from(value: #ty) -> savvy::Result<Self> {
                    use savvy::IntoExtPtrSexp;

                    Ok(value.into_external_pointer())
                }
            }
        );

        let impl_try_from_sexp_to_ref_ty: syn::ItemImpl = parse_quote!(
            impl TryFrom<savvy::Sexp> for &#ty {
                type Error = savvy::Error;

                fn try_from(value: savvy::Sexp) -> savvy::Result<Self> {
                    // Return error if the SEXP is not an external pointer
                    value.assert_external_pointer()?;

                    let x = unsafe { savvy::get_external_pointer_addr(value.0)? as *mut #ty };
                    let res = unsafe { x.as_ref() };
                    res.ok_or("Failed to convert the external pointer to the Rust object".into())
                }
            }
        );

        let impl_try_from_sexp_to_ref_mut_ty: syn::ItemImpl = parse_quote!(
            impl TryFrom<savvy::Sexp> for &mut #ty {
                type Error = savvy::Error;

                fn try_from(value: savvy::Sexp) -> savvy::Result<Self> {
                    // Return error if the SEXP is not an external pointer
                    value.assert_external_pointer()?;

                    let x = unsafe { savvy::get_external_pointer_addr(value.0)? as *mut #ty };
                    let res = unsafe { x.as_mut() };
                    res.ok_or("Failed to convert the external pointer to the Rust object".into())
                }
            }
        );

        let impl_try_from_sexp_to_ty: syn::ItemImpl = parse_quote!(
            impl TryFrom<savvy::Sexp> for #ty {
                type Error = savvy::Error;

                fn try_from(value: savvy::Sexp) -> savvy::Result<Self> {
                    // Return error if the SEXP is not an external pointer
                    value.assert_external_pointer()?;

                    unsafe { savvy::take_external_pointer_value::<#ty>(value.0) }
                }
            }
        );

        vec![
            impl_into_external_pointer,
            impl_try_from_ty_to_sexp,
            impl_try_from_sexp_to_ref_ty,
            impl_try_from_sexp_to_ref_mut_ty,
            impl_try_from_sexp_to_ty,
        ]
    }
}
