use crate::{ArrayEntry, ArrayKey, Value};

impl<'a> From<bool> for Value<'a> {
    fn from(value: bool) -> Value<'a> {
        Value::Boolean(value)
    }
}

impl<'a> From<u8> for Value<'a> {
    fn from(value: u8) -> Value<'a> {
        Value::Long(value.into())
    }
}

impl<'a> From<u16> for Value<'a> {
    fn from(value: u16) -> Value<'a> {
        Value::Long(value.into())
    }
}

impl<'a> From<u32> for Value<'a> {
    fn from(value: u32) -> Value<'a> {
        Value::Long(value.into())
    }
}

impl<'a> From<i8> for Value<'a> {
    fn from(value: i8) -> Value<'a> {
        Value::Long(value.into())
    }
}

impl<'a> From<i16> for Value<'a> {
    fn from(value: i16) -> Value<'a> {
        Value::Long(value.into())
    }
}

impl<'a> From<i32> for Value<'a> {
    fn from(value: i32) -> Value<'a> {
        Value::Long(value.into())
    }
}

impl<'a> From<i64> for Value<'a> {
    fn from(value: i64) -> Value<'a> {
        Value::Long(value)
    }
}

impl<'a> From<f32> for Value<'a> {
    fn from(value: f32) -> Value<'a> {
        Value::Double(value.into())
    }
}

impl<'a> From<f64> for Value<'a> {
    fn from(value: f64) -> Value<'a> {
        Value::Double(value)
    }
}

impl<'a> From<&'a [u8]> for Value<'a> {
    fn from(value: &'a [u8]) -> Value<'a> {
        Value::String(value)
    }
}

impl<'a> From<&'a str> for Value<'a> {
    fn from(value: &'a str) -> Value<'a> {
        Value::String(value.as_bytes())
    }
}

impl<'a, T> From<Option<T>> for Value<'a>
where
    T: Into<Value<'a>>,
{
    fn from(value: Option<T>) -> Value<'a> {
        match value {
            Some(value) => value.into(),
            None => Value::Null,
        }
    }
}

impl<'a, V> From<Vec<V>> for Value<'a>
where
    V: Into<Value<'a>>,
{
    fn from(value: Vec<V>) -> Value<'a> {
        Value::Array(
            value
                .into_iter()
                .enumerate()
                .map(|(i, value)| ArrayEntry {
                    key: ArrayKey::Long(i.try_into().unwrap()),
                    value: value.into(),
                })
                .collect(),
        )
    }
}

impl<'a> From<u8> for ArrayKey<'a> {
    fn from(value: u8) -> ArrayKey<'a> {
        ArrayKey::Long(value.into())
    }
}

impl<'a> From<u16> for ArrayKey<'a> {
    fn from(value: u16) -> ArrayKey<'a> {
        ArrayKey::Long(value.into())
    }
}

impl<'a> From<u32> for ArrayKey<'a> {
    fn from(value: u32) -> ArrayKey<'a> {
        ArrayKey::Long(value.into())
    }
}

impl<'a> From<i8> for ArrayKey<'a> {
    fn from(value: i8) -> ArrayKey<'a> {
        ArrayKey::Long(value.into())
    }
}

impl<'a> From<i16> for ArrayKey<'a> {
    fn from(value: i16) -> ArrayKey<'a> {
        ArrayKey::Long(value.into())
    }
}

impl<'a> From<i32> for ArrayKey<'a> {
    fn from(value: i32) -> ArrayKey<'a> {
        ArrayKey::Long(value.into())
    }
}

impl<'a> From<i64> for ArrayKey<'a> {
    fn from(value: i64) -> ArrayKey<'a> {
        ArrayKey::Long(value)
    }
}

impl<'a> From<&'a [u8]> for ArrayKey<'a> {
    fn from(value: &'a [u8]) -> ArrayKey<'a> {
        ArrayKey::String(value)
    }
}

impl<'a> From<&'a str> for ArrayKey<'a> {
    fn from(value: &'a str) -> ArrayKey<'a> {
        ArrayKey::String(value.as_bytes())
    }
}

impl<'a, K, V> From<Vec<(K, V)>> for Value<'a>
where
    K: Into<ArrayKey<'a>>,
    V: Into<Value<'a>>,
{
    fn from(value: Vec<(K, V)>) -> Value<'a> {
        Value::Array(
            value
                .into_iter()
                .map(|(key, value)| ArrayEntry {
                    key: key.into(),
                    value: value.into(),
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_from_ints() {
        assert_eq!(Value::from(42u8), Value::Long(42),);
        assert_eq!(Value::from(42u16), Value::Long(42),);
        assert_eq!(Value::from(42u32), Value::Long(42),);
        assert_eq!(Value::from(42i8), Value::Long(42),);
        assert_eq!(Value::from(42i16), Value::Long(42),);
        assert_eq!(Value::from(42i32), Value::Long(42),);
        assert_eq!(Value::from(42i64), Value::Long(42),);
        assert_eq!(Value::from(-42i8), Value::Long(-42),);
        assert_eq!(Value::from(-42i16), Value::Long(-42),);
        assert_eq!(Value::from(-42i32), Value::Long(-42),);
        assert_eq!(Value::from(-42i64), Value::Long(-42),);
    }

    #[test]
    fn convert_from_floats() {
        assert_eq!(Value::from(-42.0f32), Value::Double(-42.0),);
        assert_eq!(Value::from(-42.12f64), Value::Double(-42.12),);
    }

    #[test]
    fn convert_from_vec_of_ints() {
        assert_eq!(
            Value::from(vec![5, 4, 3, 2, 1]),
            Value::Array(vec![
                ArrayEntry {
                    key: ArrayKey::Long(0),
                    value: Value::Long(5),
                },
                ArrayEntry {
                    key: ArrayKey::Long(1),
                    value: Value::Long(4),
                },
                ArrayEntry {
                    key: ArrayKey::Long(2),
                    value: Value::Long(3),
                },
                ArrayEntry {
                    key: ArrayKey::Long(3),
                    value: Value::Long(2),
                },
                ArrayEntry {
                    key: ArrayKey::Long(4),
                    value: Value::Long(1),
                },
            ]),
        );
    }

    #[test]
    fn convert_from_vec_of_key_value_pairs() {
        assert_eq!(
            Value::from(vec![("one", 1i64), ("two", 2), ("three", 3), ("four", 4)]),
            Value::Array(vec![
                ArrayEntry {
                    key: ArrayKey::String(b"one"),
                    value: Value::Long(1),
                },
                ArrayEntry {
                    key: ArrayKey::String(b"two"),
                    value: Value::Long(2),
                },
                ArrayEntry {
                    key: ArrayKey::String(b"three"),
                    value: Value::Long(3),
                },
                ArrayEntry {
                    key: ArrayKey::String(b"four"),
                    value: Value::Long(4),
                },
            ]),
        );
    }

    #[test]
    fn convert_from_vec_of_mixed_values() {
        assert_eq!(
            Value::from(vec![
                ("one", Value::from("string")),
                ("two", Value::from(2)),
                ("three", Value::from(true)),
                ("four", Value::from(vec![1, 2, 3]))
            ]),
            Value::Array(vec![
                ArrayEntry {
                    key: ArrayKey::String(b"one"),
                    value: Value::String(b"string"),
                },
                ArrayEntry {
                    key: ArrayKey::String(b"two"),
                    value: Value::Long(2),
                },
                ArrayEntry {
                    key: ArrayKey::String(b"three"),
                    value: Value::Boolean(true),
                },
                ArrayEntry {
                    key: ArrayKey::String(b"four"),
                    value: Value::Array(vec![
                        ArrayEntry {
                            key: ArrayKey::Long(0),
                            value: Value::Long(1),
                        },
                        ArrayEntry {
                            key: ArrayKey::Long(1),
                            value: Value::Long(2),
                        },
                        ArrayEntry {
                            key: ArrayKey::Long(2),
                            value: Value::Long(3),
                        },
                    ]),
                },
            ]),
        );
    }
}
