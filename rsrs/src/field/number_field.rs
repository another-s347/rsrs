use std::ops::RangeBounds;

use crate::{Expr, Field, SchemaField};

pub trait NumberField: Field {
    type Number;

    fn in_range<T: RangeBounds<Self::Number>>(&self, range: T) -> Expr;

    fn eq(&self, number: Self::Number) -> Expr;

    fn ne(&self, number: Self::Number) -> Expr;
}

macro_rules! impl_number_field {
    ($ty:ty, $name:ident) => {
        pub struct $name {
            pub field_name: &'static str,
            pub sortable: bool,
            pub unf: bool,
            pub no_index: bool,
        }

        impl Field for $name {
            fn field_name(&self) -> &'static str {
                self.field_name
            }

            fn to_schema_fields(&self) -> SchemaField {
                SchemaField {
                    identifier: self.field_name.to_string(),
                    attribute: None,
                    field_type: "NUMERIC",
                    options: crate::create::FieldOption {
                        sortable: Some(self.sortable),
                        unf: Some(self.unf),
                        noindex: Some(self.no_index),
                        ..Default::default()
                    },
                }
            }
        }

        impl NumberField for $name {
            type Number = $ty;

            fn in_range<T: RangeBounds<Self::Number>>(&self, range: T) -> Expr {
                let start = match range.start_bound() {
                    core::ops::Bound::Included(start) => format!("{}", start),
                    core::ops::Bound::Excluded(start) => format!("({}", start),
                    core::ops::Bound::Unbounded => "-inf".to_string(),
                };

                let end = match range.end_bound() {
                    core::ops::Bound::Included(end) => format!("{}", end),
                    core::ops::Bound::Excluded(end) => format!("({}", end),
                    core::ops::Bound::Unbounded => "+inf".to_string(),
                };

                Expr {
                    filter: format!("@{}:[{} {}]", self.field_name(), start, end),
                    ..Default::default()
                }
            }

            fn eq(&self, number: Self::Number) -> Expr {
                Expr {
                    filter: format!("@{}:[{} {}]", self.field_name(), number, number),
                    ..Default::default()
                }
            }

            fn ne(&self, number: Self::Number) -> Expr {
                return Expr {
                    filter: format!("-@{}:[{} {}]", self.field_name(), number, number),
                    ..Default::default()
                };
            }
        }
    };
}

impl_number_field!(usize, NumberFieldUSIZE);
impl_number_field!(u8, NumberFieldU8);
impl_number_field!(u16, NumberFieldU16);
impl_number_field!(u32, NumberFieldU32);
impl_number_field!(u64, NumberFieldU64);
impl_number_field!(u128, NumberFieldU128);
impl_number_field!(isize, NumberFieldISIZE);
impl_number_field!(i8, NumberFieldI8);
impl_number_field!(i16, NumberFieldI16);
impl_number_field!(i32, NumberFieldI32);
impl_number_field!(i64, NumberFieldI64);
impl_number_field!(i128, NumberFieldI128);
impl_number_field!(f32, NumberFieldF32);
impl_number_field!(f64, NumberFieldF64);
