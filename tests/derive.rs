use serde_derive::Serialize;
use std::marker::PhantomData;

fn to_json<T: serde::Serialize>(value: &T) -> String {
    evil_json::to_string(value).unwrap()
}

#[derive(Serialize)]
struct UnitStruct;

#[test]
fn unit_struct() {
    assert_eq!(to_json(&UnitStruct), "null");
}

#[derive(Serialize)]
struct NewTypeStruct(i32);

#[test]
fn new_type_struct() {
    assert_eq!(to_json(&NewTypeStruct(i32::min_value())), "-2147483648");
}

#[derive(Serialize)]
struct EmptyStruct {}

#[test]
fn empty_struct() {
    assert_eq!(to_json(&EmptyStruct {}), "{}");
}

#[derive(Serialize)]
struct NamedStruct {
    field1: PhantomData<()>,
    field2: Option<String>,
}

#[test]
fn named_struct() {
    assert_eq!(
        to_json(&NamedStruct {
            field1: PhantomData,
            field2: Some("string".to_owned())
        }),
        r#"{"field1":null,"field2":"string"}"#
    );
}

#[derive(Serialize)]
enum SimpleEnum {
    Unit,
    Newtype(i16),
    EmptyTuple(),
    EmptyStruct {},
    Tuple(u8, &'static str),
    Struct { inner: () },
}

#[test]
fn simple_enum() {
    assert_eq!(to_json(&SimpleEnum::Unit), r#""Unit""#);
    assert_eq!(to_json(&SimpleEnum::Newtype(-99)), r#"{"Newtype":-99}"#);
    assert_eq!(to_json(&SimpleEnum::EmptyTuple()), r#"{"EmptyTuple":[]}"#);
    assert_eq!(
        to_json(&SimpleEnum::EmptyStruct {}),
        r#"{"EmptyStruct":{}}"#
    );
    assert_eq!(
        to_json(&SimpleEnum::Tuple(255, "val")),
        r#"{"Tuple":[255,"val"]}"#
    );
    assert_eq!(
        to_json(&SimpleEnum::Struct { inner: () }),
        r#"{"Struct":{"inner":null}}"#
    );
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum Tagged {
    A { value: i16 },
    B { value: i32 },
}

#[test]
fn tagged() {
    assert_eq!(
        to_json(&Tagged::A { value: 32767 }),
        r#"{"type":"A","value":32767}"#
    );
    assert_eq!(
        to_json(&Tagged::B { value: -1 }),
        r#"{"type":"B","value":-1}"#
    );
}

#[derive(Serialize)]
#[serde(tag = "t", content = "c")]
enum Adjacent<'a> {
    Owned(String),
    Borrowed(&'a str),
}

#[test]
fn adjacent() {
    assert_eq!(
        to_json(&Adjacent::Owned("foo\r\nbar".to_owned())),
        r#"{"t":"Owned","c":"foo\r\nbar"}"#
    );
    assert_eq!(
        to_json(&Adjacent::Borrowed("タグ付きEnum")),
        r#"{"t":"Borrowed","c":"タグ付きEnum"}"#
    );
}

#[derive(Serialize)]
#[serde(untagged)]
enum Untagged {
    Integer(u64),
    Pair(String, String),
}

#[test]
fn untagged() {
    assert_eq!(to_json(&Untagged::Integer(0)), "0");
    assert_eq!(
        to_json(&Untagged::Pair("foo".to_owned(), "".to_owned())),
        r#"["foo",""]"#
    );
}

#[derive(Serialize)]
struct StructKeyEscape {
    #[serde(rename = "line\nbreak")]
    field: (i32, &'static str),
}

#[test]
fn struct_key_escape() {
    assert_eq!(
        to_json(&StructKeyEscape {
            field: (10, "name")
        }),
        r#"{"line\nbreak":[10,"name"]}"#
    );
}

#[derive(Serialize)]
enum UnitVariantEscape {
    #[serde(rename = "null\x08")]
    Null,
}

#[test]
fn unit_variant_escape() {
    assert_eq!(to_json(&UnitVariantEscape::Null), r#""null\b""#);
}

#[derive(Serialize)]
enum NewtypeVariantEscape {
    #[serde(rename = "back\\slash")]
    Some(bool),
    #[allow(dead_code)]
    None,
}

#[test]
fn newtype_variant_escape() {
    assert_eq!(
        to_json(&NewtypeVariantEscape::Some(true)),
        r#"{"back\\slash":true}"#
    );
}

#[derive(Serialize)]
enum TupleVariantEscape {
    #[serde(rename = "r\tg\tb")]
    Rgb(u8, u8, u8),
}

#[test]
fn tuple_variant_escape() {
    assert_eq!(
        to_json(&TupleVariantEscape::Rgb(13, 92, 8)),
        r#"{"r\tg\tb":[13,92,8]}"#
    );
}

#[derive(Serialize)]
enum StructVariantEscape {
    #[serde(rename = "\x00")]
    S {
        #[serde(rename = "\x01")]
        value: f32,
    },
}

#[test]
fn struct_variant_escape() {
    assert_eq!(
        to_json(&StructVariantEscape::S { value: -1.0 }),
        r#"{"\u0000":{"\u0001":-1.0}}"#
    );
}
