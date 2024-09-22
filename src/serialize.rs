use std::io::Write;

use crate::value::{ArrayEntry, ArrayKey, SessionEntry, Value};

/// Encode data to PHP's `serialize` format
///
/// # Errors
///
/// Will return `Err` if write fail
pub fn serialize<W: Write>(w: &mut W, value: &Value) -> std::result::Result<usize, std::io::Error> {
    match value {
        Value::Null => w.write(b"N;"),
        Value::Boolean(false) => w.write(b"b:0;"),
        Value::Boolean(true) => w.write(b"b:1;"),
        Value::Long(n) => w.write(format!("i:{n};").as_bytes()),
        Value::Double(d) => {
            if d.is_nan() {
                w.write(b"d:NAN;")
            } else if d.is_infinite() {
                if d.is_sign_positive() {
                    w.write(b"d:INF;")
                } else {
                    w.write(b"d:-INF;")
                }
            } else {
                w.write(format!("d:{d};").as_bytes())
            }
        }
        Value::String(string) => {
            let mut count = 0;
            count += w.write(format!("s:{}:\"", string.len()).as_bytes())?;
            count += w.write(string)?;
            count += w.write(b"\";")?;
            Ok(count)
        }
        Value::Array(items) => {
            let mut count = 0;
            count += w.write(format!("a:{}:{{", items.len()).as_bytes())?;
            for ArrayEntry { key, value } in items {
                match key {
                    ArrayKey::Long(key) => {
                        count += w.write(format!("i:{key};").as_bytes())?;
                    }
                    ArrayKey::String(key) => {
                        count += w.write(format!("s:{}:\"", key.len()).as_bytes())?;
                        count += w.write(key)?;
                        count += w.write(b"\";")?;
                    }
                }
                count += serialize(w, value)?;
            }
            count += w.write(b"}")?;
            Ok(count)
        }
        Value::ValueReference(idx) => w.write(format!("R:{idx};").as_bytes()),
        Value::ObjectReference(idx) => w.write(format!("r:{idx};").as_bytes()),
        _ => unimplemented!(),
    }
}

/// Encode data to PHP's session format, compatible with `session_decode()`.
///
/// # Errors
///
/// Will return `Err` if write fail
pub fn session_encode<W: Write>(
    w: &mut W,
    session: &[SessionEntry],
) -> std::result::Result<usize, std::io::Error> {
    let mut count = 0;
    for entry in session {
        count += w.write(entry.key)?;
        count += w.write(b"|")?;
        count += serialize(w, &entry.value)?;
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::num::NonZeroUsize;

    fn run_encode_cases(cases: &[(Value, &[u8])]) {
        let mut buffer = Vec::<u8>::new();
        for (input, expected) in cases {
            buffer.clear();
            let count = serialize(&mut buffer, input).unwrap();
            assert_eq!(&buffer.as_slice(), expected);
            assert_eq!(count, expected.len());
        }
    }

    fn run_session_encode_cases(cases: &[(Vec<SessionEntry>, &[u8])]) {
        let mut buffer = Vec::<u8>::new();
        for (input, expected) in cases {
            buffer.clear();
            let count = session_encode(&mut buffer, input).unwrap();
            assert_eq!(&buffer.as_slice(), expected);
            assert_eq!(count, expected.len());
        }
    }

    #[test]
    fn encode_value_null() {
        let mut buffer = Vec::<u8>::new();
        serialize(&mut buffer, &Value::Null).unwrap();
        assert_eq!(buffer.as_slice(), b"N;");
    }

    #[test]
    fn encode_value_boolean() {
        let cases = [
            (Value::Boolean(false), b"b:0;".as_slice()),
            (Value::Boolean(true), b"b:1;".as_slice()),
        ];
        run_encode_cases(&cases);
    }

    #[test]
    fn encode_value_integer() {
        let cases = [
            (Value::Long(0), b"i:0;".as_slice()),
            (Value::Long(123), b"i:123;".as_slice()),
            (Value::Long(-23), b"i:-23;".as_slice()),
            (Value::Long(-23_432_123), b"i:-23432123;".as_slice()),
        ];
        run_encode_cases(&cases);
    }

    #[test]
    fn encode_value_decimal() {
        let cases = [
            (Value::Double(0.0), b"d:0;".as_slice()),
            (Value::Double(0.2), b"d:0.2;".as_slice()),
            (Value::Double(-0.2), b"d:-0.2;".as_slice()),
            (Value::Double(f64::NAN), b"d:NAN;".as_slice()),
            (Value::Double(f64::INFINITY), b"d:INF;".as_slice()),
            (Value::Double(f64::NEG_INFINITY), b"d:-INF;".as_slice()),
        ];
        run_encode_cases(&cases);
    }

    #[test]
    fn encode_value_string() {
        let cases = [
            (Value::String(b"".as_slice()), b"s:0:\"\";".as_slice()),
            (Value::String(b"foo".as_slice()), b"s:3:\"foo\";".as_slice()),
        ];
        run_encode_cases(&cases);
    }

    #[test]
    fn encode_value_references() {
        let cases = [
            (
                Value::ValueReference(NonZeroUsize::new(10).unwrap()),
                b"R:10;".as_slice(),
            ),
            (
                Value::ObjectReference(NonZeroUsize::new(42).unwrap()),
                b"r:42;".as_slice(),
            ),
        ];
        run_encode_cases(&cases);
    }

    #[test]
    fn encode_value_array() {
        let cases = [
            (Value::Array(vec![]), b"a:0:{}".as_slice()),
            (
                Value::Array(vec![ArrayEntry {
                    key: ArrayKey::String(b"foo"),
                    value: Value::String(b"bar"),
                }]),
                b"a:1:{s:3:\"foo\";s:3:\"bar\";}".as_slice(),
            ),
            (
                Value::Array(vec![ArrayEntry {
                    key: ArrayKey::Long(0),
                    value: Value::Array(vec![ArrayEntry {
                        key: ArrayKey::Long(0),
                        value: Value::Array(vec![ArrayEntry {
                            key: ArrayKey::Long(0),
                            value: Value::Array(vec![]),
                        }]),
                    }]),
                }]),
                b"a:1:{i:0;a:1:{i:0;a:1:{i:0;a:0:{}}}}".as_slice(),
            ),
            (
                Value::Array(vec![
                    ArrayEntry {
                        key: ArrayKey::Long(0),
                        value: Value::Long(1),
                    },
                    ArrayEntry {
                        key: ArrayKey::Long(1),
                        value: Value::Long(1),
                    },
                    ArrayEntry {
                        key: ArrayKey::Long(2),
                        value: Value::Long(2),
                    },
                    ArrayEntry {
                        key: ArrayKey::Long(3),
                        value: Value::Long(3),
                    },
                    ArrayEntry {
                        key: ArrayKey::Long(4),
                        value: Value::Long(5),
                    },
                ]),
                b"a:5:{i:0;i:1;i:1;i:1;i:2;i:2;i:3;i:3;i:4;i:5;}".as_slice(),
            ),
        ];
        run_encode_cases(&cases);
    }

    #[test]
    fn encode_session() {
        let cases = [
            (vec![], b"".as_slice()),
            (
                vec![SessionEntry {
                    key: b"foo",
                    value: Value::Long(42),
                }],
                b"foo|i:42;".as_slice(),
            ),
            (
                vec![
                    SessionEntry {
                        key: b"foo",
                        value: Value::Long(42),
                    },
                    SessionEntry {
                        key: b"bar",
                        value: Value::String(b"baz".as_slice()),
                    },
                ],
                b"foo|i:42;bar|s:3:\"baz\";".as_slice(),
            ),
            (
                vec![
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
                ],
                b"foo|i:42;bar|s:7:\"baz|qux\";pub|i:1337;".as_slice(),
            ),
        ];
        run_session_encode_cases(&cases);
    }
}
