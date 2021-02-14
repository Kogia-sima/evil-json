use evil_json::Error;

#[test]
fn null() {
    assert_eq!(evil_json::to_string(&()).unwrap(), "null");
    assert_eq!(evil_json::to_string(&Option::<u8>::None).unwrap(), "null");
}

#[test]
fn integer() {
    assert_eq!(evil_json::to_string(&false).unwrap(), "false");
    assert_eq!(evil_json::to_string(&0u32).unwrap(), "0");
    assert_eq!(evil_json::to_string(&u16::max_value()).unwrap(), "65535");
    assert_eq!(evil_json::to_string(&-81_isize).unwrap(), "-81");
    assert_eq!(
        evil_json::to_string(&u128::max_value()).unwrap(),
        "340282366920938463463374607431768211455"
    );
    assert_eq!(
        evil_json::to_string(&i128::min_value()).unwrap(),
        "-170141183460469231731687303715884105728"
    );
}

#[test]
fn float() {
    assert_eq!(evil_json::to_string(&3.14f32).unwrap(), "3.14");
    assert_eq!(evil_json::to_string(&-1e-21).unwrap(), "-1e-21");
    assert_eq!(
        evil_json::to_string(&2.718281828459045).unwrap(),
        "2.718281828459045"
    );
    assert!(matches!(
        evil_json::to_string(&f32::INFINITY),
        Err(Error::NonFiniteFloat)
    ));
    assert!(matches!(
        evil_json::to_string(&f64::NAN),
        Err(Error::NonFiniteFloat)
    ));
}

#[test]
fn char_no_escape() {
    assert_eq!(evil_json::to_string(&'a').unwrap(), "\"a\"");
    assert_eq!(evil_json::to_string(&'é').unwrap(), "\"é\"");
    assert_eq!(evil_json::to_string(&'Ａ').unwrap(), "\"Ａ\"");
    assert_eq!(evil_json::to_string(&'🄫').unwrap(), "\"🄫\"");
}

#[test]
fn char_escape() {
    assert_eq!(evil_json::to_string(&'\"').unwrap(), "\"\\\"\"");
    assert_eq!(evil_json::to_string(&'\r').unwrap(), "\"\\r\"");
    assert_eq!(evil_json::to_string(&'\\').unwrap(), "\"\\\\\"");
    assert_eq!(evil_json::to_string(&'\x1f').unwrap(), "\"\\u001f\"");
}

#[test]
fn string() {
    assert_eq!(evil_json::to_string(&"").unwrap(), "\"\"");
    assert_eq!(
        evil_json::to_string(&"This is ascii text.").unwrap(),
        "\"This is ascii text.\""
    );
    assert_eq!(evil_json::to_string(&"Καλιμέρα").unwrap(), "\"Καλιμέρα\"");
}

#[test]
fn slice() {
    assert_eq!(evil_json::to_string::<[u8; 0]>(&[]).unwrap(), "[]");
    assert_eq!(evil_json::to_string(&[18782_i16]).unwrap(), "[18782]");
    assert_eq!(evil_json::to_string(&["apple", "banana"]).unwrap(), "[\"apple\",\"banana\"]");
    assert_eq!(evil_json::to_string(&b"\x03\x07\x01\x0a"[..]).unwrap(), "[3,7,1,10]");
}