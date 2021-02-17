macro_rules! btreemap {
    () => { ::std::collections::BTreeMap::<i8, ()>::new() };
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut m = ::std::collections::BTreeMap::new();
        $(
            m.insert($key, $value);
        )*

        m
    }};
}

fn to_json<T: serde::Serialize>(value: &T) -> String {
    evil_json::to_string(value).unwrap()
}

#[test]
fn empty() {
    assert_eq!(to_json(&btreemap! {}), "{}");
    assert_eq!(to_json(&vec![btreemap! {}]), "[{}]");
}

#[test]
fn int_key() {
    assert_eq!(to_json(&btreemap! {1=>2, 4=>8}), r#"{"1":2,"4":8}"#);
    assert_eq!(
        to_json(&btreemap! {-1i16=>"-3",5=>"15"}),
        r#"{"-1":"-3","5":"15"}"#
    );
    assert_eq!(
        to_json(&btreemap! {i128::max_value()=>()}),
        r#"{"170141183460469231731687303715884105727":null}"#
    );
    assert_eq!(
        to_json(&btreemap! {0u8=>2..4, 1=>4..8}),
        r#"{"0":{"start":2,"end":4},"1":{"start":4,"end":8}}"#
    );
    assert_eq!(to_json(&btreemap! {255u8=>-255}), r#"{"255":-255}"#);
}

#[test]
fn nested() {
    assert_eq!(
        to_json(&btreemap! {65535u16=>btreemap!{}}),
        r#"{"65535":{}}"#
    );
    assert_eq!(
        to_json(&btreemap! {0u32=>btreemap!{"foo"=>"bar","baz"=>"qux"}}),
        r#"{"0":{"baz":"qux","foo":"bar"}}"#
    );
    assert_eq!(
        to_json(&btreemap! {"a"=>btreemap!{-128i8=>"-0x80"},"b"=>btreemap!{127=>"0x7f"}}),
        r#"{"a":{"-128":"-0x80"},"b":{"127":"0x7f"}}"#
    );
    assert_eq!(
        to_json(&btreemap!(1i64=>btreemap!{2u64=>btreemap!{}})),
        r#"{"1":{"2":{}}}"#
    );
}

#[test]
fn key_escape() {
    assert_eq!(
        to_json(&btreemap! {"lorem\tipsum"=>false}),
        r#"{"lorem\tipsum":false}"#
    );
    assert_eq!(
        to_json(&btreemap! {"normal"=>0,"\"quoted\""=>1}),
        r#"{"\"quoted\"":1,"normal":0}"#
    );
}
