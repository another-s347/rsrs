use bytes::Bytes;
use redis::ToRedisArgs;

pub struct FTSearch {
    pub query: String,
    pub index: String,
    pub option: FTSearchOption,
}

impl ToRedisArgs for FTSearch {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        self.index.write_redis_args(out);
        self.query.write_redis_args(out);
        self.option.write_redis_args(out);
    }
}

pub struct FTSearchOption {
    pub returns: Option<Vec<FTSearchReturn>>,
    pub params: Option<Vec<FTSearchParam>>,
    pub sortby: Option<SortBy>,
    pub dialect: Option<usize>,
}

impl ToRedisArgs for FTSearchOption {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        if let Some(returns) = &self.returns {
            "RETURN".write_redis_args(out);
            returns.len().write_redis_args(out);
            returns.write_redis_args(out);
        }
        match &self.params {
            Some(params) if params.len() > 0 => {
                "PARAMS".write_redis_args(out);
                (params.len() * 2).write_redis_args(out);
                params.write_redis_args(out);
            }
            _ => {}
        }
        if let Some(sort_by) = &self.sortby {
            "SORTBY".write_redis_args(out);
            sort_by.write_redis_args(out);
        }
        if let Some(dialect) = self.dialect {
            "DIALECT".write_redis_args(out);
            dialect.write_redis_args(out);
        }
    }
}

pub struct SortBy {
    pub attribute: &'static str,
    pub asc: bool,
    pub with_count: Option<usize>,
}

impl ToRedisArgs for SortBy {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        self.attribute.write_redis_args(out);
        (if self.asc { "ASC" } else { "DESC" }).write_redis_args(out);
        if let Some(with_count) = self.with_count {
            with_count.write_redis_args(out);
        }
    }
}

pub struct FTSearchReturn {
    pub identifier: &'static str,
    pub property: &'static str,
}

impl ToRedisArgs for FTSearchReturn {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        self.identifier.write_redis_args(out);
        // "AS".write_redis_args(out);
        // self.property.write_redis_args(out);
    }
}

pub struct FTSearchParam {
    pub name: String,
    pub value: Bytes,
}

impl ToRedisArgs for FTSearchParam {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        self.name.write_redis_args(out);
        self.value.as_ref().write_redis_args(out);
    }
}
