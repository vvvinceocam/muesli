use muesli::{session_encode, SessionEntry};

fn main() {
    let data = vec![
        SessionEntry {
            key: b"foo",
            value: 42.into(),
        },
        SessionEntry {
            key: b"bar",
            value: b"baz|qux".as_slice().into(),
        },
        SessionEntry {
            key: b"array",
            value: vec![1, 2, 3, 4, 5].into(),
        },
    ];

    let mut session = Vec::<u8>::new();
    session_encode(&mut session, &data).unwrap();
    assert_eq!(
        session,
        b"foo|i:42;bar|s:7:\"baz|qux\";array|a:5:{i:0;i:1;i:1;i:2;i:2;i:3;i:3;i:4;i:4;i:5;}"
            .as_slice()
    );
}
