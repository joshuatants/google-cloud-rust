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

// Rust: This attribute configures the conditional compilation for documentation.
// `docsrs` is a configuration flag used by docs.rs (the Rust documentation host).
// `feature(doc_cfg)` is an unstable feature that allows documenting platform-specific
// or feature-specific items (e.g., "This item is only available on Linux").
#![cfg_attr(docsrs, feature(doc_cfg))]

// SDK: Re-exporting `Result` and `Error` from `google_cloud_gax`.
// `google_cloud_gax` is the Google Api eXtensions library, which provides common functionality
// for all Google Cloud client libraries.
// Rust: `pub use` re-exports an item, making it available to users of this crate.
// This allows users to access `Result` and `Error` directly from the `storage` crate
// without needing to import `google_cloud_gax` explicitly.
pub use google_cloud_gax::Result;
pub use google_cloud_gax::error::Error;

// Rust: `pub(crate)` makes these items visible only within the current crate.
// They are not exposed to users of the library.
// These lines create aliases for types from `google_cloud_gax` to be used internally.
pub(crate) use google_cloud_gax::client_builder::ClientBuilder;
pub(crate) use google_cloud_gax::client_builder::Result as ClientBuilderResult;
pub(crate) use google_cloud_gax::client_builder::internal::ClientFactory;
pub(crate) use google_cloud_gax::client_builder::internal::new_builder as new_client_builder;
pub(crate) use google_cloud_gax::options::RequestOptions;
pub(crate) use google_cloud_gax::options::internal::RequestBuilder;
pub(crate) use google_cloud_gax::response::Response;

// Rust: `pub mod` declares a public module. This looks for a file with the same name
// (e.g., `backoff_policy.rs` or `backoff_policy/mod.rs`) and includes its contents.
// This organizes the code into logical units.
// SDK: These modules provide specific functionality for the Storage client.
pub mod backoff_policy;
pub mod object_descriptor;
pub mod read_object;
pub mod read_resume_policy;
pub mod retry_policy;
pub mod signed_url;

// Rust: Re-exporting items from internal modules.
// `crate::storage` refers to the `storage` module defined later in this file.
// This flattens the import path for internal use or for cleaner public API if these were public.
pub use crate::storage::request_options;
pub use crate::storage::streaming_source;

/// Re-export types from the `http` crate used in this module.
pub mod http {
    /// HTTP method used by the [SignedUrlBuilder][crate::builder::storage::SignedUrlBuilder].
    pub use http::Method;

    /// Metadata attributes used by the [Client::open_object][crate::client::Storage::open_object].
    pub use http::HeaderMap;
}

// Rust: `mod` declares a private module. These modules are not accessible outside this crate,
// unless items are re-exported (as seen below).
mod control;
mod storage;

// Rust: Defining a public module `client` inline.
// This module aggregates the main client structs.
pub mod client {
    //! Clients to interact with Google Cloud Storage.
    // SDK: `StorageControl` is likely for control plane operations (e.g., managing buckets).
    pub use crate::control::client::StorageControl;
    // SDK: `Storage` is the main client for data plane operations (e.g., uploading/downloading objects).
    pub use crate::storage::client::Storage;
}

// Rust: Defining a public module `builder` inline.
// This module organizes builders for various requests.
// Builders are a common pattern in Rust (and other languages) to construct complex objects with optional parameters.
pub mod builder {
    //! Request builders.
    pub mod storage {
        //! Request builders for [Storage][crate::client::Storage].
        // Rust: Re-exporting specific builders from internal modules to be accessible under `builder::storage`.
        pub use crate::storage::client::ClientBuilder;
        pub use crate::storage::open_object::OpenObject;
        pub use crate::storage::read_object::ReadObject;
        pub use crate::storage::signed_url::SignedUrlBuilder;
        pub use crate::storage::write_object::WriteObject;
    }
    pub mod storage_control {
        //! Request builders for [StorageControl][crate::client::StorageControl].
        // Rust: `*` imports all public items from `crate::control::builder`.
        pub use crate::control::builder::*;
        pub use crate::control::client::ClientBuilder;
    }
}

pub mod error;

/// The messages and enums that are part of this client library.
// SDK: Re-exports the data models (Protobuf messages) used by the client.
pub use crate::control::model;

pub mod builder_ext;
pub mod model_ext;

// Rust: The `stub` module is useful for testing.
// It allows developers to mock the client behaviors.
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

// Rust: `#[allow(dead_code)]` suppresses warnings about unused code.
// This is common in generated code or libraries where not all functionality is used internally.
#[allow(dead_code)]
pub(crate) mod generated;

// Rust: This module structure mirrors the Google API protobuf structure.
// It uses `include!` macro to include generated Rust code from Protobuf definitions.
// SDK: This is where the low-level gRPC/Protobuf definitions reside.
#[allow(dead_code)]
pub(crate) mod google {
    pub mod iam {
        pub mod v1 {
            // Rust: `include!` inserts the contents of the specified file here.
            // These files are typically generated by `tonic-build` or `prost-build`.
            include!("generated/protos/storage/google.iam.v1.rs");
            include!("generated/convert/iam/convert.rs");
        }
    }
    pub mod longrunning {
        include!("generated/protos/control/google.longrunning.rs");
        include!("generated/convert/longrunning/convert.rs");
    }
    pub mod r#type {
        // Rust: `r#type` is used because `type` is a reserved keyword in Rust.
        // `r#` allows using keywords as identifiers.
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
        // SDK: Importing `Empty` from `gaxi::prost`. `gaxi` seems to be an internal crate or alias.
        pub use gaxi::prost::Empty;
    }
}
