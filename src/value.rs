use std::num::NonZeroUsize;

#[derive(Debug, Clone, PartialEq)]
pub enum Value<'a> {
    Null,
    Boolean(bool),
    Integer(i64),
    Decimal(f64),
    String(&'a [u8]),
    Array(Vec<(ArrayKey<'a>, Value<'a>)>),
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

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ArrayKey<'a> {
    Integer(i64),
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

#[derive(Debug, Clone, PartialEq)]
pub struct SessionEntry<'a> {
    pub key: &'a [u8],
    pub value: Value<'a>,
}
