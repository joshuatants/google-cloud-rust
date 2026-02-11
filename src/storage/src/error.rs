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

//! Custom errors for the Cloud Storage client.
//!
//! The storage client defines additional error types. These are often returned
//! as the `source()` of an [Error][crate::Error].

// Import Object and ObjectChecksums from the model module.
use crate::model::{Object, ObjectChecksums};

/// Indicates that a checksum mismatch was detected while reading or writing
/// Cloud Storage object.
///
/// When performing a full read of an object, the client library automatically
/// computes the CRC32C checksum (and optionally the MD5 hash) of the received
/// data. At the end of the read The client library automatically computes this
/// checksum to the values reported by the service. If the values do not match,
/// the read operation completes with an error and the error includes this type
/// showing which checksums did not match.
///
/// Likewise, when performing an object write, the client library automatically
/// compares the CRC32C checksum (and optionally the MD5 hash) of the data sent
/// to the service against the values reported by the service when the object is
/// finalized. If the values do not match, the write operation completes with an
/// error and the error includes this type.
// Define a public enum named ChecksumMismatch. derive Clone and Debug traits.
#[derive(Clone, Debug)]
// Mark as non_exhaustive to allow future variants without breaking changes.
#[non_exhaustive]
pub enum ChecksumMismatch {
    /// The CRC32C checksum sent by the service does not match the computed (or expected) value.
    Crc32c { got: u32, want: u32 },

    /// The MD5 hash sent by the service does not match the computed (or expected) value.
    Md5 {
        got: bytes::Bytes,
        want: bytes::Bytes,
    },

    /// The CRC32C checksum **and** the MD5 hash sent by the service do not
    /// match the computed (or expected) values.
    Both {
        got: Box<ObjectChecksums>,
        want: Box<ObjectChecksums>,
    },
}

// Implement the Display trait for ChecksumMismatch to provide user-friendly error messages.
impl std::fmt::Display for ChecksumMismatch {
    // Define the fmt method to format the error message.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Match on self to handle each variant.
        match self {
            // Handle Crc32c mismatch.
            Self::Crc32c { got, want } => write!(
                f,
                "the CRC32C checksums do not match: got=0x{got:08x}, want=0x{want:08x}"
            ),
            // Handle Md5 mismatch.
            Self::Md5 { got, want } => write!(
                f,
                "the MD5 hashes do not match: got={:0x?}, want={:0x?}",
                &got, &want
            ),
            // Handle both checksums mismatch.
            Self::Both { got, want } => {
                write!(
                    f,
                    "both the CRC32C checksums and MD5 hashes do not match: got.crc32c=0x{:08x}, want.crc32c=0x{:08x}, got.md5={:x?}, want.md5={:x?}",
                    got.crc32c.unwrap_or_default(),
                    want.crc32c.unwrap_or_default(),
                    got.md5_hash,
                    want.md5_hash
                )
            }
        }
    }
}

/// Represents errors that can occur when converting to [KeyAes256] instances.
///
/// # Example:
/// ```
/// # use google_cloud_storage::{model_ext::KeyAes256, error::KeyAes256Error};
/// let invalid_key_bytes: &[u8] = b"too_short_key"; // Less than 32 bytes
/// let result = KeyAes256::new(invalid_key_bytes);
///
/// assert!(matches!(result, Err(KeyAes256Error::InvalidLength)));
/// ```
///
/// [KeyAes256]: crate::model_ext::KeyAes256
// Define a public enum named KeyAes256Error. derive thiserror::Error and Debug traits.
#[derive(thiserror::Error, Debug)]
// Mark as non_exhaustive.
#[non_exhaustive]
pub enum KeyAes256Error {
    /// The provided key's length was not exactly 32 bytes.
    // Define the error message for InvalidLength variant.
    #[error("Key has an invalid length: expected 32 bytes.")]
    InvalidLength,
}

// Define a type alias for a boxed error that is Send and Sync.
type BoxedSource = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Represents an error that can occur when reading response data.
// Define a public enum named ReadError. derive thiserror::Error and Debug traits.
#[derive(thiserror::Error, Debug)]
// Mark as non_exhaustive.
#[non_exhaustive]
pub enum ReadError {
    /// The calculated crc32c did not match server provided crc32c.
    // Define the error message for ChecksumMismatch variant.
    #[error("checksum mismatch {0}")]
    ChecksumMismatch(ChecksumMismatch),

    /// The read was interrupted before all the expected bytes arrived.
    // Define the error message for ShortRead variant.
    #[error("missing {0} bytes at the end of the stream")]
    ShortRead(u64),

    /// The read received more bytes than expected.
    // Define the error message for LongRead variant.
    #[error("too many bytes received: expected {expected}, stopped read at {got}")]
    LongRead { got: u64, expected: u64 },

    /// Only 200 and 206 status codes are expected in successful responses.
    // Define the error message for UnexpectedSuccessCode variant.
    #[error("unexpected success code {0} in read request, only 200 and 206 are expected")]
    UnexpectedSuccessCode(u16),

    /// Successful HTTP response must include some headers.
    // Define the error message for MissingHeader variant.
    #[error("the response is missing '{0}', a required header")]
    MissingHeader(&'static str),

    /// The received header format is invalid.
    // Define the error message for BadHeaderFormat variant.
    #[error("the format for header '{0}' is incorrect")]
    BadHeaderFormat(&'static str, #[source] BoxedSource),

    /// A bidi read was interrupted with an unrecoverable error.
    // Define the error message for UnrecoverableBidiReadInterrupt variant.
    #[error("cannot recover from an underlying read error: {0}")]
    UnrecoverableBidiReadInterrupt(#[source] std::sync::Arc<crate::Error>),

    /// A bidi read received an invalid offset.
    ///
    /// # Troubleshooting
    ///
    /// This indicates a bug in the client, the service, or a message corrupted
    /// while in transit. Please [open an issue] or contact [Google Cloud support]
    /// with as much detail as possible.
    ///
    /// [open an issue]: https://github.com/googleapis/google-cloud-rust/issues/new/choose
    /// [Google Cloud support]: https://cloud.google.com/support
    // Define the error message for InvalidBidiStreamingReadResponse variant.
    #[error("the bidi streaming read response is invalid: {0}")]
    InvalidBidiStreamingReadResponse(#[source] BoxedSource),
}

// Implement methods for ReadError.
impl ReadError {
    // Define a helper function to create an InvalidBidiStreamingReadResponse error when offsets don't match.
    pub(crate) fn bidi_out_of_order(expected: i64, got: i64) -> Self {
        // Return a new InvalidBidiStreamingReadResponse error with a formatted message.
        Self::InvalidBidiStreamingReadResponse(
            format!("message offset mismatch, expected={expected}, got={got}").into(),
        )
    }
}

/// An unrecoverable problem in the upload protocol.
///
/// # Example
/// ```
/// # use google_cloud_storage::{client::Storage, error::WriteError};
/// # async fn sample(client: &Storage) -> anyhow::Result<()> {
/// use std::error::Error as _;
/// let writer = client
///     .write_object("projects/_/buckets/my-bucket", "my-object", "hello world")
///     .set_if_generation_not_match(0);
/// match writer.send_buffered().await {
///     Ok(object) => println!("Successfully created the object {object:?}"),
///     Err(error) if error.is_serialization() => {
///         println!("Some problem {error:?} sending the data to the service");
///         if let Some(m) = error.source().and_then(|e| e.downcast_ref::<WriteError>()) {
///             println!("{m}");
///         }
///     },
///     Err(e) => return Err(e.into()), // not handled in this example
/// }
/// # Ok(()) }
/// ```
///
// Define a public enum named WriteError. derive thiserror::Error and Debug traits.
#[derive(thiserror::Error, Debug)]
// Mark as non_exhaustive.
#[non_exhaustive]
pub enum WriteError {
    /// The service has "uncommitted" previously persisted bytes.
    ///
    /// # Troubleshoot
    ///
    /// In the resumable upload protocol the service reports how many bytes are
    /// persisted. This error indicates that the service previously reported
    /// more bytes as persisted than in the latest report. This could indicate:
    /// - a corrupted message from the service, either the earlier message
    ///   reporting more bytes persisted than actually are, or the current
    ///   message indicating fewer bytes persisted.
    /// - a bug in the service, where it reported bytes as persisted when they
    ///   were not.
    /// - a bug in the client, maybe storing the incorrect byte count, or
    ///   parsing the messages incorrectly.
    ///
    /// All of these conditions indicate a bug, and in Rust it is idiomatic to
    /// `panic!()` when a bug is detected. However, in this case it seems more
    /// appropriate to report the problem, as the client library cannot
    /// determine the location of the bug.
    // Define the error message for UnexpectedRewind variant.
    #[error(
        "the service previously persisted {offset} bytes, but now reports only {persisted} as persisted"
    )]
    UnexpectedRewind { offset: u64, persisted: u64 },

    /// The service reports more bytes persisted than sent.
    ///
    /// # Troubleshoot
    ///
    /// Most likely this indicates that two concurrent uploads are using the
    /// same session. Review your application design to avoid concurrent
    /// uploads.
    ///
    /// It is possible that this indicates a bug in the service, client, or
    /// messages corrupted in transit.
    // Define the error message for TooMuchProgress variant.
    #[error("the service reports {persisted} bytes as persisted, but we only sent {sent} bytes")]
    TooMuchProgress { sent: u64, persisted: u64 },

    /// The checksums reported by the service do not match the expected checksums.
    ///
    /// # Troubleshoot
    ///
    /// The client library compares the CRC32C checksum and/or MD5 hash of the
    /// uploaded data against the hash reported by the service at the end of
    /// the upload. This error indicates the hashes did not match.
    ///
    /// If you provided known values for these checksums verify those values are
    /// correct.
    ///
    /// Otherwise, this is probably a data corruption problem. These are
    /// notoriously difficult to root cause. They probably indicate faulty
    /// equipment, such as the physical machine hosting your client, the network
    /// elements between your client and the service, or the physical machine
    /// hosting the service.
    ///
    /// If possible, resend the data from a different machine.
    // Define the error message for ChecksumMismatch variant.
    #[error("checksum mismatch {mismatch} when uploading {} to {}", object.name, object.bucket)]
    ChecksumMismatch {
        mismatch: ChecksumMismatch,
        object: Box<Object>,
    },
}

// Define a type alias for a boxed error that is Send and Sync.
type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Signed URL creation errors.
// Define a public struct named SigningError. derive thiserror::Error and Debug traits.
#[derive(thiserror::Error, Debug)]
// Delegate error formatting to the inner error.
#[error(transparent)]
pub struct SigningError(SigningErrorKind);

// Implement methods for SigningError.
impl SigningError {
    // Check if the error is a signing error.
    pub fn is_signing(&self) -> bool {
        matches!(self.0, SigningErrorKind::Signing(_))
    }

    // Check if the error is an invalid parameter error.
    pub fn is_invalid_parameter(&self) -> bool {
        matches!(self.0, SigningErrorKind::InvalidParameter(_, _))
    }

    /// A problem to sign the URL.
    // Helper to create a signing error from a source error.
    pub(crate) fn signing<T>(source: T) -> SigningError
    where
        T: Into<BoxError>,
    {
        SigningError(SigningErrorKind::Signing(source.into()))
    }

    /// A problem to sign the URL due to invalid input.
    // Helper to create an invalid parameter error.
    pub(crate) fn invalid_parameter<S: Into<String>, T>(field: S, source: T) -> SigningError
    where
        T: Into<BoxError>,
    {
        SigningError(SigningErrorKind::InvalidParameter(
            field.into(),
            source.into(),
        ))
    }
}

// Define private enum SigningErrorKind. derive thiserror::Error and Debug.
#[derive(thiserror::Error, Debug)]
// Allow dead code warnings for now.
#[allow(dead_code)]
enum SigningErrorKind {
    /// The signing operation failed.
    // Define error message for Signing variant.
    #[error("signing failed: {0}")]
    Signing(#[source] BoxError),

    /// An invalid input was provided to generate a signed URL.
    // Define error message for InvalidParameter variant.
    #[error("invalid `{0}` parameter: {1}")]
    InvalidParameter(String, #[source] BoxError),
}

// Conditionally compile the tests module only when running tests.
#[cfg(test)]
mod tests {
    // Import everything from the parent module.
    use super::*;

    // Test case for CRC32C checksum mismatch formatting.
    #[test]
    fn mismatch_crc32c() {
        // Create a ChecksumMismatch::Crc32c instance.
        let value = ChecksumMismatch::Crc32c {
            got: 0x01020304_u32,
            want: 0x02030405_u32,
        };
        // Convert to string.
        let fmt = value.to_string();
        // Assert it contains the "got" value.
        assert!(fmt.contains("got=0x01020304"), "{value:?} => {fmt}");
        // Assert it contains the "want" value.
        assert!(fmt.contains("want=0x02030405"), "{value:?} => {fmt}");
    }

    // Test case for MD5 checksum mismatch formatting.
    #[test]
    fn mismatch_md5() {
        // Create a ChecksumMismatch::Md5 instance.
        let value = ChecksumMismatch::Md5 {
            got: bytes::Bytes::from_owner([0x01_u8, 0x02, 0x03, 0x04]),
            want: bytes::Bytes::from_owner([0x02_u8, 0x03, 0x04, 0x05]),
        };
        // Convert to string.
        let fmt = value.to_string();
        // Assert it contains the "got" value formatted as bytes.
        assert!(
            fmt.contains(r#"got=b"\x01\x02\x03\x04""#),
            "{value:?} => {fmt}"
        );
        // Assert it contains the "want" value formatted as bytes.
        assert!(
            fmt.contains(r#"want=b"\x02\x03\x04\x05""#),
            "{value:?} => {fmt}"
        );
    }

    // Test case for both checksums mismatch formatting.
    #[test]
    fn mismatch_both() {
        // Create the "got" ObjectChecksums.
        let got = ObjectChecksums::new()
            .set_crc32c(0x01020304_u32)
            .set_md5_hash(bytes::Bytes::from_owner([0x01_u8, 0x02, 0x03, 0x04]));
        // Create the "want" ObjectChecksums.
        let want = ObjectChecksums::new()
            .set_crc32c(0x02030405_u32)
            .set_md5_hash(bytes::Bytes::from_owner([0x02_u8, 0x03, 0x04, 0x05]));
        // Create a ChecksumMismatch::Both instance.
        let value = ChecksumMismatch::Both {
            got: Box::new(got),
            want: Box::new(want),
        };
        // Convert to string.
        let fmt = value.to_string();
        // Assert it contains the "got" CRC32C value.
        assert!(fmt.contains("got.crc32c=0x01020304"), "{value:?} => {fmt}");
        // Assert it contains the "want" CRC32C value.
        assert!(fmt.contains("want.crc32c=0x02030405"), "{value:?} => {fmt}");
        // Assert it contains the "got" MD5 value.
        assert!(
            fmt.contains(r#"got.md5=b"\x01\x02\x03\x04""#),
            "{value:?} => {fmt}"
        );
        // Assert it contains the "want" MD5 value.
        assert!(
            fmt.contains(r#"want.md5=b"\x02\x03\x04\x05""#),
            "{value:?} => {fmt}"
        );
    }

    // Test case for signing error formatting.
    #[test]
    fn signing_errors() {
        // Create a signing error.
        let value = SigningError::signing("sign error".to_string());
        // Convert to string.
        let fmt = value.to_string();
        // Assert the formatted string contains the error message.
        assert!(
            fmt.contains("signing failed: sign error"),
            "{value:?} => {fmt}"
        );

        // Create an invalid parameter error.
        let value = SigningError::invalid_parameter("endpoint", "missing scheme".to_string());
        // Convert to string.
        let fmt = value.to_string();
        // Assert the formatted string contains the parameter name and error message.
        assert!(
            fmt.contains("invalid `endpoint` parameter: missing scheme"),
            "{value:?} => {fmt}"
        );
    }
}
