use crate::{
    create::{FTCreate, IndexOption},
    query::FTSearch,
    Expr,
};

pub trait Document {
    type Operator;

    fn op() -> Self::Operator;

    fn search(index: &str, expr: Expr) -> crate::Result<FTSearch> {
        expr.ft_search(index)
    }

    fn create_index(index: &str, option: IndexOption) -> FTCreate;
}
