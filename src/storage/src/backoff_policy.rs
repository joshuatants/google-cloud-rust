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

//! Types and functions related to the default backoff policy.

// Import the BackoffPolicy trait and ExponentialBackoffBuilder struct from the google_cloud_gax crate.
use google_cloud_gax::{
    backoff_policy::BackoffPolicy, exponential_backoff::ExponentialBackoffBuilder,
};
// Import the Duration struct from the standard library for time manipulation.
use std::time::Duration;

/// The default backoff policy for the Storage clients.
///
/// The service recommends exponential backoff with jitter, starting with a one
/// second backoff and doubling on each attempt.
// Define a public function named default that returns a type implementing BackoffPolicy.
pub fn default() -> impl BackoffPolicy {
    // Create a new builder for an exponential backoff policy.
    ExponentialBackoffBuilder::new()
        // Set the initial delay to 1 second.
        .with_initial_delay(Duration::from_secs(1))
        // Set the maximum delay to 60 seconds.
        .with_maximum_delay(Duration::from_secs(60))
        // Set the scaling factor to 2.0, meaning the delay doubles with each retry.
        .with_scaling(2.0)
        // Build the backoff policy from the configuration.
        .build()
        // Unwrap the result, panicking with a message if building the policy fails (which shouldn't happen here).
        .expect("statically configured policy should succeed")
}

// Conditionally compile the tests module only when running tests.
#[cfg(test)]
mod tests {
    // Import everything from the parent module.
    use super::*;
    // Import RetryState from google_cloud_gax for testing retry logic.
    use google_cloud_gax::retry_state::RetryState;

    // Define a test function named default.
    #[test]
    fn default() {
        // Call the default function from the parent module to get the backoff policy.
        let policy = super::default();

        // Calculate the delay for the first failure (attempt count 1).
        let delay = policy.on_failure(&RetryState::new(true).set_attempt_count(1_u32));
        // Assert that the delay is less than or equal to 1 second.
        assert!(
            delay <= Duration::from_secs(1),
            "{delay:?}, policy={policy:?}"
        );

        // Calculate the delay for the second failure (attempt count 2).
        let delay = policy.on_failure(&RetryState::new(true).set_attempt_count(2_u32));
        // Assert that the delay is less than or equal to 2 seconds.
        assert!(
            delay <= Duration::from_secs(2),
            "{delay:?}, policy={policy:?}"
        );
    }
}
