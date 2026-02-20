// [Jules: Rust] Standard copyright header.
// [Jules: Rust] Apache 2.0 License.
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

// [Jules: SDK] Region tag.
// [START storage_control_managed_folder_list]
// [Jules: SDK] `ItemPaginator` provides a convenient iterator-like interface for paginated APIs.
use google_cloud_gax::paginator::ItemPaginator;
use google_cloud_storage::client::StorageControl;

// [Jules: SDK] Lists all Managed Folders in a bucket.
pub async fn sample(client: &StorageControl, bucket_id: &str) -> anyhow::Result<()> {
    // [Jules: SDK] Initiate the list request.
    let mut items = client
        .list_managed_folders()
        .set_parent(format!("projects/_/buckets/{bucket_id}"))
        // [Jules: SDK] `by_item()` returns a stream that yields individual `ManagedFolder` items.
        // [Jules: SDK] It automatically handles pagination, fetching subsequent pages as needed.
        .by_item();
    println!("Listing managed folders in bucket {bucket_id}");
    // [Jules: Rust] Iterate over the async stream.
    // [Jules: Rust] `items.next()` returns a `Future`. `transpose()?` handles potential errors during iteration (e.g., network errors fetching the next page).
    while let Some(folder) = items.next().await.transpose()? {
        println!("{folder:?}");
    }
    println!("DONE");
    Ok(())
}
// [END storage_control_managed_folder_list]
