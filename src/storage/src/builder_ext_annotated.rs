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
//! Extends [builder][crate::builder] with types that improve type safety and/or
//! ergonomics.

// [Jules: Rust]
// `///` is a documentation comment for the trait that follows.
//
// [Jules: SDK]
// The `RewriteObject` operation in Google Cloud Storage might not finish in a single request if the object is large.
// It returns a "rewrite token" if there's more work to do.
// This extension trait adds a convenience method to handle that loop for you.
/// An extension trait for `RewriteObject` to provide a convenient way
/// to poll a rewrite operation until it is complete.
// [Jules: Rust]
// Traits in Rust cannot usually have `async` functions directly (at least not without recent language features or boxing).
// The `#[async_trait::async_trait]` attribute is a macro from the `async_trait` crate that transforms the async function
// into one that returns a `Box<Future...>`, allowing it to be used in a trait.
#[async_trait::async_trait]
pub trait RewriteObjectExt {
    // [Jules: Rust]
    // This is the function signature defined by the trait.
    // It takes `self` by value, meaning it consumes the builder.
    // It returns a `Result` containing the final `Object` or an error.
    /// Sends the request and polls the operation until it is complete.
    ///
    /// This helper function simplifies the process of handling a
    /// [StorageControl::rewrite_object][crate::client::StorageControl::rewrite_object]
    /// operation, which may require multiple requests to complete. It automatically
    /// handles the logic of sending the
    /// [rewrite_token][crate::generated::gapic::model::RewriteObjectRequest::rewrite_token]
    /// from one response in the next request.
    ///
    /// For more details on this loop, see the "Rewriting objects" section of the
    /// user guide:
    /// <https://googleapis.github.io/google-cloud-rust/storage/rewrite_object.html>
    ///
    /// # Example
    ///
    /// ```
    /// # use google_cloud_storage::client::StorageControl;
    /// # use google_cloud_storage::builder_ext::RewriteObjectExt;
    /// # async fn sample(client: &StorageControl) -> anyhow::Result<()> {
    /// const SOURCE_NAME: &str = "object-to-copy";
    /// const DEST_NAME: &str = "copied-object";
    /// let source_bucket_id = "source-bucket";
    /// let dest_bucket_id = "dest-bucket";
    /// let copied = client
    ///     .rewrite_object()
    ///     .set_source_bucket(format!("projects/_/buckets/{source_bucket_id}"))
    ///     .set_source_object(SOURCE_NAME)
    ///     .set_destination_bucket(format!("projects/_/buckets/{dest_bucket_id}"))
    ///     .set_destination_name(DEST_NAME)
    ///     .rewrite_until_done()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn rewrite_until_done(self) -> crate::Result<crate::model::Object>;
}

// [Jules: Rust]
// Here we implement the extension trait for the specific type `crate::builder::storage_control::RewriteObject`.
// This is how we "attach" the new method to the existing type.
// We must also apply `#[async_trait::async_trait]` to the implementation block.
#[async_trait::async_trait]
impl RewriteObjectExt for crate::builder::storage_control::RewriteObject {
    async fn rewrite_until_done(mut self) -> crate::Result<crate::model::Object> {
        // [Jules: Rust]
        // `loop` creates an infinite loop. We must break or return out of it.
        loop {
            // [Jules: SDK]
            // We send the current rewrite request.
            // Note `self.clone()`: we need to keep `self` (the request builder) valid for the next iteration if this one isn't done.
            // `send()` is an async method, so we `await` it. `?` propagates any errors.
            let resp = self.clone().send().await?;
            if resp.done {
                // [Jules: Rust]
                // `expect` extracts the value from an `Option`. If it's `None`, it panics with the message.
                // The SDK guarantees `resource` is present when `done` is true.
                return Ok(resp
                    .resource
                    .expect("an object is always returned when the rewrite operation is done"));
            }
            // [Jules: SDK]
            // If not done, we update the request builder with the `rewrite_token` received from the server.
            // This token tells the server where to resume the copy operation.
            self = self.set_rewrite_token(resp.rewrite_token);
        }
    }
}

// [Jules: Rust]
// Unit tests are placed in a `tests` module, conditional on `cfg(test)`.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::StorageControl;
    use crate::model::{Object, RewriteObjectRequest, RewriteResponse};
    use google_cloud_gax::error::rpc::{Code, Status};
    use google_cloud_gax::options::RequestOptions;
    use google_cloud_gax::response::Response;

    // [Jules: Rust]
    // This looks like usage of the `mockall` crate.
    // `mock!` creates a mock struct that can be used to simulate behavior for testing.
    // Here we are mocking `StorageControl` to simulate network responses without hitting real Google Cloud.
    mockall::mock! {
        // [Jules: Rust]
        // We derive `Debug` so the mock can be printed.
        #[derive(Debug)]
        StorageControl {}
        // [Jules: Rust]
        // We implement the trait `crate::stub::StorageControl` for our mock.
        // This allows us to define expectations for the `rewrite_object` method.
        impl crate::stub::StorageControl for StorageControl {
            async fn rewrite_object( &self, _req: RewriteObjectRequest, _options: RequestOptions) -> crate::Result<Response<RewriteResponse>>;
        }
    }

    // [Jules: Rust]
    // `#[tokio::test]` marks this function as an async test that should be run by the Tokio runtime.
    #[tokio::test]
    async fn test_rewrite_until_done() -> anyhow::Result<()> {
        // [Jules: Rust]
        // Create a new instance of the mock.
        let mut mock = MockStorageControl::new();
        let final_object = Object::new().set_name("final-object");

        // [Jules: Rust]
        // `mockall::Sequence` ensures that expectations are met in a specific order.
        let mut seq = mockall::Sequence::new();

        // [Jules: Rust]
        // First expectation: `rewrite_object` is called with an empty token.
        mock.expect_rewrite_object()
            .withf(|req: &RewriteObjectRequest, _| req.rewrite_token.is_empty())
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_, _| {
                // [Jules: Rust]
                // Return a successful response saying "not done yet" and providing "token1".
                Ok(Response::from(
                    RewriteResponse::new()
                        .set_done(false)
                        .set_rewrite_token("token1"),
                ))
            });

        // [Jules: Rust]
        // Second expectation: `rewrite_object` is called with "token1".
        mock.expect_rewrite_object()
            .withf(|req: &RewriteObjectRequest, _| req.rewrite_token == "token1")
            .times(1)
            .in_sequence(&mut seq)
            .returning({
                // [Jules: Rust]
                // We clone `final_object` to move it into the closure.
                let obj = final_object.clone();
                move |_, _| {
                    // [Jules: Rust]
                    // Return a successful response saying "done" and providing the final object.
                    Ok(Response::from(
                        RewriteResponse::new()
                            .set_done(true)
                            .set_resource(obj.clone()),
                    ))
                }
            });

        // [Jules: Rust]
        // Inject the mock into the client.
        let client = StorageControl::from_stub(mock);

        // [Jules: SDK]
        // Call the method we are testing. It should run the loop twice based on our mock setup.
        let result = client.rewrite_object().rewrite_until_done().await?;

        // [Jules: Rust]
        // Verify the result matches what we expected.
        assert_eq!(result, final_object);
        Ok(())
    }

    #[tokio::test]
    async fn test_rewrite_until_done_error() -> anyhow::Result<()> {
        let mut mock = MockStorageControl::new();
        // [Jules: Rust]
        // Expect a call that returns an error.
        mock.expect_rewrite_object()
            .withf(|req: &RewriteObjectRequest, _| req.rewrite_token.is_empty())
            .times(1)
            .returning(|_, _| {
                // [Jules: SDK]
                // Return a service error (e.g., Unavailable).
                Err(crate::Error::service(
                    Status::default().set_code(Code::Unavailable),
                ))
            });

        let client = StorageControl::from_stub(mock);

        // [Jules: Rust]
        // We expect an error here, so we call `.unwrap_err()` on the result.
        // If it was `Ok`, this would panic.
        let _ = client
            .rewrite_object()
            .rewrite_until_done()
            .await
            .unwrap_err();

        Ok(())
    }
}
