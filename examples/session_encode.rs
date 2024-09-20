use muesli::{session_encode, SessionEntry, Value};

fn main() {
    let data = vec![
        SessionEntry {
            key: b"foo",
            value: Value::Integer(42),
        },
        SessionEntry {
            key: b"bar",
            value: Value::String(b"baz|qux".as_slice()),
        },
        SessionEntry {
            key: b"pub",
            value: Value::Integer(1337),
        },
    ];

    let mut session = Vec::<u8>::new();
    session_encode(&mut session, &data).unwrap();
    assert_eq!(session, b"foo|i:42;bar|s:3:\"baz\";".as_slice());
}
