use syn::parse_quote;

use crate::extract_docs;

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
                let e = syn::Error::new_spanned(v, "savvy only supports a fieldless enum");
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
}
