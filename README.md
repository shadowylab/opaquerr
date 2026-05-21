# opaquerr

Opaque error type with a user-defined kind.

`opaquerr` is a small `no_std` error helper for libraries that want to expose a
stable error kind while keeping the concrete error payload opaque. The kind stays
easy to compare and match on, while the displayed message can come from the kind,
a custom message, or an underlying source error.

## Installation

```toml
[dependencies]
opaquerr = "<version>"
```

Enable `alloc` when you need boxed source errors:

```toml
[dependencies]
opaquerr = { version = "<version>", features = ["alloc"] }
```

## Example

```rust
opaquerr::define_kind! {
    /// Category for a library error.
    pub ErrorKind {
        /// Input is invalid.
        Invalid => "input is invalid",
        /// Anything not covered by the categories above.
        Other => "other error",
    }
}

opaquerr::define_error! {
    /// Library error.
    pub Error(ErrorKind)
}

pub fn parse(input: &str) -> Result<(), Error> {
    if input.is_empty() {
        return Err(Error::with_static_message(ErrorKind::Invalid, "empty input"));
    }

    Ok(())
}
```

With the `alloc` feature enabled, `define_error!` can map source errors once and then
propagate them with `?`:

```rust,ignore
opaquerr::define_error! {
    pub Error(ErrorKind)

    from {
        core::num::ParseIntError => ErrorKind::Other,
    }
}
```

## Changelog

All notable changes to this library are documented in the [CHANGELOG.md](./CHANGELOG.md).

## License

This project is distributed under the MIT software license. See the [LICENSE](./LICENSE) file for details.
