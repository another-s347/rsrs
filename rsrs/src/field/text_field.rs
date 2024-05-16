use itertools::Itertools;

use crate::{Expr, Field, SchemaField};

pub struct TextField {
    pub field_name: &'static str,
    pub weight: Option<f32>,
    pub no_stem: Option<bool>,
    pub phonetic: Option<PhoneticMatcher>,
    pub sortable: Option<bool>,
    pub no_index: Option<bool>,
    pub with_suffix_trie: Option<bool>,
}

impl Field for TextField {
    fn field_name(&self) -> &'static str {
        self.field_name
    }

    fn to_schema_fields(&self) -> SchemaField {
        let mut options = vec![];
        if let Some(weight) = self.weight {
            options.push("WEIGHT".to_owned());
            options.push(format!("{}", weight));
        }
        SchemaField {
            identifier: self.field_name.to_string(),
            attribute: None,
            field_type: "TEXT",
            options: crate::create::FieldOption {
                weight: self.weight,
                nostem: self.no_stem,
                phonetic: self.phonetic,
                sortable: self.sortable,
                noindex: self.no_index,
                // with_suffix_trie: self.with_suffix_trie,
                ..Default::default()
            },
        }
    }
}

impl TextField {
    pub fn eq<T: AsRef<str>>(&self, value: T) -> Expr {
        self.contains(&[value])
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

    pub fn not_contains<T: AsRef<str>>(&self, values: &[T]) -> Expr {
        Expr {
            filter: format!(
                "-@{}:{{ {} }}",
                self.field_name,
                values.iter().map(|x| x.as_ref()).join(" | ")
            ),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PhoneticMatcher {
    DMEN,
    DMFR,
    DMPT,
    DMES,
}

impl PhoneticMatcher {
    pub fn as_str(&self) -> &str {
        match self {
            PhoneticMatcher::DMEN => "dm:en",
            PhoneticMatcher::DMFR => "dm:fr",
            PhoneticMatcher::DMPT => "dm:pt",
            PhoneticMatcher::DMES => "dm:es",
        }
    }
}
