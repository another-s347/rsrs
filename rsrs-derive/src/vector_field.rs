use quote::quote;
use syn::{parse::Parse, Error, LitInt, Token};

#[derive(Debug, Clone, Copy)]
enum VectorAlgorithm {
    FLAT,
    HNSW,
}

#[derive(Debug, Clone, Copy)]
pub enum VectorDataType {
    F32,
    F64,
}

#[derive(Debug, Clone, Copy)]
enum DistanceMetric {
    L2,
    IP,
    COSINE,
}

#[derive(Default, Debug)]
pub struct VectorOption {
    pub datatype: Option<VectorDataType>,
    algorithm: Option<VectorAlgorithm>,
    dim: Option<usize>,
    distance_metric: Option<DistanceMetric>,
    initial_cap: Option<usize>,
    block_size: Option<usize>,
    m: Option<usize>,
    ef_construction: Option<usize>,
    ef_runtime: Option<usize>,
    epsilon: Option<usize>,
}

impl VectorOption {
    pub fn as_field_constructor_tokens(&self) -> Vec<proc_macro2::TokenStream> {
        let mut ret = vec![];
        if let Some(dim) = self.dim {
            ret.push(quote! {dim: #dim})
        }
        match self.algorithm.unwrap() {
            VectorAlgorithm::FLAT => ret.push(quote! {algorithm: ::rsrs::VectorAlgorithm::FLAT}),
            VectorAlgorithm::HNSW => ret.push(quote! {algorithm: ::rsrs::VectorAlgorithm::HNSW}),
        }
        match self.distance_metric.unwrap() {
            DistanceMetric::COSINE => {
                ret.push(quote! {distance_metric: ::rsrs::DistanceMetric::COSINE})
            }
            DistanceMetric::IP => ret.push(quote! {distance_metric: ::rsrs::DistanceMetric::IP}),
            DistanceMetric::L2 => ret.push(quote! {distance_metric: ::rsrs::DistanceMetric::L2}),
        }
        match self.initial_cap {
            Some(i) => ret.push(quote! {initial_cap: Some(#i)}),
            None => ret.push(quote! {initial_cap: None}),
        }
        match self.block_size {
            Some(i) => ret.push(quote! {block_size: Some(#i)}),
            None => ret.push(quote! {block_size: None}),
        }
        match self.m {
            Some(i) => ret.push(quote! {m: Some(#i)}),
            None => ret.push(quote! {m: None}),
        }
        match self.ef_construction {
            Some(i) => ret.push(quote! {ef_construction: Some(#i)}),
            None => ret.push(quote! {ef_construction: None}),
        }
        match self.ef_runtime {
            Some(i) => ret.push(quote! {ef_runtime: Some(#i)}),
            None => ret.push(quote! {ef_runtime: None}),
        }
        match self.epsilon {
            Some(i) => ret.push(quote! {epsilon: Some(#i)}),
            None => ret.push(quote! {epsilon: None}),
        }
        ret
    }
}

impl Parse for VectorOption {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut option = VectorOption::default();
        while input.peek(Token![,]) {
            let _: Token![,] = input.parse()?;
            let ident: syn::Ident = input.parse()?;
            match ident.to_string().to_lowercase().as_str() {
                "dim" => {
                    let _: Token![=] = input.parse()?;
                    let dim: LitInt = input.parse()?;
                    option.dim = Some(dim.base10_parse()?)
                }
                "m" => {
                    let _: Token![=] = input.parse()?;
                    let m: LitInt = input.parse()?;
                    option.m = Some(m.base10_parse()?)
                }
                "ef_construction" => {
                    let _: Token![=] = input.parse()?;
                    let ef_construction: LitInt = input.parse()?;
                    option.ef_construction = Some(ef_construction.base10_parse()?)
                }
                "ef_runtime" => {
                    let _: Token![=] = input.parse()?;
                    let ef_runtime: LitInt = input.parse()?;
                    option.ef_runtime = Some(ef_runtime.base10_parse()?)
                }
                "epsilon" => {
                    let _: Token![=] = input.parse()?;
                    let epsilon: LitInt = input.parse()?;
                    option.epsilon = Some(epsilon.base10_parse()?)
                }
                "flat" => option.algorithm = Some(VectorAlgorithm::FLAT),
                "hnsw" => option.algorithm = Some(VectorAlgorithm::HNSW),
                "f64" => option.datatype = Some(VectorDataType::F64),
                "f32" => option.datatype = Some(VectorDataType::F32),
                "distance_metric" => {
                    let _: Token![=] = input.parse()?;
                    let distance_metric: syn::Ident = input.parse()?;
                    option.distance_metric =
                        match distance_metric.to_string().to_lowercase().as_str() {
                            "l2" => DistanceMetric::L2,
                            "ip" => DistanceMetric::IP,
                            "cosine" => DistanceMetric::COSINE,
                            other => {
                                return syn::Result::Err(Error::new(
                                    input.span(),
                                    format!("unexpected distance metric {}", other),
                                ))
                            }
                        }
                        .into()
                }
                "initial_cap" => {
                    let _: Token![=] = input.parse()?;
                    let inital_cap: LitInt = input.parse()?;
                    option.initial_cap = Some(inital_cap.base10_parse()?)
                }
                "block_size" => {
                    let _: Token![=] = input.parse()?;
                    let block_size: LitInt = input.parse()?;
                    option.block_size = Some(block_size.base10_parse()?)
                }
                other => {
                    return syn::Result::Err(Error::new(
                        input.span(),
                        format!("unexpected attribute {}", other),
                    ))
                }
            }
        }
        if option.algorithm.is_none() {
            return syn::Result::Err(Error::new(
                input.span(),
                format!("missing attribute: hnsw or flat"),
            ));
        }
        if option.datatype.is_none() {
            return syn::Result::Err(Error::new(
                input.span(),
                format!("missing attribute: f32 or f64"),
            ));
        }
        if option.distance_metric.is_none() {
            return syn::Result::Err(Error::new(
                input.span(),
                format!("missing attribute: distance_metric"),
            ));
        }

        Ok(option)
    }
}
