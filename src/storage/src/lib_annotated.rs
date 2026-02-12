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

// [Jules: Rust] The `//!` syntax indicates module-level documentation. Unlike `///`, which documents the item
// [Jules: Rust] that follows it, `//!` documents the item that *contains* it (in this case, the crate root).
// [Jules: Rust] This documentation will appear at the top of the generated HTML documentation for the crate.
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

// [Jules: Rust] The `#![...]` syntax is used for crate-level attributes. Here, `cfg_attr` conditionally
// [Jules: Rust] applies an attribute. Specifically, if the configuration matches `docsrs` (which is true when
// [Jules: Rust] building documentation on docs.rs), it applies `feature(doc_cfg)`.
// [Jules: Rust] `feature(doc_cfg)` allows annotating items with `#[doc(cfg(...))]` to show in the documentation
// [Jules: Rust] that an item is only available when certain features are enabled. This is an unstable feature,
// [Jules: Rust] hence the need to enable it explicitly.
#![cfg_attr(docsrs, feature(doc_cfg))]

// [Jules: Rust] `pub use` re-exports items from another module or crate, making them part of this module's public API.
// [Jules: Rust] This allows users to access `Result` directly from `google_cloud_storage` instead of digging into dependencies.
// [Jules: SDK] We re-export `Result` and `Error` from `google_cloud_gax`. GAX (Google API Extensions) provides
// [Jules: SDK] common functionality for Google Cloud client libraries. By standardizing these types, we ensure
// [Jules: SDK] consistency across different Google Cloud services (Storage, Pub/Sub, etc.).
pub use google_cloud_gax::Result;
pub use google_cloud_gax::error::Error;

// [Jules: Rust] `pub(crate)` makes an item public only within the current crate. It is not accessible to users of the library.
// [Jules: Rust] This is useful for sharing code between modules inside the crate without exposing implementation details.
// Define some shortcuts for imported crates.
pub(crate) use google_cloud_gax::client_builder::ClientBuilder;
pub(crate) use google_cloud_gax::client_builder::Result as ClientBuilderResult;
pub(crate) use google_cloud_gax::client_builder::internal::ClientFactory;
pub(crate) use google_cloud_gax::client_builder::internal::new_builder as new_client_builder;
pub(crate) use google_cloud_gax::options::RequestOptions;
pub(crate) use google_cloud_gax::options::internal::RequestBuilder;
pub(crate) use google_cloud_gax::response::Response;

// [Jules: Rust] `pub mod` declares a public module. This looks for a file with the same name (e.g., `backoff_policy.rs`)
// [Jules: Rust] or a directory with a `mod.rs` file. It exposes the module and its public items to users of the crate.
// [Jules: SDK] These modules contain policies and utilities specific to Google Cloud Storage interactions.
pub mod backoff_policy;
pub mod object_descriptor;
pub mod read_object;
pub mod read_resume_policy;
pub mod retry_policy;
pub mod signed_url;

// [Jules: Rust] `pub use crate::...` re-exports items defined elsewhere in this crate.
// [Jules: Rust] This helps organize the public API structure independently of the internal file structure.
// [Jules: SDK] `request_options` and `streaming_source` are likely internal implementations that we want to expose
// [Jules: SDK] at the top level for convenience or logical grouping.
pub use crate::storage::request_options;
pub use crate::storage::streaming_source;

/// Re-export types from the `http` crate used in this module.
pub mod http {
    /// HTTP method used by the [SignedUrlBuilder][crate::builder::storage::SignedUrlBuilder].
    pub use http::Method;

    /// Metadata attributes used by the [Client::open_object][crate::client::Storage::open_object].
    pub use http::HeaderMap;
}

// [Jules: Rust] `mod control;` and `mod storage;` declare modules that are private to this crate (since they lack `pub`).
// [Jules: Rust] Users cannot access `google_cloud_storage::control` directly, unless items are re-exported.
mod control;
mod storage;

// [Jules: Rust] We define a public module `client` inline using curly braces `{ ... }`.
// [Jules: Rust] This is an alternative to putting the module content in a separate file.
pub mod client {
    //! Clients to interact with Google Cloud Storage.
    // [Jules: SDK] We re-export the main client structs here. `StorageControl` is for control plane operations
    // [Jules: SDK] (like creating buckets), while `Storage` is for data plane operations (like reading/writing objects).
    pub use crate::control::client::StorageControl;
    pub use crate::storage::client::Storage;
}

// [Jules: Rust] The `builder` module organizes the builder pattern implementations used to construct requests.
pub mod builder {
    //! Request builders.
    pub mod storage {
        //! Request builders for [Storage][crate::client::Storage].
        // [Jules: SDK] These builders correspond to specific operations on the Storage client.
        // [Jules: SDK] For example, `OpenObject` is used to configure parameters before opening an object for reading.
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

// [Jules: Rust] Expose the `error` module so users can handle errors specific to this crate.
pub mod error;

/// The messages and enums that are part of this client library.
// [Jules: SDK] The `model` module typically contains data structures representing the API resources (e.g., Bucket, Object).
// [Jules: SDK] It's common in Google Cloud libraries to separate the data model from the client logic.
pub use crate::control::model;

// [Jules: Rust] `builder_ext` and `model_ext` likely contain extension traits or helper structs that add functionality
// [Jules: Rust] to the generated builders and models. The `_ext` suffix is a common convention for "extensions".
pub mod builder_ext;
pub mod model_ext;

// [Jules: Rust] The `stub` module is useful for testing. It defines traits that abstract the underlying service interactions.
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
    // [Jules: Rust] Renaming `Storage` to `DefaultStorage` on re-export to avoid confusion with the trait or other types named `Storage`.
    pub use crate::storage::transport::Storage as DefaultStorage;
}

// [Jules: Rust] `#[allow(dead_code)]` suppresses warnings about unused code. This is common in generated code or
// [Jules: Rust] library code where not all internal functions are used within the crate itself.
#[allow(dead_code)]
pub(crate) mod generated;

// [Jules: SDK] This module structure mirrors the Google Cloud API protobuf package structure.
// [Jules: SDK] It includes the generated Rust code from the Protocol Buffers definitions.
// [Jules: SDK] `iam`, `longrunning`, `type`, `rpc` are common dependencies shared across Google Cloud APIs.
#[allow(dead_code)]
pub(crate) mod google {
    pub mod iam {
        pub mod v1 {
            // [Jules: Rust] `include!` macro includes the file content as if it were written here.
            // [Jules: Rust] This is used to include code generated by `prost` (a Protocol Buffers implementation) during the build.
            include!("generated/protos/storage/google.iam.v1.rs");
            include!("generated/convert/iam/convert.rs");
        }
    }
    pub mod longrunning {
        include!("generated/protos/control/google.longrunning.rs");
        include!("generated/convert/longrunning/convert.rs");
    }
    pub mod r#type {
        // [Jules: Rust] `r#type` uses a raw identifier because `type` is a reserved keyword in Rust.
        // [Jules: Rust] The `r#` prefix allows using keywords as identifiers.
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
