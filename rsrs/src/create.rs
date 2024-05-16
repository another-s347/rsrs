use redis::ToRedisArgs;

#[derive(Debug)]
pub struct FTCreate {
    pub name: String,
    pub option: IndexOption,
    pub schema: Vec<SchemaField>,
}

impl FTCreate {
    pub fn new(name: String, option: IndexOption, mut schema: Vec<SchemaField>) -> FTCreate {
        if let Some(DataType::Json) = option.on {
            for field in schema.as_mut_slice() {
                field.mut_as_default_json_field();
            }
        }

        FTCreate {
            name,
            option,
            schema,
        }
    }
}

#[derive(Debug)]
pub struct SchemaField {
    pub identifier: String,
    pub attribute: Option<String>,
    pub field_type: &'static str,
    pub options: FieldOption,
}

impl SchemaField {
    pub fn mut_as_default_json_field(&mut self) {
        if self.attribute.is_some() {
            return;
        }

        let attribute = std::mem::replace(&mut self.identifier, String::new());
        self.identifier = format!("$.{}", attribute);
        self.attribute = Some(attribute);
    }
}

impl redis::ToRedisArgs for FTCreate {
    fn is_single_arg(&self) -> bool {
        false
    }

    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        out.write_arg(self.name.as_bytes());
        self.option.write_redis_args(out);
        out.write_arg("SCHEMA".as_bytes());
        for schema in &self.schema {
            schema.write_redis_args(out);
        }
    }
}

impl redis::ToRedisArgs for SchemaField {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        self.identifier.write_redis_args(out);
        if let Some(attribute) = &self.attribute {
            "AS".write_redis_args(out);
            attribute.write_redis_args(out);
        }
        self.field_type.write_redis_args(out);
        self.options.write_redis_args(out);
    }
}

#[derive(Default, Debug)]
pub struct IndexOption {
    pub on: Option<DataType>,
    pub prefix: Option<Vec<String>>,
    pub filter: Option<String>,
    pub language: Option<&'static str>,
    pub score: Option<f32>,
}

impl ToRedisArgs for IndexOption {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        if let Some(data_type) = &self.on {
            out.write_arg("ON".as_bytes());
            out.write_arg(data_type.as_str().as_bytes());
        }
        if let Some(prefix) = &self.prefix {
            out.write_arg("PREFIX".as_bytes());
            out.write_arg_fmt(prefix.len());
            for p in prefix {
                out.write_arg(p.as_bytes());
            }
        }
        if let Some(filter) = &self.filter {
            out.write_arg("FILTER".as_bytes());
            out.write_arg(filter.as_bytes());
        }
        if let Some(language) = &self.language {
            out.write_arg("LANGUAGE".as_bytes());
            out.write_arg(language.as_bytes());
        }
        if let Some(score) = &self.score {
            out.write_arg("SCORE".as_bytes());
            out.write_arg_fmt(score);
        }
    }
}

#[derive(Debug)]
pub enum DataType {
    Hash,
    Json,
}

impl DataType {
    pub fn as_str(&self) -> &str {
        match self {
            DataType::Hash => "HASH",
            DataType::Json => "JSON",
        }
    }
}

pub struct Prefix {
    pub prefixs: Vec<&'static str>,
}

#[derive(Default, Debug)]
pub struct FieldOption {
    pub sortable: Option<bool>,
    pub unf: Option<bool>,
    pub nostem: Option<bool>,
    pub noindex: Option<bool>,
    pub phonetic: Option<crate::field::PhoneticMatcher>,
    pub weight: Option<f32>,
    pub separator: Option<&'static str>,
    pub casesensitive: Option<bool>,
    pub withsuffixtrie: Option<bool>,
    // vector options
    pub algorithm: Option<crate::field::VectorAlgorithm>,
    pub count: Option<usize>,
    pub vector_type: Option<crate::field::VectorType>,
    pub dim: Option<usize>,
    pub distance_metric: Option<crate::field::DistanceMetric>,
    pub initial_cap: Option<usize>,
    pub block_size: Option<usize>,
    pub m: Option<usize>,
    pub ef_construction: Option<usize>,
    pub ef_runtime: Option<usize>,
    pub epsilon: Option<usize>,
}

impl ToRedisArgs for FieldOption {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        if self.sortable.unwrap_or_default() {
            out.write_arg("SORTABLE".as_bytes());
        }
        if self.unf.unwrap_or_default() {
            out.write_arg("UNF".as_bytes());
        }
        if self.nostem.unwrap_or_default() {
            out.write_arg("NOSTEM".as_bytes());
        }
        if self.noindex.unwrap_or_default() {
            out.write_arg("NOINDEX".as_bytes());
        }
        if let Some(p) = &self.phonetic {
            out.write_arg("PHONETIC".as_bytes());
            out.write_arg(p.as_str().as_bytes());
        }
        if let Some(weight) = self.weight {
            out.write_arg("WEIGHT".as_bytes());
            out.write_arg_fmt(weight);
        }
        if let Some(sep) = &self.separator {
            out.write_arg("SEPARATOR".as_bytes());
            out.write_arg(sep.as_bytes());
        }
        if self.casesensitive.unwrap_or_default() {
            out.write_arg("CASESENSITIVE".as_bytes());
        }
        if self.withsuffixtrie.unwrap_or_default() {
            out.write_arg("WITHSUFFIXTRIE".as_bytes());
        }
        if let Some(algorithm) = &self.algorithm {
            algorithm.write_redis_args(out);
        }
        if let Some(count) = self.count {
            out.write_arg_fmt(count);
        }
        if let Some(vector_type) = &self.vector_type {
            out.write_arg("TYPE".as_bytes());
            vector_type.write_redis_args(out);
        }
        if let Some(dim) = self.dim {
            out.write_arg("DIM".as_bytes());
            out.write_arg_fmt(dim);
        }
        if let Some(distance_metric) = &self.distance_metric {
            out.write_arg("DISTANCE_METRIC".as_bytes());
            distance_metric.write_redis_args(out);
        }
        if let Some(initial_cap) = self.initial_cap {
            out.write_arg("INITIAL_CAP".as_bytes());
            out.write_arg_fmt(initial_cap);
        }
        if let Some(block_size) = self.block_size {
            out.write_arg("BLOCK_SIZE".as_bytes());
            out.write_arg_fmt(block_size);
        }
        if let Some(m) = self.m {
            out.write_arg("M".as_bytes());
            out.write_arg_fmt(m);
        }
        if let Some(ef_construction) = self.ef_construction {
            out.write_arg("EF_CONSTRUCTION".as_bytes());
            out.write_arg_fmt(ef_construction);
        }
        if let Some(ef_runtime) = self.ef_runtime {
            out.write_arg("EF_RUNTIME".as_bytes());
            out.write_arg_fmt(ef_runtime);
        }
        if let Some(epsilon) = self.epsilon {
            out.write_arg("EPSILON".as_bytes());
            out.write_arg_fmt(epsilon);
        }
    }
}
