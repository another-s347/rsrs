mod number_field;
mod text_field;
mod vector_field;

pub use number_field::*;
pub use text_field::*;
pub use vector_field::*;

use std::collections::HashMap;

use bytes::Bytes;
use itertools::Itertools;

use crate::{
    query::{FTSearch, FTSearchOption, FTSearchParam},
    SchemaField,
};

#[derive(Debug)]
pub enum SortOrder {
    ASC,
    DESC,
}

#[derive(Default, Debug)]
pub struct Expr {
    filter: String,
    knn_query: Option<String>,
    params: HashMap<String, Bytes>,
    error: Option<crate::Error>,
    dialect: Option<usize>,
    sort_by: Option<(&'static str, SortOrder)>,
}

impl Expr {
    pub fn ft_search(&self, index: &str) -> crate::Result<FTSearch> {
        if let Some(err) = self.error {
            return Err(err);
        }

        let query = match (&self.knn_query, self.filter.len()) {
            (Some(knn_query), 0) => {
                format!("*=>{}", knn_query)
            }
            (Some(knn_query), u) if u > 0 => {
                format!("({})=>{}", self.filter, knn_query)
            }
            (None, u) if u > 0 => self.filter.clone(),
            _ => return Err(crate::Error::EmptyQueryBody),
        };

        Ok(FTSearch {
            query,
            index: index.to_string(),
            option: FTSearchOption {
                returns: None,
                params: Some(
                    self.params
                        .iter()
                        .map(|(k, v)| FTSearchParam {
                            name: k.clone(),
                            value: v.clone(),
                        })
                        .collect_vec(),
                ),
                sortby: None,
                dialect: self.dialect,
            },
        })
    }

    pub fn sort_by<F: Field>(self, field: F, order: SortOrder) -> Expr {
        Expr {
            sort_by: Some((field.field_name(), order)),
            ..self
        }
    }

    pub fn and(mut self, other: Expr) -> Expr {
        if self.error.is_some() {
            return Expr {
                error: self.error,
                ..Default::default()
            };
        }
        if other.error.is_some() {
            return Expr {
                error: other.error,
                ..Default::default()
            };
        }

        if self.knn_query.is_some() && other.knn_query.is_some() {
            return Expr {
                error: Some(crate::Error::DuplicatedVectorQuery),
                ..Default::default()
            };
        }

        let count_params = self.params.len() + other.params.len();

        self.params.extend(other.params);

        if self.params.len() < count_params {
            return Expr {
                error: Some(crate::Error::DuplicatedParam),
                ..Default::default()
            };
        }

        let filter = match (self.filter.len(), other.filter.len()) {
            (0, 0) => String::new(),
            (0, x) if x > 0 => other.filter,
            (x, 0) if x > 0 => self.filter,
            (_, _) => format!("{} {}", self.filter, other.filter),
        };

        Expr {
            filter,
            knn_query: self.knn_query.or(other.knn_query),
            params: self.params,
            error: None,
            ..Default::default()
        }
    }

    pub fn or(mut self, other: Expr) -> Expr {
        if self.error.is_some() {
            return Expr {
                error: self.error,
                ..Default::default()
            };
        }
        if other.error.is_some() {
            return Expr {
                error: other.error,
                ..Default::default()
            };
        }

        if self.knn_query.is_some() && other.knn_query.is_some() {
            return Expr {
                error: Some(crate::Error::DuplicatedVectorQuery),
                ..Default::default()
            };
        }

        let count_params = self.params.len() + other.params.len();

        self.params.extend(other.params);

        if self.params.len() < count_params {
            return Expr {
                error: Some(crate::Error::DuplicatedParam),
                ..Default::default()
            };
        }

        let filter = match (self.filter.len(), other.filter.len()) {
            (0, 0) => String::new(),
            (0, x) if x > 0 => other.filter,
            (x, 0) if x > 0 => self.filter,
            (_, _) => format!("({})|({})", self.filter, other.filter),
        };

        Expr {
            filter,
            knn_query: self.knn_query.or(other.knn_query),
            params: self.params,
            error: None,
            ..Default::default()
        }
    }

    pub fn dialect(self, dialect: usize) -> Expr {
        Expr {
            dialect: Some(dialect),
            ..self
        }
    }
}

pub trait Field {
    fn field_name(&self) -> &'static str;

    fn to_schema_fields(&self) -> SchemaField;
}

pub struct GeoField {
    pub field_name: &'static str,
}

impl Field for GeoField {
    fn field_name(&self) -> &'static str {
        self.field_name
    }

    fn to_schema_fields(&self) -> SchemaField {
        SchemaField {
            identifier: self.field_name.to_string(),
            attribute: None,
            field_type: "GEO",
            options: Default::default(),
        }
    }
}

impl GeoField {
    pub fn new(name: &'static str) -> Self {
        Self { field_name: name }
    }

    pub fn query(&self, lon: f32, lat: f32, radius: usize, unit: &str) -> Expr {
        Expr {
            filter: format!("@{}:[{} {} {} {}]", self.field_name, lon, lat, radius, unit),
            ..Default::default()
        }
    }
}

pub struct TagField {
    pub field_name: &'static str,
}

impl Field for TagField {
    fn field_name(&self) -> &'static str {
        self.field_name
    }

    fn to_schema_fields(&self) -> SchemaField {
        SchemaField {
            identifier: self.field_name.to_string(),
            attribute: None,
            field_type: "TAG",
            options: Default::default(),
        }
    }
}

impl TagField {
    pub fn new(name: &'static str) -> Self {
        Self { field_name: name }
    }

    pub fn contains<T: AsRef<str>>(&self, values: &[T]) -> Expr {
        Expr {
            filter: format!(
                "@{}:{{ {} }}",
                self.field_name,
                values.iter().map(|x| x.as_ref()).join(" | ")
            ),
            ..Default::default()
        }
    }
}
