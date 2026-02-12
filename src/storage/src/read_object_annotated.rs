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

// [Jules: Rust]
// This is a documentation comment for the module (`//!`), explaining its purpose.
//! Defines the return interface for [Storage::read_object][crate::client::Storage::read_object]

use crate::Result;
use crate::model_ext::ObjectHighlights;
use crate::streaming_source::{Payload, StreamingSource};
// [Jules: Rust]
// `#[cfg(feature = "unstable-stream")]` allows conditional compilation.
// The code following this attribute is only compiled if the "unstable-stream" feature is enabled in `Cargo.toml`.
// This is useful for hiding experimental or optional functionality.
#[cfg(feature = "unstable-stream")]
use futures::Stream;

// [Jules: SDK]
// When you read an object, you don't just get the bytes. You get a stream of bytes.
// This struct wraps that stream and provides access to metadata like content type and size.
/// The result of a `ReadObject` request.
///
/// Objects can be large, and must be returned as a stream of bytes. This struct
/// also provides an accessor to retrieve the object's metadata.
// [Jules: Rust]
// `Box<dyn dynamic::ReadObjectResponse + Send>` is a boxed trait object.
// - `Box`: allocates on the heap.
// - `dyn ...`: dynamic dispatch. We don't know the exact type at compile time.
// - `+ Send`: We require the implementation to be safe to send across threads.
#[derive(Debug)]
pub struct ReadObjectResponse {
    inner: Box<dyn dynamic::ReadObjectResponse + Send>,
}

impl ReadObjectResponse {
    // [Jules: Rust]
    // A crate-private constructor.
    // It takes any type `T` that implements the internal trait and wraps it.
    pub(crate) fn new<T>(inner: Box<T>) -> Self
    where
        T: dynamic::ReadObjectResponse + Send + 'static,
    {
        Self { inner }
    }

    // [Jules: SDK]
    // This helper allows you to create a fake response for testing your application code.
    // You can pass in metadata and a data source (like a string or byte slice).
    /// Create a ReadObjectResponse, given a data source.
    ///
    /// Use this method to mock the return type of
    /// [Storage::read_object][crate::client::Storage::read_object].
    ///
    /// # Example
    /// ```
    /// # use google_cloud_storage::model_ext::ObjectHighlights;
    /// # use google_cloud_storage::read_object::ReadObjectResponse;
    /// let object = ObjectHighlights::default();
    /// let response = ReadObjectResponse::from_source(object, "payload");
    /// ```
    pub fn from_source<T, S>(object: ObjectHighlights, source: T) -> Self
    where
        T: Into<Payload<S>> + Send + Sync + 'static,
        S: StreamingSource + Send + Sync + 'static,
    {
        // [Jules: Rust]
        // We create a `FakeReadObjectResponse` (defined below) and box it.
        // `source.into()` converts the input into a `Payload` which handles the streaming logic.
        Self {
            inner: Box::new(FakeReadObjectResponse::<S> {
                object,
                source: source.into(),
            }),
        }
    }

    // [Jules: SDK]
    // Get the metadata associated with the object response.
    // This includes things like `generation`, `size`, `content_encoding`.
    /// Get the highlights of the object metadata included in the
    /// response.
    ///
    /// To get full metadata about this object, use [crate::client::StorageControl::get_object].
    ///
    /// # Example
    /// ```
    /// # use google_cloud_storage::client::Storage;
    /// # async fn sample(client: &Storage) -> anyhow::Result<()> {
    /// let object = client
    ///     .read_object("projects/_/buckets/my-bucket", "my-object")
    ///     .send()
    ///     .await?
    ///     .object();
    /// println!("object generation={}", object.generation);
    /// println!("object metageneration={}", object.metageneration);
    /// println!("object size={}", object.size);
    /// println!("object content encoding={}", object.content_encoding);
    /// # Ok(()) }
    /// ```
    pub fn object(&self) -> ObjectHighlights {
        self.inner.object()
    }

    // [Jules: SDK]
    // Fetches the next chunk of data from the stream.
    // Returns `Some(Ok(bytes))` if data is available.
    // Returns `None` if the stream is finished.
    // Returns `Some(Err(...))` if an error occurred (e.g., network disconnect).
    /// Stream the next bytes of the object.
    ///
    /// When the response has been exhausted, this will return None.
    ///
    /// # Example
    /// ```
    /// # use google_cloud_storage::client::Storage;
    /// # async fn sample(client: &Storage) -> anyhow::Result<()> {
    /// let mut resp = client
    ///     .read_object("projects/_/buckets/my-bucket", "my-object")
    ///     .send()
    ///     .await?;
    /// while let Some(next) = resp.next().await {
    ///     println!("next={:?}", next?);
    /// }
    /// # Ok(()) }
    /// ```
    pub async fn next(&mut self) -> Option<Result<bytes::Bytes>> {
        self.inner.next().await
    }

    // [Jules: Rust]
    // Only available when "unstable-stream" feature is enabled.
    // This converts the custom response into a standard `futures::Stream`.
    // This allows using stream combinators like `map`, `filter`, `fold`, etc.
    #[cfg(feature = "unstable-stream")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable-stream")))]
    /// Convert the response to a [Stream].
    pub fn into_stream(self) -> impl Stream<Item = Result<bytes::Bytes>> + Unpin {
        use futures::stream::unfold;
        // [Jules: Rust]
        // `unfold` creates a stream from a seed state (in this case `self`) and a closure.
        // The closure produces the next item and the next state.
        Box::pin(unfold(Some(self), move |state| async move {
            if let Some(mut this) = state {
                if let Some(chunk) = this.next().await {
                    return Some((chunk, Some(this)));
                }
            };
            None
        }))
    }
}

// [Jules: Rust]
// Internal module defining the trait used for dynamic dispatch.
pub(crate) mod dynamic {
    use crate::Result;
    use crate::model_ext::ObjectHighlights;

    /// A trait representing the interface to read an object
    // [Jules: Rust]
    // The `async_trait` macro is needed because traits cannot have async methods by default.
    #[async_trait::async_trait]
    pub trait ReadObjectResponse: std::fmt::Debug {
        fn object(&self) -> ObjectHighlights;
        async fn next(&mut self) -> Option<Result<bytes::Bytes>>;
    }
}

// [Jules: Rust]
// A concrete implementation of `ReadObjectResponse` used for testing (`from_source`).
struct FakeReadObjectResponse<T>
where
    T: StreamingSource + Send + Sync + 'static,
{
    object: ObjectHighlights,
    source: Payload<T>,
}

// [Jules: Rust]
// Manual implementation of `Debug` because `Payload<T>` might not implement `Debug`.
impl<T> std::fmt::Debug for FakeReadObjectResponse<T>
where
    T: StreamingSource + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FakeReadObjectResponse")
            .field("object", &self.object)
            // skip source, as it is not `Debug`
            .finish()
    }
}

// [Jules: Rust]
// Implementing the internal trait for our fake response.
#[async_trait::async_trait]
impl<T> dynamic::ReadObjectResponse for FakeReadObjectResponse<T>
where
    T: StreamingSource + Send + Sync + 'static,
{
    fn object(&self) -> ObjectHighlights {
        self.object.clone()
    }

    async fn next(&mut self) -> Option<Result<bytes::Bytes>> {
        self.source
            .next()
            .await
            .map(|r| r.map_err(crate::Error::io))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn from_source() -> anyhow::Result<()> {
        const LAZY: &str = "the quick brown fox jumps over the lazy dog";
        let object = ObjectHighlights {
            etag: "custom-etag".to_string(),
            ..Default::default()
        };

        let mut response = ReadObjectResponse::from_source(object.clone(), LAZY);
        assert_eq!(&object, &response.object());
        let mut contents = Vec::new();
        while let Some(chunk) = response.next().await.transpose()? {
            contents.extend_from_slice(&chunk);
        }
        let contents = bytes::Bytes::from_owner(contents);
        assert_eq!(contents, LAZY);
        Ok(())
    }
}
