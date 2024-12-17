//! File-system paths

use core::ffi::CStr;

use alloc::{borrow::Cow, ffi::CString, string::String};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("failed to convert to {0}")]
    FailedToConvertTo(&'static str),
}

type Result<T> = core::result::Result<T, ConversionError>;

/// Trait for passing path arguments
pub trait Arg {
    /// Convert to a null-terminated string
    ///
    /// If the contents are a valid C string, returns a borrowed string (cheap), otherwise returns a
    /// copied owned string (costs roughly a memcpy).
    fn to_c_str(&self) -> Result<Cow<CStr>>;

    /// Convert to a Rust string
    fn as_rust_str(&self) -> Result<&str>;

    /// Lossily convert to a Rust string
    ///
    // If the contents are a valid UTF-8 string, returns a borrowed string (cheap), otherwise
    // returns a copied owned string (costs roughly a memcpy).
    fn to_rust_str_lossy(&self) -> Cow<str>;
}

impl Arg for &str {
    fn to_c_str(&self) -> Result<Cow<CStr>> {
        CString::new(self.as_bytes())
            .map(Cow::Owned)
            .or(Err(ConversionError::FailedToConvertTo("c string")))
    }

    fn as_rust_str(&self) -> Result<&str> {
        Ok(self)
    }

    fn to_rust_str_lossy(&self) -> Cow<str> {
        Cow::Borrowed(self)
    }
}

impl Arg for String {
    fn to_c_str(&self) -> Result<Cow<CStr>> {
        CString::new(self.as_bytes())
            .map(Cow::Owned)
            .or(Err(ConversionError::FailedToConvertTo("c string")))
    }

    fn as_rust_str(&self) -> Result<&str> {
        Ok(self)
    }

    fn to_rust_str_lossy(&self) -> Cow<str> {
        Cow::Borrowed(self)
    }
}

impl Arg for CStr {
    fn to_c_str(&self) -> Result<Cow<CStr>> {
        Ok(Cow::Borrowed(self))
    }

    fn as_rust_str(&self) -> Result<&str> {
        self.to_str()
            .or(Err(ConversionError::FailedToConvertTo("rust string")))
    }

    fn to_rust_str_lossy(&self) -> Cow<str> {
        self.to_string_lossy()
    }
}

impl Arg for CString {
    fn to_c_str(&self) -> Result<Cow<CStr>> {
        Ok(Cow::Borrowed(self))
    }

    fn as_rust_str(&self) -> Result<&str> {
        self.to_str()
            .or(Err(ConversionError::FailedToConvertTo("rust string")))
    }

    fn to_rust_str_lossy(&self) -> Cow<str> {
        self.to_string_lossy()
    }
}

impl Arg for Cow<'_, str> {
    fn to_c_str(&self) -> Result<Cow<CStr>> {
        match self {
            Cow::Borrowed(s) => s.to_c_str(),
            Cow::Owned(s) => s.to_c_str(),
        }
    }
    fn as_rust_str(&self) -> Result<&str> {
        match self {
            Cow::Borrowed(s) => s.as_rust_str(),
            Cow::Owned(s) => s.as_rust_str(),
        }
    }
    fn to_rust_str_lossy(&self) -> Cow<str> {
        match self {
            Cow::Borrowed(s) => s.to_rust_str_lossy(),
            Cow::Owned(s) => s.to_rust_str_lossy(),
        }
    }
}

impl Arg for Cow<'_, CStr> {
    fn to_c_str(&self) -> Result<Cow<CStr>> {
        match self {
            Cow::Borrowed(s) => s.to_c_str(),
            Cow::Owned(s) => s.to_c_str(),
        }
    }
    fn as_rust_str(&self) -> Result<&str> {
        match self {
            Cow::Borrowed(s) => s.as_rust_str(),
            Cow::Owned(s) => s.as_rust_str(),
        }
    }
    fn to_rust_str_lossy(&self) -> Cow<str> {
        match self {
            Cow::Borrowed(s) => s.to_rust_str_lossy(),
            Cow::Owned(s) => s.to_rust_str_lossy(),
        }
    }
}
