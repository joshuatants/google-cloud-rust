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
// [START storage_control_managed_folder_create]
// [Jules: SDK] `StorageControl` is the client for control plane operations.
// [Jules: SDK] `ManagedFolder` is a feature for organizing objects and applying IAM policies at a folder level.
use google_cloud_storage::{client::StorageControl, model::ManagedFolder};

// [Jules: SDK] Creates a new Managed Folder in a bucket.
pub async fn sample(client: &StorageControl, bucket_id: &str) -> anyhow::Result<()> {
    const ID: &str = "example001";
    let folder = client
        .create_managed_folder()
        // [Jules: SDK] Set the parent bucket. Format is `projects/_/buckets/{bucket_id}`.
        .set_parent(format!("projects/_/buckets/{bucket_id}"))
        // [Jules: SDK] Set the ID for the new managed folder.
        .set_managed_folder_id(ID)
        // [Jules: SDK] Provide the folder metadata. `ManagedFolder::new()` creates a default empty configuration.
        .set_managed_folder(ManagedFolder::new())
        .send()
        .await?;
    println!("folder successfully created {folder:?}");
    Ok(())
}
// [END storage_control_managed_folder_create]
