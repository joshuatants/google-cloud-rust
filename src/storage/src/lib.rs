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

#![cfg_attr(docsrs, feature(doc_cfg))] // Enable the `doc_cfg` feature when building documentation with `docsrs`.

pub use google_cloud_gax::Result; // Re-export the `Result` type from `google_cloud_gax`.
pub use google_cloud_gax::error::Error; // Re-export the `Error` type from `google_cloud_gax`.
// Define some shortcuts for imported crates.
pub(crate) use google_cloud_gax::client_builder::ClientBuilder; // Re-export `ClientBuilder` for internal use within the crate.
pub(crate) use google_cloud_gax::client_builder::Result as ClientBuilderResult; // Re-export `Result` as `ClientBuilderResult` for internal use.
pub(crate) use google_cloud_gax::client_builder::internal::ClientFactory; // Re-export `ClientFactory` for internal use.
pub(crate) use google_cloud_gax::client_builder::internal::new_builder as new_client_builder; // Re-export `new_builder` as `new_client_builder` for internal use.
pub(crate) use google_cloud_gax::options::RequestOptions; // Re-export `RequestOptions` for internal use.
pub(crate) use google_cloud_gax::options::internal::RequestBuilder; // Re-export `RequestBuilder` for internal use.
pub(crate) use google_cloud_gax::response::Response; // Re-export `Response` for internal use.

pub mod backoff_policy; // Declare the public module `backoff_policy`.
pub mod object_descriptor; // Declare the public module `object_descriptor`.
pub mod read_object; // Declare the public module `read_object`.
pub mod read_resume_policy; // Declare the public module `read_resume_policy`.
pub mod retry_policy; // Declare the public module `retry_policy`.
pub mod signed_url; // Declare the public module `signed_url`.
pub use crate::storage::request_options; // Re-export `request_options` from the internal `storage` module.
pub use crate::storage::streaming_source; // Re-export `streaming_source` from the internal `storage` module.

/// Re-export types from the `http` crate used in this module.
pub mod http { // Define a public module `http`.
    /// HTTP method used by the [SignedUrlBuilder][crate::builder::storage::SignedUrlBuilder].
    pub use http::Method; // Re-export `Method` from the `http` crate.

    /// Metadata attributes used by the [Client::open_object][crate::client::Storage::open_object].
    pub use http::HeaderMap; // Re-export `HeaderMap` from the `http` crate.
} // End of module `http`.

mod control; // Declare the private module `control`.
mod storage; // Declare the private module `storage`.

pub mod client { // Define a public module `client`.
    //! Clients to interact with Google Cloud Storage.
    pub use crate::control::client::StorageControl; // Re-export `StorageControl` from the `control::client` module.
    pub use crate::storage::client::Storage; // Re-export `Storage` from the `storage::client` module.
} // End of module `client`.
pub mod builder { // Define a public module `builder`.
    //! Request builders.
    pub mod storage { // Define a public module `storage` inside `builder`.
        //! Request builders for [Storage][crate::client::Storage].
        pub use crate::storage::client::ClientBuilder; // Re-export `ClientBuilder` from `storage::client`.
        pub use crate::storage::open_object::OpenObject; // Re-export `OpenObject` from `storage::open_object`.
        pub use crate::storage::read_object::ReadObject; // Re-export `ReadObject` from `storage::read_object`.
        pub use crate::storage::signed_url::SignedUrlBuilder; // Re-export `SignedUrlBuilder` from `storage::signed_url`.
        pub use crate::storage::write_object::WriteObject; // Re-export `WriteObject` from `storage::write_object`.
    } // End of module `storage`.
    pub mod storage_control { // Define a public module `storage_control`.
        //! Request builders for [StorageControl][crate::client::StorageControl].
        pub use crate::control::builder::*; // Re-export everything from `control::builder`.
        pub use crate::control::client::ClientBuilder; // Re-export `ClientBuilder` from `control::client`.
    } // End of module `storage_control`.
} // End of module `builder`.
pub mod error; // Declare the public module `error`.
/// The messages and enums that are part of this client library.
pub use crate::control::model; // Re-export `model` from `control`.
pub mod builder_ext; // Declare the public module `builder_ext`.
pub mod model_ext; // Declare the public module `model_ext`.
pub mod stub { // Define a public module `stub`.
    //! Traits to mock the clients in this library.
    //!
    //! Application developers may need to mock the clients in this library to test
    //! how their application works with different (and sometimes hard to trigger)
    //! client and service behavior. Such test can define mocks implementing the
    //! trait(s) defined in this module, initialize the client with an instance of
    //! this mock in their tests, and verify their application responds as expected.
    pub use crate::control::stub::*; // Re-export everything from `control::stub`.
    pub use crate::storage::stub::*; // Re-export everything from `storage::stub`.
    pub use crate::storage::transport::Storage as DefaultStorage; // Re-export `Storage` as `DefaultStorage` from `storage::transport`.
} // End of module `stub`.

#[allow(dead_code)] // Allow dead code for the following module.
pub(crate) mod generated; // Declare the crate-private module `generated`.

#[allow(dead_code)] // Allow dead code for the following module.
pub(crate) mod google { // Declare the crate-private module `google`.
    pub mod iam { // Define a public module `iam`.
        pub mod v1 { // Define a public module `v1`.
            include!("generated/protos/storage/google.iam.v1.rs"); // Include generated code for IAM v1.
            include!("generated/convert/iam/convert.rs"); // Include generated conversion code for IAM.
        } // End of module `v1`.
    } // End of module `iam`.
    pub mod longrunning { // Define a public module `longrunning`.
        include!("generated/protos/control/google.longrunning.rs"); // Include generated code for longrunning operations.
        include!("generated/convert/longrunning/convert.rs"); // Include generated conversion code for longrunning operations.
    } // End of module `longrunning`.
    pub mod r#type { // Define a public module `type` (escaped because `type` is a keyword).
        include!("generated/protos/storage/google.r#type.rs"); // Include generated code for common types.
        include!("generated/convert/type/convert.rs"); // Include generated conversion code for common types.
    } // End of module `type`.
    pub mod rpc { // Define a public module `rpc`.
        include!("generated/protos/storage/google.rpc.rs"); // Include generated code for RPC.
    } // End of module `rpc`.
    pub mod storage { // Define a public module `storage`.
        #[allow(deprecated)] // Allow deprecated code in the following module.
        #[allow(clippy::large_enum_variant)] // Allow large enum variants in the following module to silence Clippy.
        pub mod v2 { // Define a public module `v2`.
            include!("generated/protos/storage/google.storage.v2.rs"); // Include generated code for Storage v2.
            include!("generated/convert/storage/convert.rs"); // Include generated conversion code for Storage.
        } // End of module `v2`.
        pub mod control { // Define a public module `control`.
            pub mod v2 { // Define a public module `v2` inside `control`.
                include!("generated/protos/control/google.storage.control.v2.rs"); // Include generated code for Storage Control v2.
                include!("generated/convert/control/convert.rs"); // Include generated conversion code for Storage Control.
            } // End of module `v2`.
        } // End of module `control`.
    } // End of module `storage`.
    #[allow(unused_imports)] // Allow unused imports in the following module.
    pub mod protobuf { // Define a public module `protobuf`.
        pub use gaxi::prost::Empty; // Re-export `Empty` from `gaxi::prost`.
    } // End of module `protobuf`.
} // End of module `google`.
