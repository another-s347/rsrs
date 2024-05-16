use itertools::Itertools;
use redis::ToRedisArgs;
use rsrs::DataType;
use rsrs::Document;
use rsrs::IndexOption;

#[derive(Document)]
struct Demo {
    #[field(type = "text", sortable, no_index)]
    description: String,
    #[field(type = "vector", flat, f32, dim = 768, distance_metric=l2)]
    vector32: Vec<f32>,
    #[field(type = "vector", flat, f64, dim = 768, distance_metric=l2)]
    vector64: Vec<f64>,
    #[field(type = "number")]
    a1: i8,
    #[field(type = "number")]
    a2: i16,
    #[field(type = "number")]
    a3: i32,
    #[field(type = "number")]
    a4: i64,
    #[field(type = "number")]
    a5: i128,
    #[field(type = "number")]
    a6: isize,
    #[field(type = "number")]
    a7: u8,
    #[field(type = "number")]
    a8: u16,
    #[field(type = "number")]
    a9: u32,
    #[field(type = "number")]
    a10: u64,
    #[field(type = "number", no_index)]
    a11: u128,
    #[field(type = "number", sortable)]
    a12: usize,
    #[field(type = "number")]
    a13: f32,
    #[field(type = "number")]
    a14: f64,
}

fn to_redis_string<T: ToRedisArgs>(t: &T) -> String {
    let mut args = t.to_redis_args();
    let x = args
        .drain(0..)
        .map(|x| {
            let len = x.len();
            match String::from_utf8(x) {
                Ok(s) => s,
                Err(_) => format!("<{}Bytes>", len),
            }
        })
        .join(" ");
    x
}

#[test]
fn test_create() {
    let ftcreate = Demo::create_index(
        "my_index",
        IndexOption {
            on: DataType::Json.into(),
            ..Default::default()
        },
    );
    assert_eq!(to_redis_string(&ftcreate).as_str(), "my_index ON JSON SCHEMA $.description AS description TEXT SORTABLE NOINDEX $.vector32 AS vector32 VECTOR FLAT 6 TYPE FLOAT32 DIM 768 DISTANCE_METRIC L2 $.vector64 AS vector64 VECTOR FLAT 6 TYPE FLOAT32 DIM 768 DISTANCE_METRIC L2 $.a1 AS a1 NUMERIC $.a2 AS a2 NUMERIC $.a3 AS a3 NUMERIC $.a4 AS a4 NUMERIC $.a5 AS a5 NUMERIC $.a6 AS a6 NUMERIC $.a7 AS a7 NUMERIC $.a8 AS a8 NUMERIC $.a9 AS a9 NUMERIC $.a10 AS a10 NUMERIC $.a11 AS a11 NUMERIC NOINDEX $.a12 AS a12 NUMERIC SORTABLE $.a13 AS a13 NUMERIC $.a14 AS a14 NUMERIC");
}

#[test]
fn test_numeric_query() {
    let op = Demo::op();
    assert_eq!(
        to_redis_string(&op.a14.eq(1.5).ft_search("my_index").unwrap()).as_str(),
        "my_index @a14:[1.5 1.5]"
    );
    assert_eq!(
        to_redis_string(&op.a14.ne(1.5).ft_search("my_index").unwrap()).as_str(),
        "my_index -@a14:[1.5 1.5]"
    );
    assert_eq!(
        to_redis_string(&op.a14.in_range(1.5..).ft_search("my_index").unwrap()).as_str(),
        "my_index @a14:[1.5 +inf]"
    );
    assert_eq!(
        to_redis_string(&op.a14.in_range(1.5..).ft_search("my_index").unwrap()).as_str(),
        "my_index @a14:[1.5 +inf]"
    );
    assert_eq!(
        to_redis_string(&op.a14.in_range(..1.5).ft_search("my_index").unwrap()).as_str(),
        "my_index @a14:[-inf (1.5]"
    );
    assert_eq!(
        to_redis_string(&op.a14.in_range(..=1.5).ft_search("my_index").unwrap()).as_str(),
        "my_index @a14:[-inf 1.5]"
    );
    assert_eq!(
        to_redis_string(&op.a14.in_range(1. ..=1.5).ft_search("my_index").unwrap()).as_str(),
        "my_index @a14:[1 1.5]"
    );
    assert_eq!(
        to_redis_string(&op.a14.in_range(1. ..1.5).ft_search("my_index").unwrap()).as_str(),
        "my_index @a14:[1 (1.5]"
    );

    assert_eq!(
        to_redis_string(&op.a1.eq(1).ft_search("my_index").unwrap()).as_str(),
        "my_index @a1:[1 1]"
    );
    assert_eq!(
        to_redis_string(&op.a1.ne(1).ft_search("my_index").unwrap()).as_str(),
        "my_index -@a1:[1 1]"
    );
    assert_eq!(
        to_redis_string(&op.a1.in_range(1..).ft_search("my_index").unwrap()).as_str(),
        "my_index @a1:[1 +inf]"
    );
    assert_eq!(
        to_redis_string(&op.a1.in_range(..1).ft_search("my_index").unwrap()).as_str(),
        "my_index @a1:[-inf (1]"
    );
    assert_eq!(
        to_redis_string(&op.a1.in_range(..1).ft_search("my_index").unwrap()).as_str(),
        "my_index @a1:[-inf (1]"
    );
    assert_eq!(
        to_redis_string(&op.a1.in_range(..=1).ft_search("my_index").unwrap()).as_str(),
        "my_index @a1:[-inf 1]"
    );
    assert_eq!(
        to_redis_string(&op.a1.in_range(-1..=1).ft_search("my_index").unwrap()).as_str(),
        "my_index @a1:[-1 1]"
    );
    assert_eq!(
        to_redis_string(&op.a1.in_range(-1..1).ft_search("my_index").unwrap()).as_str(),
        "my_index @a1:[-1 (1]"
    );
}

#[test]
fn test_vector_query() {
    let op = Demo::op();
    assert_eq!(
        to_redis_string(
            &op.vector32
                .query(&[1.], 3, None)
                .ft_search("my_index")
                .unwrap()
        )
        .as_str(),
        "my_index *=>[KNN 3 @vector32 $vec] PARAMS 2 $vec <4Bytes>"
    );

    assert_eq!(
        to_redis_string(
            &op.vector32
                .query(&[1.], 3, "blob".into())
                .ft_search("my_index")
                .unwrap()
        )
        .as_str(),
        "my_index *=>[KNN 3 @vector32 $blob] PARAMS 2 $blob <4Bytes>"
    );

    assert_eq!(
        to_redis_string(
            &op.vector32
                .query(&[1., 1.], 3, None)
                .ft_search("my_index")
                .unwrap()
        )
        .as_str(),
        "my_index *=>[KNN 3 @vector32 $vec] PARAMS 2 $vec <8Bytes>"
    );

    assert_eq!(
        to_redis_string(
            &op.vector32
                .query(&[1., 1.], 3, "ok".into())
                .dialect(2)
                .ft_search("my_index")
                .unwrap()
        )
        .as_str(),
        "my_index *=>[KNN 3 @vector32 $ok] PARAMS 2 $ok <8Bytes> DIALECT 2"
    );

    assert_eq!(
        to_redis_string(
            &op.vector64
                .query(&[1.], 3, None)
                .dialect(2)
                .ft_search("my_index")
                .unwrap()
        )
        .as_str(),
        "my_index *=>[KNN 3 @vector64 $vec] PARAMS 2 $vec <4Bytes> DIALECT 2"
    );

    assert_eq!(
        to_redis_string(
            &op.vector64
                .query(&[1., 2.], 3, None)
                .ft_search("my_index")
                .unwrap()
        )
        .as_str(),
        "my_index *=>[KNN 3 @vector64 $vec] PARAMS 2 $vec <8Bytes>"
    );

    assert_eq!(
        op.vector32
            .query(&[1.], 3, None)
            .and(op.vector32.query(&[2.], 3, None))
            .ft_search("my_index")
            .is_err(),
        true
    );
}

#[test]
fn test_text_query() {
    let op = Demo::op();
    assert_eq!(
        to_redis_string(&op.description.eq("aaa").ft_search("my_index").unwrap()).as_str(),
        "my_index @description:{ aaa }"
    );
    assert_eq!(
        to_redis_string(
            &op.description
                .contains(&["aaa", "bbb", "ccc"])
                .ft_search("my_index")
                .unwrap()
        )
        .as_str(),
        "my_index @description:{ aaa | bbb | ccc }"
    );
    assert_eq!(
        to_redis_string(
            &op.description
                .not_contains(&["aaa", "bbb", "ccc"])
                .ft_search("my_index")
                .unwrap()
        )
        .as_str(),
        "my_index -@description:{ aaa | bbb | ccc }"
    );
}

#[test]
fn test_query() {
    let op = Demo::op();

    assert_eq!(
        to_redis_string(
            &op.description
                .eq("aaa")
                .and(op.description.eq("bbb"))
                .ft_search("my_index")
                .unwrap()
        )
        .as_str(),
        "my_index @description:{ aaa } @description:{ bbb }"
    );

    assert_eq!(
        to_redis_string(
            &op.description
                .eq("aaa")
                .and(op.vector32.query(&[1.], 3, None))
                .ft_search("my_index")
                .unwrap()
        )
        .as_str(),
        "my_index (@description:{ aaa })=>[KNN 3 @vector32 $vec] PARAMS 2 $vec <4Bytes>"
    );

    assert_eq!(
        to_redis_string(
            &op.description
                .eq("aaa")
                .and(op.description.eq("bbb").or(op.description.eq("ccc")))
                .ft_search("my_index")
                .unwrap()
        )
        .as_str(),
        "my_index @description:{ aaa } (@description:{ bbb })|(@description:{ ccc })"
    );

    assert_eq!(
        to_redis_string(
            &op.a1
                .in_range(..10)
                .or(op.a1.in_range(20..))
                .ft_search("my_index")
                .unwrap()
        )
        .as_str(),
        "my_index (@a1:[-inf (10])|(@a1:[20 +inf])"
    );
}
