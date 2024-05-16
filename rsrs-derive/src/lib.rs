mod geo_field;
mod number_field;
mod tag_field;
mod text_field;
mod vector_field;

use geo_field::GeoOption;
use number_field::NumberOption;
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse::Parse, parse_macro_input, DeriveInput, Error, Fields, LitStr, Token};
use tag_field::TagOption;
use text_field::TextOption;
use vector_field::{VectorDataType, VectorOption};

#[derive(Debug)]
struct Field {
    pub name: syn::Ident,
    pub attr: FieldAttr,
    pub ty: String,
}

impl Field {
    pub fn as_field_constructor_tokens(&self) -> Vec<proc_macro2::TokenStream> {
        match &self.attr.ty {
            FieldType::Text { option } => option.as_field_constructor_tokens(),
            FieldType::Tag { option } => option.as_field_constructor_tokens(),
            FieldType::Vector { option } => option.as_field_constructor_tokens(),
            FieldType::Geo { option } => option.as_field_constructor_tokens(),
            FieldType::Number { option } => option.as_field_constructor_tokens(),
        }
    }
}

#[derive(Debug)]
enum FieldType {
    Text { option: TextOption },
    Tag { option: TagOption },
    Vector { option: VectorOption },
    Geo { option: GeoOption },
    Number { option: NumberOption },
}

impl Parse for FieldType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lit: LitStr = input.parse()?;
        Ok(match lit.value().to_lowercase().as_str() {
            "text" => FieldType::Text {
                option: TextOption::parse(input)?,
            },
            "vector" => FieldType::Vector {
                option: VectorOption::parse(input)?,
            },
            "tag" => FieldType::Tag {
                option: TagOption::parse(input)?,
            },
            "geo" => FieldType::Geo {
                option: GeoOption::parse(input)?,
            },
            "num" => FieldType::Number {
                option: NumberOption::parse(input)?,
            },
            "number" => FieldType::Number {
                option: NumberOption::parse(input)?,
            },
            unknown => {
                return syn::Result::Err(Error::new(
                    input.span(),
                    format!("unknown type '{}'", unknown),
                ))
            }
        })
    }
}

#[derive(Debug)]
struct FieldAttr {
    pub ty: FieldType,
}

impl Parse for FieldAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: Token![type] = input.parse()?;
        let _: Token![=] = input.parse()?;
        Ok(FieldAttr { ty: input.parse()? })
    }
}

fn get_type(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(path) => path.into_token_stream().to_string(),
        _ => todo!(),
    }
}

fn get_field(ty: &FieldType, field_ty: &str) -> &'static str {
    match ty {
        FieldType::Text { option: _ } => "TextField",
        FieldType::Tag { option: _ } => "TagField",
        FieldType::Vector { option } => match option.datatype.unwrap() {
            VectorDataType::F32 => "VectorFieldF32",
            VectorDataType::F64 => "VectorFieldF32",
        },
        FieldType::Geo { option: _ } => "GeoField",
        FieldType::Number { option: _ } => match field_ty {
            "i8" => "NumberFieldI8",
            "i16" => "NumberFieldI16",
            "i32" => "NumberFieldI32",
            "i64" => "NumberFieldI64",
            "i128" => "NumberFieldI128",
            "isize" => "NumberFieldISIZE",
            "u8" => "NumberFieldU8",
            "u16" => "NumberFieldU16",
            "u32" => "NumberFieldU32",
            "u64" => "NumberFieldU64",
            "u128" => "NumberFieldU128",
            "usize" => "NumberFieldUSIZE",
            "f32" => "NumberFieldF32",
            "f64" => "NumberFieldF64",
            _ => "NumberFieldUSIZE",
        },
    }
}

#[proc_macro_derive(Document, attributes(field))]
pub fn document(_input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(_input as DeriveInput);

    let data = match &input.data {
        syn::Data::Struct(data) => data,
        _ => {
            return TokenStream::from(
                syn::Error::new(input.ident.span(), "Only structs can derive `FromRow`")
                    .to_compile_error(),
            )
        }
    };

    let name = &input.ident;
    let op_name = format_ident!("_{}_op", name);

    let fields = match &data.fields {
        Fields::Named(fields) => fields,
        _ => {
            return TokenStream::from(
                syn::Error::new(input.ident.span(), "Only named fields can derive `FromRow`")
                    .to_compile_error(),
            )
        }
    };

    let mut op_fields: Vec<Field> = vec![];

    for field in fields.named.iter() {
        for attr in &field.attrs {
            if !attr.path.is_ident("field") {
                break;
            }
            let f: FieldAttr = match attr.parse_args() {
                Ok(ok) => ok,
                Err(err) => return TokenStream::from(err.to_compile_error()),
            };
            op_fields.push(Field {
                name: field.ident.clone().unwrap(),
                attr: f,
                ty: get_type(&field.ty),
            });
        }
    }

    let op_struct_fields = op_fields.iter().map(|f| {
        let name = &f.name;
        let ty = format_ident!("{}", get_field(&f.attr.ty, &f.ty));
        quote! {
            pub #name: ::rsrs::#ty
        }
    });

    let op_default_fields = op_fields.iter().map(|f| {
        let name = &f.name;
        let name_str = name.to_string();
        let ty = format_ident!("{}", get_field(&f.attr.ty, &f.ty));
        let options = f.as_field_constructor_tokens();
        quote! {
            #name: ::rsrs::#ty {
                field_name: #name_str,
                #(#options),*
            }
        }
    });

    let schema_fields = op_fields.iter().map(|f| {
        let name = &f.name;
        quote! {
            schema.push(op.#name.to_schema_fields());
        }
    });

    let output = quote! {
        use ::rsrs::*;

        struct #op_name {
            #(#op_struct_fields),*
        }

        impl Default for #op_name {
            fn default() -> Self {
                Self {
                    #(#op_default_fields),*
                }
            }
        }

        impl ::rsrs::Document for #name {
            type Operator = #op_name;

            fn op() -> #op_name {
                Default::default()
            }

            fn create_index(index: &str, option: IndexOption) -> FTCreate {
                let mut schema = vec![];

                let op = Self::op();
                #(#schema_fields)*

                ::rsrs::FTCreate::new (
                    index.to_string(),
                    option,
                    schema,
                )
            }
        }
    };
    output.into()
}
