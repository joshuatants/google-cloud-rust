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

//! Extends [builder][crate::builder] with types that improve type safety and/or
//! ergonomics.

/// An extension trait for `RewriteObject` to provide a convenient way
/// to poll a rewrite operation until it is complete.
// Apply the async_trait attribute macro to allow async methods in traits.
#[async_trait::async_trait]
pub trait RewriteObjectExt {
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
    // Define an async method named rewrite_until_done that returns a Result containing an Object.
    async fn rewrite_until_done(self) -> crate::Result<crate::model::Object>;
}

// Implement the RewriteObjectExt trait for the RewriteObject builder.
#[async_trait::async_trait]
impl RewriteObjectExt for crate::builder::storage_control::RewriteObject {
    // Implement the rewrite_until_done method.
    async fn rewrite_until_done(mut self) -> crate::Result<crate::model::Object> {
        // Start an infinite loop to poll the operation.
        loop {
            // Send the rewrite request and await the response. Clone self because send consumes it.
            let resp = self.clone().send().await?;
            // Check if the rewrite operation is done.
            if resp.done {
                // If done, return the resource (the rewritten object), unwraping the Option because it must be present when done.
                return Ok(resp
                    .resource
                    .expect("an object is always returned when the rewrite operation is done"));
            }
            // If not done, update the rewrite token in the builder for the next iteration.
            self = self.set_rewrite_token(resp.rewrite_token);
        }
    }
}

// Conditionally compile the tests module only when running tests.
#[cfg(test)]
mod tests {
    // Import everything from the parent module.
    use super::*;
    // Import StorageControl client.
    use crate::client::StorageControl;
    // Import relevant model types.
    use crate::model::{Object, RewriteObjectRequest, RewriteResponse};
    // Import gRPC error types.
    use google_cloud_gax::error::rpc::{Code, Status};
    // Import RequestOptions.
    use google_cloud_gax::options::RequestOptions;
    // Import Response wrapper.
    use google_cloud_gax::response::Response;

    // Use mockall to create a mock implementation of StorageControl.
    mockall::mock! {
        // Define the mock struct name and derive Debug.
        #[derive(Debug)]
        StorageControl {}
        // Implement the crate::stub::StorageControl trait for the mock.
        impl crate::stub::StorageControl for StorageControl {
            // Mock the rewrite_object method.
            async fn rewrite_object( &self, _req: RewriteObjectRequest, _options: RequestOptions) -> crate::Result<Response<RewriteResponse>>;
        }
    }

    // Define an async test function for rewrite_until_done success case.
    #[tokio::test]
    async fn test_rewrite_until_done() -> anyhow::Result<()> {
        // Create a new instance of the mock storage control.
        let mut mock = MockStorageControl::new();
        // Create a final object to be returned.
        let final_object = Object::new().set_name("final-object");

        // Create a sequence to enforce call order.
        let mut seq = mockall::Sequence::new();
        // Expect the first call to rewrite_object.
        mock.expect_rewrite_object()
            // Verify that the rewrite token is empty in the first request.
            .withf(|req: &RewriteObjectRequest, _| req.rewrite_token.is_empty())
            // It should be called exactly once.
            .times(1)
            // Enforce it happens in sequence.
            .in_sequence(&mut seq)
            // Return a response indicating the operation is not done, with a rewrite token.
            .returning(|_, _| {
                Ok(Response::from(
                    RewriteResponse::new()
                        .set_done(false)
                        .set_rewrite_token("token1"),
                ))
            });

        // Expect the second call to rewrite_object.
        mock.expect_rewrite_object()
            // Verify that the rewrite token matches the one from the previous response.
            .withf(|req: &RewriteObjectRequest, _| req.rewrite_token == "token1")
            // It should be called exactly once.
            .times(1)
            // Enforce it happens in sequence.
            .in_sequence(&mut seq)
            // Return a response indicating the operation is done, with the final object.
            .returning({
                let obj = final_object.clone();
                move |_, _| {
                    Ok(Response::from(
                        RewriteResponse::new()
                            .set_done(true)
                            .set_resource(obj.clone()),
                    ))
                }
            });

        // Create a StorageControl client from the stub.
        let client = StorageControl::from_stub(mock);
        // Call rewrite_until_done on the rewrite object builder.
        let result = client.rewrite_object().rewrite_until_done().await?;

        // Assert that the result matches the expected final object.
        assert_eq!(result, final_object);
        // Return Ok result.
        Ok(())
    }

    // Define an async test function for rewrite_until_done error case.
    #[tokio::test]
    async fn test_rewrite_until_done_error() -> anyhow::Result<()> {
        // Create a new instance of the mock storage control.
        let mut mock = MockStorageControl::new();
        // Expect a call to rewrite_object.
        mock.expect_rewrite_object()
            // Verify that the rewrite token is empty.
            .withf(|req: &RewriteObjectRequest, _| req.rewrite_token.is_empty())
            // It should be called exactly once.
            .times(1)
            // Return an error (Unavailable).
            .returning(|_, _| {
                Err(crate::Error::service(
                    Status::default().set_code(Code::Unavailable),
                ))
            });

        // Create a StorageControl client from the stub.
        let client = StorageControl::from_stub(mock);
        // Call rewrite_until_done and expect an error.
        let _ = client
            .rewrite_object()
            .rewrite_until_done()
            .await
            .unwrap_err();

        // Return Ok result.
        Ok(())
    }
}
