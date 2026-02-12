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

// [START storage_list_files]
use google_cloud_gax::paginator::ItemPaginator;
use google_cloud_storage::client::StorageControl;

pub async fn sample(client: &StorageControl, bucket_id: &str) -> anyhow::Result<()> {
    // [Jules: SDK]
    // `list_objects` creates a builder for listing objects.
    // The `set_parent` specifies which bucket to list.
    // `by_item()` turns the paginated response (pages of objects) into a stream of individual objects.
    // The SDK handles fetching new pages automatically as you iterate.
    let mut objects = client
        .list_objects()
        .set_parent(format!("projects/_/buckets/{bucket_id}"))
        .by_item();

    println!("listing objects in bucket {bucket_id}");

    // [Jules: Rust]
    // Iterate through the stream of objects.
    // `objects.next()` returns a future resolving to `Option<Result<Object>>`.
    // `transpose()?` handles the Result, bubbling up errors.
    while let Some(object) = objects.next().await.transpose()? {
        // [Jules: Rust]
        // Print the object metadata using its Debug implementation.
        println!("{object:?}");
    }
    println!("DONE");
    Ok(())
}
// [END storage_list_files]
