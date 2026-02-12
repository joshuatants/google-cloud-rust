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

// [START storage_quickstart]
use google_cloud_gax::options::RequestOptionsBuilder;
use google_cloud_storage::{client::StorageControl, model::Bucket};

// [Jules: Rust]
// This is an async function that serves as the entry point for the quickstart example.
// It returns `anyhow::Result<()>`, which is a common way to handle errors in Rust applications.
// `anyhow` simplifies error handling by allowing you to propagate any error type easily.
pub async fn sample(
    client: &StorageControl,
    project_id: &str,
    bucket_id: &str,
) -> anyhow::Result<()> {
    // [Jules: SDK]
    // We use the `client` to start a `create_bucket` request.
    // The request is built using a builder pattern.
    let bucket = client
        .create_bucket()
        // [Jules: SDK]
        // The parent for creating a bucket is typically the special string "projects/_".
        // This means "the project associated with the credentials".
        .set_parent("projects/_")
        // [Jules: SDK]
        // The unique ID for the new bucket. Bucket names are global across all of Google Cloud.
        .set_bucket_id(bucket_id)
        // [Jules: SDK]
        // We configure the bucket properties using the `Bucket` struct.
        .set_bucket(
            Bucket::new()
                // [Jules: SDK]
                // We must explicitly set the project that will own (and pay for) the bucket.
                .set_project(format!("projects/{project_id}"))
                .set_storage_class("STANDARD")
                .set_location("US"),
        )
        // [Jules: SDK]
        // This is an idempotent request: it can only succeed once.
        // Telling the SDK this is idempotent allows it to safely retry if the network fails.
        .with_idempotency(true)
        // [Jules: Rust]
        // `send()` executes the request asynchronously.
        // `await?` waits for the result and propagates any error.
        .send()
        .await?;

    // [Jules: Rust]
    // Print the result. `{bucket:?}` uses the `Debug` formatter to print the struct details.
    println!("successfully created bucket {bucket:?}");
    Ok(())
}
// [END storage_quickstart]
