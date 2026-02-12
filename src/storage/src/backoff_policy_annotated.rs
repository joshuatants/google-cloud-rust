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
// This is a documentation comment for the module. It starts with `//!` which means it documents the
// item that contains it (in this case, the file/module itself), rather than the item that follows it.
// These comments are used to generate the crate documentation.
//! Types and functions related to the default backoff policy.

// [Jules: Rust]
// `use` brings items from other modules into the current scope.
// Here we are importing `BackoffPolicy` and `ExponentialBackoffBuilder` from the `google_cloud_gax` crate.
// `google_cloud_gax` is a lower-level library that provides common functionality for Google Cloud clients.
// We also import `Duration` from the standard library (`std`), specifically the `time` module.
use google_cloud_gax::{
    backoff_policy::BackoffPolicy, exponential_backoff::ExponentialBackoffBuilder,
};
use std::time::Duration;

// [Jules: Rust]
// `///` comments are documentation comments for the item that follows them.
// In this case, it documents the `default` function.
//
// [Jules: SDK]
// This function defines the default strategy for how the client should wait before retrying a failed request.
// "Backoff" means waiting for some time before trying again.
/// The default backoff policy for the Storage clients.
///
/// The service recommends exponential backoff with jitter, starting with a one
/// second backoff and doubling on each attempt.
// [Jules: Rust]
// The return type `impl BackoffPolicy` is an example of "impl Trait".
// It means "this function returns some type that implements the `BackoffPolicy` trait, but I'm not telling you exactly what concrete type it is."
// This is useful for hiding implementation details and making the function signature simpler.
// Instead of returning a complex type like `ExponentialBackoff<...>`, we just promise it behaves like a `BackoffPolicy`.
pub fn default() -> impl BackoffPolicy {
    // [Jules: Rust]
    // We are using the Builder pattern here. `ExponentialBackoffBuilder::new()` creates a builder.
    // We then chain method calls (`with_initial_delay`, `with_maximum_delay`, etc.) to configure the object.
    // Finally, `.build()` constructs the actual object.
    //
    // [Jules: SDK]
    // Exponential backoff means the wait time increases exponentially with each retry.
    // - `initial_delay`: Start by waiting 1 second.
    // - `maximum_delay`: Never wait more than 60 seconds.
    // - `scaling`: Multiply the wait time by 2.0 after each failure (1s, 2s, 4s, ...).
    ExponentialBackoffBuilder::new()
        .with_initial_delay(Duration::from_secs(1))
        .with_maximum_delay(Duration::from_secs(60))
        .with_scaling(2.0)
        .build()
        // [Jules: Rust]
        // `.expect(...)` is used to handle the `Result` returned by `.build()`.
        // If `.build()` returns `Ok(value)`, `.expect()` returns that value.
        // If `.build()` returns `Err(error)`, `.expect()` crashes the program (panics) with the provided message.
        // Here, we expect the build to always succeed because we are using hardcoded valid values.
        .expect("statically configured policy should succeed")
}

// [Jules: Rust]
// `#[cfg(test)]` is a conditional compilation attribute.
// It tells the compiler to only compile the following module when running tests (e.g., `cargo test`).
// This keeps test code out of the final production binary.
#[cfg(test)]
mod tests {
    // [Jules: Rust]
    // `use super::*;` imports everything from the parent module (the main part of this file) into the test module.
    // This allows the tests to access the functions and types defined above.
    use super::*;
    use google_cloud_gax::retry_state::RetryState;

    // [Jules: Rust]
    // `#[test]` marks the following function as a test case.
    // The test runner will execute this function and report if it passes or fails (panics).
    #[test]
    fn default() {
        // [Jules: Rust]
        // We call the `default` function defined in the parent module to get the policy instance.
        let policy = super::default();

        // [Jules: SDK]
        // We simulate a failure to see how long the policy tells us to wait.
        // `RetryState` holds the context of the retry loop.
        // Here, we say it's the 1st attempt.
        let delay = policy.on_failure(&RetryState::new(true).set_attempt_count(1_u32));

        // [Jules: Rust]
        // `assert!` checks that a boolean condition is true.
        // If it's false, the test fails.
        // The second argument is a format string for the error message if the assertion fails.
        // `{delay:?}` uses the `Debug` trait to print the value of `delay`.
        assert!(
            delay <= Duration::from_secs(1),
            "{delay:?}, policy={policy:?}"
        );

        // [Jules: SDK]
        // Now we simulate a second failure (attempt count 2).
        // The delay should have increased, but due to jitter (randomness), we just check it's within bounds.
        let delay = policy.on_failure(&RetryState::new(true).set_attempt_count(2_u32));
        assert!(
            delay <= Duration::from_secs(2),
            "{delay:?}, policy={policy:?}"
        );
    }
}
