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

// Define a public module named builder.
pub mod builder {
    // Re-export storage_control builders from generated gapic code.
    pub use crate::generated::gapic::builder::storage_control::*;
    // Re-export storage_control builders from generated gapic_control code.
    pub use crate::generated::gapic_control::builder::storage_control::*;
}
// Define a public module named model.
pub mod model {
    // Re-export model types from generated gapic code.
    pub use crate::generated::gapic::model::*;
    // Re-export model types from generated gapic_control code.
    pub use crate::generated::gapic_control::model::*;
}
// Declare a public module named client.
pub mod client;
/// Traits to mock the clients in this library.
///
/// Application developers may need to mock the clients in this library to test
/// how their application works with different (and sometimes hard to trigger)
/// client and service behavior. Such test can define mocks implementing the
/// trait(s) defined in this module, initialize the client with an instance of
/// this mock in their tests, and verify their application responds as expected.
// Re-export stub module from generated code, which likely contains trait definitions for mocking.
pub use generated::stub;

// Declare a private module named convert.
mod convert;
// Declare a private module named status.
mod status;

// Declare a private module named generated.
mod generated;
