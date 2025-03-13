#![forbid(unsafe_code)]
// Silence the noise in development!
#![cfg_attr(debug_assertions, allow(dead_code, unused_variables))]
// Docs and linting rules
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]
#![cfg_attr(test, allow(clippy::float_cmp))]
#![cfg_attr(not(test), deny(clippy::print_stdout, clippy::dbg_macro))]
// - Lint for missing docs
#![cfg_attr(not(debug_assertions), deny(missing_docs))]
//!
//! # thestack
//!
//! Write some docs about the thestack here!
#![doc = include_str!("../docs/thestack.md")]

mod context;
mod provider;
mod state;

// TODO -> Inline docs
pub mod security {
    //!
    //! This is the security module for thestack
    pub use crate::{
        context::{
            handler::Handler,
            sign_in::{SignInContext, SignInHandler},
            sign_out::{SignOutContext, SignOutHandler},
        },
        provider::Provider,
        state::State,
    };
    pub mod state {
        pub use crate::state::Entry;
    }
}

#[doc(inline)]
/// Re-exports the [`new`](https://docs.rs/derive-new/latest/derive_new/derive/fn.new.html) macro from the `derive-new` crate.
///
/// This is a procedural macro that derives a basic constructor for structs and enums.
/// The macro is provided by the [`derive-new`](https://crates.io/crates/derive-new) crate.
///
/// # Features
/// This re-export is only available when the `derive-new` feature is enabled.
///
/// # Example
/// ```rust
/// use your_crate::new;
///
/// #[derive(new)]
/// struct Point {
///     x: i32,
///     y: i32,
/// }
///
/// let point = Point::new(1, 2);
/// ```
///
/// # License
/// The `derive-new` crate is licensed under either of:
/// * Apache License, Version 2.0
/// * MIT License
///
/// at your option.
///
/// # Attribution
/// Original crate authored by Nika Layzell and maintained by the Rust community.
/// For more information, visit the [derive-new repository](https://github.com/nrc/derive-new).
#[cfg(feature = "new")]
pub use derive_new::new;

/// Retrieves an environment variable's value.
///
/// # Arguments
///
/// * `name` - The name of the environment variable to retrieve
///
/// # Returns
///
/// * `Ok(String)` - The value of the environment variable
/// * `Err(String)` - Error message if the variable is not found
///
/// # Example
///
/// ```rust
/// let database_url = get_env("DATABASE_URL")?;
/// ```
#[inline]
pub fn get_env(name: &'static str) -> Result<String, String> {
    std::env::var(name).map_err(|_| format!("Environment variable '{name}' not found"))
}

/// Retrieves and parses an environment variable into a specified type.
///
/// # Type Parameters
///
/// * `T` - The target type that implements FromStr
///
/// # Arguments
///
/// * `name` - The name of the environment variable to retrieve
///
/// # Returns
///
/// * `Ok(T)` - The parsed value of the environment variable
/// * `Err(String)` - Error message if variable is not found or parsing fails
///
/// # Example
///
/// ```rust
/// let port: u16 = get_env_parse("PORT")?;
/// ```
#[inline]
pub fn get_env_parse<T>(name: &'static str) -> Result<T, String>
where
    T: std::str::FromStr,
{
    get_env(name).and_then(|value| {
        value
            .parse::<T>()
            .map_err(|_| format!("Failed to parse '{name}' into {}", std::any::type_name::<T>()))
    })
}

#[macro_export]
macro_rules! lazy_lock {
    ($definition:expr) => {
        std::sync::LazyLock::new(|| $definition)
    };
    (() => $block:block) => {
        std::sync::LazyLock::new(|| $block)
    };
}

/// Ensure a predicate is true; return an error otherwise.
#[macro_export]
macro_rules! ensure {
    ($pred:expr, $err:expr) => {
        if !$pred {
            return Err($err);
        }
    };
}

/// Always return an error; used for early exits.
#[macro_export]
macro_rules! err {
    ($err:expr) => {
        return Err($err)
    };
}

/// Clone an expression.
#[macro_export]
macro_rules! clone {
    ($expr:expr) => {
        $expr.clone()
    };
}

/// Get the duration since a specific `Instant`.
#[macro_export]
macro_rules! duration_since {
    ($earlier:expr) => {
        std::time::Instant::now().duration_since($earlier)
    };
}

/// Simplified string formatting.
#[macro_export]
macro_rules! f {
    ($($arg:tt)*) => {
        format!($($arg)*)
    };
}

/// Implement `Error` and `Display` for a type.
#[macro_export]
macro_rules! impl_error_display {
    ($ident:ident) => {
        impl std::error::Error for $ident {}
        impl std::fmt::Display for $ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Error: {:?}", self)
            }
        }
    };
}

/// Return `Some` for an optional value, or `None` otherwise.
#[macro_export]
macro_rules! opt {
    ($( $value:expr )?) => {
        match ($(Some($value))?) {
            Some(_) => Some($value),
            _ => None,
        }
    };
}

/// Create an `Arc` for a value.
#[macro_export]
macro_rules! arc {
    ($value:expr) => {
        std::sync::Arc::new($value)
    };
}

/// Create a `Mutex` for a value.
#[macro_export]
macro_rules! mutex {
    ($value:expr) => {
        std::sync::Mutex::new($value)
    };
}

/// Create a static reference from a type and data using `LazyLock`.
#[macro_export]
macro_rules! to_static {
    ($ty:ty, $data:expr) => {
        static DATA: std::sync::LazyLock<$ty> = $crate::lazy_lock!($data);
        &*DATA
    };
}

/// A macro to create `String` instances in various formats.
///
/// # Usage
/// ```rust
/// use thestack::string;
///
/// let empty = string!();
/// let simple = string!("Hello, world!");
/// let with_capacity = string!("Hello", 20);
///
/// let from_utf8 = string!(u8: vec![72, 101, 108, 108, 111]).unwrap();
/// let from_utf8_lossy = string!(u8l: &[255, 72, 101, 108, 108, 111]);
///
/// let from_utf16 = string!(u16: &[72, 101, 108, 108, 111]).unwrap();
/// let from_utf16_lossy = string!(u16l: &[72, 101, 108, 108, 111]);
///
/// let repeat_chars = string!(repeat: 'A', 5);
/// let from_char_iter = string!(iter: ['H', 'e', 'l', 'l', 'o']);
///
/// let with_capacity_only = string!(capacity: 30);
/// ```
///
/// # Supported Variants
/// - `string!()` → Creates an empty `String`.
/// - `string!(content)` → Converts input into a `String`.
/// - `string!(content, capacity)` → Creates a `String` with initial capacity and content.
/// - `string!(u8: content)` → Creates a `String` from `Vec<u8>`. Can fail.
/// - `string!(u8l: content)` → Creates a lossy `String` from `&[u8]`.
/// - `string!(u16: content)` → Creates a `String` from `&[u16]`. Can fail.
/// - `string!(u16l: content)` → Creates a lossy `String` from `&[u16]`.
/// - `string!(repeat: char, count)` → Creates a `String` by repeating a character `count` times.
/// - `string!(iter: iterable)` → Creates a `String` from an iterator of characters.
/// - `string!(capacity: size)` → Creates an empty `String` with the given capacity.
/// A macro for flexible string creation with various input types and configurations.
///
/// # Variants
///
/// - `string!()` - Creates an empty string
/// - `string!("content")` - Creates a string from a string literal
/// - `string!("content", capacity)` - Creates a string with specified content and capacity
/// - `string!(u8: vec![...])` - Creates a string from UTF-8 bytes (Vec<u8>)
/// - `string!(u8l: &[...])` - Creates a string from UTF-8 bytes with lossy conversion
/// - `string!(u16: &[...])` - Creates a string from UTF-16 bytes
/// - `string!(u16l: &[...])` - Creates a string from UTF-16 bytes with lossy conversion
/// - `string!(repeat: 'c', n)` - Creates a string by repeating a character n times
/// - `string!(iter: iterator)` - Creates a string from an iterator of chars
/// - `string!(capacity: n)` - Creates an empty string with specified capacity
///
/// # Examples
///
/// ```
/// // Empty string
/// let empty = string!();
///
/// // From string literal
/// let hello = string!("Hello, World!");
///
/// // With capacity
/// let with_cap = string!("Hello", 10);
///
/// // From UTF-8 bytes
/// let bytes = vec![72, 101, 108, 108, 111];
/// let from_utf8 = string!(u8: bytes);
///
/// // From UTF-16 bytes
/// let utf16 = vec![72, 101, 108, 108, 111];
/// let from_utf16 = string!(u16: &utf16);
///
/// // Repeat character
/// let repeated = string!(repeat: '*', 5); // "*****"
///
/// // From iterator
/// let chars = vec!['H', 'e', 'l', 'l', 'o'];
/// let from_iter = string!(iter: chars);
///
/// // With capacity
/// let reserved = string!(capacity: 100);
/// ```
///
/// # Note
///
/// The macro provides both safe and potentially fallible conversions:
/// - `u8` and `u16` variants return `Result` types that should be handled
/// - `u8l` and `u16l` variants perform lossy conversion, always succeeding
#[macro_export]
macro_rules! string {
    // Empty string
    () => {
        String::new()
    };

    // String from content
    ($content:expr) => {
        String::from($content)
    };

    // String from content with specified capacity
    ($content:expr, $cap:expr) => {{
        let mut string = String::with_capacity($cap);
        string.push_str($content);
        string
    }};

    // String from Vec<u8> (UTF-8), returns Result<String, Utf8Error>
    (u8: $content:expr) => {
        String::from_utf8($content)
    };

    // Lossy String from &[u8] (UTF-8)
    (u8l: $content:expr) => {
        String::from_utf8_lossy($content).to_string()
    };

    // String from &[u16] (UTF-16), returns Result<String, Utf16Error>
    (u16: $content:expr) => {
        String::from_utf16($content)
    };

    // Lossy String from &[u16] (UTF-16)
    (u16l: $content:expr) => {
        String::from_utf16_lossy($content)
    };

    // Repeat a character `count` times
    (repeat: $ch:expr, $count:expr) => {
        std::iter::repeat($ch)
            .take($count)
            .collect::<String>()
    };

    // Create String from iterator of chars
    (iter: $iterable:expr) => {
        $iterable
            .into_iter()
            .collect::<String>()
    };

    // Create String with specific capacity
    (capacity: $size:expr) => {
        String::with_capacity($size)
    };
}
