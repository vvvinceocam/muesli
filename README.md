<p align="center">
  <img src="./muesli-logo.png" alt="a lively bowl of muesli" />
</p>
<h1 align="center">muesli</h1>

> healthy implementation of PHP's serialization and session formats in Rust

[![Made With Rust][made-with-rust]][rust]
[![Crates.io][badge-crates.io]][muesli-crates.io]
[![Docs.rs][badge-docs.rs]][muesli-docs.rs]

*muesli* is Rust implementation of PHP's [`serialize()`][php-serialize], [`unserialize()`][php-unserialize],
[`session_encode()`][php-session-encode], and [`session_decode()`][php-session-decode] functions.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
muesli = "0.0.1"
```

or run:

```bash
cargo add muesli
```

## Usage

```rust
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
    assert_eq!(session, b"foo|i:42;bar|s:7:\"baz|qux\";pub|i:1337;".as_slice());
}
```

## Rust Version Compatibility

Compiler support: requires rustc 1.79.0+

## Development

An automated development environment is provided through [`devenv`](https://devenv.sh) tool. It's the preferred
way to work on `muesli`.

### Main development loop

The `devenv` shell provides the `devloop` command that run linters, tests and build command on files changes:

```shell
devloop
```

[rust]: https://www.rust-lang.org/
[php-serialize]: https://www.php.net/manual/en/function.serialize.php
[php-unserialize]: https://www.php.net/manual/en/function.unserialize.php
[php-session-encode]: https://www.php.net/manual/en/function.session-encode.php
[php-session-decode]: https://www.php.net/manual/en/function.session-decode.php
[made-with-rust]: https://img.shields.io/badge/rust-1.79.0-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust'
[badge-crates.io]: https://img.shields.io/badge/crates.io-v0.0.1-orange.svg?style=for-the-badge 'View on crates.rs'
[muesli-crates.io]: https://crates.io/crates/muesli
[badge-docs.rs]: https://img.shields.io/badge/docs.rs-muesli-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs 'Read doc on docs.rs'
[muesli-docs.rs]: https://docs.rs/muesli
