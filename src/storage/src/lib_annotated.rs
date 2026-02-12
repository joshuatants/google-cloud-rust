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

// RUST: The `cfg_attr` attribute allows you to conditionally apply attributes based on
// compilation configuration. Here, it says "if we are building documentation (docsrs),
// enable the feature `doc_cfg`".
//
// SDK: This is used to generate better documentation on docs.rs, often showing
// which features are required for certain APIs.
#![cfg_attr(docsrs, feature(doc_cfg))]

// RUST: `pub use` re-exports items from other crates or modules, making them available
// as part of this crate's public API. This is a common pattern to expose types from
// dependencies without forcing users to import those dependencies directly.
//
// SDK: `Result` is a type alias for `std::result::Result<T, google_cloud_gax::error::Error>`.
// This simplifies error handling throughout the SDK, as most functions return this `Result`.
pub use google_cloud_gax::Result;
pub use google_cloud_gax::error::Error;

// RUST: `pub(crate)` makes an item visible only within the current crate. It is not
// part of the public API.
//
// SDK: These are internal shortcuts for types from `google_cloud_gax`, a common library
// used by all Google Cloud Rust clients.
pub(crate) use google_cloud_gax::client_builder::ClientBuilder;
pub(crate) use google_cloud_gax::client_builder::Result as ClientBuilderResult;
pub(crate) use google_cloud_gax::client_builder::internal::ClientFactory;
pub(crate) use google_cloud_gax::client_builder::internal::new_builder as new_client_builder;
pub(crate) use google_cloud_gax::options::RequestOptions;
pub(crate) use google_cloud_gax::options::internal::RequestBuilder;
pub(crate) use google_cloud_gax::response::Response;

// RUST: `pub mod` declares a public module. This means the contents of the file
// (e.g., `backoff_policy.rs`) are part of the module tree and accessible publicly.
//
// SDK: These modules contain specific functionalities or policies used by the Storage client.
pub mod backoff_policy;
pub mod object_descriptor;
pub mod read_object;
pub mod read_resume_policy;
pub mod retry_policy;
pub mod signed_url;

// RUST: Re-exporting items from a child module (`storage`) to the current module scope.
// This flattens the API structure, making `request_options` and `streaming_source`
// appear as if they were defined in `lib.rs`.
pub use crate::storage::request_options;
pub use crate::storage::streaming_source;

/// Re-export types from the `http` crate used in this module.
pub mod http {
    /// HTTP method used by the [SignedUrlBuilder][crate::builder::storage::SignedUrlBuilder].
    pub use http::Method;

    /// Metadata attributes used by the [Client::open_object][crate::client::Storage::open_object].
    pub use http::HeaderMap;
}

// RUST: `mod control` declares a module named `control`. Since it's not `pub`, it's
// private to this crate. The compiler will look for `control.rs` or `control/mod.rs`.
//
// SDK: `control` likely contains logic for the Storage Control API, a separate
// but related service.
mod control;
mod storage;

// RUST: This `pub mod client` block defines a module inline. It serves as a facade,
// grouping the main client types (`StorageControl` and `Storage`) in one place.
pub mod client {
    //! Clients to interact with Google Cloud Storage.
    pub use crate::control::client::StorageControl;
    pub use crate::storage::client::Storage;
}

// RUST: The `builder` module groups types used to build requests. This is a common
// pattern in Rust to organize related builder structs.
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

// RUST: Re-exporting the `error` module so users can access error types.
pub mod error;

/// The messages and enums that are part of this client library.
// SDK: `model` typically contains data structures (DTOs) representing the resources
// managed by the API (e.g., Bucket, Object).
pub use crate::control::model;

// SDK: `builder_ext` and `model_ext` likely contain extension traits or additional functionality
// that enhances the core builders and models, possibly making them easier to use.
pub mod builder_ext;
pub mod model_ext;

// RUST: `pub mod stub` exposes types for mocking.
//
// SDK: This is crucial for testing. It allows developers to create mock implementations
// of the storage client to test their applications without making real network requests.
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

// RUST: `#[allow(dead_code)]` suppresses compiler warnings about unused code.
// This is common for generated code where not all generated functions are used.
//
// SDK: `generated` contains code generated from Protocol Buffers (protos).
#[allow(dead_code)]
pub(crate) mod generated;

// RUST: This module structure mimics the package structure of the Google Cloud APIs.
//
// SDK: It includes the generated Rust code for IAM, Longrunning operations, and Storage types.
// The `include!` macro literally pastes the contents of the specified file here.
#[allow(dead_code)]
pub(crate) mod google {
    pub mod iam {
        pub mod v1 {
            include!("generated/protos/storage/google.iam.v1.rs");
            include!("generated/convert/iam/convert.rs");
        }
    }
    pub mod longrunning {
        include!("generated/protos/control/google.longrunning.rs");
        include!("generated/convert/longrunning/convert.rs");
    }
    pub mod r#type {
        // RUST: `r#type` is used because `type` is a reserved keyword in Rust.
        // The `r#` prefix allows using reserved keywords as identifiers.
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
        // SDK: Re-exports `Empty` from `prost`, which represents the `google.protobuf.Empty` message.
        pub use gaxi::prost::Empty;
    }
}
