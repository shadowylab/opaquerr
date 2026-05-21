/// Defines an error kind enum.
///
/// The generated enum is `#[non_exhaustive]` and derives `Debug`, `Clone`,
/// `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, and `Hash`. A `Display` implementation is generated from
/// the string assigned to each variant.
///
/// # Examples
///
/// ```
/// opaquerr::define_kind! {
///     /// Category for a library error.
///     pub ErrorKind {
///         /// Input is invalid.
///         Invalid => "input is invalid",
///         /// Anything not covered by the categories above.
///         Other => "other error",
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_kind {
    (
        $(#[$enum_meta:meta])*
        $vis:vis $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident => $message:expr
            ),+ $(,)?
        }
    ) => {
        $crate::define_kind! {
            @base
            $(#[$enum_meta])*
            $vis $name {
                $(
                    $(#[$variant_meta])*
                    $variant => $message,
                )+
            }
        }
    };

    (
        $(#[$enum_meta:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident => $message:expr
            ),+ $(,)?
        }
    ) => {
        $crate::define_kind! {
            @base
            $(#[$enum_meta])*
            $vis $name {
                $(
                    $(#[$variant_meta])*
                    $variant => $message,
                )+
            }
        }
    };

    (
        @base
        $(#[$enum_meta:meta])*
        $vis:vis $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident => $message:expr
            ),+ $(,)?
        }
    ) => {
        $(#[$enum_meta])*
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis enum $name {
            $(
                $(#[$variant_meta])*
                $variant,
            )+
        }

        impl $name {
            #[inline]
            const fn as_str(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant => $message,
                    )+
                }
            }
        }

        impl core::fmt::Display for $name {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}

/// Defines an opaque error newtype for an error kind.
///
/// The generated type is `#[repr(transparent)]` and stores an
/// [`opaquerr::Error`](crate::Error) internally. It delegates the standard
/// constructors and trait implementations, while keeping the wrapper local to
/// the crate that invokes the macro. This allows that crate to implement
/// `From` conversions and use the `?` operator.
///
/// With the `alloc` feature enabled, the optional `from` block maps source
/// errors to error kinds by generating `From` implementations that preserve the
/// source error.
///
/// # Examples
///
/// ```
/// opaquerr::define_kind! {
///     pub ErrorKind {
///         Invalid => "input is invalid",
///         Other => "other error",
///     }
/// }
///
/// opaquerr::define_error! {
///     /// Library error.
///     pub Error(ErrorKind)
/// }
///
/// # fn example() -> Result<(), Error> {
/// let result: Result<(), ErrorKind> = Err(ErrorKind::Invalid);
/// result?;
/// Ok(())
/// # }
/// ```
///
/// With the `alloc` feature enabled:
///
/// ```
/// # #[cfg(feature = "alloc")]
/// # {
/// # opaquerr::define_kind! {
/// #     pub ErrorKind {
/// #         Other => "other error",
/// #     }
/// # }
/// opaquerr::define_error! {
///     pub Error(ErrorKind)
///
///     from {
///         core::num::ParseIntError => ErrorKind::Other,
///     }
/// }
/// # }
/// ```
#[cfg(not(feature = "alloc"))]
#[macro_export]
macro_rules! define_error {
    (
        $(#[$error_meta:meta])*
        $vis:vis $name:ident($kind:ty)
        $(;)?
    ) => {
        $crate::define_error! {
            @base
            $(#[$error_meta])*
            $vis $name($kind);
        }
    };

    (
        $(#[$error_meta:meta])*
        $vis:vis $name:ident($kind:ty)
        from {
            $(
                $source:ty => $source_kind:expr
            ),+ $(,)?
        }
        $(;)?
    ) => {
        compile_error!("opaquerr::error! `from` requires the `alloc` feature");
    };

    (
        @base
        $(#[$error_meta:meta])*
        $vis:vis $name:ident($kind:ty);
    ) => {
        $(#[$error_meta])*
        #[repr(transparent)]
        $vis struct $name($crate::Error<$kind>);

        impl $name {
            /// Creates an error without a specific message or source.
            #[inline]
            pub const fn simple(kind: $kind) -> Self {
                Self($crate::Error::simple(kind))
            }

            /// Creates an error with a static message.
            #[inline]
            pub const fn with_static_message(kind: $kind, message: &'static str) -> Self {
                Self($crate::Error::with_static_message(kind, message))
            }

            /// Returns the error kind.
            #[inline]
            pub const fn kind(&self) -> $kind
            where
                $kind: Copy,
            {
                self.0.kind()
            }
        }

        impl core::fmt::Debug for $name
        where
            $kind: core::fmt::Debug,
        {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl core::fmt::Display for $name
        where
            $kind: core::fmt::Display,
        {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl core::error::Error for $name
        where
            $kind: core::fmt::Debug + core::fmt::Display,
        {
            #[inline]
            fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
                self.0.source()
            }
        }

        impl From<$kind> for $name {
            #[inline]
            fn from(kind: $kind) -> Self {
                Self::simple(kind)
            }
        }

        impl From<($kind, &'static str)> for $name {
            #[inline]
            fn from((kind, message): ($kind, &'static str)) -> Self {
                Self::with_static_message(kind, message)
            }
        }
    };
}

/// Defines an opaque error newtype for an error kind.
#[cfg(feature = "alloc")]
#[macro_export]
macro_rules! define_error {
    (
        $(#[$error_meta:meta])*
        $vis:vis $name:ident($kind:ty)
        $(;)?
    ) => {
        $crate::define_error! {
            @base
            $(#[$error_meta])*
            $vis $name($kind);
        }
    };

    (
        $(#[$error_meta:meta])*
        $vis:vis $name:ident($kind:ty)
        from {
            $(
                $source:ty => $source_kind:expr
            ),+ $(,)?
        }
        $(;)?
    ) => {
        $crate::define_error! {
            @base
            $(#[$error_meta])*
            $vis $name($kind);
        }

        $(
            impl From<$source> for $name {
                #[inline]
                fn from(error: $source) -> Self {
                    Self::new($source_kind, error)
                }
            }
        )+
    };

    (
        @base
        $(#[$error_meta:meta])*
        $vis:vis $name:ident($kind:ty);
    ) => {
        $(#[$error_meta])*
        #[repr(transparent)]
        $vis struct $name($crate::Error<$kind>);

        impl $name {
            /// Creates a new error from a kind and an arbitrary error payload.
            #[inline]
            pub fn new<E>(kind: $kind, error: E) -> Self
            where
                E: Into<$crate::BoxedError>,
            {
                Self($crate::Error::new(kind, error))
            }

            /// Creates an error without a specific message or source.
            #[inline]
            pub const fn simple(kind: $kind) -> Self {
                Self($crate::Error::simple(kind))
            }

            /// Creates an error with a static message.
            #[inline]
            pub const fn with_static_message(kind: $kind, message: &'static str) -> Self {
                Self($crate::Error::with_static_message(kind, message))
            }

            /// Returns the error kind.
            #[inline]
            pub const fn kind(&self) -> $kind
            where
                $kind: Copy,
            {
                self.0.kind()
            }
        }

        impl core::fmt::Debug for $name
        where
            $kind: core::fmt::Debug,
        {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl core::fmt::Display for $name
        where
            $kind: core::fmt::Display,
        {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl core::error::Error for $name
        where
            $kind: core::fmt::Debug + core::fmt::Display,
        {
            #[inline]
            fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
                self.0.source()
            }
        }

        impl From<$kind> for $name {
            #[inline]
            fn from(kind: $kind) -> Self {
                Self::simple(kind)
            }
        }

        impl From<($kind, &'static str)> for $name {
            #[inline]
            fn from((kind, message): ($kind, &'static str)) -> Self {
                Self::with_static_message(kind, message)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]

    #[cfg(feature = "alloc")]
    use alloc::string::ToString;
    #[cfg(feature = "alloc")]
    use core::error::Error as _;

    use crate::Error;

    define_kind! {
        /// Test error kind.
        pub ErrorKind {
            /// Invalid input.
            Invalid => "invalid input",
            /// Other error.
            Other => "other error",
        }
    }

    define_error! {
        /// Test opaque error.
        pub TestError(ErrorKind)
    }

    #[cfg(feature = "alloc")]
    #[derive(Debug)]
    struct SourceError;

    #[cfg(feature = "alloc")]
    impl core::fmt::Display for SourceError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str("source error")
        }
    }

    #[cfg(feature = "alloc")]
    impl core::error::Error for SourceError {}

    #[cfg(feature = "alloc")]
    define_error! {
        /// Test mapped opaque error.
        pub MappedError(ErrorKind)

        from {
            SourceError => ErrorKind::Other,
        }
    }

    #[test]
    fn simple_error_uses_kind_display() {
        let error = Error::simple(ErrorKind::Invalid);

        assert_eq!(error.kind(), ErrorKind::Invalid);
        #[cfg(feature = "alloc")]
        assert_eq!(error.to_string(), "invalid input");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn custom_error_preserves_source() {
        let error = Error::new(ErrorKind::Other, "source error");

        assert_eq!(error.kind(), ErrorKind::Other);
        assert_eq!(error.to_string(), "source error");
        assert!(error.source().is_some());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn custom_error_accepts_owned_string_source() {
        let error = Error::new(ErrorKind::Other, "source error".to_string());

        assert_eq!(error.kind(), ErrorKind::Other);
        assert_eq!(error.to_string(), "source error");
        assert!(error.source().is_some());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn opaque_error_new_preserves_source_without_mapping() {
        let error = TestError::new(ErrorKind::Other, SourceError);

        assert_eq!(error.kind(), ErrorKind::Other);
        assert_eq!(error.to_string(), "source error");
        assert!(error.source().is_some());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn opaque_error_new_accepts_boxed_source() {
        let source: crate::BoxedError = alloc::boxed::Box::new(SourceError);
        let error = TestError::new(ErrorKind::Other, source);

        assert_eq!(error.kind(), ErrorKind::Other);
        assert_eq!(error.to_string(), "source error");
        assert!(error.source().is_some());
    }

    #[test]
    fn opaque_error_wraps_inner_error_without_size_overhead() {
        let error = TestError::simple(ErrorKind::Invalid);

        assert_eq!(error.kind(), ErrorKind::Invalid);
        assert_eq!(
            core::mem::size_of::<TestError>(),
            core::mem::size_of::<Error<ErrorKind>>()
        );
    }

    #[test]
    fn opaque_error_converts_from_kind() {
        let error: TestError = ErrorKind::Invalid.into();

        assert_eq!(error.kind(), ErrorKind::Invalid);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn opaque_error_maps_source_errors() {
        let error: MappedError = SourceError.into();

        assert_eq!(error.kind(), ErrorKind::Other);
        assert_eq!(error.to_string(), "source error");
        assert!(error.source().is_some());
    }
}
