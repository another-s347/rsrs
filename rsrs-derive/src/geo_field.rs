use quote::quote;
use syn::{parse::Parse, Error, Token};

#[derive(Default, Debug)]
pub struct GeoOption {
    sortable: Option<bool>,
    no_index: Option<bool>,
}

impl GeoOption {
    pub fn as_field_constructor_tokens(&self) -> Vec<proc_macro2::TokenStream> {
        let mut ret = vec![];
        match self.sortable {
            Some(i) => ret.push(quote! {sortable: Some(#i)}),
            None => ret.push(quote! {sortable: None}),
        }
        match self.no_index {
            Some(i) => ret.push(quote! {no_index: Some(#i)}),
            None => ret.push(quote! {no_index: None}),
        }
        ret
    }
}

impl Parse for GeoOption {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut option = GeoOption::default();
        while input.peek(Token![,]) {
            let _: Token![,] = input.parse()?;
            let ident: syn::Ident = input.parse()?;
            match ident.to_string().to_lowercase().as_str() {
                "sortable" => option.sortable = Some(true),
                "no_index" => option.no_index = Some(true),
                other => {
                    return syn::Result::Err(Error::new(
                        input.span(),
                        format!("unexpected attribute {}", other),
                    ))
                }
            }
        }

        Ok(option)
    }
}
