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

// [Jules]
// In Rust, documentation comments that start with `//!` document the *containing* item.
// In this case, these comments document the `lib.rs` file itself, which corresponds
// to the root of the crate (the library). This text will appear at the very top of
// the generated documentation for the crate.
//
// Contrast this with `///` comments, which document the item *following* the comment.
//
// From an SDK perspective, this section provides a high-level overview of what the
// crate does (Google Cloud Storage client), links to key structs (`Storage`, `StorageControl`),
// and explains feature flags. It's the "front door" for users reading the docs.

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

// [Jules]
// `#![...]` syntax indicates an attribute that applies to the entire file (crate root).
// `cfg_attr(condition, attribute)` applies the `attribute` only if `condition` is met.
//
// Here, `docsrs` is a configuration flag set by the documentation build server (docs.rs).
// `feature(doc_cfg)` enables an unstable Rust feature that allows documenting platform-specific
// items (e.g., "This API is only available on Linux").
//
// So, this line says: "If we are building documentation on docs.rs, enable the `doc_cfg` feature."
#![cfg_attr(docsrs, feature(doc_cfg))]

// [Jules]
// `pub use` is a re-export. It imports an item from another path and makes it available
// as part of *this* module's public API.
//
// SDK Perspective: `google_cloud_gax` is a support library (Google API Extensions) that
// provides common functionality for all Google Cloud Rust clients (like error handling,
// retries, and authentication).
//
// By re-exporting `Result` and `Error` here, the storage crate allows users to simply usage
// `google_cloud_storage::Result` instead of needing to know about `google_cloud_gax`.
// It unifies the error types across different Google Cloud libraries.
pub use google_cloud_gax::Result;
pub use google_cloud_gax::error::Error;

// [Jules]
// `pub(crate)` means the item is public *within this crate*, but not exposed to external users.
// These are internal shortcuts/aliases used throughout the storage library implementation.
//
// `ClientBuilder` and `ClientFactory` are patterns used to construct clients with
// specific configurations (credentials, retry policies, etc.).
// `RequestOptions` allows users to pass optional parameters (like timeouts or headers)
// with their API requests.
pub(crate) use google_cloud_gax::client_builder::ClientBuilder;
pub(crate) use google_cloud_gax::client_builder::Result as ClientBuilderResult;
pub(crate) use google_cloud_gax::client_builder::internal::ClientFactory;
pub(crate) use google_cloud_gax::client_builder::internal::new_builder as new_client_builder;
pub(crate) use google_cloud_gax::options::RequestOptions;
pub(crate) use google_cloud_gax::options::internal::RequestBuilder;
pub(crate) use google_cloud_gax::response::Response;

// [Jules]
// `pub mod` declares a module and makes it public. This tells Rust to look for a file
// named (e.g.) `backoff_policy.rs` or `backoff_policy/mod.rs` and include its contents
// as a submodule.
//
// These modules organize the library's functionality:
// - `backoff_policy`: How to wait between retries.
// - `object_descriptor`: Metadata about GCS objects.
// - `read_object`: Logic for reading data.
// - `signed_url`: Generating URLs for temporary access.
pub mod backoff_policy;
pub mod object_descriptor;
pub mod read_object;
pub mod read_resume_policy;
pub mod retry_policy;
pub mod signed_url;

// [Jules]
// Here we re-export items from internal submodules (`crate::storage::...`) to the top level.
// This flattens the API structure, so users can access `google_cloud_storage::request_options`
// instead of digging into deep paths.
pub use crate::storage::request_options;
pub use crate::storage::streaming_source;

/// Re-export types from the `http` crate used in this module.
pub mod http {
    /// HTTP method used by the [SignedUrlBuilder][crate::builder::storage::SignedUrlBuilder].
    pub use http::Method;

    /// Metadata attributes used by the [Client::open_object][crate::client::Storage::open_object].
    pub use http::HeaderMap;
}

// [Jules]
// `mod control` and `mod storage` declare private modules (no `pub`).
// These contain the core implementation logic. Users cannot access `google_cloud_storage::control`
// directly. Instead, specific items are exposed via `pub use` in other places (like the `client` module below).
mod control;
mod storage;

// [Jules]
// This defines a public `client` module inline (using curly braces instead of a separate file).
// It serves as the main entry point for users to find the Client structs.
//
// SDK Perspective: Segregating `Storage` (data plane) and `StorageControl` (control plane)
// helps keep the API clean. `Storage` is for reading/writing files. `StorageControl` is for
// managing buckets, permissions, etc.
pub mod client {
    //! Clients to interact with Google Cloud Storage.
    pub use crate::control::client::StorageControl;
    pub use crate::storage::client::Storage;
}

// [Jules]
// The `builder` module organizes "Builder" pattern structs.
// Builders are used to construct complex requests step-by-step (e.g. `OpenObject` might let
// you set headers, encryption keys, etc. before actually opening the object).
//
// SDK Perspective: Grouping builders by service (`storage` vs `storage_control`) mirrors the
// client structure.
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

// [Jules]
// Exposing models (data structures) like `Bucket`, `Object`, etc.
// `crate::control::model` likely contains the generated Protobuf message definitions
// for the API resources.
/// The messages and enums that are part of this client library.
pub use crate::control::model;

pub mod builder_ext;
pub mod model_ext;

// [Jules]
// The `stub` module is crucial for testing.
// It allows users to mock the underlying transport layer. Instead of making real network calls
// to Google Cloud, the client talks to a "stub" which returns predefined responses.
// This enables deterministic, fast unit tests for applications using this library.
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

// [Jules]
// `#[allow(dead_code)]` suppresses compiler warnings about unused code.
// This is often used for generated code (like Protobufs) where not every generated function
// or field is used by the library, but we want to keep the generated files intact.
#[allow(dead_code)]
pub(crate) mod generated;

// [Jules]
// This `google` module hierarchy mimics the package structure of the Google Cloud APIs (Protos).
// e.g. `google.storage.v2`.
//
// The `include!` macro is a very powerful Rust feature. It literally pastes the contents of
// another file into this location during compilation.
//
// SDK Perspective: We are including code generated by `tonic-build` (or `prost`) from `.proto` files.
// These files contain the Rust structs representing the API requests/responses and the gRPC client code.
// The `convert` modules likely contain code to convert between these generated types and easier-to-use
// high-level types exposed by the library.
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
        // [Jules]
        // `r#type` is used because `type` is a reserved keyword in Rust.
        // `r#` (raw identifier) allows you to use keywords as names.
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
