use muesli::{session_encode, SessionEntry, Value};

fn main() {
    let data = vec![
        SessionEntry {
            key: b"foo",
            value: Value::Long(42),
        },
        SessionEntry {
            key: b"bar",
            value: Value::String(b"baz|qux".as_slice()),
        },
        SessionEntry {
            key: b"pub",
            value: Value::Long(1337),
        },
    ];

    let mut session = Vec::<u8>::new();
    session_encode(&mut session, &data).unwrap();
    assert_eq!(
        session,
        b"foo|i:42;bar|s:7:\"baz|qux\";pub|i:1337;".as_slice()
    );
}
