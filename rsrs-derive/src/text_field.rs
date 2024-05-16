use quote::quote;
use syn::{parse::Parse, Error, LitFloat, LitStr, Token};

#[derive(Default, Debug)]
pub struct TextOption {
    weight: Option<f32>,
    no_stem: Option<bool>,
    phonetic: Option<PhoneticMatcher>,
    sortable: Option<bool>,
    no_index: Option<bool>,
    with_suffix_trie: Option<bool>,
}

impl TextOption {
    pub fn as_field_constructor_tokens(&self) -> Vec<proc_macro2::TokenStream> {
        let mut ret = vec![];
        match self.weight {
            Some(i) => ret.push(quote! {weight: Some(#i)}),
            None => ret.push(quote! {weight: None}),
        }
        match self.no_stem {
            Some(i) => ret.push(quote! {no_stem: Some(#i)}),
            None => ret.push(quote! {no_stem: None}),
        }
        match self.phonetic {
            Some(PhoneticMatcher::DMEN) => {
                ret.push(quote! {phonetic: Some(::rsrs::PhoneticMatcher::DMEN)})
            }
            Some(PhoneticMatcher::DMFR) => {
                ret.push(quote! {phonetic: Some(::rsrs::PhoneticMatcher::DMFR)})
            }
            Some(PhoneticMatcher::DMPT) => {
                ret.push(quote! {phonetic: Some(::rsrs::PhoneticMatcher::DMPT)})
            }
            Some(PhoneticMatcher::DMES) => {
                ret.push(quote! {phonetic: Some(::rsrs::PhoneticMatcher::DMES)})
            }
            None => ret.push(quote! {phonetic: None}),
        }
        match self.sortable {
            Some(i) => ret.push(quote! {sortable: Some(#i)}),
            None => ret.push(quote! {sortable: None}),
        }
        match self.no_index {
            Some(i) => ret.push(quote! {no_index: Some(#i)}),
            None => ret.push(quote! {no_index: None}),
        }
        match self.with_suffix_trie {
            Some(i) => ret.push(quote! {with_suffix_trie: Some(#i)}),
            None => ret.push(quote! {with_suffix_trie: None}),
        }
        ret
    }
}

impl Parse for TextOption {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut option = TextOption::default();
        while input.peek(Token![,]) {
            let _: Token![,] = input.parse()?;
            let ident: syn::Ident = input.parse()?;
            match ident.to_string().to_lowercase().as_str() {
                "weight" => {
                    let _: Token![=] = input.parse()?;
                    let weight: LitFloat = input.parse()?;
                    option.weight = Some(weight.base10_parse()?)
                }
                "no_stem" => option.no_stem = Some(true),
                "sortable" => option.sortable = Some(true),
                "no_index" => option.no_index = Some(true),
                "with_suffix_trie" => option.with_suffix_trie = Some(true),
                "phonetic" => {
                    let _: Token![=] = input.parse()?;
                    let phonetic: LitStr = input.parse()?;
                    option.phonetic = match phonetic.value().to_lowercase().as_str() {
                        "dm:en" => PhoneticMatcher::DMEN,
                        "dm:fr" => PhoneticMatcher::DMFR,
                        "dm:pt" => PhoneticMatcher::DMPT,
                        "dm:es" => PhoneticMatcher::DMES,
                        other => {
                            return syn::Result::Err(Error::new(
                                input.span(),
                                format!("unexpected phonetic: {}", other),
                            ))
                        }
                    }
                    .into()
                }
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

#[derive(Debug, Clone, Copy)]
pub enum PhoneticMatcher {
    DMEN,
    DMFR,
    DMPT,
    DMES,
}
