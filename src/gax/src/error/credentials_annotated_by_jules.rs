// [Jules: Rust] This is the standard copyright header found in all files of this project.
// [Jules: Rust] It specifies the license (Apache 2.0) under which the code is released.
// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// [Jules: Rust] We import the `Error` trait from the standard library. This is the base trait for all error types in Rust.
use std::error::Error;
// [Jules: Rust] We import formatting traits. `Debug` is for developer-facing output, `Display` is for user-facing output.
// [Jules: Rust] `Formatter` and `Result` are used when implementing `Display` or `Debug`.
use std::fmt::{Debug, Display, Formatter, Result};
// [Jules: Rust] `Arc` (Atomic Reference Counted) is a smart pointer that enables thread-safe shared ownership of data.
use std::sync::Arc;

// [Jules: Rust] We define a type alias `ArcError`.
// [Jules: Rust] `dyn Error` means this is a "trait object" - it can hold any type that implements the `Error` trait.
// [Jules: Rust] `+ Send + Sync` are trait bounds ensuring the error can be safely sent across threads and shared between threads.
// [Jules: Rust] This is useful because `Arc` requires `Send` and `Sync` to be thread-safe.
type ArcError = Arc<dyn Error + Send + Sync>;

// [Jules: SDK] This struct represents an error that occurred while dealing with authentication credentials.
/// Represents an error using [Credentials].
///
/// The Google Cloud client libraries may experience problems using credentials
/// to create the necessary authentication headers. For example, a temporary
/// failure to retrieve or create [access tokens]. Note that these failures may
/// happen even after the credentials files are successfully loaded and parsed.
///
/// Applications rarely need to create instances of this error type. The
/// exception might be when testing application code, where the application is
/// mocking a client library behavior. Such tests are extremely rare, most
/// applications should only work with the [Error][crate::error::Error] type.
///
/// # Example
/// ```
/// use google_cloud_gax::error::CredentialsError;
/// let mut headers = fetch_headers();
/// while let Err(e) = &headers {
///     if e.is_transient() {
///         headers = fetch_headers();
///     }
/// }
///
/// fn fetch_headers() -> Result<http::HeaderMap, CredentialsError> {
///   # Ok(http::HeaderMap::new())
/// }
/// ```
///
/// [access tokens]: https://cloud.google.com/docs/authentication/token-types
/// [Credentials]: https://docs.rs/google-cloud-auth/latest/google_cloud_auth/credentials/struct.Credential.html
// [Jules: Rust] `#[derive(Clone, Debug)]` automatically implements the `Clone` and `Debug` traits for this struct.
// [Jules: Rust] `Clone` allows creating a duplicate of the error. `Debug` allows printing it with `{:?}`.
#[derive(Clone, Debug)]
pub struct CredentialsError {
    // [Jules: SDK] `is_transient` indicates whether the error is temporary (e.g., network glitch) and might succeed if retried.
    is_transient: bool,
    // [Jules: Rust] `Option<String>` means this field can either hold a `String` or be `None`.
    message: Option<String>,
    // [Jules: Rust] `Option<ArcError>` can hold a shared reference to an underlying error, or `None`.
    source: Option<ArcError>,
}

impl CredentialsError {
    /// Creates a new `CredentialsError`.
    ///
    /// This function is only intended for use in the client libraries
    /// implementation. Application may use this in mocks, though we do not
    /// recommend that you write tests for specific error cases. Most tests
    /// should use the generic [Error][crate::error::Error] type.
    ///
    /// # Example
    /// ```
    /// use google_cloud_gax::error::CredentialsError;
    /// let mut headers = fetch_headers();
    /// while let Err(e) = &headers {
    ///     if e.is_transient() {
    ///         headers = fetch_headers();
    ///     }
    /// }
    ///
    /// fn fetch_headers() -> Result<http::HeaderMap, CredentialsError> {
    ///   # Ok(http::HeaderMap::new())
    /// }
    /// ```
    ///
    /// # Parameters
    /// * `is_transient` - if true, the operation may succeed in future attempts.
    /// * `source` - The underlying error that caused the auth failure.
    // [Jules: Rust] `#[cfg_attr(...)]` applies an attribute based on a configuration predicate.
    // [Jules: Rust] Here, if the feature `_internal-semver` is NOT enabled, the `doc(hidden)` attribute is applied, hiding this function from documentation.
    #[cfg_attr(not(feature = "_internal-semver"), doc(hidden))]
    // [Jules: Rust] This function takes a generic type `T`.
    // [Jules: Rust] The constraints `Error + Send + Sync + 'static` ensure `T` is a thread-safe error type that owns its data.
    pub fn from_source<T: Error + Send + Sync + 'static>(is_transient: bool, source: T) -> Self {
        CredentialsError {
            is_transient,
            // [Jules: Rust] We wrap the source error in an `Arc` to make it shareable and `Clone`-able.
            source: Some(Arc::new(source)),
            message: None,
        }
    }

    /// Creates a new `CredentialsError`.
    ///
    /// This function is only intended for use in the client libraries
    /// implementation. Application may use this in mocks, though we do not
    /// recommend that you write tests for specific error cases. Most tests
    /// should use the generic [Error][crate::error::Error] type.
    ///
    /// # Example
    /// ```
    /// # use google_cloud_gax::error::CredentialsError;
    /// let err = CredentialsError::from_msg(
    ///     true, "simulated retryable error while trying to create credentials");
    /// assert!(err.is_transient());
    /// assert!(format!("{err}").contains("simulated retryable error"));
    /// ```
    ///
    /// # Parameters
    /// * `is_transient` - if true, the operation may succeed in future attempts.
    /// * `message` - The underlying error that caused the auth failure.
    #[cfg_attr(not(feature = "_internal-semver"), doc(hidden))]
    // [Jules: Rust] `T: Into<String>` allows passing any type that can be converted into a `String` (like `&str` or `String`).
    pub fn from_msg<T: Into<String>>(is_transient: bool, message: T) -> Self {
        CredentialsError {
            is_transient,
            // [Jules: Rust] `.into()` performs the conversion to `String`.
            message: Some(message.into()),
            source: None,
        }
    }

    /// Creates a new `CredentialsError`.
    ///
    /// This function is only intended for use in the client libraries
    /// implementation. Application may use this in mocks, though we do not
    /// recommend that you write tests for specific error cases. Most tests
    /// should use the generic [Error][crate::error::Error] type.
    ///
    /// # Example
    /// ```
    /// # use google_cloud_gax::error::CredentialsError;
    /// let source = std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "cannot connect");
    /// let err = CredentialsError::new(
    ///     true,
    ///     "simulated retryable error while trying to create credentials",
    ///     source);
    /// assert!(err.is_transient());
    /// assert!(format!("{err}").contains("simulated retryable error"));
    /// ```
    ///
    /// # Parameters
    /// * `is_transient` - if true, the operation may succeed in future attempts.
    /// * `message` - The underlying error that caused the auth failure.
    #[cfg_attr(not(feature = "_internal-semver"), doc(hidden))]
    // [Jules: Rust] This constructor takes both a message and a source error.
    // [Jules: Rust] It uses a `where` clause to specify the trait bounds for clearer readability.
    pub fn new<M, S>(is_transient: bool, message: M, source: S) -> Self
    where
        M: Into<String>,
        S: std::error::Error + Send + Sync + 'static,
    {
        CredentialsError {
            is_transient,
            message: Some(message.into()),
            source: Some(Arc::new(source)),
        }
    }

    /// Returns true if the error is transient and may succeed in future attempts.
    ///
    /// # Example
    /// ```
    /// # use google_cloud_gax::error::CredentialsError;
    /// let mut headers = fetch_headers();
    /// while let Err(e) = &headers {
    ///     if e.is_transient() {
    ///         headers = fetch_headers();
    ///     }
    /// }
    ///
    /// fn fetch_headers() -> Result<http::HeaderMap, CredentialsError> {
    ///   # Ok(http::HeaderMap::new())
    /// }
    /// ```
    // [Jules: Rust] Simple getter method for the `is_transient` field.
    pub fn is_transient(&self) -> bool {
        self.is_transient
    }
}

// [Jules: Rust] We implement the `std::error::Error` trait to integrate with Rust's error handling ecosystem.
impl std::error::Error for CredentialsError {
    // [Jules: Rust] The `source` method allows traversing the error chain.
    // [Jules: Rust] It returns the underlying cause of the error, if any.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_ref()
            // [Jules: Rust] We need to cast the `Arc`'s content to a `dyn Error` reference.
            .map(|arc| arc.as_ref() as &(dyn std::error::Error + 'static))
    }
}

// [Jules: Rust] Constants for error messages.
const TRANSIENT_MSG: &str = "but future attempts may succeed";
const PERMANENT_MSG: &str = "and future attempts will not succeed";

// [Jules: Rust] Implementing `Display` allows the error to be formatted as a user-friendly string (e.g., with `{}`).
impl Display for CredentialsError {
    /// Formats the error message to include retryability and source.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let msg = if self.is_transient {
            TRANSIENT_MSG
        } else {
            PERMANENT_MSG
        };
        // [Jules: Rust] We format the message based on whether a custom message is present.
        match &self.message {
            None => write!(f, "cannot create auth headers {msg}"),
            Some(m) => write!(f, "{m} {msg}"),
        }
    }
}

// [Jules: Rust] `#[cfg(test)]` marks this module for compilation only during testing (`cargo test`).
#[cfg(test)]
mod tests {
    // [Jules: Rust] `use super::*;` imports everything from the parent module into the test module scope.
    use super::*;
    // [Jules: Rust] `test_case` is a crate that allows parameterized tests.
    use test_case::test_case;

    // [Jules: Rust] `#[test_case(...)]` runs the test function multiple times with different arguments.
    #[test_case(true)]
    #[test_case(false)]
    fn from_source(transient: bool) {
        // [Jules: Rust] We simulate a timestamp error.
        let source = wkt::TimestampError::OutOfRange;
        let got = CredentialsError::from_source(transient, source);
        // [Jules: Rust] `assert_eq!` checks if left == right.
        assert_eq!(got.is_transient(), transient, "{got:?}");
        // [Jules: Rust] We verify that the source error is correctly preserved and can be downcasted back to its original type.
        assert!(
            got.source()
                .and_then(|e| e.downcast_ref::<wkt::TimestampError>())
                .is_some(),
            "{got:?}"
        );
        assert!(
            got.to_string().contains("cannot create auth headers"),
            "{got:?}"
        );
    }

    #[test_case(true)]
    #[test_case(false)]
    fn from_str(transient: bool) {
        let got = CredentialsError::from_msg(transient, "test-only");
        assert_eq!(got.is_transient(), transient, "{got:?}");
        assert!(got.source().is_none(), "{got:?}");
        assert!(got.to_string().contains("test-only"), "{got}");
    }

    #[test_case(true)]
    #[test_case(false)]
    fn new(transient: bool) {
        let source = wkt::TimestampError::OutOfRange;
        let got = CredentialsError::new(transient, "additional information", source);
        assert_eq!(got.is_transient(), transient, "{got:?}");
        assert!(
            got.source()
                .and_then(|e| e.downcast_ref::<wkt::TimestampError>())
                .is_some(),
            "{got:?}"
        );
        assert!(
            got.to_string().contains("additional information"),
            "{got:?}"
        );
    }

    // [Jules: Rust] `#[test]` marks a function as a unit test.
    #[test]
    fn fmt() {
        let e = CredentialsError::from_msg(true, "test-only-err-123");
        let got = format!("{e}");
        assert!(got.contains("test-only-err-123"), "{got}");
        assert!(got.contains(TRANSIENT_MSG), "{got}");

        let e = CredentialsError::from_msg(false, "test-only-err-123");
        let got = format!("{e}");
        assert!(got.contains("test-only-err-123"), "{got}");
        assert!(got.contains(PERMANENT_MSG), "{got}");
    }
}
