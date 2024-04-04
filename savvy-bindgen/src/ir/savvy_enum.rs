use syn::{parse_quote, ItemEnum, ItemImpl};

use crate::extract_docs;

#[derive(Clone)]
pub struct SavvyEnum {
    /// Doc comments
    pub docs: Vec<String>,
    /// Attributes except for `#[savvy]`
    pub attrs: Vec<syn::Attribute>,
    /// Type name of the enum
    pub ty: syn::Ident,
    /// Variants
    pub variants: Vec<syn::Ident>,
}

impl SavvyEnum {
    pub fn new(orig: &syn::ItemEnum) -> syn::Result<Self> {
        let mut attrs = orig.attrs.clone();
        // Remove #[savvy]
        attrs.retain(|attr| attr != &parse_quote!(#[savvy]));
        // Extract doc comments
        let docs = extract_docs(attrs.as_slice());

        let ty = orig.ident.clone();

        let mut variants = Vec::new();

        for v in &orig.variants {
            if !matches!(v.fields, syn::Fields::Unit) {
                let e = syn::Error::new_spanned(
                    v.fields.clone(),
                    "savvy only supports a fieldless enum",
                );
                return Err(e);
            }

            if v.discriminant.is_some() {
                let e = syn::Error::new_spanned(
                    v.discriminant.clone().unwrap().1,
                    "savvy doesn't support an enum with discreminant",
                );
                return Err(e);
            }

            variants.push(v.ident.clone());
        }

        Ok(Self {
            docs,
            attrs,
            ty,
            variants,
        })
    }

    pub fn generate_enum_with_discriminant(&self) -> ItemEnum {
        let ty = &self.ty;
        let attrs = &self.attrs;

        let variants_tweaked = self
            .variants
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let lit_i = syn::LitInt::new(&i.to_string(), v.span());
                parse_quote!(#v = #lit_i)
            })
            .collect::<Vec<syn::Variant>>();

        parse_quote!(
            #(#attrs)*
            pub enum #ty {
                #(#variants_tweaked),*
            }
        )
    }

    pub fn generate_try_from_impls(&self) -> Vec<ItemImpl> {
        let ty = &self.ty;

        let match_arms_ref = self
            .variants
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let lit_i = syn::LitInt::new(&i.to_string(), v.span());
                parse_quote!(#lit_i => Ok(&#ty::#v))
            })
            .collect::<Vec<syn::Arm>>();

        let match_arms = self
            .variants
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let lit_i = syn::LitInt::new(&i.to_string(), v.span());
                parse_quote!(#lit_i => Ok(#ty::#v))
            })
            .collect::<Vec<syn::Arm>>();

        vec![
            parse_quote!(
            impl TryFrom<#ty> for savvy::Sexp {
                type Error = savvy::Error;

                fn try_from(value: #ty) -> savvy::Result<Self> {
                    (value as i32).try_into()
                }
            }),
            parse_quote!(
                impl TryFrom<savvy::Sexp> for &#ty {
                    type Error = savvy::Error;

                    fn try_from(value: savvy::Sexp) -> savvy::Result<Self> {
                        let i = <i32>::try_from(value)?;
                        match i {
                            #(#match_arms_ref),*,
                            _ => Err("Unexpected enum variant".into()),
                        }
                    }
                }
            ),
            parse_quote!(
                impl TryFrom<savvy::Sexp> for #ty {
                    type Error = savvy::Error;

                    fn try_from(value: savvy::Sexp) -> savvy::Result<Self> {
                        let i = <i32>::try_from(value)?;
                        match i {
                            #(#match_arms),*,
                            _ => Err("Unexpected enum variant".into()),
                        }
                    }
                }
            ),
        ]
    }
}
