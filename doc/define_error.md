Defines an opaque error newtype for an error kind.

The generated type is `#[repr(transparent)]` and stores an
[`opaquerr::Error`](crate::Error) internally. It delegates the standard
constructors and trait implementations, while keeping the wrapper local to the
crate that invokes the macro. This allows that crate to implement `From`
conversions and use the `?` operator.

Equality is structural: errors must use the same representation and have equal
kinds and messages. Source error payloads remain opaque and do not participate
in the comparison.

With the `alloc` feature enabled, the optional `from` block maps source errors
to error kinds by generating `From` implementations that preserve the source
error. Its display output is captured as the error message when the opaque error
is created.

# Examples

```
opaquerr::define_kind! {
    pub ErrorKind {
        Invalid => "input is invalid",
        Other => "other error",
    }
}

opaquerr::define_error! {
    /// Library error.
    pub Error(ErrorKind)
}

# fn example() -> Result<(), Error> {
let result: Result<(), ErrorKind> = Err(ErrorKind::Invalid);
result?;
Ok(())
# }
```

With the `alloc` feature enabled:

```
# #[cfg(feature = "alloc")]
# {
# opaquerr::define_kind! {
#     pub ErrorKind {
#         Other => "other error",
#     }
# }
opaquerr::define_error! {
    pub Error(ErrorKind)

    from {
        core::num::ParseIntError => ErrorKind::Other,
    }
}
# }
```

Source mappings may be conditionally compiled:

```ignore
opaquerr::define_error! {
    pub Error(ErrorKind)

    from {
        #[cfg(feature = "json")]
        serde_json::Error => ErrorKind::Other,
    }
}
```
