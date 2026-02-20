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
// [START storage_control_quickstart_sample]
// [Jules: SDK] `StorageControl` provides access to control plane operations, such as managing storage layouts for hierarchical namespaces.
use google_cloud_storage::client::StorageControl;

// [Jules: SDK] A quickstart sample demonstrating how to retrieve the storage layout of a bucket.
// [Jules: SDK] Storage Layout describes the configuration for Hierarchical Namespaces (HNS).
pub async fn sample(client: &StorageControl, bucket_id: &str) -> anyhow::Result<()> {
    let layout = client
        .get_storage_layout()
        // [Jules: SDK] Set the resource name. The format is `projects/_/buckets/{bucket_id}/storageLayout`.
        .set_name(format!("projects/_/buckets/{bucket_id}/storageLayout"))
        .send()
        .await?;
    println!("successfully retrieved storage layout: {layout:?}");
    Ok(())
}
// [END storage_control_quickstart_sample]
