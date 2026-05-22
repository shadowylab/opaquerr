#![no_std]
#![warn(missing_docs)]
#![warn(rustdoc::bare_urls)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;
use core::{error, fmt};

mod macros;

#[doc(hidden)]
#[cfg(feature = "alloc")]
pub mod __private {
    pub use alloc::boxed::Box;
}

#[cfg(feature = "alloc")]
struct Custom<K> {
    kind: K,
    error: Box<dyn error::Error + Send + Sync>,
}

enum Inner<K> {
    Simple(K),
    Message(K, &'static str),
    #[cfg(feature = "alloc")]
    Custom(Custom<K>),
}

/// An opaque error with a user-defined kind
pub struct Error<K>(Inner<K>);

impl<K> fmt::Debug for Error<K>
where
    K: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Inner::Simple(kind) => f.debug_tuple("Error").field(kind).finish(),
            Inner::Message(kind, message) => {
                f.debug_tuple("Error").field(kind).field(message).finish()
            }
            #[cfg(feature = "alloc")]
            Inner::Custom(e) => f
                .debug_tuple("Error")
                .field(&e.kind)
                .field(&e.error)
                .finish(),
        }
    }
}

impl<K> error::Error for Error<K>
where
    K: fmt::Debug + fmt::Display,
{
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.0 {
            Inner::Simple(_) => None,
            Inner::Message(_, _) => None,
            #[cfg(feature = "alloc")]
            Inner::Custom(e) => Some(&*e.error),
        }
    }
}

impl<K> fmt::Display for Error<K>
where
    K: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Inner::Simple(kind) => kind.fmt(f),
            Inner::Message(_, message) => f.write_str(message),
            #[cfg(feature = "alloc")]
            Inner::Custom(e) => e.error.fmt(f),
        }
    }
}

impl<K> Error<K> {
    /// Creates a new error from a kind and an arbitrary error payload.
    #[inline]
    #[cfg(feature = "alloc")]
    pub fn new<E>(kind: K, error: E) -> Self
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self(Inner::Custom(Custom {
            kind,
            error: error.into(),
        }))
    }

    /// Creates an error without a specific message or source.
    #[inline]
    pub const fn simple(kind: K) -> Self {
        Self(Inner::Simple(kind))
    }

    /// Creates an error with a static message.
    #[inline]
    pub const fn with_static_message(kind: K, message: &'static str) -> Self {
        Self(Inner::Message(kind, message))
    }
}

impl<K> Error<K>
where
    K: Copy,
{
    /// Returns the error kind.
    #[inline]
    pub const fn kind(&self) -> K {
        match &self.0 {
            Inner::Simple(kind) => *kind,
            Inner::Message(kind, _) => *kind,
            #[cfg(feature = "alloc")]
            Inner::Custom(e) => e.kind,
        }
    }
}

impl<K> From<K> for Error<K> {
    #[inline]
    fn from(kind: K) -> Self {
        Self(Inner::Simple(kind))
    }
}

impl<K> From<(K, &'static str)> for Error<K> {
    #[inline]
    fn from((kind, message): (K, &'static str)) -> Self {
        Self::with_static_message(kind, message)
    }
}

#[cfg(feature = "alloc")]
impl<K> From<(K, Box<dyn error::Error + Send + Sync>)> for Error<K> {
    #[inline]
    fn from((kind, error): (K, Box<dyn error::Error + Send + Sync>)) -> Self {
        Self::new(kind, error)
    }
}
