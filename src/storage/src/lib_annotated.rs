// ANNOTATED AND DONE
// [Jules] Rust: The following lines are copyright notices and license information.
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

// [Jules] Rust: These comments starting with `//!` are doc comments for the crate or module itself.
// They will appear at the top of the generated documentation page for this crate.
// Regular comments `//` are ignored by rustdoc, and doc comments `///` are for the item following them.
//! Google Cloud Client Libraries for Rust - Storage
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

// [Jules] Rust: This attribute configures the `docsrs` feature.
// It allows enabling `doc_cfg` feature when building documentation on docs.rs,
// which is useful for documenting platform-specific or feature-specific items.
#![cfg_attr(docsrs, feature(doc_cfg))]

// [Jules] Rust: `pub use` re-exports items from another module or crate, making them available as part of this module's public API.
// Here we are re-exporting `Result` and `Error` from `google_cloud_gax`.
// [Jules] SDK: `google_cloud_gax` is a support library for Google Cloud client libraries in Rust.
// It provides common types and utilities like error handling and client configuration.
pub use google_cloud_gax::Result;
pub use google_cloud_gax::error::Error;
// Define some shortcuts for imported crates.
// [Jules] Rust: `pub(crate)` makes the item visible only within the current crate, not to external users.
// This is useful for internal helpers or types that shouldn't be part of the public API.
pub(crate) use google_cloud_gax::client_builder::ClientBuilder;
pub(crate) use google_cloud_gax::client_builder::Result as ClientBuilderResult;
pub(crate) use google_cloud_gax::client_builder::internal::ClientFactory;
pub(crate) use google_cloud_gax::client_builder::internal::new_builder as new_client_builder;
pub(crate) use google_cloud_gax::options::RequestOptions;
pub(crate) use google_cloud_gax::options::internal::RequestBuilder;
pub(crate) use google_cloud_gax::response::Response;

// [Jules] Rust: `pub mod` declares a public module.
// This looks for a file named `backoff_policy.rs` or `backoff_policy/mod.rs` in the same directory.
// It makes the contents of that module available under `crate::backoff_policy`.
pub mod backoff_policy;
pub mod object_descriptor;
pub mod read_object;
pub mod read_resume_policy;
pub mod retry_policy;
pub mod signed_url;
// [Jules] Rust: Re-exporting items from `crate::storage`.
// `crate::` refers to the root of the current crate.
pub use crate::storage::request_options;
pub use crate::storage::streaming_source;

/// Re-export types from the `http` crate used in this module.
pub mod http {
    /// HTTP method used by the [SignedUrlBuilder][crate::builder::storage::SignedUrlBuilder].
    pub use http::Method;

    /// Metadata attributes used by the [Client::open_object][crate::client::Storage::open_object].
    pub use http::HeaderMap;
}

// [Jules] Rust: `mod` without `pub` declares a private module.
// These modules are only accessible within this crate.
mod control;
mod storage;

pub mod client {
    //! Clients to interact with Google Cloud Storage.
    // [Jules] Rust: Re-exporting specific items from private modules to make them public in this `client` module.
    // This allows controlling exactly what is exposed to the user while keeping the implementation details private.
    pub use crate::control::client::StorageControl;
    pub use crate::storage::client::Storage;
}
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

// [Jules] Rust: `#[allow(dead_code)]` suppresses warnings about unused code.
// This is often used for generated code where not all parts might be used by the library.
#[allow(dead_code)]
pub(crate) mod generated;

#[allow(dead_code)]
pub(crate) mod google {
    pub mod iam {
        pub mod v1 {
            // [Jules] Rust: `include!` macro inserts the contents of the specified file as if it were written here.
            // This is commonly used to include generated code (e.g., from Protocol Buffers).
            // [Jules] SDK: These files contain the Rust code generated from the Google Cloud Storage API definition (Proto).
            include!("generated/protos/storage/google.iam.v1.rs");
            include!("generated/convert/iam/convert.rs");
        }
    }
    pub mod longrunning {
        include!("generated/protos/control/google.longrunning.rs");
        include!("generated/convert/longrunning/convert.rs");
    }
    pub mod r#type {
        // [Jules] Rust: `r#` is used to use keywords as identifiers.
        // `type` is a keyword in Rust, so we must escape it as `r#type` to use it as a module name.
        include!("generated/protos/storage/google.r#type.rs");
        include!("generated/convert/type/convert.rs");
    }
    pub mod rpc {
        include!("generated/protos/storage/google.rpc.rs");
    }
    pub mod storage {
        #[allow(deprecated)]
        #[allow(clippy::large_enum_variant)]
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
