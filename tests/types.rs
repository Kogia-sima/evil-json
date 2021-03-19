fn to_json<T: serde::Serialize + ?Sized>(value: &T) -> String {
    evil_json::to_string(value).unwrap()
}

#[test]
fn null() {
    assert_eq!(to_json(&()), "null");
    assert_eq!(to_json(&Option::<u8>::None), "null");
}

#[test]
fn integer() {
    assert_eq!(to_json(&false), "false");
    assert_eq!(to_json(&0u32), "0");
    assert_eq!(to_json(&u16::max_value()), "65535");
    assert_eq!(to_json(&-81_isize), "-81");
    assert_eq!(
        to_json(&u128::max_value()),
        "340282366920938463463374607431768211455"
    );
    assert_eq!(
        to_json(&i128::min_value()),
        "-170141183460469231731687303715884105728"
    );
}

#[test]
fn float() {
    assert_eq!(to_json(&3.14f32), "3.14");
    assert_eq!(to_json(&-0.0), "-0.0");
    assert_eq!(to_json(&2.718281828459045), "2.718281828459045");
    assert_eq!(to_json(&f32::INFINITY), "null");
    assert_eq!(to_json(&f64::INFINITY), "null");
}

#[test]
fn char_no_escape() {
    assert_eq!(to_json(&'a'), "\"a\"");
    assert_eq!(to_json(&'é'), "\"é\"");
    assert_eq!(to_json(&'Ａ'), "\"Ａ\"");
    assert_eq!(to_json(&'🄫'), "\"🄫\"");
}

#[test]
fn char_escape() {
    assert_eq!(to_json(&'\"'), "\"\\\"\"");
    assert_eq!(to_json(&'\r'), "\"\\r\"");
    assert_eq!(to_json(&'\\'), "\"\\\\\"");
    assert_eq!(to_json(&'\x1f'), "\"\\u001f\"");
}

#[test]
fn string() {
    assert_eq!(to_json(&""), "\"\"");
    assert_eq!(to_json(&"This is ascii text."), "\"This is ascii text.\"");
    assert_eq!(to_json(&"Καλιμέρα"), "\"Καλιμέρα\"");
}

#[test]
fn slice() {
    assert_eq!(to_json::<[u8; 0]>(&[]), "[]");
    assert_eq!(to_json(&[18782_i16]), "[18782]");
    assert_eq!(to_json(&["apple", "banana"]), "[\"apple\",\"banana\"]");
    assert_eq!(to_json(&b"\x03\x07\x01\x0a"[..]), "[3,7,1,10]");
}

#[test]
fn tuple() {
    assert_eq!(to_json(&(-1i8,)), "[-1]");
    assert_eq!(to_json(&(9i64, "abc")), "[9,\"abc\"]");
    assert_eq!(to_json(&(9i64, b"abc")), "[9,[97,98,99]]");
}
