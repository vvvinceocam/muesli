use muesli::{serialize, session_decode, session_encode, unserialize};

#[test]
fn base_serialize_case() {
    let data = br#"a:9:{i:0;i:12;i:1;s:2:"ok";i:2;N;i:3;d:NAN;s:3:"foo";s:3:"bar";s:6:"inside";a:6:{i:0;s:1:"i";i:1;s:1:"n";i:2;s:1:"s";i:3;s:1:"i";i:4;s:1:"d";i:5;s:1:"e";}i:4;d:3.1416;i:5;b:1;i:6;b:0;}"#.as_slice();

    let mut buffer = Vec::new();
    serialize(&mut buffer, &unserialize(data).unwrap()).unwrap();

    assert_eq!(buffer, data);
}

#[test]
fn base_session_case() {
    let data = br#"foo|a:4:{i:0;i:1;i:1;i:2;i:2;i:3;i:3;i:4;}bar|s:22:"complicated string "|!";baz|d:NAN;qux|b:1;"#.as_slice();

    let mut buffer = Vec::new();
    session_encode(&mut buffer, &session_decode(data).unwrap()).unwrap();

    assert_eq!(buffer, data);
}

#[test]
fn large_session_case() {
    let data = include_bytes!("data/large.session");

    let mut buffer = Vec::new();
    session_encode(&mut buffer, &session_decode(data).unwrap()).unwrap();

    assert_eq!(buffer, data);
}
