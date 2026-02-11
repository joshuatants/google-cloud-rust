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

use super::request_options::RequestOptions; // Import `RequestOptions` from the parent module's `request_options` submodule.
use crate::Error; // Import `Error` from the crate root.
use crate::builder::storage::ReadObject; // Import `ReadObject` from the `builder::storage` module.
use crate::builder::storage::WriteObject; // Import `WriteObject` from the `builder::storage` module.
use crate::read_resume_policy::ReadResumePolicy; // Import `ReadResumePolicy` from the `read_resume_policy` module.
use crate::storage::bidi::OpenObject; // Import `OpenObject` from the `storage::bidi` module.
use crate::storage::common_options::CommonOptions; // Import `CommonOptions` from the `storage::common_options` module.
use crate::streaming_source::Payload; // Import `Payload` from the `streaming_source` module.
use base64::Engine; // Import the `Engine` trait from the `base64` crate.
use base64::prelude::BASE64_STANDARD; // Import the standard base64 engine.
use gaxi::http::reqwest::RequestBuilder; // Import `RequestBuilder` from `gaxi::http::reqwest`.
use gaxi::options::{ClientConfig, Credentials}; // Import `ClientConfig` and `Credentials` from `gaxi::options`.
use google_cloud_auth::credentials::{Builder as CredentialsBuilder, CacheableResource}; // Import `Builder` as `CredentialsBuilder` and `CacheableResource` from `google_cloud_auth::credentials`.
use google_cloud_gax::client_builder::{Error as BuilderError, Result as BuilderResult}; // Import `Error` as `BuilderError` and `Result` as `BuilderResult` from `google_cloud_gax::client_builder`.
use http::Extensions; // Import `Extensions` from the `http` crate.
use std::sync::Arc; // Import `Arc` from the standard library.

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
#[derive(Clone, Debug)] // Derive `Clone` and `Debug` for `Storage`.
pub struct Storage<S = crate::stub::DefaultStorage> // Define `Storage` struct with a generic parameter `S` defaulting to `DefaultStorage`.
where
    S: crate::stub::Storage + 'static, // Ensure `S` implements `Storage` and has a 'static lifetime.
{
    stub: std::sync::Arc<S>, // The storage stub, wrapped in an Arc for thread safety.
    options: RequestOptions, // Request options associated with the client.
}

#[derive(Clone, Debug)] // Derive `Clone` and `Debug` for `StorageInner`.
pub(crate) struct StorageInner { // Define a crate-public struct `StorageInner`.
    pub client: gaxi::http::ReqwestClient, // The HTTP client used for requests.
    pub cred: Credentials, // Credentials used for authentication.
    pub options: RequestOptions, // Request options.
    pub grpc: gaxi::grpc::Client, // The gRPC client.
}

impl Storage { // Implement methods for `Storage`.
    /// Returns a builder for [Storage].
    ///
    /// # Example
    /// ```
    /// # use google_cloud_storage::client::Storage;
    /// # async fn sample() -> anyhow::Result<()> {
    /// let client = Storage::builder().build().await?;
    /// # Ok(()) }
    /// ```
    pub fn builder() -> ClientBuilder { // Define a public function `builder` that returns a `ClientBuilder`.
        ClientBuilder::new() // Return a new instance of `ClientBuilder`.
    }
}

impl<S> Storage<S> // Implement methods for `Storage<S>`.
where
    S: crate::storage::stub::Storage + 'static, // Ensure `S` implements `Storage` and has a 'static lifetime.
{
    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub(stub: S) -> Self // Define a public function `from_stub` that takes a stub and returns `Self`.
    where
        S: super::stub::Storage + 'static, // Ensure `S` implements `Storage` and has a 'static lifetime.
    {
        Self { // Construct a new `Storage` instance.
            stub: std::sync::Arc::new(stub), // Wrap the stub in an `Arc`.
            options: RequestOptions::new(), // Create new `RequestOptions`.
        }
    }

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
    pub fn write_object<B, O, T, P>(&self, bucket: B, object: O, payload: T) -> WriteObject<P, S> // Define `write_object` generic over bucket name, object name, payload, and payload type.
    where
        B: Into<String>, // `B` must be convertible into a String.
        O: Into<String>, // `O` must be convertible into a String.
        T: Into<Payload<P>>, // `T` must be convertible into `Payload<P>`.
    {
        WriteObject::new( // Create a new `WriteObject` builder.
            self.stub.clone(), // Clone the stub.
            bucket, // Pass the bucket name.
            object, // Pass the object name.
            payload, // Pass the payload.
            self.options.clone(), // Clone the request options.
        )
    }

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
    pub fn read_object<B, O>(&self, bucket: B, object: O) -> ReadObject<S> // Define `read_object` generic over bucket name and object name.
    where
        B: Into<String>, // `B` must be convertible into a String.
        O: Into<String>, // `O` must be convertible into a String.
    {
        ReadObject::new(self.stub.clone(), bucket, object, self.options.clone()) // Create a new `ReadObject` builder with the stub, bucket, object, and options.
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
    pub fn open_object<B, O>(&self, bucket: B, object: O) -> OpenObject<S> // Define `open_object` generic over bucket name and object name.
    where
        B: Into<String>, // `B` must be convertible into a String.
        O: Into<String>, // `O` must be convertible into a String.
    {
        OpenObject::new( // Create a new `OpenObject` builder.
            bucket.into(), // Convert bucket to String.
            object.into(), // Convert object to String.
            self.stub.clone(), // Clone the stub.
            self.options.clone(), // Clone the request options.
        )
    }
}

impl Storage { // Implement methods for `Storage`.
    pub(crate) async fn new(builder: ClientBuilder) -> BuilderResult<Self> { // Define an async crate-public function `new` that takes a `ClientBuilder`.
        let inner = StorageInner::from_parts(builder).await?; // Create `StorageInner` from the builder parts.
        let options = inner.options.clone(); // Clone the options from inner.
        let stub = crate::storage::transport::Storage::new(Arc::new(inner)); // Create a new storage transport stub wrapping `inner`.
        Ok(Self { stub, options }) // Return the new `Storage` instance.
    }
}

impl StorageInner { // Implement methods for `StorageInner`.
    /// Builds a client assuming `config.cred` and `config.endpoint` are initialized, panics otherwise.
    pub(self) fn new( // Define a private function `new`.
        client: gaxi::http::ReqwestClient, // HTTP client.
        cred: Credentials, // Credentials.
        options: RequestOptions, // Request options.
        grpc: gaxi::grpc::Client, // gRPC client.
    ) -> Self { // Return `Self`.
        Self { // Construct `StorageInner`.
            client, // Set client.
            cred, // Set cred.
            options, // Set options.
            grpc, // Set grpc.
        }
    }

    pub(self) async fn from_parts(builder: ClientBuilder) -> BuilderResult<Self> { // Define an async private function `from_parts`.
        let (mut config, options) = builder.into_parts()?; // Decompose the builder into config and options.
        config.disable_automatic_decompression = true; // Disable automatic decompression in config.
        let cred = config // Get credentials from config.
            .cred // Access `cred` field.
            .clone() // Clone it.
            .expect("into_parts() assigns default credentials"); // Expect it to be present.

        let client = gaxi::http::ReqwestClient::new(config.clone(), super::DEFAULT_HOST).await?; // Create a new ReqwestClient.

        let inner = StorageInner::new( // Create a new `StorageInner`.
            client, // Pass client.
            cred, // Pass cred.
            options, // Pass options.
            gaxi::grpc::Client::new(config, super::DEFAULT_HOST).await?, // Create and pass gRPC client.
        );
        Ok(inner) // Return the inner struct.
    }

    // Helper method to apply authentication headers to the request builder.
    pub async fn apply_auth_headers( // Define an async public function `apply_auth_headers`.
        &self, // Borrow self.
        builder: RequestBuilder, // Take a RequestBuilder.
    ) -> crate::Result<RequestBuilder> { // Return a Result with RequestBuilder.
        let cached_auth_headers = self // Get cached auth headers.
            .cred // Access credentials.
            .headers(Extensions::new()) // Get headers with new extensions.
            .await // Await the future.
            .map_err(Error::authentication)?; // Map error to authentication error.

        let auth_headers = match cached_auth_headers { // Match on cached headers.
            CacheableResource::New { data, .. } => data, // If new, get data.
            CacheableResource::NotModified => { // If not modified.
                unreachable!("headers are not cached"); // Panic as unreachable.
            }
        };

        let builder = builder.headers(auth_headers); // Apply headers to builder.
        Ok(builder) // Return builder.
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
pub struct ClientBuilder { // Define `ClientBuilder` struct.
    // Common options for all clients (generated or not).
    pub(crate) config: ClientConfig, // Crate-public `config` of type `ClientConfig`.
    // Specific options for the storage client. `RequestOptions` also requires
    // these, it makes sense to share them.
    common_options: CommonOptions, // Private `common_options` of type `CommonOptions`.
}

impl ClientBuilder { // Implement `ClientBuilder`.
    pub(crate) fn new() -> Self { // Define crate-public `new`.
        let mut config = ClientConfig::default(); // Create default config.
        config.retry_policy = Some(Arc::new(crate::retry_policy::storage_default())); // Set default retry policy.
        config.backoff_policy = Some(Arc::new(crate::backoff_policy::default())); // Set default backoff policy.
        { // Block for scope.
            let count = std::thread::available_parallelism().ok(); // Get available parallelism.
            config.grpc_subchannel_count = Some(count.map(|x| x.get()).unwrap_or(1)); // Set subchannel count.
        }
        let common_options = CommonOptions::new(); // Create new common options.
        Self { // Return new `ClientBuilder`.
            config, // Set config.
            common_options, // Set common_options.
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
    pub async fn build(self) -> BuilderResult<Storage> { // Define async public `build`.
        Storage::new(self).await // Call `Storage::new` and await.
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
    pub fn with_endpoint<V: Into<String>>(mut self, v: V) -> Self { // Define public `with_endpoint`.
        self.config.endpoint = Some(v.into()); // Set endpoint.
        self // Return self.
    }

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
    pub fn with_credentials<V: Into<Credentials>>(mut self, v: V) -> Self { // Define public `with_credentials`.
        self.config.cred = Some(v.into()); // Set credentials.
        self // Return self.
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
    pub fn with_retry_policy<V: Into<google_cloud_gax::retry_policy::RetryPolicyArg>>( // Define public `with_retry_policy`.
        mut self, // Take mutable self.
        v: V, // Take argument v.
    ) -> Self { // Return Self.
        self.config.retry_policy = Some(v.into().into()); // Set retry policy.
        self // Return self.
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
    pub fn with_backoff_policy<V: Into<google_cloud_gax::backoff_policy::BackoffPolicyArg>>( // Define public `with_backoff_policy`.
        mut self, // Take mutable self.
        v: V, // Take argument v.
    ) -> Self { // Return Self.
        self.config.backoff_policy = Some(v.into().into()); // Set backoff policy.
        self // Return self.
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
    pub fn with_retry_throttler<V: Into<google_cloud_gax::retry_throttler::RetryThrottlerArg>>( // Define public `with_retry_throttler`.
        mut self, // Take mutable self.
        v: V, // Take argument v.
    ) -> Self { // Return Self.
        self.config.retry_throttler = v.into().into(); // Set retry throttler.
        self // Return self.
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
    pub fn with_resumable_upload_threshold<V: Into<usize>>(mut self, v: V) -> Self { // Define public `with_resumable_upload_threshold`.
        self.common_options.resumable_upload_threshold = v.into(); // Set threshold.
        self // Return self.
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
    pub fn with_resumable_upload_buffer_size<V: Into<usize>>(mut self, v: V) -> Self { // Define public `with_resumable_upload_buffer_size`.
        self.common_options.resumable_upload_buffer_size = v.into(); // Set buffer size.
        self // Return self.
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
    pub fn with_read_resume_policy<V>(mut self, v: V) -> Self // Define public `with_read_resume_policy`.
    where
        V: ReadResumePolicy + 'static, // Ensure V implements ReadResumePolicy.
    {
        self.common_options.read_resume_policy = Arc::new(v); // Set read resume policy.
        self // Return self.
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
    pub fn with_grpc_subchannel_count(mut self, v: usize) -> Self { // Define public `with_grpc_subchannel_count`.
        self.config.grpc_subchannel_count = Some(v); // Set subchannel count.
        self // Return self.
    }

    pub(crate) fn apply_default_credentials(&mut self) -> BuilderResult<()> { // Define crate-public `apply_default_credentials`.
        if self.config.cred.is_some() { // Check if cred is present.
            return Ok(()); // Return Ok.
        };
        let default = CredentialsBuilder::default() // Create default credentials builder.
            .build() // Build credentials.
            .map_err(BuilderError::cred)?; // Map error.
        self.config.cred = Some(default); // Set cred.
        Ok(()) // Return Ok.
    }

    pub(crate) fn apply_default_endpoint(&mut self) -> BuilderResult<()> { // Define crate-public `apply_default_endpoint`.
        let _ = self // Use self.
            .config // Access config.
            .endpoint // Access endpoint.
            .get_or_insert_with(|| super::DEFAULT_HOST.to_string()); // Insert default host if empty.
        Ok(()) // Return Ok.
    }

    // Breaks the builder into its parts, with defaults applied.
    pub(crate) fn into_parts( // Define crate-public `into_parts`.
        mut self, // Take mutable self.
    ) -> google_cloud_gax::client_builder::Result<(ClientConfig, RequestOptions)> { // Return Result with config and options.
        self.apply_default_credentials()?; // Apply default credentials.
        self.apply_default_endpoint()?; // Apply default endpoint.
        let request_options = // Create request options.
            RequestOptions::new_with_client_config(&self.config, self.common_options); // From config and common options.
        Ok((self.config, request_options)) // Return tuple.
    }
}

/// The set of characters that are percent encoded.
///
/// This set is defined at https://cloud.google.com/storage/docs/request-endpoints#encoding:
///
/// Encode the following characters when they appear in either the object name
/// or query string of a request URL:
///     !, #, $, &, ', (, ), *, +, ,, /, :, ;, =, ?, @, [, ], and space characters.
pub(crate) const ENCODED_CHARS: percent_encoding::AsciiSet = percent_encoding::CONTROLS // Define crate-public constant `ENCODED_CHARS`.
    .add(b'!') // Add '!'.
    .add(b'#') // Add '#'.
    .add(b'$') // Add '$'.
    .add(b'&') // Add '&'.
    .add(b'\'') // Add '\''.
    .add(b'(') // Add '('.
    .add(b')') // Add ')'.
    .add(b'*') // Add '*'.
    .add(b'+') // Add '+'.
    .add(b',') // Add ','.
    .add(b'/') // Add '/'.
    .add(b':') // Add ':'.
    .add(b';') // Add ';'.
    .add(b'=') // Add '='.
    .add(b'?') // Add '?'.
    .add(b'@') // Add '@'.
    .add(b'[') // Add '['.
    .add(b']') // Add ']'.
    .add(b' '); // Add space.

/// Percent encode a string.
///
/// To ensure compatibility certain characters need to be encoded when they appear
/// in either the object name or query string of a request URL.
pub(crate) fn enc(value: &str) -> String { // Define crate-public function `enc`.
    percent_encoding::utf8_percent_encode(value, &ENCODED_CHARS).to_string() // Percent encode the value and convert to string.
}

pub(crate) fn apply_customer_supplied_encryption_headers( // Define crate-public function `apply_customer_supplied_encryption_headers`.
    builder: RequestBuilder, // Take RequestBuilder.
    common_object_request_params: &Option<crate::model::CommonObjectRequestParams>, // Take optional request params.
) -> RequestBuilder { // Return RequestBuilder.
    common_object_request_params.iter().fold(builder, |b, v| { // Iterate over params and fold builder.
        b.header( // Add header.
            "x-goog-encryption-algorithm", // Header name.
            v.encryption_algorithm.clone(), // Header value.
        )
        .header( // Add header.
            "x-goog-encryption-key", // Header name.
            BASE64_STANDARD.encode(v.encryption_key_bytes.clone()), // Encoded key value.
        )
        .header( // Add header.
            "x-goog-encryption-key-sha256", // Header name.
            BASE64_STANDARD.encode(v.encryption_key_sha256_bytes.clone()), // Encoded key sha256 value.
        )
    })
}

#[cfg(test)] // Conditional compilation for test configuration.
pub(crate) mod tests { // Define crate-public module `tests`.
    use super::*; // Import from parent module.
    use google_cloud_auth::credentials::anonymous::Builder as Anonymous; // Import Anonymous credentials builder.
    use google_cloud_gax::retry_result::RetryResult; // Import RetryResult.
    use google_cloud_gax::retry_state::RetryState; // Import RetryState.
    use std::{sync::Arc, time::Duration}; // Import Arc and Duration.

    #[test] // Define a test function.
    fn default_settings() { // Test default settings.
        let builder = ClientBuilder::new().with_credentials(Anonymous::new().build()); // Create builder with anonymous creds.
        let config = builder.config; // Get config.
        assert!(config.retry_policy.is_some(), "{config:?}"); // Assert retry policy is set.
        assert!(config.backoff_policy.is_some(), "{config:?}"); // Assert backoff policy is set.
        { // Block.
            assert!( // Assert.
                config.grpc_subchannel_count.is_some_and(|v| v >= 1), // Assert subchannel count >= 1.
                "{config:?}" // Error message.
            );
        }
    }

    #[test] // Define a test function.
    fn subchannel_count() { // Test subchannel count.
        let builder = ClientBuilder::new() // Create builder.
            .with_credentials(Anonymous::new().build()) // With anonymous creds.
            .with_grpc_subchannel_count(42); // Set subchannel count to 42.
        let config = builder.config; // Get config.
        assert!( // Assert.
            config.grpc_subchannel_count.is_some_and(|v| v == 42), // Assert subchannel count is 42.
            "{config:?}" // Error message.
        );
    }

    pub(crate) fn test_builder() -> ClientBuilder { // Define helper function `test_builder`.
        ClientBuilder::new() // Create new builder.
            .with_credentials(Anonymous::new().build()) // With anonymous creds.
            .with_endpoint("http://private.googleapis.com") // With custom endpoint.
            .with_backoff_policy( // With backoff policy.
                google_cloud_gax::exponential_backoff::ExponentialBackoffBuilder::new() // Create exponential backoff builder.
                    .with_initial_delay(Duration::from_millis(1)) // Set initial delay.
                    .with_maximum_delay(Duration::from_millis(2)) // Set maximum delay.
                    .build() // Build it.
                    .expect("hard coded policy should build correctly"), // Expect success.
            )
    }

    /// This is used by the request builder tests.
    pub(crate) async fn test_inner_client(builder: ClientBuilder) -> Arc<StorageInner> { // Define async helper `test_inner_client`.
        let inner = StorageInner::from_parts(builder) // Create inner from builder.
            .await // Await.
            .expect("creating an test inner client succeeds"); // Expect success.
        Arc::new(inner) // Return Arc of inner.
    }

    mockall::mock! { // Mock macro.
        #[derive(Debug)] // Derive Debug.
        pub RetryThrottler {} // Define RetryThrottler struct.

        impl google_cloud_gax::retry_throttler::RetryThrottler for RetryThrottler { // Implement RetryThrottler trait.
            fn throttle_retry_attempt(&self) -> bool; // method.
            fn on_retry_failure(&mut self, flow: &RetryResult); // method.
            fn on_success(&mut self); // method.
        }
    }

    mockall::mock! { // Mock macro.
        #[derive(Debug)] // Derive Debug.
        pub RetryPolicy {} // Define RetryPolicy struct.

        impl google_cloud_gax::retry_policy::RetryPolicy for RetryPolicy { // Implement RetryPolicy trait.
            fn on_error(&self, state: &RetryState, error: google_cloud_gax::error::Error) -> RetryResult; // method.
        }
    }

    mockall::mock! { // Mock macro.
        #[derive(Debug)] // Derive Debug.
        pub BackoffPolicy {} // Define BackoffPolicy struct.

        impl google_cloud_gax::backoff_policy::BackoffPolicy for BackoffPolicy { // Implement BackoffPolicy trait.
            fn on_failure(&self, state: &RetryState) -> std::time::Duration; // method.
        }
    }

    mockall::mock! { // Mock macro.
        #[derive(Debug)] // Derive Debug.
        pub ReadResumePolicy {} // Define ReadResumePolicy struct.

        impl crate::read_resume_policy::ReadResumePolicy for ReadResumePolicy { // Implement ReadResumePolicy trait.
            fn on_error(&self, query: &crate::read_resume_policy::ResumeQuery, error: google_cloud_gax::error::Error) -> crate::read_resume_policy::ResumeResult; // method.
        }
    }
}
