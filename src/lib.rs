//! # Muesli
//!
//! *muesli* is Rust implementation of PHP's [`serialize()`][php-serialize], [`unserialize()`][php-unserialize],
//! [`session_encode()`][php-session-encode], and [`session_decode()`][php-session-decode] functions.
//!
//! ## Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! muesli = "0.0.2"
//! ```
//!
//! or run:
//!
//! ```bash
//! cargo add muesli
//! ```
//!
//! ## Usage
//!
//! ```
//! use muesli::{session_encode, SessionEntry, Value};
//!
//! let data = vec![
//!     SessionEntry {
//!         key: b"foo",
//!         value: Value::Integer(42),
//!     },
//!     SessionEntry {
//!         key: b"bar",
//!         value: Value::String(b"baz|qux".as_slice()),
//!     },
//!     SessionEntry {
//!         key: b"pub",
//!         value: Value::Integer(1337),
//!     },
//! ];
//!
//! let mut session = Vec::<u8>::new();
//! session_encode(&mut session, &data).unwrap();
//! assert_eq!(session, b"foo|i:42;bar|s:7:\"baz|qux\";pub|i:1337;".as_slice());
//! ```
//!
//! [php-serialize]: https://www.php.net/manual/en/function.serialize.php
//! [php-unserialize]: https://www.php.net/manual/en/function.unserialize.php
//! [php-session-encode]: https://www.php.net/manual/en/function.session-encode.php
//! [php-session-decode]: https://www.php.net/manual/en/function.session-decode.php

mod de;
mod ser;
pub mod value;

pub use de::{session_decode, unserialize};
pub use ser::{serialize, session_encode};
pub use value::*;
