//! Structures for *raw* representation of deserialized data.
//!
//! Largely based on [https://www.phpinternalsbook.com/php5/classes_objects/serialization.html](https://www.phpinternalsbook.com/php5/classes_objects/serialization.html)
//!
use std::num::NonZeroUsize;

pub type Session<'a> = Vec<SessionEntry<'a>>;

#[derive(Debug, Clone, PartialEq)]
pub struct SessionEntry<'a> {
    pub key: &'a [u8],
    pub value: Value<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value<'a> {
    Null,
    Boolean(bool),
    Long(i64),
    Double(f64),
    String(&'a [u8]),
    Array(Vec<ArrayEntry<'a>>),
    Object {
        class_name: &'a [u8],
        properties: Vec<ObjectProperty<'a>>,
    },
    CustomObject {
        class_name: &'a [u8],
        data: &'a [u8],
    },
    ValueReference(NonZeroUsize),
    ObjectReference(NonZeroUsize),
}
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayEntry<'a> {
    pub key: ArrayKey<'a>,
    pub value: Value<'a>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ArrayKey<'a> {
    Long(i64),
    String(&'a [u8]),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectPropertyVisibility {
    Public,
    Protected,
    Private,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectProperty<'a> {
    pub visibility: ObjectPropertyVisibility,
    pub name: &'a [u8],
    pub value: Value<'a>,
}
