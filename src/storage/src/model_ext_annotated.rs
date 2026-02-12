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

// [Jules: Rust]
// This is a documentation comment for the module (`//!`), explaining its purpose.
//! Extends [model][crate::model] with types that improve type safety and/or
//! ergonomics.

use crate::error::KeyAes256Error;
use base64::{Engine, prelude::BASE64_STANDARD};
use sha2::{Digest, Sha256};

mod open_object_request;
// [Jules: Rust]
// `pub use` re-exports the `OpenObjectRequest` type so it can be accessed directly from this module.
pub use open_object_request::OpenObjectRequest;

// [Jules: SDK]
// `Object` in the SDK is a very large struct with many fields.
// `ObjectHighlights` is a smaller, more focused struct that contains the most commonly used metadata.
/// ObjectHighlights contains select metadata from a [crate::model::Object].
// [Jules: Rust]
// `#[derive(Clone, Debug, Default, PartialEq)]` automatically implements common traits.
// `Default` allows creating an instance with default values (0 for numbers, empty strings, etc.).
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct ObjectHighlights {
    // [Jules: SDK]
    // Google Cloud Storage uses generations for object versioning.
    // If you overwrite an object, the new version gets a new generation number.
    /// The content generation of this object. Used for object versioning.
    pub generation: i64,

    // [Jules: SDK]
    // Metageneration tracks changes to the object's *metadata* (like ACLs or content-type), not the content itself.
    // It resets to 1 when a new generation of the object is created.
    /// The version of the metadata for this generation of this
    /// object. Used for preconditions and for detecting changes in metadata. A
    /// metageneration number is only meaningful in the context of a particular
    /// generation of a particular object.
    pub metageneration: i64,

    /// Content-Length of the object data in bytes, matching [RFC 7230 §3.3.2].
    ///
    /// [rfc 7230 §3.3.2]: https://tools.ietf.org/html/rfc7230#section-3.3.2
    pub size: i64,

    /// Content-Encoding of the object data, matching [RFC 7231 §3.1.2.2].
    ///
    /// [rfc 7231 §3.1.2.2]: https://tools.ietf.org/html/rfc7231#section-3.1.2.2
    pub content_encoding: String,

    /// Hashes for the data part of this object. The checksums of the complete
    /// object regardless of data range. If the object is read in full, the
    /// client should compute one of these checksums over the read object and
    /// compare it against the value provided here.
    // [Jules: Rust]
    // `std::option::Option` expresses that a value might be present (`Some(value)`) or missing (`None`).
    // This is how Rust handles nullability safely.
    pub checksums: std::option::Option<crate::model::ObjectChecksums>,

    /// Storage class of the object.
    pub storage_class: String,

    /// Content-Language of the object data, matching [RFC 7231 §3.1.3.2].
    ///
    /// [rfc 7231 §3.1.3.2]: https://tools.ietf.org/html/rfc7231#section-3.1.3.2
    pub content_language: String,

    /// Content-Type of the object data, matching [RFC 7231 §3.1.1.5]. If an
    /// object is stored without a Content-Type, it is served as
    /// `application/octet-stream`.
    ///
    /// [rfc 7231 §3.1.1.5]: https://tools.ietf.org/html/rfc7231#section-3.1.1.5
    pub content_type: String,

    /// Content-Disposition of the object data, matching [RFC 6266].
    ///
    /// [rfc 6266]: https://tools.ietf.org/html/rfc6266
    pub content_disposition: String,

    /// The etag of the object.
    pub etag: String,
}

// [Jules: SDK]
// Customer-Supplied Encryption Keys (CSEK) allow you to encrypt your data with your own keys before it's stored in Google Cloud.
#[derive(Debug, Clone)]
/// KeyAes256 represents an AES-256 encryption key used with the
/// Customer-Supplied Encryption Keys (CSEK) feature.
///
/// This key must be exactly 32 bytes in length and should be provided in its
/// raw (unencoded) byte format.
///
/// # Examples
///
/// Creating a `KeyAes256` instance from a valid byte slice:
/// ```
/// # use google_cloud_storage::{model_ext::KeyAes256, error::KeyAes256Error};
/// let raw_key_bytes: [u8; 32] = [0x42; 32]; // Example 32-byte key
/// let key_aes_256 = KeyAes256::new(&raw_key_bytes)?;
/// # Ok::<(), KeyAes256Error>(())
/// ```
///
/// Handling an error for an invalid key length:
/// ```
/// # use google_cloud_storage::{model_ext::KeyAes256, error::KeyAes256Error};
/// let invalid_key_bytes: &[u8] = b"too_short_key"; // Less than 32 bytes
/// let result = KeyAes256::new(invalid_key_bytes);
///
/// assert!(matches!(result, Err(KeyAes256Error::InvalidLength)));
/// ```
pub struct KeyAes256 {
    // [Jules: Rust]
    // The key is stored as a fixed-size array of 32 bytes (`[u8; 32]`).
    // This ensures at the type level that if we have an instance of `KeyAes256`, it definitely has 32 bytes.
    key: [u8; 32],
}

impl KeyAes256 {
    // [Jules: Rust]
    // This is the constructor. It takes a slice of bytes (`&[u8]`) and returns a `Result`.
    /// Attempts to create a new [KeyAes256].
    ///
    /// This conversion will succeed only if the input slice is exactly 32 bytes long.
    ///
    /// # Example
    /// ```
    /// # use google_cloud_storage::{model_ext::KeyAes256, error::KeyAes256Error};
    /// let raw_key_bytes: [u8; 32] = [0x42; 32]; // Example 32-byte key
    /// let key_aes_256 = KeyAes256::new(&raw_key_bytes)?;
    /// # Ok::<(), KeyAes256Error>(())
    /// ```
    pub fn new(key: &[u8]) -> std::result::Result<Self, KeyAes256Error> {
        // [Jules: Rust]
        // We match on the length of the slice.
        match key.len() {
            32 => Ok(Self {
                // [Jules: Rust]
                // `try_into()` converts the slice `&[u8]` into a fixed-size array `[u8; 32]`.
                // Since we already checked the length is 32, this unwrap is safe.
                key: key[..32].try_into().unwrap(),
            }),
            _ => Err(KeyAes256Error::InvalidLength),
        }
    }
}

// [Jules: Rust]
// This implements the `From` trait, allowing `KeyAes256` to be converted into `CommonObjectRequestParams`.
// This is useful for passing the key into API request builders.
impl std::convert::From<KeyAes256> for crate::model::CommonObjectRequestParams {
    fn from(value: KeyAes256) -> Self {
        // sha2::digest::generic_array::GenericArray::<T, N>::as_slice is deprecated.
        // Our dependencies need to update to generic_array 1.x.
        // See https://github.com/RustCrypto/traits/issues/2036 for more info.
        #[allow(deprecated)]
        crate::model::CommonObjectRequestParams::new()
            .set_encryption_algorithm("AES256")
            .set_encryption_key_bytes(value.key.to_vec())
            // [Jules: SDK]
            // We must also send the SHA256 hash of the key so Google can verify integrity.
            .set_encryption_key_sha256_bytes(Sha256::digest(value.key).as_slice().to_owned())
    }
}

// [Jules: Rust]
// `Display` allows the key to be printed (e.g., for logging).
// We print the base64 encoded version, not raw bytes, which is safer and standard for keys.
impl std::fmt::Display for KeyAes256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", BASE64_STANDARD.encode(self.key))
    }
}

/// Define read ranges for use with [ReadObject].
///
/// # Example: read the first 100 bytes of an object
/// ```
/// # use google_cloud_storage::client::Storage;
/// # use google_cloud_storage::model_ext::ReadRange;
/// # async fn sample(client: &Storage) -> anyhow::Result<()> {
/// let response = client
///     .read_object("projects/_/buckets/my-bucket", "my-object")
///     .set_read_range(ReadRange::head(100))
///     .send()
///     .await?;
/// println!("response details={response:?}");
/// # Ok(()) }
/// ```
///
/// Cloud Storage supports reading a portion of an object. These portions can
/// be specified as offsets from the beginning of the object, offsets from the
/// end of the object, or as ranges with a starting and ending bytes. This type
/// defines a type-safe interface to represent only valid ranges.
///
/// [ReadObject]: crate::builder::storage::ReadObject
// [Jules: Rust]
// A "tuple struct" wrapping an enum `RequestedRange`.
// `pub(crate)` means the inner field is only accessible within the crate, not by users.
// This enforces encapsulation: users must use the public constructors like `all()`, `offset()`, etc.
#[derive(Clone, Debug, PartialEq)]
pub struct ReadRange(pub(crate) RequestedRange);

impl ReadRange {
    /// Returns a range representing all the bytes in the object.
    ///
    /// # Example
    /// ```
    /// # use google_cloud_storage::client::Storage;
    /// # use google_cloud_storage::model_ext::ReadRange;
    /// # async fn sample(client: &Storage) -> anyhow::Result<()> {
    /// let response = client
    ///     .read_object("projects/_/buckets/my-bucket", "my-object")
    ///     .set_read_range(ReadRange::all())
    ///     .send()
    ///     .await?;
    /// println!("response details={response:?}");
    /// # Ok(()) }
    pub fn all() -> Self {
        Self::offset(0)
    }

    /// Returns a range representing the bytes starting at `offset`.
    ///
    /// # Example
    /// ```
    /// # use google_cloud_storage::client::Storage;
    /// # use google_cloud_storage::model_ext::ReadRange;
    /// # async fn sample(client: &Storage) -> anyhow::Result<()> {
    /// let response = client
    ///     .read_object("projects/_/buckets/my-bucket", "my-object")
    ///     .set_read_range(ReadRange::offset(1_000_000))
    ///     .send()
    ///     .await?;
    /// println!("response details={response:?}");
    /// # Ok(()) }
    pub fn offset(offset: u64) -> Self {
        Self(RequestedRange::Offset(offset))
    }

    /// Returns a range representing the last `count` bytes of the object.
    ///
    /// # Example
    /// ```
    /// # use google_cloud_storage::client::Storage;
    /// # use google_cloud_storage::model_ext::ReadRange;
    /// # async fn sample(client: &Storage) -> anyhow::Result<()> {
    /// let response = client
    ///     .read_object("projects/_/buckets/my-bucket", "my-object")
    ///     .set_read_range(ReadRange::tail(100))
    ///     .send()
    ///     .await?;
    /// println!("response details={response:?}");
    /// # Ok(()) }
    pub fn tail(count: u64) -> Self {
        Self(RequestedRange::Tail(count))
    }

    /// Returns a range representing the first `count` bytes of the object.
    ///
    /// # Example
    /// ```
    /// # use google_cloud_storage::client::Storage;
    /// # use google_cloud_storage::model_ext::ReadRange;
    /// # async fn sample(client: &Storage) -> anyhow::Result<()> {
    /// let response = client
    ///     .read_object("projects/_/buckets/my-bucket", "my-object")
    ///     .set_read_range(ReadRange::head(100))
    ///     .send()
    ///     .await?;
    /// println!("response details={response:?}");
    /// # Ok(()) }
    pub fn head(count: u64) -> Self {
        Self::segment(0, count)
    }

    /// Returns a range representing the `count` bytes starting at `offset`.
    ///
    /// # Example
    /// ```
    /// # use google_cloud_storage::client::Storage;
    /// # use google_cloud_storage::model_ext::ReadRange;
    /// # async fn sample(client: &Storage) -> anyhow::Result<()> {
    /// let response = client
    ///     .read_object("projects/_/buckets/my-bucket", "my-object")
    ///     .set_read_range(ReadRange::segment(1_000_000, 1_000))
    ///     .send()
    ///     .await?;
    /// println!("response details={response:?}");
    /// # Ok(()) }
    pub fn segment(offset: u64, count: u64) -> Self {
        Self(RequestedRange::Segment {
            offset,
            limit: count,
        })
    }
}

// [Jules: Rust]
// This implementation block adds a private helper method `with_range` to `ReadObjectRequest`.
// It translates the high-level `ReadRange` enum into the specific offset/limit fields expected by the API.
impl crate::model::ReadObjectRequest {
    pub(crate) fn with_range(&mut self, range: ReadRange) {
        // The limit for GCS objects is (currently) 5TiB, and the gRPC protocol
        // uses i64 for the offset and limit. Clamping the values to the
        // `[0, i64::MAX]`` range is safe, in that it does not lose any
        // functionality.
        match range.0 {
            RequestedRange::Offset(o) => {
                // [Jules: Rust]
                // `clamp` ensures the value is within the given range.
                // We cast to `i64` because that's what the gRPC message uses.
                self.read_offset = o.clamp(0, i64::MAX as u64) as i64;
            }
            RequestedRange::Tail(t) => {
                // Yes, -i64::MAX is different from i64::MIN, but both are
                // safe in this context.
                // [Jules: SDK]
                // A negative offset in the gRPC API often means "from the end".
                self.read_offset = -(t.clamp(0, i64::MAX as u64) as i64);
            }
            RequestedRange::Segment { offset, limit } => {
                self.read_offset = offset.clamp(0, i64::MAX as u64) as i64;
                self.read_limit = limit.clamp(0, i64::MAX as u64) as i64;
            }
        }
    }
}

// [Jules: Rust]
// The internal enum representing the different types of ranges.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum RequestedRange {
    Offset(u64),
    Tail(u64),
    Segment { offset: u64, limit: u64 },
}

/// Represents the parameters of a [WriteObject] request.
///
/// This type is only used in mocks of the `Storage` client.
///
/// [WriteObject]: crate::builder::storage::WriteObject
#[derive(Debug, PartialEq)]
#[non_exhaustive]
#[allow(dead_code)]
pub struct WriteObjectRequest {
    pub spec: crate::model::WriteObjectSpec,
    pub params: Option<crate::model::CommonObjectRequestParams>,
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::model::ReadObjectRequest;
    use base64::{Engine, prelude::BASE64_STANDARD};
    use test_case::test_case;

    type Result = anyhow::Result<()>;

    /// This is used by the request builder tests.
    pub(crate) fn create_key_helper() -> (Vec<u8>, String, Vec<u8>, String) {
        // Make a 32-byte key.
        let key = vec![b'a'; 32];
        let key_base64 = BASE64_STANDARD.encode(key.clone());

        let key_sha256 = Sha256::digest(key.clone());
        let key_sha256_base64 = BASE64_STANDARD.encode(key_sha256);
        (key, key_base64, key_sha256.to_vec(), key_sha256_base64)
    }

    #[test]
    // This tests converting to KeyAes256 from some different types
    // that can get converted to &[u8].
    fn test_key_aes_256() -> Result {
        let v_slice: &[u8] = &[b'c'; 32];
        KeyAes256::new(v_slice)?;

        let v_vec: Vec<u8> = vec![b'a'; 32];
        KeyAes256::new(&v_vec)?;

        let v_array: [u8; 32] = [b'a'; 32];
        KeyAes256::new(&v_array)?;

        let v_bytes: bytes::Bytes = bytes::Bytes::copy_from_slice(&v_array);
        KeyAes256::new(&v_bytes)?;

        Ok(())
    }

    #[test_case(&[b'a'; 0]; "no bytes")]
    #[test_case(&[b'a'; 1]; "not enough bytes")]
    #[test_case(&[b'a'; 33]; "too many bytes")]
    fn test_key_aes_256_err(input: &[u8]) {
        KeyAes256::new(input).unwrap_err();
    }

    #[test]
    fn test_key_aes_256_to_control_model_object() -> Result {
        let (key, _, key_sha256, _) = create_key_helper();
        let key_aes_256 = KeyAes256::new(&key)?;
        let params = crate::model::CommonObjectRequestParams::from(key_aes_256);
        assert_eq!(params.encryption_algorithm, "AES256");
        assert_eq!(params.encryption_key_bytes, key);
        assert_eq!(params.encryption_key_sha256_bytes, key_sha256);
        Ok(())
    }

    #[test_case(100, 100)]
    #[test_case(u64::MAX, i64::MAX)]
    #[test_case(0, 0)]
    fn apply_offset(input: u64, want: i64) {
        let range = ReadRange::offset(input);
        let mut request = ReadObjectRequest::new();
        request.with_range(range);
        assert_eq!(request.read_offset, want);
        assert_eq!(request.read_limit, 0);
    }

    #[test_case(100, 100)]
    #[test_case(u64::MAX, i64::MAX)]
    #[test_case(0, 0)]
    fn apply_head(input: u64, want: i64) {
        let range = ReadRange::head(input);
        let mut request = ReadObjectRequest::new();
        request.with_range(range);
        assert_eq!(request.read_offset, 0);
        assert_eq!(request.read_limit, want);
    }

    #[test_case(100, -100)]
    #[test_case(u64::MAX, -i64::MAX)]
    #[test_case(0, 0)]
    fn apply_tail(input: u64, want: i64) {
        let range = ReadRange::tail(input);
        let mut request = ReadObjectRequest::new();
        request.with_range(range);
        assert_eq!(request.read_offset, want);
        assert_eq!(request.read_limit, 0);
    }

    #[test_case(100, 100)]
    #[test_case(u64::MAX, i64::MAX)]
    #[test_case(0, 0)]
    fn apply_segment_offset(input: u64, want: i64) {
        let range = ReadRange::segment(input, 2000);
        let mut request = ReadObjectRequest::new();
        request.with_range(range);
        assert_eq!(request.read_offset, want);
        assert_eq!(request.read_limit, 2000);
    }

    #[test_case(100, 100)]
    #[test_case(u64::MAX, i64::MAX)]
    #[test_case(0, 0)]
    fn apply_segment_limit(input: u64, want: i64) {
        let range = ReadRange::segment(1000, input);
        let mut request = ReadObjectRequest::new();
        request.with_range(range);
        assert_eq!(request.read_offset, 1000);
        assert_eq!(request.read_limit, want);
    }

    #[test]
    fn test_key_aes_256_display() -> Result {
        let (key, key_base64, _, _) = create_key_helper();
        let key_aes_256 = KeyAes256::new(&key)?;
        assert_eq!(key_aes_256.to_string(), key_base64);
        Ok(())
    }
}
