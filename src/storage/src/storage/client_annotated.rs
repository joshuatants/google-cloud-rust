// ANNOTATED AND DONE
// [Jules: SDK] This file defines the `Storage` client, which is the primary interface for interacting
// [Jules: SDK] with the Google Cloud Storage service. It handles connection pooling, authentication,
// [Jules: SDK] and provides methods to perform operations like reading and writing objects.
// [Jules: Rust] The `client` module is often where the main business logic of a library resides.

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

use super::request_options::RequestOptions;
// [Jules: Rust] `super` refers to the parent module (`storage`).
// [Jules: SDK] `RequestOptions` allows configuring individual requests (e.g., headers, timeout).

use crate::Error;
// [Jules: Rust] `crate` refers to the root of the crate.

use crate::builder::storage::ReadObject;
use crate::builder::storage::WriteObject;
// [Jules: SDK] Builders for specific operations. This pattern allows for optional parameters
// [Jules: SDK] without function overloading (which Rust doesn't support directly).

use crate::read_resume_policy::ReadResumePolicy;
use crate::storage::bidi::OpenObject;
use crate::storage::common_options::CommonOptions;
use crate::streaming_source::Payload;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use gaxi::http::reqwest::RequestBuilder;
use gaxi::options::{ClientConfig, Credentials};
use google_cloud_auth::credentials::{Builder as CredentialsBuilder, CacheableResource};
use google_cloud_gax::client_builder::{Error as BuilderError, Result as BuilderResult};
use http::Extensions;
use std::sync::Arc;
// [Jules: Rust] `Arc` (Atomic Reference Counted) is a thread-safe reference-counting pointer.
// [Jules: Rust] It allows multiple parts of the program to share ownership of the same data.
// [Jules: Rust] When the last `Arc` is dropped, the data is cleaned up.

/// Implements a client for the Cloud Storage API.
///
/// # Example
/// ```
/// # async fn sample() -> anyhow::Result<()> {
/// # use google_cloud_storage::client::Storage;
/// let client = Storage::builder().build().await?;
/// // use `client` to make requests to Cloud Storage.
/// # Ok(()) }
/// ```
///
/// # Configuration
///
/// To configure `Storage` use the `with_*` methods in the type returned
/// by [builder()][Storage::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://storage.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
///   with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// # Pooling and Cloning
///
/// `Storage` holds a connection pool internally, it is advised to
/// create one and then reuse it.  You do not need to wrap `Storage` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
///
/// # Service Description
///
/// The Cloud Storage API allows applications to read and write data through
/// the abstractions of buckets and objects. For a description of these
/// abstractions please see <https://cloud.google.com/storage/docs>.
///
/// Resources are named as follows:
///
/// - Projects are referred to as they are defined by the Resource Manager API,
///   using strings like `projects/123456` or `projects/my-string-id`.
///
/// - Buckets are named using string names of the form:
///   `projects/{project}/buckets/{bucket}`
///   For globally unique buckets, `_` may be substituted for the project.
///
/// - Objects are uniquely identified by their name along with the name of the
///   bucket they belong to, as separate strings in this API. For example:
///   ```no_rust
///   bucket = "projects/_/buckets/my-bucket"
///   object = "my-object/with/a/folder-like/name"
///   ```
///   Note that object names can contain `/` characters, which are treated as
///   any other character (no special directory semantics).
///
/// [with_endpoint()]: ClientBuilder::with_endpoint
/// [with_credentials()]: ClientBuilder::with_credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
#[derive(Clone, Debug)]
// [Jules: Rust] `#[derive(...)]` automatically implements traits for the struct.
// [Jules: Rust] `Clone` allows creating a copy of the client (cheap because of `Arc`).
// [Jules: Rust] `Debug` allows printing the struct for debugging purposes using `{:?}`.
pub struct Storage<S = crate::stub::DefaultStorage>
where
    S: crate::stub::Storage + 'static,
{
    stub: std::sync::Arc<S>,
    options: RequestOptions,
}
// [Jules: Rust] This struct is generic over `S`, which defaults to `DefaultStorage`.
// [Jules: Rust] `where` clause specifies constraints: `S` must implement the `Storage` trait
// [Jules: Rust] and live for the `'static` lifetime (essentially the entire program duration).
// [Jules: SDK] By using a generic `S`, we can swap the real network transport (`DefaultStorage`)
// [Jules: SDK] with a mock implementation for testing (`mockall` generated mocks).

#[derive(Clone, Debug)]
pub(crate) struct StorageInner {
    pub client: gaxi::http::ReqwestClient,
    pub cred: Credentials,
    pub options: RequestOptions,
    pub grpc: gaxi::grpc::Client,
}
// [Jules: Rust] `pub(crate)` makes this struct visible only within this crate.
// [Jules: SDK] `StorageInner` holds the actual state: HTTP client, credentials, and gRPC client.
// [Jules: SDK] The `Storage` struct wraps this (via the stub) to provide a clean public API.

impl Storage {
    // [Jules: Rust] `impl Storage` block defines methods associated with the `Storage` struct
    // [Jules: Rust] when `S` is the default type.

    /// Returns a builder for [Storage].
    ///
    /// # Example
    /// ```
    /// # use google_cloud_storage::client::Storage;
    /// # async fn sample() -> anyhow::Result<()> {
    /// let client = Storage::builder().build().await?;
    /// # Ok(()) }
    /// ```
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }
}

impl<S> Storage<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    // [Jules: Rust] This `impl` block defines methods for `Storage` with *any* type `S`
    // [Jules: Rust] that implements the required trait.

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub(stub: S) -> Self
    where
        S: super::stub::Storage + 'static,
    {
        Self {
            stub: std::sync::Arc::new(stub),
            options: RequestOptions::new(),
        }
    }
    // [Jules: Rust] `Self` refers to the type `Storage<S>`.

    /// Write an object with data from any data source.
    ///
    /// # Example
    /// ```
    /// # use google_cloud_storage::client::Storage;
    /// # async fn sample(client: &Storage) -> anyhow::Result<()> {
    /// let response = client
    ///     .write_object("projects/_/buckets/my-bucket", "my-object", "hello world")
    ///     .send_buffered()
    ///     .await?;
    /// println!("response details={response:?}");
    /// # Ok(()) }
    /// ```
    ///
    /// # Example
    /// ```
    /// # use google_cloud_storage::client::Storage;
    /// # async fn sample(client: &Storage) -> anyhow::Result<()> {
    /// let response = client
    ///     .write_object("projects/_/buckets/my-bucket", "my-object", "hello world")
    ///     .send_unbuffered()
    ///     .await?;
    /// println!("response details={response:?}");
    /// # Ok(()) }
    /// ```
    ///
    /// You can use many different types as the payload. For example, a string,
    /// a [bytes::Bytes], a [tokio::fs::File], or a custom type that implements
    /// the [StreamingSource] trait.
    ///
    /// If your data source also implements [Seek], prefer [send_unbuffered()]
    /// to start the write. Otherwise use [send_buffered()].
    ///
    /// # Parameters
    /// * `bucket` - the bucket name containing the object. In
    ///   `projects/_/buckets/{bucket_id}` format.
    /// * `object` - the object name.
    /// * `payload` - the object data.
    ///
    /// [Seek]: crate::streaming_source::Seek
    /// [StreamingSource]: crate::streaming_source::StreamingSource
    /// [send_buffered()]: crate::builder::storage::WriteObject::send_buffered
    /// [send_unbuffered()]: crate::builder::storage::WriteObject::send_unbuffered
    pub fn write_object<B, O, T, P>(&self, bucket: B, object: O, payload: T) -> WriteObject<P, S>
    where
        B: Into<String>,
        O: Into<String>,
        T: Into<Payload<P>>,
    {
        // [Jules: Rust] Generics `B`, `O`, `T` allow flexibility. `Into<String>` means the function
        // [Jules: Rust] accepts anything that can be converted into a `String` (e.g., `&str`, `String`).
        WriteObject::new(
            self.stub.clone(),
            bucket,
            object,
            payload,
            self.options.clone(),
        )
    }
    // [Jules: SDK] This method returns a `WriteObject` builder, allowing further configuration
    // [Jules: SDK] before executing the request (e.g., setting metadata).

    /// Reads the contents of an object.
    ///
    /// # Example
    /// ```
    /// # use google_cloud_storage::client::Storage;
    /// # async fn sample(client: &Storage) -> anyhow::Result<()> {
    /// let mut resp = client
    ///     .read_object("projects/_/buckets/my-bucket", "my-object")
    ///     .send()
    ///     .await?;
    /// let mut contents = Vec::new();
    /// while let Some(chunk) = resp.next().await.transpose()? {
    ///   contents.extend_from_slice(&chunk);
    /// }
    /// println!("object contents={:?}", bytes::Bytes::from_owner(contents));
    /// # Ok(()) }
    /// ```
    ///
    /// # Parameters
    /// * `bucket` - the bucket name containing the object. In
    ///   `projects/_/buckets/{bucket_id}` format.
    /// * `object` - the object name.
    pub fn read_object<B, O>(&self, bucket: B, object: O) -> ReadObject<S>
    where
        B: Into<String>,
        O: Into<String>,
    {
        ReadObject::new(self.stub.clone(), bucket, object, self.options.clone())
    }

    /// Opens an object to read its contents using concurrent ranged reads.
    ///
    /// # Example
    /// ```
    /// # use google_cloud_storage::client::Storage;
    /// # async fn sample(client: &Storage) -> anyhow::Result<()> {
    /// use google_cloud_storage::model_ext::ReadRange;
    /// let descriptor = client
    ///     .open_object("projects/_/buckets/my-bucket", "my-object")
    ///     .send()
    ///     .await?;
    /// // Print the object metadata
    /// println!("metadata = {:?}", descriptor.object());
    /// // Read 2000 bytes starting at offset 1000.
    /// let mut reader = descriptor.read_range(ReadRange::segment(1000, 2000)).await;
    /// let mut contents = Vec::new();
    /// while let Some(chunk) = reader.next().await.transpose()? {
    ///   contents.extend_from_slice(&chunk);
    /// }
    /// println!("range contents={:?}", bytes::Bytes::from_owner(contents));
    /// // `descriptor` can be used to read more ranges, concurrently if needed.
    /// # Ok(()) }
    /// ```
    ///
    /// # Example: open and read in a single RPC
    /// ```
    /// # use google_cloud_storage::client::Storage;
    /// # async fn sample(client: &Storage) -> anyhow::Result<()> {
    /// use google_cloud_storage::model_ext::ReadRange;
    /// let (descriptor, mut reader) = client
    ///     .open_object("projects/_/buckets/my-bucket", "my-object")
    ///     .send_and_read(ReadRange::segment(1000, 2000))
    ///     .await?;
    /// // `descriptor` can be used to read more ranges.
    /// # Ok(()) }
    /// ```
    ///
    /// <div class="warning">
    /// The APIs used by this method are only enabled for some projects and
    /// buckets. Contact your account team to enable this API.
    /// </div>
    ///
    /// # Parameters
    /// * `bucket` - the bucket name containing the object. In
    ///   `projects/_/buckets/{bucket_id}` format.
    /// * `object` - the object name.
    pub fn open_object<B, O>(&self, bucket: B, object: O) -> OpenObject<S>
    where
        B: Into<String>,
        O: Into<String>,
    {
        OpenObject::new(
            bucket.into(),
            object.into(),
            self.stub.clone(),
            self.options.clone(),
        )
    }
}

impl Storage {
    pub(crate) async fn new(builder: ClientBuilder) -> BuilderResult<Self> {
        // [Jules: Rust] `async fn` indicates this function is asynchronous and returns a `Future`.
        // [Jules: Rust] It must be `.await`ed to execute.
        let inner = StorageInner::from_parts(builder).await?;
        // [Jules: Rust] `?` operator propagates errors. If `await` returns `Err`, the function returns early with that `Err`.
        let options = inner.options.clone();
        let stub = crate::storage::transport::Storage::new(Arc::new(inner));
        Ok(Self { stub, options })
    }
}

impl StorageInner {
    /// Builds a client assuming `config.cred` and `config.endpoint` are initialized, panics otherwise.
    pub(self) fn new(
        // [Jules: Rust] `pub(self)` means private to this module (same as no modifier usually, but explicit).
        client: gaxi::http::ReqwestClient,
        cred: Credentials,
        options: RequestOptions,
        grpc: gaxi::grpc::Client,
    ) -> Self {
        Self {
            client,
            cred,
            options,
            grpc,
        }
    }

    pub(self) async fn from_parts(builder: ClientBuilder) -> BuilderResult<Self> {
        let (mut config, options) = builder.into_parts()?;
        config.disable_automatic_decompression = true;
        let cred = config
            .cred
            .clone()
            .expect("into_parts() assigns default credentials");
        // [Jules: Rust] `expect` panics with the given message if the value is `None`.

        let client = gaxi::http::ReqwestClient::new(config.clone(), super::DEFAULT_HOST).await?;

        let inner = StorageInner::new(
            client,
            cred,
            options,
            gaxi::grpc::Client::new(config, super::DEFAULT_HOST).await?,
        );
        Ok(inner)
    }

    // Helper method to apply authentication headers to the request builder.
    pub async fn apply_auth_headers(
        &self,
        builder: RequestBuilder,
    ) -> crate::Result<RequestBuilder> {
        let cached_auth_headers = self
            .cred
            .headers(Extensions::new())
            .await
            .map_err(Error::authentication)?;

        let auth_headers = match cached_auth_headers {
            CacheableResource::New { data, .. } => data,
            CacheableResource::NotModified => {
                unreachable!("headers are not cached");
                // [Jules: Rust] `unreachable!` macro panics indicating code flow that should never happen.
            }
        };

        let builder = builder.headers(auth_headers);
        Ok(builder)
    }
}

/// A builder for [Storage].
///
/// ```
/// # use google_cloud_storage::client::Storage;
/// # async fn sample() -> anyhow::Result<()> {
/// let builder = Storage::builder();
/// let client = builder
///     .with_endpoint("https://storage.googleapis.com")
///     .build()
///     .await?;
/// # Ok(()) }
/// ```
pub struct ClientBuilder {
    // Common options for all clients (generated or not).
    pub(crate) config: ClientConfig,
    // Specific options for the storage client. `RequestOptions` also requires
    // these, it makes sense to share them.
    common_options: CommonOptions,
}

impl ClientBuilder {
    pub(crate) fn new() -> Self {
        let mut config = ClientConfig::default();
        config.retry_policy = Some(Arc::new(crate::retry_policy::storage_default()));
        config.backoff_policy = Some(Arc::new(crate::backoff_policy::default()));
        {
            let count = std::thread::available_parallelism().ok();
            config.grpc_subchannel_count = Some(count.map(|x| x.get()).unwrap_or(1));
            // [Jules: Rust] `Option::map` applies a function to the contained value if it exists.
            // [Jules: Rust] `unwrap_or(default)` returns the value if `Some`, otherwise `default`.
        }
        let common_options = CommonOptions::new();
        Self {
            config,
            common_options,
        }
    }

    /// Creates a new client.
    ///
    /// # Example
    /// ```
    /// # use google_cloud_storage::client::Storage;
    /// # async fn sample() -> anyhow::Result<()> {
    /// let client = Storage::builder().build().await?;
    /// # Ok(()) }
    /// ```
    pub async fn build(self) -> BuilderResult<Storage> {
        Storage::new(self).await
    }

    /// Sets the endpoint.
    ///
    /// # Example
    /// ```
    /// # use google_cloud_storage::client::Storage;
    /// # async fn sample() -> anyhow::Result<()> {
    /// let client = Storage::builder()
    ///     .with_endpoint("https://private.googleapis.com")
    ///     .build()
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub fn with_endpoint<V: Into<String>>(mut self, v: V) -> Self {
        self.config.endpoint = Some(v.into());
        self
    }
    // [Jules: Rust] Builder methods typically take `mut self` to modify the builder in place and return it,
    // [Jules: Rust] allowing method chaining.

    /// Configures the authentication credentials.
    ///
    /// Google Cloud Storage requires authentication for most buckets. Use this
    /// method to change the credentials used by the client. More information
    /// about valid credentials types can be found in the [google-cloud-auth]
    /// crate documentation.
    ///
    /// # Example
    /// ```
    /// # use google_cloud_storage::client::Storage;
    /// # async fn sample() -> anyhow::Result<()> {
    /// use google_cloud_auth::credentials::mds;
    /// let client = Storage::builder()
    ///     .with_credentials(
    ///         mds::Builder::default()
    ///             .with_scopes(["https://www.googleapis.com/auth/cloud-platform.read-only"])
    ///             .build()?)
    ///     .build()
    ///     .await?;
    /// # Ok(()) }
    /// ```
    ///
    /// [google-cloud-auth]: https://docs.rs/google-cloud-auth
    pub fn with_credentials<V: Into<Credentials>>(mut self, v: V) -> Self {
        self.config.cred = Some(v.into());
        self
    }

    /// Configure the retry policy.
    ///
    /// The client libraries can automatically retry operations that fail. The
    /// retry policy controls what errors are considered retryable, sets limits
    /// on the number of attempts or the time trying to make attempts.
    ///
    /// # Example
    /// ```
    /// # use google_cloud_storage::client::Storage;
    /// # async fn sample() -> anyhow::Result<()> {
    /// use google_cloud_gax::retry_policy::{AlwaysRetry, RetryPolicyExt};
    /// let client = Storage::builder()
    ///     .with_retry_policy(AlwaysRetry.with_attempt_limit(3))
    ///     .build()
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub fn with_retry_policy<V: Into<google_cloud_gax::retry_policy::RetryPolicyArg>>(
        mut self,
        v: V,
    ) -> Self {
        self.config.retry_policy = Some(v.into().into());
        self
    }

    /// Configure the retry backoff policy.
    ///
    /// The client libraries can automatically retry operations that fail. The
    /// backoff policy controls how long to wait in between retry attempts.
    ///
    /// # Example
    /// ```
    /// # use google_cloud_storage::client::Storage;
    /// # async fn sample() -> anyhow::Result<()> {
    /// use google_cloud_gax::exponential_backoff::ExponentialBackoff;
    /// use std::time::Duration;
    /// let policy = ExponentialBackoff::default();
    /// let client = Storage::builder()
    ///     .with_backoff_policy(policy)
    ///     .build()
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub fn with_backoff_policy<V: Into<google_cloud_gax::backoff_policy::BackoffPolicyArg>>(
        mut self,
        v: V,
    ) -> Self {
        self.config.backoff_policy = Some(v.into().into());
        self
    }

    /// Configure the retry throttler.
    ///
    /// Advanced applications may want to configure a retry throttler to
    /// [Address Cascading Failures] and when [Handling Overload] conditions.
    /// The client libraries throttle their retry loop, using a policy to
    /// control the throttling algorithm. Use this method to fine tune or
    /// customize the default retry throtler.
    ///
    /// [Handling Overload]: https://sre.google/sre-book/handling-overload/
    /// [Address Cascading Failures]: https://sre.google/sre-book/addressing-cascading-failures/
    ///
    /// # Example
    /// ```
    /// # use google_cloud_storage::client::Storage;
    /// # async fn sample() -> anyhow::Result<()> {
    /// use google_cloud_gax::retry_throttler::AdaptiveThrottler;
    /// let client = Storage::builder()
    ///     .with_retry_throttler(AdaptiveThrottler::default())
    ///     .build()
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub fn with_retry_throttler<V: Into<google_cloud_gax::retry_throttler::RetryThrottlerArg>>(
        mut self,
        v: V,
    ) -> Self {
        self.config.retry_throttler = v.into().into();
        self
    }

    /// Sets the payload size threshold to switch from single-shot to resumable uploads.
    ///
    /// # Example
    /// ```
    /// # use google_cloud_storage::client::Storage;
    /// # async fn sample() -> anyhow::Result<()> {
    /// let client = Storage::builder()
    ///     .with_resumable_upload_threshold(0_usize) // Forces a resumable upload.
    ///     .build()
    ///     .await?;
    /// let response = client
    ///     .write_object("projects/_/buckets/my-bucket", "my-object", "hello world")
    ///     .send_buffered()
    ///     .await?;
    /// println!("response details={response:?}");
    /// # Ok(()) }
    /// ```
    ///
    /// The client library can write objects using [single-shot] or [resumable]
    /// uploads. For small objects, single-shot uploads offer better
    /// performance, as they require a single HTTP transfer. For larger objects,
    /// the additional request latency is not significant, and resumable uploads
    /// offer better recovery on errors.
    ///
    /// The library automatically selects resumable uploads when the payload is
    /// equal to or larger than this option. For smaller writes the client
    /// library uses single-shot uploads.
    ///
    /// The exact threshold depends on where the application is deployed and
    /// destination bucket location with respect to where the application is
    /// running. The library defaults should work well in most cases, but some
    /// applications may benefit from fine-tuning.
    ///
    /// [single-shot]: https://cloud.google.com/storage/docs/uploading-objects
    /// [resumable]: https://cloud.google.com/storage/docs/resumable-uploads
    pub fn with_resumable_upload_threshold<V: Into<usize>>(mut self, v: V) -> Self {
        self.common_options.resumable_upload_threshold = v.into();
        self
    }

    /// Changes the buffer size for some resumable uploads.
    ///
    /// # Example
    /// ```
    /// # use google_cloud_storage::client::Storage;
    /// # async fn sample() -> anyhow::Result<()> {
    /// let client = Storage::builder()
    ///     .with_resumable_upload_buffer_size(32 * 1024 * 1024_usize)
    ///     .build()
    ///     .await?;
    /// let response = client
    ///     .write_object("projects/_/buckets/my-bucket", "my-object", "hello world")
    ///     .send_buffered()
    ///     .await?;
    /// println!("response details={response:?}");
    /// # Ok(()) }
    /// ```
    ///
    /// When performing [resumable uploads] from sources without [Seek] the
    /// client library needs to buffer data in memory until it is persisted by
    /// the service. Otherwise the data would be lost if the upload is
    /// interrupted. Applications may want to tune this buffer size:
    ///
    /// - Use smaller buffer sizes to support more concurrent writes in the
    ///   same application.
    /// - Use larger buffer sizes for better throughput. Sending many small
    ///   buffers stalls the writer until the client receives a successful
    ///   response from the service.
    ///
    /// Keep in mind that there are diminishing returns on using larger buffers.
    ///
    /// [resumable uploads]: https://cloud.google.com/storage/docs/resumable-uploads
    /// [Seek]: crate::streaming_source::Seek
    pub fn with_resumable_upload_buffer_size<V: Into<usize>>(mut self, v: V) -> Self {
        self.common_options.resumable_upload_buffer_size = v.into();
        self
    }

    /// Configure the resume policy for object reads.
    ///
    /// The Cloud Storage client library can automatically resume a read request
    /// that is interrupted by a transient error. Applications may want to
    /// limit the number of read attempts, or may wish to expand the type
    /// of errors treated as retryable.
    ///
    /// # Example
    /// ```
    /// # use google_cloud_storage::client::Storage;
    /// # async fn sample() -> anyhow::Result<()> {
    /// use google_cloud_storage::read_resume_policy::{AlwaysResume, ReadResumePolicyExt};
    /// let client = Storage::builder()
    ///     .with_read_resume_policy(AlwaysResume.with_attempt_limit(3))
    ///     .build()
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub fn with_read_resume_policy<V>(mut self, v: V) -> Self
    where
        V: ReadResumePolicy + 'static,
    {
        self.common_options.read_resume_policy = Arc::new(v);
        self
    }

    /// Configure the number of subchannels used by the client.
    ///
    /// # Example
    /// ```
    /// # use google_cloud_storage::client::Storage;
    /// # async fn sample() -> anyhow::Result<()> {
    /// // By default the client uses `count` subchannels.
    /// let count = std::thread::available_parallelism()?.get();
    /// let client = Storage::builder()
    ///     .with_grpc_subchannel_count(std::cmp::max(1, count / 2))
    ///     .build()
    ///     .await?;
    /// # Ok(()) }
    /// ```
    ///
    /// gRPC-based clients may exhibit high latency if many requests need to be
    /// demuxed over a single HTTP/2 connection (often called a *subchannel* in gRPC).
    /// Consider using more subchannels if your application makes many
    /// concurrent requests. Consider using fewer subchannels if your
    /// application needs the file descriptors for other purposes.
    ///
    /// Keep in mind that Google Cloud limits the number of concurrent RPCs in
    /// a single connection to about 100.
    pub fn with_grpc_subchannel_count(mut self, v: usize) -> Self {
        self.config.grpc_subchannel_count = Some(v);
        self
    }

    pub(crate) fn apply_default_credentials(&mut self) -> BuilderResult<()> {
        if self.config.cred.is_some() {
            return Ok(());
        };
        let default = CredentialsBuilder::default()
            .build()
            .map_err(BuilderError::cred)?;
        self.config.cred = Some(default);
        Ok(())
    }

    pub(crate) fn apply_default_endpoint(&mut self) -> BuilderResult<()> {
        let _ = self
            .config
            .endpoint
            .get_or_insert_with(|| super::DEFAULT_HOST.to_string());
        Ok(())
    }

    // Breaks the builder into its parts, with defaults applied.
    pub(crate) fn into_parts(
        mut self,
    ) -> google_cloud_gax::client_builder::Result<(ClientConfig, RequestOptions)> {
        self.apply_default_credentials()?;
        self.apply_default_endpoint()?;
        let request_options =
            RequestOptions::new_with_client_config(&self.config, self.common_options);
        Ok((self.config, request_options))
    }
}

/// The set of characters that are percent encoded.
///
/// This set is defined at https://cloud.google.com/storage/docs/request-endpoints#encoding:
///
/// Encode the following characters when they appear in either the object name
/// or query string of a request URL:
///     !, #, $, &, ', (, ), *, +, ,, /, :, ;, =, ?, @, [, ], and space characters.
pub(crate) const ENCODED_CHARS: percent_encoding::AsciiSet = percent_encoding::CONTROLS
    .add(b'!')
    .add(b'#')
    .add(b'$')
    .add(b'&')
    .add(b'\'')
    .add(b'(')
    .add(b')')
    .add(b'*')
    .add(b'+')
    .add(b',')
    .add(b'/')
    .add(b':')
    .add(b';')
    .add(b'=')
    .add(b'?')
    .add(b'@')
    .add(b'[')
    .add(b']')
    .add(b' ');

/// Percent encode a string.
///
/// To ensure compatibility certain characters need to be encoded when they appear
/// in either the object name or query string of a request URL.
pub(crate) fn enc(value: &str) -> String {
    percent_encoding::utf8_percent_encode(value, &ENCODED_CHARS).to_string()
}

pub(crate) fn apply_customer_supplied_encryption_headers(
    builder: RequestBuilder,
    common_object_request_params: &Option<crate::model::CommonObjectRequestParams>,
) -> RequestBuilder {
    common_object_request_params.iter().fold(builder, |b, v| {
        // [Jules: Rust] `fold` is an iterator method that accumulates a value (the `builder` here).
        b.header(
            "x-goog-encryption-algorithm",
            v.encryption_algorithm.clone(),
        )
        .header(
            "x-goog-encryption-key",
            BASE64_STANDARD.encode(v.encryption_key_bytes.clone()),
        )
        .header(
            "x-goog-encryption-key-sha256",
            BASE64_STANDARD.encode(v.encryption_key_sha256_bytes.clone()),
        )
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use google_cloud_auth::credentials::anonymous::Builder as Anonymous;
    use google_cloud_gax::retry_result::RetryResult;
    use google_cloud_gax::retry_state::RetryState;
    use std::{sync::Arc, time::Duration};

    #[test]
    fn default_settings() {
        let builder = ClientBuilder::new().with_credentials(Anonymous::new().build());
        let config = builder.config;
        assert!(config.retry_policy.is_some(), "{config:?}");
        assert!(config.backoff_policy.is_some(), "{config:?}");
        {
            assert!(
                config.grpc_subchannel_count.is_some_and(|v| v >= 1),
                "{config:?}"
            );
        }
    }

    #[test]
    fn subchannel_count() {
        let builder = ClientBuilder::new()
            .with_credentials(Anonymous::new().build())
            .with_grpc_subchannel_count(42);
        let config = builder.config;
        assert!(
            config.grpc_subchannel_count.is_some_and(|v| v == 42),
            "{config:?}"
        );
    }

    pub(crate) fn test_builder() -> ClientBuilder {
        ClientBuilder::new()
            .with_credentials(Anonymous::new().build())
            .with_endpoint("http://private.googleapis.com")
            .with_backoff_policy(
                google_cloud_gax::exponential_backoff::ExponentialBackoffBuilder::new()
                    .with_initial_delay(Duration::from_millis(1))
                    .with_maximum_delay(Duration::from_millis(2))
                    .build()
                    .expect("hard coded policy should build correctly"),
            )
    }

    /// This is used by the request builder tests.
    pub(crate) async fn test_inner_client(builder: ClientBuilder) -> Arc<StorageInner> {
        let inner = StorageInner::from_parts(builder)
            .await
            .expect("creating an test inner client succeeds");
        Arc::new(inner)
    }

    mockall::mock! {
        #[derive(Debug)]
        pub RetryThrottler {}

        impl google_cloud_gax::retry_throttler::RetryThrottler for RetryThrottler {
            fn throttle_retry_attempt(&self) -> bool;
            fn on_retry_failure(&mut self, flow: &RetryResult);
            fn on_success(&mut self);
        }
    }

    mockall::mock! {
        #[derive(Debug)]
        pub RetryPolicy {}

        impl google_cloud_gax::retry_policy::RetryPolicy for RetryPolicy {
            fn on_error(&self, state: &RetryState, error: google_cloud_gax::error::Error) -> RetryResult;
        }
    }

    mockall::mock! {
        #[derive(Debug)]
        pub BackoffPolicy {}

        impl google_cloud_gax::backoff_policy::BackoffPolicy for BackoffPolicy {
            fn on_failure(&self, state: &RetryState) -> std::time::Duration;
        }
    }

    mockall::mock! {
        #[derive(Debug)]
        pub ReadResumePolicy {}

        impl crate::read_resume_policy::ReadResumePolicy for ReadResumePolicy {
            fn on_error(&self, query: &crate::read_resume_policy::ResumeQuery, error: google_cloud_gax::error::Error) -> crate::read_resume_policy::ResumeResult;
        }
    }
}
