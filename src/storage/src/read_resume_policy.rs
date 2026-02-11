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

//! Defines the read resume policies for Google Cloud Storage.
//!
//! Even if a read request starts successfully, it may be fail after it starts.
//! For example, the read may be interrupted or become too slow and "stall". The
//! client library can automatically recover from such errors. The application
//! may want to control what errors are treated as recoverable, and how many
//! failures are tolerated before abandoning the read request.
//!
//! The traits and types defined in this module allow for such customization.
//!
//! # Example
//! ```
//! # use google_cloud_storage::read_resume_policy::*;
//! let policy = Recommended.with_attempt_limit(3);
//! assert!(matches!(policy.on_error(&ResumeQuery::new(0), io_error()), ResumeResult::Continue(_)));
//! assert!(matches!(policy.on_error(&ResumeQuery::new(1), io_error()), ResumeResult::Continue(_)));
//! assert!(matches!(policy.on_error(&ResumeQuery::new(2), io_error()), ResumeResult::Continue(_)));
//! assert!(matches!(policy.on_error(&ResumeQuery::new(3), io_error()), ResumeResult::Exhausted(_)));
//!
//! use google_cloud_gax::error::{Error, rpc::Code, rpc::Status};
//! fn io_error() -> Error {
//!    // ... details omitted ...
//!    # Error::io("something failed in the read request")
//! }
//! ```

// Import Error from crate root.
use crate::Error;
// Import Code from google_cloud_gax::error::rpc.
use google_cloud_gax::error::rpc::Code;

// Re-export RetryResult as ResumeResult.
pub use google_cloud_gax::retry_result::RetryResult as ResumeResult;

/// Defines the interface to resume policies.
// Define a public trait ReadResumePolicy. extend Send, Sync, and Debug.
pub trait ReadResumePolicy: Send + Sync + std::fmt::Debug {
    /// Determines if the read should continue after an error.
    // Define a method on_error that takes a status and error, and returns a ResumeResult.
    fn on_error(&self, status: &ResumeQuery, error: Error) -> ResumeResult;
}

/// Extension trait for [ReadResumePolicy].
// Define a public trait ReadResumePolicyExt. extend Sized.
pub trait ReadResumePolicyExt: Sized {
    /// Decorates a [ReadResumePolicy] to limit the number of resume attempts.
    ///
    /// This policy decorates an inner policy and limits the total number of
    /// attempts. Note that `on_error()` is not called before the initial
    /// (non-retry) attempt. Therefore, setting the maximum number of attempts
    /// to 0 or 1 results in no retry attempts.
    ///
    /// The policy passes through the results from the inner policy as long as
    /// `attempt_count < maximum_attempts`. Once the maximum number of attempts
    /// is reached, the policy returns [Exhausted][ResumeResult::Exhausted] if the
    /// inner policy returns [Continue][ResumeResult::Continue].
    ///
    /// # Example
    /// ```
    /// # use google_cloud_storage::read_resume_policy::*;
    /// let policy = Recommended.with_attempt_limit(3);
    /// assert!(matches!(policy.on_error(&ResumeQuery::new(0), transient_error()), ResumeResult::Continue(_)));
    /// assert!(matches!(policy.on_error(&ResumeQuery::new(1), transient_error()), ResumeResult::Continue(_)));
    /// assert!(matches!(policy.on_error(&ResumeQuery::new(2), transient_error()), ResumeResult::Continue(_)));
    /// assert!(matches!(policy.on_error(&ResumeQuery::new(3), transient_error()), ResumeResult::Exhausted(_)));
    ///
    /// use google_cloud_gax::error::{Error, rpc::Code, rpc::Status};
    /// fn transient_error() -> Error {
    ///    // ... details omitted ...
    ///    # Error::io("something failed in the read request")
    /// }
    /// ```
    // Define a method with_attempt_limit that wraps self in LimitedAttemptCount.
    fn with_attempt_limit(self, maximum_attempts: u32) -> LimitedAttemptCount<Self> {
        // Create new LimitedAttemptCount.
        LimitedAttemptCount::new(self, maximum_attempts)
    }
}
// Implement ReadResumePolicyExt for any type T implementing ReadResumePolicy.
impl<T: ReadResumePolicy> ReadResumePolicyExt for T {}

/// The inputs into a resume policy query.
///
/// On an error, the client library queries the resume policy as to whether it
/// should attempt a new read request or not. The client library provides an
/// instance of this type to the resume policy.
///
/// We use a struct so we can grow the amount of information without breaking
/// existing resume policies.
// Define a public struct ResumeQuery. derive Debug.
#[derive(Debug)]
// Mark as non_exhaustive.
#[non_exhaustive]
pub struct ResumeQuery {
    /// The number of times the read request has been interrupted already.
    // Public field attempt_count.
    pub attempt_count: u32,
}

// Implement methods for ResumeQuery.
impl ResumeQuery {
    /// Create a new instance.
    // Define public constructor new.
    pub fn new(attempt_count: u32) -> Self {
        // Return new ResumeQuery.
        Self { attempt_count }
    }
}

/// The recommended policy when reading objects from Cloud Storage.
///
/// This policy resumes any read that fails due to I/O errors, and stops on any
/// other error kind.
///
/// # Example
/// ```
/// # use google_cloud_storage::read_resume_policy::*;
/// let policy = Recommended;
/// assert!(matches!(policy.on_error(&ResumeQuery::new(0), io_error()), ResumeResult::Continue(_)));
/// assert!(matches!(policy.on_error(&ResumeQuery::new(0), other_error()), ResumeResult::Permanent(_)));
///
/// use google_cloud_gax::error::{Error, rpc::Code, rpc::Status};
/// fn io_error() -> Error {
///    // ... details omitted ...
///    # Error::io("something failed in the read request")
/// }
/// fn other_error() -> Error {
///    // ... details omitted ...
///    # Error::deser("something failed in the read request")
/// }
/// ```
// Define public struct Recommended. derive Debug.
#[derive(Debug)]
pub struct Recommended;

// Implement ReadResumePolicy for Recommended.
impl ReadResumePolicy for Recommended {
    // Implement on_error.
    fn on_error(&self, _status: &ResumeQuery, error: Error) -> ResumeResult {
        // Match on error.
        match error {
            // If transient, continue.
            e if self::is_transient(&e) => ResumeResult::Continue(e),
            // Otherwise, permanent.
            e => ResumeResult::Permanent(e),
        }
    }
}

// Helper function to check if error is transient.
fn is_transient(error: &Error) -> bool {
    // Match on error.
    match error {
        // When using HTTP the only error after the read starts are I/O errors.
        e if e.is_io() => true,
        // When using gRPC the errors may include more information.
        e if e.is_transport() => true,
        // Timeout errors are transient.
        e if e.is_timeout() => true,
        // Check status code if present.
        e => e.status().is_some_and(|s| is_transient_code(s.code)),
    }
}

// Helper function to check if status code is transient.
fn is_transient_code(code: Code) -> bool {
    // DeadlineExceeded is safe in this context because local deadline errors are not reported via e.status()
    // Match on code.
    matches!(
        code,
        // Unavailable, ResourceExhausted, Internal, DeadlineExceeded are transient.
        Code::Unavailable | Code::ResourceExhausted | Code::Internal | Code::DeadlineExceeded
    )
}

/// A resume policy that resumes regardless of the error type.
///
/// This may be useful in tests, or if used with a very low limit on the number
/// of allowed failures.
///
/// # Example
/// ```
/// # use google_cloud_storage::read_resume_policy::*;
/// let policy = AlwaysResume.with_attempt_limit(3);
/// assert!(matches!(policy.on_error(&ResumeQuery::new(0), scary_error()), ResumeResult::Continue(_)));
/// assert!(matches!(policy.on_error(&ResumeQuery::new(1), scary_error()), ResumeResult::Continue(_)));
/// assert!(matches!(policy.on_error(&ResumeQuery::new(2), scary_error()), ResumeResult::Continue(_)));
/// assert!(matches!(policy.on_error(&ResumeQuery::new(3), scary_error()), ResumeResult::Exhausted(_)));
///
/// use google_cloud_gax::error::{Error, rpc::Code, rpc::Status};
/// fn scary_error() -> Error {
///    // ... details omitted ...
///    # Error::deser("something failed in the read request")
/// }
/// ```
// Define public struct AlwaysResume. derive Debug.
#[derive(Debug)]
pub struct AlwaysResume;

// Implement ReadResumePolicy for AlwaysResume.
impl ReadResumePolicy for AlwaysResume {
    // Implement on_error.
    fn on_error(&self, _status: &ResumeQuery, error: Error) -> ResumeResult {
        // Always return Continue.
        ResumeResult::Continue(error)
    }
}

/// A resume policy that never resumes, regardless of the error type.
///
/// This is useful to disable the default resume policy.
///
/// # Example
/// ```
/// # use google_cloud_storage::read_resume_policy::*;
/// let policy = NeverResume.with_attempt_limit(3);
/// assert!(matches!(policy.on_error(&ResumeQuery::new(0), io_error()), ResumeResult::Permanent(_)));
/// assert!(matches!(policy.on_error(&ResumeQuery::new(1), io_error()), ResumeResult::Permanent(_)));
/// assert!(matches!(policy.on_error(&ResumeQuery::new(2), io_error()), ResumeResult::Permanent(_)));
/// assert!(matches!(policy.on_error(&ResumeQuery::new(3), io_error()), ResumeResult::Permanent(_)));
///
/// use google_cloud_gax::error::{Error, rpc::Code, rpc::Status};
/// fn io_error() -> Error {
///    // ... details omitted ...
///    # Error::io("something failed in the read request")
/// }
/// ```
// Define public struct NeverResume. derive Debug.
#[derive(Debug)]
pub struct NeverResume;
// Implement ReadResumePolicy for NeverResume.
impl ReadResumePolicy for NeverResume {
    // Implement on_error.
    fn on_error(&self, _status: &ResumeQuery, error: Error) -> ResumeResult {
        // Always return Permanent.
        ResumeResult::Permanent(error)
    }
}

/// Decorates a resume policy to stop resuming after a certain number of attempts.
///
/// # Example
/// ```
/// # use google_cloud_storage::read_resume_policy::*;
/// let policy = LimitedAttemptCount::new(AlwaysResume, 3);
/// assert!(matches!(policy.on_error(&ResumeQuery::new(0), scary_error()), ResumeResult::Continue(_)));
/// assert!(matches!(policy.on_error(&ResumeQuery::new(1), scary_error()), ResumeResult::Continue(_)));
/// assert!(matches!(policy.on_error(&ResumeQuery::new(2), scary_error()), ResumeResult::Continue(_)));
/// assert!(matches!(policy.on_error(&ResumeQuery::new(3), scary_error()), ResumeResult::Exhausted(_)));
///
/// use google_cloud_gax::error::{Error, rpc::Code, rpc::Status};
/// fn scary_error() -> Error {
///    // ... details omitted ...
///    # Error::deser("something failed in the read request")
/// }
/// ```
// Define public struct LimitedAttemptCount. derive Debug.
#[derive(Debug)]
pub struct LimitedAttemptCount<P> {
    // Inner policy.
    inner: P,
    // Maximum attempts.
    maximum_attempts: u32,
}

// Implement methods for LimitedAttemptCount.
impl<P> LimitedAttemptCount<P> {
    /// Create a new instance.
    // Define public constructor new.
    pub fn new(inner: P, maximum_attempts: u32) -> Self {
        // Return new LimitedAttemptCount.
        Self {
            inner,
            maximum_attempts,
        }
    }
}

// Implement ReadResumePolicy for LimitedAttemptCount where P implements ReadResumePolicy.
impl<P> ReadResumePolicy for LimitedAttemptCount<P>
where
    P: ReadResumePolicy,
{
    // Implement on_error.
    fn on_error(&self, status: &ResumeQuery, error: Error) -> ResumeResult {
        // Delegate to inner policy.
        match self.inner.on_error(status, error) {
            // If inner policy says continue, check attempt count.
            ResumeResult::Continue(e) if status.attempt_count >= self.maximum_attempts => {
                // If limit reached, return Exhausted.
                ResumeResult::Exhausted(e)
            }
            // Otherwise return inner result.
            result => result,
        }
    }
}

// Conditionally compile the tests module only when running tests.
#[cfg(test)]
mod tests {
    // Import everything from the parent module.
    use super::*;

    // Test recommended policy.
    #[test]
    fn recommended() {
        // Create Recommended policy.
        let policy = Recommended;
        // Test with transient error.
        let r = policy.on_error(&ResumeQuery::new(0), common_transient());
        assert!(matches!(r, ResumeResult::Continue(_)), "{r:?}");
        // Test with timeout error.
        let r = policy.on_error(&ResumeQuery::new(0), common_timeout());
        assert!(matches!(r, ResumeResult::Continue(_)), "{r:?}");
        // Test with HTTP transient error.
        let r = policy.on_error(&ResumeQuery::new(0), http_transient());
        assert!(matches!(r, ResumeResult::Continue(_)), "{r:?}");
        // Test with gRPC DeadlineExceeded.
        let r = policy.on_error(&ResumeQuery::new(0), grpc_deadline_exceeded());
        assert!(matches!(r, ResumeResult::Continue(_)), "{r:?}");
        // Test with gRPC Internal.
        let r = policy.on_error(&ResumeQuery::new(0), grpc_internal());
        assert!(matches!(r, ResumeResult::Continue(_)), "{r:?}");
        // Test with gRPC ResourceExhausted.
        let r = policy.on_error(&ResumeQuery::new(0), grpc_resource_exhausted());
        assert!(matches!(r, ResumeResult::Continue(_)), "{r:?}");
        // Test with gRPC Unavailable.
        let r = policy.on_error(&ResumeQuery::new(0), grpc_unavailable());
        assert!(matches!(r, ResumeResult::Continue(_)), "{r:?}");

        // Test with HTTP permanent error.
        let r = policy.on_error(&ResumeQuery::new(0), http_permanent());
        assert!(matches!(r, ResumeResult::Permanent(_)), "{r:?}");
        // Test with gRPC permanent error.
        let r = policy.on_error(&ResumeQuery::new(0), grpc_permanent());
        assert!(matches!(r, ResumeResult::Permanent(_)), "{r:?}");
    }

    // Test always resume policy.
    #[test]
    fn always_resume() {
        // Create AlwaysResume policy.
        let policy = AlwaysResume;
        // Test with transient error.
        let r = policy.on_error(&ResumeQuery::new(0), http_transient());
        assert!(matches!(r, ResumeResult::Continue(_)), "{r:?}");
        // Test with permanent error.
        let r = policy.on_error(&ResumeQuery::new(0), http_permanent());
        assert!(matches!(r, ResumeResult::Continue(_)), "{r:?}");
    }

    // Test never resume policy.
    #[test]
    fn never_resume() {
        // Create NeverResume policy.
        let policy = NeverResume;
        // Test with transient error.
        let r = policy.on_error(&ResumeQuery::new(0), http_transient());
        assert!(matches!(r, ResumeResult::Permanent(_)), "{r:?}");
        // Test with permanent error.
        let r = policy.on_error(&ResumeQuery::new(0), http_permanent());
        assert!(matches!(r, ResumeResult::Permanent(_)), "{r:?}");
    }

    // Test attempt limit.
    #[test]
    fn attempt_limit() {
        // Create Recommended policy with limit 3.
        let policy = Recommended.with_attempt_limit(3);
        // Test attempt 0.
        let r = policy.on_error(&ResumeQuery::new(0), http_transient());
        assert!(matches!(r, ResumeResult::Continue(_)), "{r:?}");
        // Test attempt 1.
        let r = policy.on_error(&ResumeQuery::new(1), http_transient());
        assert!(matches!(r, ResumeResult::Continue(_)), "{r:?}");
        // Test attempt 2.
        let r = policy.on_error(&ResumeQuery::new(2), http_transient());
        assert!(matches!(r, ResumeResult::Continue(_)), "{r:?}");
        // Test attempt 3 (limit reached).
        let r = policy.on_error(&ResumeQuery::new(3), http_transient());
        assert!(matches!(r, ResumeResult::Exhausted(_)), "{r:?}");

        // Test permanent error attempt 0.
        let r = policy.on_error(&ResumeQuery::new(0), http_permanent());
        assert!(matches!(r, ResumeResult::Permanent(_)), "{r:?}");
        // Test permanent error attempt 3.
        let r = policy.on_error(&ResumeQuery::new(3), http_permanent());
        assert!(matches!(r, ResumeResult::Permanent(_)), "{r:?}");
    }

    // Test stacked attempt limits.
    #[test]
    fn attempt_limit_inner_exhausted() {
        // Create policy with two limits.
        let policy = AlwaysResume.with_attempt_limit(3).with_attempt_limit(5);
        // Test attempt 3 (inner limit reached).
        let r = policy.on_error(&ResumeQuery::new(3), http_transient());
        assert!(matches!(r, ResumeResult::Exhausted(_)), "{r:?}");
    }

    // Helper to create HTTP transient error.
    fn http_transient() -> Error {
        Error::io("test only")
    }

    // Helper to create HTTP permanent error.
    fn http_permanent() -> Error {
        Error::deser("bad data")
    }

    // Helper to create common transient error.
    fn common_transient() -> Error {
        Error::transport(http::HeaderMap::new(), "test-only")
    }

    // Helper to create common timeout error.
    fn common_timeout() -> Error {
        Error::timeout("simulated timeout")
    }

    // Helper to create gRPC DeadlineExceeded error.
    fn grpc_deadline_exceeded() -> Error {
        grpc_error(Code::DeadlineExceeded)
    }

    // Helper to create gRPC Internal error.
    fn grpc_internal() -> Error {
        grpc_error(Code::Internal)
    }

    // Helper to create gRPC ResourceExhausted error.
    fn grpc_resource_exhausted() -> Error {
        grpc_error(Code::ResourceExhausted)
    }

    // Helper to create gRPC Unavailable error.
    fn grpc_unavailable() -> Error {
        grpc_error(Code::Unavailable)
    }

    // Helper to create gRPC permanent error.
    fn grpc_permanent() -> Error {
        grpc_error(Code::PermissionDenied)
    }

    // Helper to create gRPC error from code.
    fn grpc_error(code: Code) -> Error {
        let status = google_cloud_gax::error::rpc::Status::default().set_code(code);
        Error::service(status)
    }
}
