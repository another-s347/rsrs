use quote::quote;
use syn::{parse::Parse, Error, LitStr, Token};

#[derive(Default, Debug)]
pub struct TagOption {
    separator: Option<String>,
    case_sensitive: Option<bool>,
}

impl TagOption {
    pub fn as_field_constructor_tokens(&self) -> Vec<proc_macro2::TokenStream> {
        let mut ret = vec![];
        match &self.separator {
            Some(i) => ret.push(quote! {separator: Some(#i)}),
            None => ret.push(quote! {separator: None}),
        }
        match self.case_sensitive {
            Some(i) => ret.push(quote! {case_sensitive: Some(#i)}),
            None => ret.push(quote! {case_sensitive: None}),
        }
        ret
    }
}

impl Parse for TagOption {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut option = TagOption::default();
        while input.peek(Token![,]) {
            let _: Token![,] = input.parse()?;
            let ident: syn::Ident = input.parse()?;
            match ident.to_string().to_lowercase().as_str() {
                "separator" => {
                    let _: Token![=] = input.parse()?;
                    let weight: LitStr = input.parse()?;
                    option.separator = Some(weight.value())
                }
                "case_sensitive" => option.case_sensitive = Some(true),
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
