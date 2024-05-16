use quote::quote;
use syn::{parse::Parse, Error, Token};

#[derive(Default, Debug)]
pub struct NumberOption {
    sortable: Option<bool>,
    unf: Option<bool>,
    no_index: Option<bool>,
}

impl NumberOption {
    pub fn as_field_constructor_tokens(&self) -> Vec<proc_macro2::TokenStream> {
        let mut ret = vec![];
        let sortable = self.sortable.unwrap_or_default();
        ret.push(quote! {sortable: #sortable});
        let unf = self.unf.unwrap_or_default();
        ret.push(quote! {unf: #unf});
        let no_index = self.no_index.unwrap_or_default();
        ret.push(quote! {no_index: #no_index});
        ret
    }
}

impl Parse for NumberOption {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut option = NumberOption::default();
        while input.peek(Token![,]) {
            let _: Token![,] = input.parse()?;
            let ident: syn::Ident = input.parse()?;
            match ident.to_string().as_str() {
                "sortable" => option.sortable = Some(true),
                "unf" => option.unf = Some(true),
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
