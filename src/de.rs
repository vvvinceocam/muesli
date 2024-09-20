mod raw;

use winnow::{
    binary::{length_repeat, length_take},
    combinator::{alt, delimited, empty, repeat, rest, separated_pair, terminated},
    error::{ContextError, ParseError, StrContext},
    seq,
    token::{one_of, take_until},
    PResult, Parser,
};

use crate::value::{ArrayKey, ObjectProperty, ObjectPropertyVisibility, SessionEntry, Value};

fn any_value<'s>(input: &mut &'s [u8]) -> PResult<Value<'s>> {
    alt((
        value_null,
        value_boolean,
        value_integer,
        value_decimal,
        value_string,
        value_array,
        value_object,
        value_custom_object,
        value_reference_to_value,
        value_reference_to_object,
    ))
    .parse_next(input)
}

fn value_null<'s>(input: &mut &'s [u8]) -> PResult<Value<'s>> {
    b"N;".value(Value::Null).parse_next(input)
}

fn value_boolean<'s>(input: &mut &'s [u8]) -> PResult<Value<'s>> {
    delimited(b"b:", one_of(b"01"), b';')
        .map(|bool| Value::Boolean(bool == b'1'))
        .parse_next(input)
}

fn value_integer<'s>(input: &mut &'s [u8]) -> PResult<Value<'s>> {
    delimited(b"i:", raw::signed_integer, b';')
        .parse_to()
        .map(Value::Integer)
        .parse_next(input)
}

fn value_decimal<'s>(input: &mut &'s [u8]) -> PResult<Value<'s>> {
    delimited(
        b"d:",
        alt((
            b"NAN".value(f64::NAN),
            b"INF".value(f64::INFINITY),
            b"-INF".value(f64::NEG_INFINITY),
            raw::float.parse_to(),
        )),
        b';',
    )
    .map(Value::Decimal)
    .parse_next(input)
}

fn value_string<'s>(input: &mut &'s [u8]) -> PResult<Value<'s>> {
    delimited(b"s:", raw::sized_string, b';')
        .map(Value::String)
        .parse_next(input)
}

fn array_key<'s>(input: &mut &'s [u8]) -> PResult<ArrayKey<'s>> {
    alt((
        delimited(b"i:", raw::signed_integer, b';')
            .parse_to()
            .map(ArrayKey::Integer),
        delimited(b"s:", raw::sized_string, b';').map(ArrayKey::String),
    ))
    .parse_next(input)
}

fn array_pair<'s>(input: &mut &'s [u8]) -> PResult<(ArrayKey<'s>, Value<'s>)> {
    (array_key, any_value).parse_next(input)
}

fn value_array<'s>(input: &mut &'s [u8]) -> PResult<Value<'s>> {
    delimited(
        b"a:",
        length_repeat(terminated(raw::size, b":{"), array_pair),
        b'}',
    )
    .map(Value::Array)
    .parse_next(input)
}

fn value_reference_to_value<'s>(input: &mut &'s [u8]) -> PResult<Value<'s>> {
    delimited(b"R:", raw::unsigned_integer, b';')
        .parse_to()
        .map(Value::ValueReference)
        .parse_next(input)
}

fn value_reference_to_object<'s>(input: &mut &'s [u8]) -> PResult<Value<'s>> {
    delimited(b"r:", raw::unsigned_integer, b';')
        .parse_to()
        .map(Value::ObjectReference)
        .parse_next(input)
}

fn object_property_name<'s>(input: &mut &'s [u8]) -> PResult<(ObjectPropertyVisibility, &'s [u8])> {
    use ObjectPropertyVisibility::{Private, Protected, Public};

    delimited(
        b"s:",
        raw::sized_string.and_then((
            alt((
                b"\0*\0".value(Protected),
                (b'\0', take_until(1.., b'\0'), b'\0').value(Private),
                empty.value(Public),
            ))
            .context(StrContext::Label("property visibility")),
            rest,
        )),
        b';',
    )
    .parse_next(input)
}

fn object_property<'s>(input: &mut &'s [u8]) -> PResult<ObjectProperty<'s>> {
    (object_property_name, any_value)
        .map(|((visibility, name), value)| ObjectProperty {
            visibility,
            name,
            value,
        })
        .parse_next(input)
}

fn value_object<'s>(input: &mut &'s [u8]) -> PResult<Value<'s>> {
    seq!(
        _: b"O:",
        raw::sized_string
        .context(StrContext::Label("class name")),
        _: b':',
        length_repeat(
            terminated(raw::size, b":{"),
            object_property.context(StrContext::Label("object property")),
        ).context(StrContext::Label("object properties")),
        _: b'}',
    )
    .map(|(class_name, properties)| Value::Object {
        class_name,
        properties,
    })
    .parse_next(input)
}

fn value_custom_object<'s>(input: &mut &'s [u8]) -> PResult<Value<'s>> {
    seq!(
        _: b"C:",
        raw::sized_string
        .context(StrContext::Label("class name")),
        _: b':',
        length_take(
            terminated(raw::size, b":{"),
        ).context(StrContext::Label("object properties")),
        _: b'}',
    )
    .map(|(class_name, data)| Value::CustomObject { class_name, data })
    .parse_next(input)
}

/// Decode PHP serialize/unserialize formated binary string,
///
/// # Errors
///
/// Will return `Err` if input is not a valid PHP serialize value.
pub fn unserialize(input: &[u8]) -> Result<Value, ParseError<&[u8], ContextError>> {
    any_value.parse(input)
}

fn session_key<'s>(input: &mut &'s [u8]) -> PResult<&'s [u8]> {
    take_until(0.., '|').parse_next(input)
}

/// Decode PHP session binary representation.
///
/// # Errors
///
/// Will return `Err` if input is not a valid PHP session.
pub fn session_decode(input: &[u8]) -> Result<Vec<SessionEntry>, ParseError<&[u8], ContextError>> {
    repeat(
        0..,
        separated_pair(session_key, '|', any_value).map(|(key, value)| SessionEntry { key, value }),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::type_complexity)]
    pub(crate) fn run_cases<'s, O, E>(
        mut parser: impl Parser<&'s [u8], O, E>,
        cases: &[(&'s [u8], Option<(&[u8], O)>)],
    ) where
        O: std::fmt::Debug + PartialEq,
        E: std::fmt::Debug + PartialEq,
    {
        for (mut input, expected) in cases {
            let output = parser.parse_next(&mut input);
            match expected {
                Some(expected) => {
                    assert_eq!(output.as_ref(), Ok(&expected.1));
                    assert_eq!(input, expected.0);
                }
                None => assert!(output.is_err()),
            }
        }
    }

    #[test]
    fn parse_null_value() {
        let cases = [
            (b"N;".as_slice(), Some((b"".as_slice(), Value::Null))),
            (
                b"N;extra".as_slice(),
                Some((b"extra".as_slice(), Value::Null)),
            ),
        ];
        run_cases(value_null, &cases);
    }

    #[test]
    fn parse_bool_value() {
        let cases = [
            (
                b"b:0;".as_slice(),
                Some((b"".as_slice(), Value::Boolean(false))),
            ),
            (
                b"b:0;extra".as_slice(),
                Some((b"extra".as_slice(), Value::Boolean(false))),
            ),
            (
                b"b:1;".as_slice(),
                Some((b"".as_slice(), Value::Boolean(true))),
            ),
            (
                b"b:1;extra".as_slice(),
                Some((b"extra".as_slice(), Value::Boolean(true))),
            ),
        ];
        run_cases(value_boolean, &cases);
    }

    #[test]
    fn parse_positive_integer_value() {
        let cases = [
            (
                b"i:0;".as_slice(),
                Some((b"".as_slice(), Value::Integer(0))),
            ),
            (
                b"i:1;".as_slice(),
                Some((b"".as_slice(), Value::Integer(1))),
            ),
            (
                b"i:01;".as_slice(),
                Some((b"".as_slice(), Value::Integer(1))),
            ),
            (
                b"i:23;".as_slice(),
                Some((b"".as_slice(), Value::Integer(23))),
            ),
            (
                b"i:1234567890;".as_slice(),
                Some((b"".as_slice(), Value::Integer(1_234_567_890))),
            ),
            (
                b"i:1234567890;extra".as_slice(),
                Some((b"extra".as_slice(), Value::Integer(1_234_567_890))),
            ),
        ];

        run_cases(value_integer, &cases);
    }

    #[test]
    fn parse_negative_integer_value() {
        let cases = [
            (
                b"i:-0;".as_slice(),
                Some((b"".as_slice(), Value::Integer(0))),
            ),
            (
                b"i:-1;".as_slice(),
                Some((b"".as_slice(), Value::Integer(-1))),
            ),
            (
                b"i:-01;".as_slice(),
                Some((b"".as_slice(), Value::Integer(-1))),
            ),
            (
                b"i:-23;".as_slice(),
                Some((b"".as_slice(), Value::Integer(-23))),
            ),
            (
                b"i:-1234567890;".as_slice(),
                Some((b"".as_slice(), Value::Integer(-1_234_567_890))),
            ),
            (
                b"i:-1234567890;extra".as_slice(),
                Some((b"extra".as_slice(), Value::Integer(-1_234_567_890))),
            ),
        ];

        run_cases(value_integer, &cases);
    }

    #[test]
    fn parse_decimal_value() {
        let cases = [
            (
                b"d:0.0;".as_slice(),
                Some((b"".as_slice(), Value::Decimal(0.0))),
            ),
            (
                b"d:-1.0;".as_slice(),
                Some((b"".as_slice(), Value::Decimal(-1.0))),
            ),
            (
                b"d:INF;".as_slice(),
                Some((b"".as_slice(), Value::Decimal(f64::INFINITY))),
            ),
            (
                b"d:-INF;".as_slice(),
                Some((b"".as_slice(), Value::Decimal(f64::NEG_INFINITY))),
            ),
        ];

        run_cases(value_decimal, &cases);
    }

    #[test]
    fn parse_nan_decimal_value() {
        let mut input = b"d:NAN;".as_slice();

        let output = value_decimal.parse_next(&mut input).unwrap();
        match output {
            Value::Decimal(d) => assert!(d.is_nan()),
            _ => panic!("should have been a decimal"),
        }
    }

    #[test]
    fn parse_string_value() {
        let cases = [
            (
                b"s:10:\"1234567890\";".as_slice(),
                Some((b"".as_slice(), Value::String("1234567890".as_ref()))),
            ),
            (
                b"s:11:\"123456\"7890\";".as_slice(),
                Some((b"".as_slice(), Value::String("123456\"7890".as_ref()))),
            ),
            (
                b"s:12:\"123456\";7890\";".as_slice(),
                Some((b"".as_slice(), Value::String("123456\";7890".as_ref()))),
            ),
        ];

        run_cases(value_string, &cases);
    }

    #[test]
    fn parse_array_pair() {
        let cases = [
            (
                b"s:10:\"1234567890\";i:10;".as_slice(),
                Some((
                    b"".as_slice(),
                    (ArrayKey::String("1234567890".as_ref()), Value::Integer(10)),
                )),
            ),
            (
                b"i:10;s:10:\"1234567890\";".as_slice(),
                Some((
                    b"".as_slice(),
                    (ArrayKey::Integer(10), Value::String("1234567890".as_ref())),
                )),
            ),
        ];

        run_cases(array_pair, &cases);
    }

    #[test]
    fn parse_array_value() {
        let cases = [
            (
                b"a:1:{s:3:\"foo\";s:3:\"bar\";}".as_slice(),
                Some((
                    b"".as_slice(),
                    Value::Array(vec![
                        (ArrayKey::String(b"foo"), Value::String(b"bar"))
                    ]),
                )),
            ),
            (
                b"a:1:{i:3;s:3:\"baz\";}".as_slice(),
                Some((
                    b"".as_slice(),
                    Value::Array(vec![
                        (ArrayKey::Integer(3), Value::String(b"baz"))
                    ]),
                )),
            ),
            (
                r#"a:3:{i:12;d:0.12;s:3:"foo";a:1:{s:10:"some-value";s:15:""other";"value"";}i:43;i:76;}"#.as_bytes(),
                Some((
                    b"".as_slice(),
                    Value::Array(vec![
                        (ArrayKey::Integer(12), Value::Decimal(0.12)),
                        (ArrayKey::String(b"foo"), Value::Array(vec![
                            (
                                ArrayKey::String(b"some-value"),
                                Value::String(r#""other";"value""#.as_bytes())
                            )
                        ])),
                        (ArrayKey::Integer(43), Value::Integer(76)),
                    ]),
                )),
            ),
        ];

        run_cases(value_array, &cases);
    }

    #[test]
    fn parse_object_property() {
        let cases = [
            (
                b"s:6:\"public\";i:1;".as_slice(),
                Some((
                    b"".as_slice(),
                    ObjectProperty {
                        name: b"public".as_slice(),
                        visibility: ObjectPropertyVisibility::Public,
                        value: Value::Integer(1),
                    },
                )),
            ),
            (
                b"s:12:\"\0*\0protected\";i:42;".as_slice(),
                Some((
                    b"".as_slice(),
                    ObjectProperty {
                        name: b"protected".as_slice(),
                        visibility: ObjectPropertyVisibility::Protected,
                        value: Value::Integer(42),
                    },
                )),
            ),
            (
                b"s:18:\"\0ClassName\0private\";s:5:\"value\";".as_slice(),
                Some((
                    b"".as_slice(),
                    ObjectProperty {
                        name: b"private".as_slice(),
                        visibility: ObjectPropertyVisibility::Private,
                        value: Value::String(b"value".as_slice()),
                    },
                )),
            ),
        ];

        run_cases(object_property, &cases);
    }

    #[test]
    fn parse_object() {
        let cases = [
            (
                b"O:4:\"Test\":3:{s:6:\"public\";i:1;s:12:\"\0*\0protected\";i:2;s:13:\"\0Test\0private\";i:3;}".as_slice(),
                Some((
                    b"".as_slice(),
                    Value::Object {
                        class_name: b"Test".as_slice(),
                        properties: vec![
                            ObjectProperty {
                                name: b"public".as_slice(),
                                visibility: ObjectPropertyVisibility::Public,
                                value: Value::Integer(1),
                            },
                            ObjectProperty {
                                name: b"protected".as_slice(),
                                visibility: ObjectPropertyVisibility::Protected,
                                value: Value::Integer(2),
                            },
                            ObjectProperty {
                                name: b"private".as_slice(),
                                visibility: ObjectPropertyVisibility::Private,
                                value: Value::Integer(3),
                            },
                        ],
                    }
                )),
            ),
        ];

        run_cases(value_object, &cases);
    }

    #[test]
    fn parse_any_value() {
        let cases = [
            (
                b"i:-0;".as_slice(),
                Some((b"".as_slice(), Value::Integer(0))),
            ),
            (
                b"i:3465;".as_slice(),
                Some((b"".as_slice(), Value::Integer(3465))),
            ),
            (b"N;".as_slice(), Some((b"".as_slice(), Value::Null))),
            (
                b"b:0;".as_slice(),
                Some((b"".as_slice(), Value::Boolean(false))),
            ),
            (
                b"i:3465;N;".as_slice(),
                Some((b"N;".as_slice(), Value::Integer(3465))),
            ),
            (
                b"d:-10;".as_slice(),
                Some((b"".as_slice(), Value::Decimal(-10.0))),
            ),
            (
                b"d:4.123456789;".as_slice(),
                Some((b"".as_slice(), Value::Decimal(4.123_456_789))),
            ),
            (
                b"s:12:\"123456\";7890\";".as_slice(),
                Some((b"".as_slice(), Value::String("123456\";7890".as_ref()))),
            ),
            (
                b"O:7:\"MyClass\":1:{s:10:\"myProperty\";i:42;}".as_slice(),
                Some((
                    b"".as_slice(),
                    Value::Object {
                        class_name: b"MyClass".as_slice(),
                        properties: vec![ObjectProperty {
                            name: b"myProperty".as_slice(),
                            visibility: ObjectPropertyVisibility::Public,
                            value: Value::Integer(42),
                        }],
                    },
                )),
            ),
            (
                b"a:2:{i:0;s:3:\"foo\";i:1;R:2;}",
                Some((
                    b"".as_slice(),
                    Value::Array(vec![
                        (ArrayKey::Integer(0), Value::String(b"foo".as_slice())),
                        (
                            ArrayKey::Integer(1),
                            Value::ValueReference(2.try_into().unwrap()),
                        ),
                    ]),
                )),
            ),
            (
                b"O:8:\"stdClass\":1:{s:3:\"foo\";r:1;}",
                Some((
                    b"".as_slice(),
                    Value::Object {
                        class_name: b"stdClass".as_slice(),
                        properties: vec![ObjectProperty {
                            name: b"foo".as_slice(),
                            visibility: ObjectPropertyVisibility::Public,
                            value: Value::ObjectReference(1.try_into().unwrap()),
                        }],
                    },
                )),
            ),
            (
                b"C:23:\"CustomSerializableClass\":6:{foobar}",
                Some((
                    b"".as_slice(),
                    Value::CustomObject {
                        class_name: b"CustomSerializableClass".as_slice(),
                        data: b"foobar".as_slice(),
                    },
                )),
            ),
            (b"i;-0;".as_slice(), None),
            (b"i:12_320;".as_slice(), None),
            (b"i:86A23;".as_slice(), None),
            (b"b;2;".as_slice(), None),
            (b"k:2;".as_slice(), None),
            (b"d:12-12".as_slice(), None),
        ];

        run_cases(any_value, &cases);
    }

    #[test]
    fn decode_session() {
        let cases = [
            (
                b"".as_slice(),
                Some(vec![]),
            ),
            (
                b"foo|i:42;".as_slice(),
                Some(
                    vec![
                        SessionEntry {
                            key: b"foo",
                            value: Value::Integer(42),
                        },
                    ],
                ),
            ),
            (
                b"|s:4:\"okay\";".as_slice(),
                Some(
                    vec![
                        SessionEntry {
                            key: b"",
                            value: Value::String(b"okay"),
                        },
                    ],
                ),
            ),
            (
                b"|s:4:\"okay\";foo|i:42;".as_slice(),
                Some(
                    vec![
                        SessionEntry {
                            key: b"",
                            value: Value::String(b"okay"),
                        },
                        SessionEntry {
                            key: b"foo",
                            value: Value::Integer(42),
                        },
                    ],
                ),
            ),
            (
                b"|s:4:\"okay\";foo|i:42;a:1{\"not an array\"}|a:1:{s:10:\"some-value\";s:15:\"\"other\";\"value\"\";}".as_slice(),
                Some(
                    vec![
                        SessionEntry {
                            key: b"",
                            value: Value::String(b"okay"),
                        },
                        SessionEntry {
                            key: b"foo",
                            value: Value::Integer(42),
                        },
                        SessionEntry {
                            key: b"a:1{\"not an array\"}",
                            value: Value::Array(vec![
                                (
                                    ArrayKey::String(b"some-value"),
                                    Value::String(r#""other";"value""#.as_bytes())
                                )
                            ]),
                        },
                    ],
                ),
            ),
        ];

        for (input, expected) in cases {
            let output = session_decode(input);
            match expected {
                Some(expected) => assert_eq!(output, Ok(expected)),
                None => assert!(output.is_err()),
            }
        }
    }
}
