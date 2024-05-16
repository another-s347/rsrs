use std::collections::HashMap;

use bytes::Bytes;
use redis::ToRedisArgs;

use crate::{Expr, Field, SchemaField};

pub trait VectorField: Field {
    type Number;

    fn query(&self, embedding: &[Self::Number], topk: usize, param_name: Option<&str>) -> Expr;
}

#[derive(Debug, Clone, Copy)]
pub enum VectorAlgorithm {
    FLAT,
    HNSW,
}

impl ToRedisArgs for VectorAlgorithm {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        match self {
            Self::FLAT => "FLAT".write_redis_args(out),
            Self::HNSW => "HNSW".write_redis_args(out),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DistanceMetric {
    L2,
    IP,
    COSINE,
}

impl ToRedisArgs for DistanceMetric {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        match self {
            Self::L2 => "L2".write_redis_args(out),
            Self::IP => "IP".write_redis_args(out),
            Self::COSINE => "COSINE".write_redis_args(out),
        }
    }
}

macro_rules! impl_vector_field {
    ($ty:ty, $name:ident, $vector_type:ident) => {
        pub struct $name {
            pub field_name: &'static str,
            pub dim: usize,
            pub algorithm: VectorAlgorithm,
            pub distance_metric: DistanceMetric,
            pub initial_cap: Option<usize>,
            pub block_size: Option<usize>,
            pub m: Option<usize>,
            pub ef_construction: Option<usize>,
            pub ef_runtime: Option<usize>,
            pub epsilon: Option<usize>,
        }

        impl Field for $name {
            fn field_name(&self) -> &'static str {
                self.field_name
            }

            fn to_schema_fields(&self) -> SchemaField {
                let count = 6 + 2
                    * (self.initial_cap.is_some() as u8
                        + self.block_size.is_some() as u8
                        + self.m.is_some() as u8
                        + self.ef_construction.is_some() as u8
                        + self.ef_runtime.is_some() as u8
                        + self.epsilon.is_some() as u8);
                SchemaField {
                    identifier: self.field_name.to_string(),
                    attribute: None,
                    field_type: "VECTOR",
                    options: crate::create::FieldOption {
                        dim: Some(self.dim),
                        algorithm: Some(self.algorithm),
                        count: Some(count as usize),
                        distance_metric: Some(self.distance_metric),
                        initial_cap: self.initial_cap,
                        vector_type: Some(VectorType::$vector_type),
                        block_size: self.block_size,
                        m: self.m,
                        ef_construction: self.ef_construction,
                        ef_runtime: self.ef_runtime,
                        epsilon: self.epsilon,
                        ..Default::default()
                    },
                }
            }
        }

        impl VectorField for $name {
            type Number = $ty;

            fn query(
                &self,
                embedding: &[Self::Number],
                topk: usize,
                param_name: Option<&str>,
            ) -> Expr {
                let mut params = HashMap::new();

                let embeddingbytes: &[u8] = bytemuck::cast_slice(embedding);
                let param = format!("${}", param_name.unwrap_or("vec"));
                params.insert(param.clone(), Bytes::copy_from_slice(embeddingbytes));

                Expr {
                    knn_query: Some(format!("[KNN {} @{} {}]", topk, self.field_name, param)),
                    params,
                    ..Default::default()
                }
            }
        }
    };
}

impl_vector_field!(f32, VectorFieldF32, Float32);
impl_vector_field!(f64, VectorFieldF64, Float64);

#[derive(Debug)]
pub enum VectorType {
    Float32,
    Float64,
}

impl ToRedisArgs for VectorType {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        match self {
            Self::Float32 => "FLOAT32".write_redis_args(out),
            Self::Float64 => "FLOAT64".write_redis_args(out),
        }
    }
}
