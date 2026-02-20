// [Jules: Rust] Standard copyright header.
// [Jules: Rust] Apache 2.0 License.
// Copyright 2026 Google LLC
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

// [Jules: SDK] This module selectively re-exports types from the `reqwest` crate.
// [Jules: SDK] This allows users of `google-cloud-gax` to use these types without directly depending on `reqwest`,
// [Jules: SDK] ensuring version compatibility.
//! Re-export symbols from the [reqwest] crate.
//!
//! [reqwest]: [::reqwest]

// [Jules: Rust] Re-export `Body`, `Method`, `Request`, etc.
// [Jules: Rust] `pub use` makes the item available from this module.
pub use reqwest::Body;
pub use reqwest::Method;
pub use reqwest::Request;
pub use reqwest::RequestBuilder;
pub use reqwest::Response;
pub use reqwest::StatusCode;
pub use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
// [Jules: Rust] Conditionally re-export multipart support if the feature is enabled.
#[cfg(feature = "_internal-http-multipart")]
pub use reqwest::multipart;
