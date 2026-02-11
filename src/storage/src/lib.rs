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

// Enable the doc_cfg feature when running docsrs to show feature-gated documentation.
#![cfg_attr(docsrs, feature(doc_cfg))]

// Re-export Result from google_cloud_gax.
pub use google_cloud_gax::Result;
// Re-export Error from google_cloud_gax::error.
pub use google_cloud_gax::error::Error;
// Define some shortcuts for imported crates.
// Re-export ClientBuilder for internal use.
pub(crate) use google_cloud_gax::client_builder::ClientBuilder;
// Re-export ClientBuilderResult for internal use.
pub(crate) use google_cloud_gax::client_builder::Result as ClientBuilderResult;
// Re-export ClientFactory for internal use.
pub(crate) use google_cloud_gax::client_builder::internal::ClientFactory;
// Re-export new_builder as new_client_builder for internal use.
pub(crate) use google_cloud_gax::client_builder::internal::new_builder as new_client_builder;
// Re-export RequestOptions for internal use.
pub(crate) use google_cloud_gax::options::RequestOptions;
// Re-export RequestBuilder for internal use.
pub(crate) use google_cloud_gax::options::internal::RequestBuilder;
// Re-export Response from google_cloud_gax.
pub(crate) use google_cloud_gax::response::Response;

// Declare public module backoff_policy.
pub mod backoff_policy;
// Declare public module object_descriptor.
pub mod object_descriptor;
// Declare public module read_object.
pub mod read_object;
// Declare public module read_resume_policy.
pub mod read_resume_policy;
// Declare public module retry_policy.
pub mod retry_policy;
// Declare public module signed_url.
pub mod signed_url;
// Re-export request_options from the storage module.
pub use crate::storage::request_options;
// Re-export streaming_source from the storage module.
pub use crate::storage::streaming_source;

/// Re-export types from the `http` crate used in this module.
// Define a public module named http.
pub mod http {
    /// HTTP method used by the [SignedUrlBuilder][crate::builder::storage::SignedUrlBuilder].
    // Re-export Method from the http crate.
    pub use http::Method;

    /// Metadata attributes used by the [Client::open_object][crate::client::Storage::open_object].
    // Re-export HeaderMap from the http crate.
    pub use http::HeaderMap;
}

// Declare private module control.
mod control;
// Declare private module storage.
mod storage;

// Define a public module named client.
pub mod client {
    //! Clients to interact with Google Cloud Storage.
    // Re-export StorageControl from the control::client module.
    pub use crate::control::client::StorageControl;
    // Re-export Storage from the storage::client module.
    pub use crate::storage::client::Storage;
}
// Define a public module named builder.
pub mod builder {
    //! Request builders.
    // Define a public module named storage inside builder.
    pub mod storage {
        //! Request builders for [Storage][crate::client::Storage].
        // Re-export ClientBuilder from storage::client.
        pub use crate::storage::client::ClientBuilder;
        // Re-export OpenObject from storage::open_object.
        pub use crate::storage::open_object::OpenObject;
        // Re-export ReadObject from storage::read_object.
        pub use crate::storage::read_object::ReadObject;
        // Re-export SignedUrlBuilder from storage::signed_url.
        pub use crate::storage::signed_url::SignedUrlBuilder;
        // Re-export WriteObject from storage::write_object.
        pub use crate::storage::write_object::WriteObject;
    }
    // Define a public module named storage_control inside builder.
    pub mod storage_control {
        //! Request builders for [StorageControl][crate::client::StorageControl].
        // Re-export everything from control::builder.
        pub use crate::control::builder::*;
        // Re-export ClientBuilder from control::client.
        pub use crate::control::client::ClientBuilder;
    }
}
// Declare public module error.
pub mod error;
/// The messages and enums that are part of this client library.
// Re-export model from control.
pub use crate::control::model;
// Declare public module builder_ext.
pub mod builder_ext;
// Declare public module model_ext.
pub mod model_ext;
// Declare public module stub.
pub mod stub {
    //! Traits to mock the clients in this library.
    //!
    //! Application developers may need to mock the clients in this library to test
    //! how their application works with different (and sometimes hard to trigger)
    //! client and service behavior. Such test can define mocks implementing the
    //! trait(s) defined in this module, initialize the client with an instance of
    //! this mock in their tests, and verify their application responds as expected.
    // Re-export everything from control::stub.
    pub use crate::control::stub::*;
    // Re-export everything from storage::stub.
    pub use crate::storage::stub::*;
    // Re-export Storage as DefaultStorage from storage::transport.
    pub use crate::storage::transport::Storage as DefaultStorage;
}

// Allow dead code warnings for the generated module.
#[allow(dead_code)]
// Declare crate-private module generated.
pub(crate) mod generated;

// Allow dead code warnings for the google module.
#[allow(dead_code)]
// Declare crate-private module google.
pub(crate) mod google {
    // Define public module iam.
    pub mod iam {
        // Define public module v1.
        pub mod v1 {
            // Include generated protos for iam v1.
            include!("generated/protos/storage/google.iam.v1.rs");
            // Include generated converters for iam v1.
            include!("generated/convert/iam/convert.rs");
        }
    }
    // Define public module longrunning.
    pub mod longrunning {
        // Include generated protos for longrunning operations.
        include!("generated/protos/control/google.longrunning.rs");
        // Include generated converters for longrunning operations.
        include!("generated/convert/longrunning/convert.rs");
    }
    // Define public module type (escaped as r#type).
    pub mod r#type {
        // Include generated protos for google.type.
        include!("generated/protos/storage/google.r#type.rs");
        // Include generated converters for google.type.
        include!("generated/convert/type/convert.rs");
    }
    // Define public module rpc.
    pub mod rpc {
        // Include generated protos for google.rpc.
        include!("generated/protos/storage/google.rpc.rs");
    }
    // Define public module storage.
    pub mod storage {
        // Allow deprecated items and large enum variants.
        #[allow(deprecated)]
        #[allow(clippy::large_enum_variant)]
        // Define public module v2.
        pub mod v2 {
            // Include generated protos for storage v2.
            include!("generated/protos/storage/google.storage.v2.rs");
            // Include generated converters for storage v2.
            include!("generated/convert/storage/convert.rs");
        }
        // Define public module control.
        pub mod control {
            // Define public module v2.
            pub mod v2 {
                // Include generated protos for storage control v2.
                include!("generated/protos/control/google.storage.control.v2.rs");
                // Include generated converters for storage control v2.
                include!("generated/convert/control/convert.rs");
            }
        }
    }
    // Allow unused imports in protobuf module.
    #[allow(unused_imports)]
    // Define public module protobuf.
    pub mod protobuf {
        // Re-export Empty from gaxi::prost.
        pub use gaxi::prost::Empty;
    }
}
