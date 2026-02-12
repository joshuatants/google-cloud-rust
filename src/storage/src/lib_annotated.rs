// ANNOTATED AND DONE
// [Jules: SDK] This file is the root of the crate (library). It defines the public API
// [Jules: SDK] and module structure of the `google-cloud-storage` library.
// [Jules: Rust] The `lib.rs` file is the default entry point for a Rust library crate.
// [Jules: Rust] Unlike other languages where file structure might just be organizational,
// [Jules: Rust] in Rust, `lib.rs` explicitly declares what modules are part of the library.

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

//! Google Cloud Client Libraries for Rust - Storage
// [Jules: Rust] The `//!` syntax indicates documentation comments for the module itself (or crate root here).
// [Jules: Rust] These comments will appear at the top of the generated documentation page.
//!
//! This crate contains traits, types, and functions to interact with [Google
//! Cloud Storage]. Most applications will use the structs defined in the
//! [client] module. More specifically:
//!
//! * [Storage][client::Storage]
//! * [StorageControl][client::StorageControl]
//! * [SignedUrlBuilder][builder::storage::SignedUrlBuilder]
//!
//! **NOTE:** This crate used to contain a different implementation, with a
//! different surface. [@yoshidan](https://github.com/yoshidan) generously
//! donated the crate name to Google. Their crate continues to live as
//! [gcloud-storage].
//!
//! # Features
//!
//! - `default-rustls-provider`: enabled by default. Use the default rustls crypto
//!   provider ([aws-lc-rs]) for TLS and authentication. Applications with specific
//!   requirements for cryptography (such as exclusively using the [ring] crate)
//!   should disable this default and call
//!   `rustls::crypto::CryptoProvider::install_default()`.
//! - `unstable-stream`: enable the (unstable) features to convert several types to
//!   a `future::Stream`.
//!
//! [aws-lc-rs]: https://crates.io/crates/aws-lc-rs
//! [gcloud-storage]: https://crates.io/crates/gcloud-storage
//! [Google Cloud Storage]: https://cloud.google.com/storage
//! [ring]: https://crates.io/crates/ring

#![cfg_attr(docsrs, feature(doc_cfg))]
// [Jules: Rust] This attribute configures conditional compilation for documentation.
// [Jules: Rust] `cfg_attr(condition, attribute)` applies `attribute` only if `condition` is true.
// [Jules: Rust] Here, if compiling for `docsrs` (the documentation hosting service for crates.io),
// [Jules: Rust] it enables the unstable `doc_cfg` feature, allowing documentation to show feature requirements.

pub use google_cloud_gax::Result;
// [Jules: Rust] `pub use` re-exports an item from another module or crate, making it part of this module's public API.
// [Jules: SDK] `google_cloud_gax` is a support library for Google Cloud Rust clients (GAX = Google API Extensions).
// [Jules: SDK] By re-exporting `Result`, we provide a consistent error handling type for users of this crate.

pub use google_cloud_gax::error::Error;
// [Jules: Rust] Similarly, we re-export the `Error` type so users don't need to import `google_cloud_gax` directly.

// Define some shortcuts for imported crates.
pub(crate) use google_cloud_gax::client_builder::ClientBuilder;
// [Jules: Rust] `pub(crate)` makes the item public only within the current crate.
// [Jules: Rust] This is useful for sharing types internally without exposing them to external users.

pub(crate) use google_cloud_gax::client_builder::Result as ClientBuilderResult;
// [Jules: Rust] Here we alias `Result` as `ClientBuilderResult` to avoid naming conflicts and improve clarity.

pub(crate) use google_cloud_gax::client_builder::internal::ClientFactory;
pub(crate) use google_cloud_gax::client_builder::internal::new_builder as new_client_builder;
pub(crate) use google_cloud_gax::options::RequestOptions;
pub(crate) use google_cloud_gax::options::internal::RequestBuilder;
pub(crate) use google_cloud_gax::response::Response;

pub mod backoff_policy;
// [Jules: Rust] `pub mod` declares a public module. The compiler will look for a file named `backoff_policy.rs`
// [Jules: Rust] or `backoff_policy/mod.rs` in the same directory.
// [Jules: SDK] This module likely contains configuration for exponential backoff during retries.

pub mod object_descriptor;
pub mod read_object;
pub mod read_resume_policy;
pub mod retry_policy;
pub mod signed_url;
pub use crate::storage::request_options;
// [Jules: Rust] `crate::` refers to the root of the current crate.
// [Jules: Rust] This re-exports `request_options` from the `storage` module (declared below) to the top level.

pub use crate::storage::streaming_source;

/// Re-export types from the `http` crate used in this module.
pub mod http {
    /// HTTP method used by the [SignedUrlBuilder][crate::builder::storage::SignedUrlBuilder].
    pub use http::Method;

    /// Metadata attributes used by the [Client::open_object][crate::client::Storage::open_object].
    pub use http::HeaderMap;
}

mod control;
mod storage;
// [Jules: Rust] `mod` without `pub` declares a private module. It is only accessible within this file (and submodules).
// [Jules: SDK] The SDK splits functionality into `control` (likely management APIs) and `storage` (data operations).

pub mod client {
    //! Clients to interact with Google Cloud Storage.
    pub use crate::control::client::StorageControl;
    pub use crate::storage::client::Storage;
}
// [Jules: SDK] The `client` module is the main entry point for users. It re-exports the `Storage` client.

pub mod builder {
    //! Request builders.
    pub mod storage {
        //! Request builders for [Storage][crate::client::Storage].
        pub use crate::storage::client::ClientBuilder;
        pub use crate::storage::open_object::OpenObject;
        pub use crate::storage::read_object::ReadObject;
        pub use crate::storage::signed_url::SignedUrlBuilder;
        pub use crate::storage::write_object::WriteObject;
    }
    pub mod storage_control {
        //! Request builders for [StorageControl][crate::client::StorageControl].
        pub use crate::control::builder::*;
        pub use crate::control::client::ClientBuilder;
    }
}
// [Jules: SDK] Builders are used to construct complex requests or clients step-by-step.

pub mod error;
/// The messages and enums that are part of this client library.
pub use crate::control::model;
pub mod builder_ext;
pub mod model_ext;
pub mod stub {
    //! Traits to mock the clients in this library.
    //!
    //! Application developers may need to mock the clients in this library to test
    //! how their application works with different (and sometimes hard to trigger)
    //! client and service behavior. Such test can define mocks implementing the
    //! trait(s) defined in this module, initialize the client with an instance of
    //! this mock in their tests, and verify their application responds as expected.
    pub use crate::control::stub::*;
    pub use crate::storage::stub::*;
    pub use crate::storage::transport::Storage as DefaultStorage;
}
// [Jules: SDK] The `stub` module provides traits for mocking, allowing users to write unit tests
// [Jules: SDK] that don't hit the real Google Cloud Storage API.

#[allow(dead_code)]
pub(crate) mod generated;
// [Jules: Rust] `#[allow(dead_code)]` suppresses warnings about unused code.
// [Jules: SDK] Generated code (e.g., from Protocol Buffers) might contain types or functions not used by the handwritten SDK.

#[allow(dead_code)]
pub(crate) mod google {
    pub mod iam {
        pub mod v1 {
            include!("generated/protos/storage/google.iam.v1.rs");
            // [Jules: Rust] `include!` is a macro that literally pastes the content of the specified file here.
            // [Jules: Rust] This is often used for generated code (like Protobuf definitions) to avoid manual copy-pasting
            // [Jules: Rust] and to keep the source tree clean.
            include!("generated/convert/iam/convert.rs");
        }
    }
    pub mod longrunning {
        include!("generated/protos/control/google.longrunning.rs");
        include!("generated/convert/longrunning/convert.rs");
    }
    pub mod r#type {
        // [Jules: Rust] `r#type` uses a raw identifier because `type` is a reserved keyword in Rust.
        // [Jules: Rust] Prefixing with `r#` allows using keywords as identifiers.
        include!("generated/protos/storage/google.r#type.rs");
        include!("generated/convert/type/convert.rs");
    }
    pub mod rpc {
        include!("generated/protos/storage/google.rpc.rs");
    }
    pub mod storage {
        #[allow(deprecated)]
        #[allow(clippy::large_enum_variant)]
        // [Jules: Rust] `clippy::large_enum_variant` suppresses a Clippy lint warning about an enum variant being much larger
        // [Jules: Rust] than others, which can cause memory inefficiency. In generated code, we often accept this.
        pub mod v2 {
            include!("generated/protos/storage/google.storage.v2.rs");
            include!("generated/convert/storage/convert.rs");
        }
        pub mod control {
            pub mod v2 {
                include!("generated/protos/control/google.storage.control.v2.rs");
                include!("generated/convert/control/convert.rs");
            }
        }
    }
    #[allow(unused_imports)]
    pub mod protobuf {
        pub use gaxi::prost::Empty;
    }
}
