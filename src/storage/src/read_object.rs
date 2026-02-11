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

//! Defines the return interface for [Storage::read_object][crate::client::Storage::read_object]

// Import Result from the crate root.
use crate::Result;
// Import ObjectHighlights from model_ext.
use crate::model_ext::ObjectHighlights;
// Import Payload and StreamingSource from streaming_source.
use crate::streaming_source::{Payload, StreamingSource};
// Conditionally import Stream from futures if unstable-stream feature is enabled.
#[cfg(feature = "unstable-stream")]
use futures::Stream;

/// The result of a `ReadObject` request.
///
/// Objects can be large, and must be returned as a stream of bytes. This struct
/// also provides an accessor to retrieve the object's metadata.
// Define a public struct named ReadObjectResponse. derive Debug.
#[derive(Debug)]
pub struct ReadObjectResponse {
    // Inner field storing the implementation of dynamic::ReadObjectResponse boxed.
    inner: Box<dyn dynamic::ReadObjectResponse + Send>,
}

// Implement methods for ReadObjectResponse.
impl ReadObjectResponse {
    // Define a crate-private constructor new.
    pub(crate) fn new<T>(inner: Box<T>) -> Self
    where
        // Constraint: T must implement dynamic::ReadObjectResponse, Send, and 'static.
        T: dynamic::ReadObjectResponse + Send + 'static,
    {
        // Return a new ReadObjectResponse.
        Self { inner }
    }

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
    // Define a public method from_source to create a response from a source.
    pub fn from_source<T, S>(object: ObjectHighlights, source: T) -> Self
    where
        // Constraint: T can be converted into Payload<S>, is Send, Sync, and 'static.
        T: Into<Payload<S>> + Send + Sync + 'static,
        // Constraint: S implements StreamingSource, is Send, Sync, and 'static.
        S: StreamingSource + Send + Sync + 'static,
    {
        // Return a new ReadObjectResponse wrapping a FakeReadObjectResponse.
        Self {
            inner: Box::new(FakeReadObjectResponse::<S> {
                object,
                source: source.into(),
            }),
        }
    }

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
    // Define a public method object that returns the object metadata.
    pub fn object(&self) -> ObjectHighlights {
        // Delegate to the inner implementation.
        self.inner.object()
    }

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
    // Define a public async method next to get the next chunk of data.
    pub async fn next(&mut self) -> Option<Result<bytes::Bytes>> {
        // Delegate to the inner implementation.
        self.inner.next().await
    }

    // Conditionally compile into_stream if unstable-stream feature is enabled.
    #[cfg(feature = "unstable-stream")]
    // Document that this item depends on the feature.
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable-stream")))]
    /// Convert the response to a [Stream].
    // Define a public method into_stream returning a Stream.
    pub fn into_stream(self) -> impl Stream<Item = Result<bytes::Bytes>> + Unpin {
        // Import unfold from futures::stream.
        use futures::stream::unfold;
        // Create a pinned stream using unfold.
        Box::pin(unfold(Some(self), move |state| async move {
            // Check if state has a value (the response object).
            if let Some(mut this) = state {
                // Call next() on the response.
                if let Some(chunk) = this.next().await {
                    // If chunk exists, yield it and keep state.
                    return Some((chunk, Some(this)));
                }
            };
            // If exhausted, return None to end stream.
            None
        }))
    }
}

// Define a crate-private module dynamic.
pub(crate) mod dynamic {
    // Import Result.
    use crate::Result;
    // Import ObjectHighlights.
    use crate::model_ext::ObjectHighlights;

    /// A trait representing the interface to read an object
    // Use async_trait to allow async methods in traits.
    #[async_trait::async_trait]
    // Define a public trait ReadObjectResponse extending Debug.
    pub trait ReadObjectResponse: std::fmt::Debug {
        // Method to get object metadata.
        fn object(&self) -> ObjectHighlights;
        // Method to get next chunk.
        async fn next(&mut self) -> Option<Result<bytes::Bytes>>;
    }
}

// Define a struct FakeReadObjectResponse for testing/mocking.
struct FakeReadObjectResponse<T>
where
    // Constraint: T must implement StreamingSource, Send, Sync, and 'static.
    T: StreamingSource + Send + Sync + 'static,
{
    // Field for object metadata.
    object: ObjectHighlights,
    // Field for data source.
    source: Payload<T>,
}

// Implement Debug for FakeReadObjectResponse.
impl<T> std::fmt::Debug for FakeReadObjectResponse<T>
where
    T: StreamingSource + Send + Sync + 'static,
{
    // Define fmt method.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Format struct, skipping source.
        f.debug_struct("FakeReadObjectResponse")
            .field("object", &self.object)
            // skip source, as it is not `Debug`
            .finish()
    }
}

// Implement dynamic::ReadObjectResponse for FakeReadObjectResponse.
#[async_trait::async_trait]
impl<T> dynamic::ReadObjectResponse for FakeReadObjectResponse<T>
where
    T: StreamingSource + Send + Sync + 'static,
{
    // Implement object() method.
    fn object(&self) -> ObjectHighlights {
        // Return a clone of the object metadata.
        self.object.clone()
    }

    // Implement next() method.
    async fn next(&mut self) -> Option<Result<bytes::Bytes>> {
        // Call next() on the source.
        self.source
            .next()
            .await
            // Map the error to crate::Error::io.
            .map(|r| r.map_err(crate::Error::io))
    }
}

// Conditionally compile the tests module only when running tests.
#[cfg(test)]
mod tests {
    // Import everything from the parent module.
    use super::*;

    // Test from_source functionality.
    #[tokio::test]
    async fn from_source() -> anyhow::Result<()> {
        // Define test content.
        const LAZY: &str = "the quick brown fox jumps over the lazy dog";
        // Create object metadata.
        let object = ObjectHighlights {
            etag: "custom-etag".to_string(),
            ..Default::default()
        };

        // Create response from source.
        let mut response = ReadObjectResponse::from_source(object.clone(), LAZY);
        // Assert object metadata matches.
        assert_eq!(&object, &response.object());
        // Read contents.
        let mut contents = Vec::new();
        while let Some(chunk) = response.next().await.transpose()? {
            contents.extend_from_slice(&chunk);
        }
        // Create Bytes from contents.
        let contents = bytes::Bytes::from_owner(contents);
        // Assert contents match.
        assert_eq!(contents, LAZY);
        // Return Ok.
        Ok(())
    }
}
