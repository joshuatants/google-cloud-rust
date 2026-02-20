// [Jules: Rust] The standard copyright header.
// [Jules: Rust] This project is licensed under Apache 2.0.
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

// [Jules: Rust] We import from the `rand` crate, which is the standard crate for random number generation in Rust.
// [Jules: Rust] `RngExt` provides extension methods for random number generators.
// [Jules: Rust] `Alphanumeric` is a distribution that samples `u8` (bytes) in the range of A-Z, a-z, and 0-9.
use rand::{RngExt, distr::Alphanumeric};

// [Jules: SDK] This helper function generates a random object name, which is useful for avoiding naming collisions during benchmarks or tests.
// [Jules: Rust] The function returns a `String`.
pub fn random_object_name() -> String {
    // [Jules: Rust] `rand::rng()` gets the thread-local random number generator.
    rand::rng()
        // [Jules: Rust] `sample_iter` creates an infinite iterator of random values sampled from the given distribution (`Alphanumeric`).
        .sample_iter(&Alphanumeric)
        // [Jules: Rust] `take(32)` limits the infinite iterator to the first 32 elements.
        .take(32)
        // [Jules: Rust] `map(char::from)` converts each `u8` (byte) from the distribution into a `char`.
        .map(char::from)
        // [Jules: Rust] `collect()` consumes the iterator and collects the characters into a `String`.
        // [Jules: Rust] Rust infers the target type `String` from the return type of the function.
        .collect()
}

// [Jules: Rust] The `#[cfg(test)]` attribute ensures this module is only compiled when running tests.
#[cfg(test)]
mod tests {
    // [Jules: Rust] Bring items from the parent module into scope.
    use super::*;
    // [Jules: Rust] `HashSet` is a collection that stores unique elements. We use it here to verify uniqueness of generated names.
    use std::collections::HashSet;

    // [Jules: Rust] A unit test to verify that `random_object_name` generates unique strings.
    #[test]
    fn names() {
        // [Jules: Rust] `const` defines a compile-time constant.
        const COUNT: usize = 128;
        // [Jules: Rust] We generate `COUNT` random names and collect them into a `HashSet`.
        // [Jules: Rust] `(0..COUNT)` creates a range iterator. `map` is used to generate a name for each iteration.
        // [Jules: Rust] The `_` in `|_|` indicates we don't care about the index.
        let generated: HashSet<String> = (0..COUNT).map(|_| random_object_name()).collect();
        // [Jules: Rust] Since `HashSet` only stores unique values, if `generated.len()` equals `COUNT`, it means all generated names were unique.
        assert_eq!(generated.len(), COUNT);
    }
}
