// ANNOTATED AND DONE
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

//! Extends [model][crate::model] with types that improve type safety and/or
//! ergonomics.

// Import KeyAes256Error from the error module.
use crate::error::KeyAes256Error;
// Import Engine and BASE64_STANDARD from the base64 crate.
use base64::{Engine, prelude::BASE64_STANDARD};
// Import Digest and Sha256 from the sha2 crate.
use sha2::{Digest, Sha256};

// Declare private module open_object_request.
mod open_object_request;
// Re-export OpenObjectRequest from open_object_request module.
pub use open_object_request::OpenObjectRequest;

/// ObjectHighlights contains select metadata from a [crate::model::Object].
// Define a public struct named ObjectHighlights. derive Clone, Debug, Default, and PartialEq traits.
#[derive(Clone, Debug, Default, PartialEq)]
// Mark as non_exhaustive.
#[non_exhaustive]
pub struct ObjectHighlights {
    /// The content generation of this object. Used for object versioning.
    // Public field generation of type i64.
    pub generation: i64,

    /// The version of the metadata for this generation of this
    /// object. Used for preconditions and for detecting changes in metadata. A
    /// metageneration number is only meaningful in the context of a particular
    /// generation of a particular object.
    // Public field metageneration of type i64.
    pub metageneration: i64,

    /// Content-Length of the object data in bytes, matching [RFC 7230 §3.3.2].
    ///
    /// [rfc 7230 §3.3.2]: https://tools.ietf.org/html/rfc7230#section-3.3.2
    // Public field size of type i64.
    pub size: i64,

    /// Content-Encoding of the object data, matching [RFC 7231 §3.1.2.2].
    ///
    /// [rfc 7231 §3.1.2.2]: https://tools.ietf.org/html/rfc7231#section-3.1.2.2
    // Public field content_encoding of type String.
    pub content_encoding: String,

    /// Hashes for the data part of this object. The checksums of the complete
    /// object regardless of data range. If the object is read in full, the
    /// client should compute one of these checksums over the read object and
    /// compare it against the value provided here.
    // Public field checksums of type Option<ObjectChecksums>.
    pub checksums: std::option::Option<crate::model::ObjectChecksums>,

    /// Storage class of the object.
    // Public field storage_class of type String.
    pub storage_class: String,

    /// Content-Language of the object data, matching [RFC 7231 §3.1.3.2].
    ///
    /// [rfc 7231 §3.1.3.2]: https://tools.ietf.org/html/rfc7231#section-3.1.3.2
    // Public field content_language of type String.
    pub content_language: String,

    /// Content-Type of the object data, matching [RFC 7231 §3.1.1.5]. If an
    /// object is stored without a Content-Type, it is served as
    /// `application/octet-stream`.
    ///
    /// [rfc 7231 §3.1.1.5]: https://tools.ietf.org/html/rfc7231#section-3.1.1.5
    // Public field content_type of type String.
    pub content_type: String,

    /// Content-Disposition of the object data, matching [RFC 6266].
    ///
    /// [rfc 6266]: https://tools.ietf.org/html/rfc6266
    // Public field content_disposition of type String.
    pub content_disposition: String,

    /// The etag of the object.
    // Public field etag of type String.
    pub etag: String,
}

// Derive Debug and Clone traits for KeyAes256.
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
// Define a public struct named KeyAes256.
pub struct KeyAes256 {
    // Private field key as a 32-byte array.
    key: [u8; 32],
}

// Implement methods for KeyAes256.
impl KeyAes256 {
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
    // Define a public constructor new that takes a byte slice and returns a Result.
    pub fn new(key: &[u8]) -> std::result::Result<Self, KeyAes256Error> {
        // Match on the length of the key.
        match key.len() {
            // If length is 32, create a new instance.
            32 => Ok(Self {
                key: key[..32].try_into().unwrap(),
            }),
            // Otherwise, return an InvalidLength error.
            _ => Err(KeyAes256Error::InvalidLength),
        }
    }
}

// Implement From<KeyAes256> for CommonObjectRequestParams.
impl std::convert::From<KeyAes256> for crate::model::CommonObjectRequestParams {
    // Define the conversion method.
    fn from(value: KeyAes256) -> Self {
        // sha2::digest::generic_array::GenericArray::<T, N>::as_slice is deprecated.
        // Our dependencies need to update to generic_array 1.x.
        // See https://github.com/RustCrypto/traits/issues/2036 for more info.
        #[allow(deprecated)]
        // Create a new CommonObjectRequestParams and set encryption fields.
        crate::model::CommonObjectRequestParams::new()
            // Set encryption algorithm to AES256.
            .set_encryption_algorithm("AES256")
            // Set encryption key bytes.
            .set_encryption_key_bytes(value.key.to_vec())
            // Set encryption key SHA256 bytes.
            .set_encryption_key_sha256_bytes(Sha256::digest(value.key).as_slice().to_owned())
    }
}

// Implement Display for KeyAes256.
impl std::fmt::Display for KeyAes256 {
    // Define the fmt method.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Format the key as a base64 string.
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
// Define a public struct ReadRange wrapping RequestedRange.
#[derive(Clone, Debug, PartialEq)]
pub struct ReadRange(pub(crate) RequestedRange);

// Implement methods for ReadRange.
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
    // Define a public method all returning Self.
    pub fn all() -> Self {
        // Return a range starting from offset 0.
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
    // Define a public method offset returning Self.
    pub fn offset(offset: u64) -> Self {
        // Wrap RequestedRange::Offset.
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
    // Define a public method tail returning Self.
    pub fn tail(count: u64) -> Self {
        // Wrap RequestedRange::Tail.
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
    // Define a public method head returning Self.
    pub fn head(count: u64) -> Self {
        // Implement head as a segment starting at 0.
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
    // Define a public method segment returning Self.
    pub fn segment(offset: u64, count: u64) -> Self {
        // Wrap RequestedRange::Segment.
        Self(RequestedRange::Segment {
            offset,
            limit: count,
        })
    }
}

// Implement helper method for ReadObjectRequest.
impl crate::model::ReadObjectRequest {
    // Define a crate-private method to apply a ReadRange to the request.
    pub(crate) fn with_range(&mut self, range: ReadRange) {
        // The limit for GCS objects is (currently) 5TiB, and the gRPC protocol
        // uses i64 for the offset and limit. Clamping the values to the
        // `[0, i64::MAX]`` range is safe, in that it does not lose any
        // functionality.
        // Match on the inner RequestedRange.
        match range.0 {
            // Handle Offset range.
            RequestedRange::Offset(o) => {
                // Set read_offset, clamping to i64::MAX.
                self.read_offset = o.clamp(0, i64::MAX as u64) as i64;
            }
            // Handle Tail range.
            RequestedRange::Tail(t) => {
                // Yes, -i64::MAX is different from i64::MIN, but both are
                // safe in this context.
                // Set read_offset to negative value for tail read.
                self.read_offset = -(t.clamp(0, i64::MAX as u64) as i64);
            }
            // Handle Segment range.
            RequestedRange::Segment { offset, limit } => {
                // Set read_offset.
                self.read_offset = offset.clamp(0, i64::MAX as u64) as i64;
                // Set read_limit.
                self.read_limit = limit.clamp(0, i64::MAX as u64) as i64;
            }
        }
    }
}

// Define crate-private enum RequestedRange.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum RequestedRange {
    // Range starting from an offset.
    Offset(u64),
    // Last n bytes.
    Tail(u64),
    // Specific segment with offset and limit.
    Segment { offset: u64, limit: u64 },
}

/// Represents the parameters of a [WriteObject] request.
///
/// This type is only used in mocks of the `Storage` client.
///
/// [WriteObject]: crate::builder::storage::WriteObject
// Define a public struct WriteObjectRequest.
#[derive(Debug, PartialEq)]
// Mark as non_exhaustive.
#[non_exhaustive]
// Allow dead code.
#[allow(dead_code)]
pub struct WriteObjectRequest {
    // Public field spec of type WriteObjectSpec.
    pub spec: crate::model::WriteObjectSpec,
    // Public field params of type Option<CommonObjectRequestParams>.
    pub params: Option<crate::model::CommonObjectRequestParams>,
}

// Conditionally compile the tests module only when running tests.
#[cfg(test)]
pub(crate) mod tests {
    // Import everything from the parent module.
    use super::*;
    // Import ReadObjectRequest from the model module.
    use crate::model::ReadObjectRequest;
    // Import Engine and BASE64_STANDARD from base64.
    use base64::{Engine, prelude::BASE64_STANDARD};
    // Import test_case macro.
    use test_case::test_case;

    // Define a Result alias.
    type Result = anyhow::Result<()>;

    /// This is used by the request builder tests.
    // Define a helper function to create key data for tests.
    pub(crate) fn create_key_helper() -> (Vec<u8>, String, Vec<u8>, String) {
        // Make a 32-byte key.
        let key = vec![b'a'; 32];
        // Encode key to base64.
        let key_base64 = BASE64_STANDARD.encode(key.clone());

        // Compute SHA256 of the key.
        let key_sha256 = Sha256::digest(key.clone());
        // Encode SHA256 to base64.
        let key_sha256_base64 = BASE64_STANDARD.encode(key_sha256);
        // Return the tuple of values.
        (key, key_base64, key_sha256.to_vec(), key_sha256_base64)
    }

    #[test]
    // This tests converting to KeyAes256 from some different types
    // that can get converted to &[u8].
    fn test_key_aes_256() -> Result {
        // Create a byte slice of length 32.
        let v_slice: &[u8] = &[b'c'; 32];
        // Test creation from slice.
        KeyAes256::new(v_slice)?;

        // Create a vector of length 32.
        let v_vec: Vec<u8> = vec![b'a'; 32];
        // Test creation from vector reference.
        KeyAes256::new(&v_vec)?;

        // Create an array of length 32.
        let v_array: [u8; 32] = [b'a'; 32];
        // Test creation from array reference.
        KeyAes256::new(&v_array)?;

        // Create Bytes from array.
        let v_bytes: bytes::Bytes = bytes::Bytes::copy_from_slice(&v_array);
        // Test creation from Bytes reference.
        KeyAes256::new(&v_bytes)?;

        // Return Ok.
        Ok(())
    }

    // Test cases for invalid key lengths.
    #[test_case(&[b'a'; 0]; "no bytes")]
    #[test_case(&[b'a'; 1]; "not enough bytes")]
    #[test_case(&[b'a'; 33]; "too many bytes")]
    fn test_key_aes_256_err(input: &[u8]) {
        // Assert that creation fails.
        KeyAes256::new(input).unwrap_err();
    }

    // Test conversion from KeyAes256 to CommonObjectRequestParams.
    #[test]
    fn test_key_aes_256_to_control_model_object() -> Result {
        // Create key helper data.
        let (key, _, key_sha256, _) = create_key_helper();
        // Create KeyAes256 instance.
        let key_aes_256 = KeyAes256::new(&key)?;
        // Convert to CommonObjectRequestParams.
        let params = crate::model::CommonObjectRequestParams::from(key_aes_256);
        // Assert encryption algorithm is AES256.
        assert_eq!(params.encryption_algorithm, "AES256");
        // Assert encryption key bytes match.
        assert_eq!(params.encryption_key_bytes, key);
        // Assert encryption key SHA256 bytes match.
        assert_eq!(params.encryption_key_sha256_bytes, key_sha256);
        // Return Ok.
        Ok(())
    }

    // Test apply_offset logic with various inputs.
    #[test_case(100, 100)]
    #[test_case(u64::MAX, i64::MAX)]
    #[test_case(0, 0)]
    fn apply_offset(input: u64, want: i64) {
        // Create ReadRange with offset.
        let range = ReadRange::offset(input);
        // Create new ReadObjectRequest.
        let mut request = ReadObjectRequest::new();
        // Apply range to request.
        request.with_range(range);
        // Assert read_offset is correct.
        assert_eq!(request.read_offset, want);
        // Assert read_limit is 0.
        assert_eq!(request.read_limit, 0);
    }

    // Test apply_head logic with various inputs.
    #[test_case(100, 100)]
    #[test_case(u64::MAX, i64::MAX)]
    #[test_case(0, 0)]
    fn apply_head(input: u64, want: i64) {
        // Create ReadRange with head.
        let range = ReadRange::head(input);
        // Create new ReadObjectRequest.
        let mut request = ReadObjectRequest::new();
        // Apply range to request.
        request.with_range(range);
        // Assert read_offset is 0.
        assert_eq!(request.read_offset, 0);
        // Assert read_limit is correct.
        assert_eq!(request.read_limit, want);
    }

    // Test apply_tail logic with various inputs.
    #[test_case(100, -100)]
    #[test_case(u64::MAX, -i64::MAX)]
    #[test_case(0, 0)]
    fn apply_tail(input: u64, want: i64) {
        // Create ReadRange with tail.
        let range = ReadRange::tail(input);
        // Create new ReadObjectRequest.
        let mut request = ReadObjectRequest::new();
        // Apply range to request.
        request.with_range(range);
        // Assert read_offset is correct (negative).
        assert_eq!(request.read_offset, want);
        // Assert read_limit is 0.
        assert_eq!(request.read_limit, 0);
    }

    // Test apply_segment logic for offset with various inputs.
    #[test_case(100, 100)]
    #[test_case(u64::MAX, i64::MAX)]
    #[test_case(0, 0)]
    fn apply_segment_offset(input: u64, want: i64) {
        // Create ReadRange with segment.
        let range = ReadRange::segment(input, 2000);
        // Create new ReadObjectRequest.
        let mut request = ReadObjectRequest::new();
        // Apply range to request.
        request.with_range(range);
        // Assert read_offset is correct.
        assert_eq!(request.read_offset, want);
        // Assert read_limit is 2000.
        assert_eq!(request.read_limit, 2000);
    }

    // Test apply_segment logic for limit with various inputs.
    #[test_case(100, 100)]
    #[test_case(u64::MAX, i64::MAX)]
    #[test_case(0, 0)]
    fn apply_segment_limit(input: u64, want: i64) {
        // Create ReadRange with segment.
        let range = ReadRange::segment(1000, input);
        // Create new ReadObjectRequest.
        let mut request = ReadObjectRequest::new();
        // Apply range to request.
        request.with_range(range);
        // Assert read_offset is 1000.
        assert_eq!(request.read_offset, 1000);
        // Assert read_limit is correct.
        assert_eq!(request.read_limit, want);
    }

    // Test display trait implementation for KeyAes256.
    #[test]
    fn test_key_aes_256_display() -> Result {
        // Create key helper data.
        let (key, key_base64, _, _) = create_key_helper();
        // Create KeyAes256 instance.
        let key_aes_256 = KeyAes256::new(&key)?;
        // Assert to_string() returns base64 string.
        assert_eq!(key_aes_256.to_string(), key_base64);
        // Return Ok.
        Ok(())
    }
}
