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
// [START storage_disable_bucket_lifecycle_management]
use google_cloud_storage::client::StorageControl;
use google_cloud_storage::model::bucket::Lifecycle;
use google_cloud_wkt::FieldMask;

// [Jules: SDK] Disables lifecycle management for a bucket.
// [Jules: SDK] This effectively removes all lifecycle rules (like auto-deletion or class transitions).
pub async fn sample(client: &StorageControl, bucket_id: &str) -> anyhow::Result<()> {
    // [Jules: SDK] Fetch the current bucket metadata to get the metageneration.
    let bucket = client
        .get_bucket()
        .set_name(format!("projects/_/buckets/{bucket_id}"))
        .send()
        .await?;
    let metageneration = bucket.metageneration;
    // [Jules: SDK] Update the bucket.
    let bucket = client
        .update_bucket()
        // [Jules: SDK] Set the lifecycle configuration to an empty `Lifecycle` object.
        // [Jules: SDK] This clears any existing rules.
        .set_bucket(bucket.set_lifecycle(Lifecycle::new()))
        // [Jules: SDK] Use metageneration for optimistic concurrency control.
        .set_if_metageneration_match(metageneration)
        // [Jules: SDK] Specify that we only want to update the "lifecycle" field.
        .set_update_mask(FieldMask::default().set_paths(["lifecycle"]))
        .send()
        .await?;
    println!("Lifecycle management disabled for bucket {bucket_id}: {bucket:?}");
    Ok(())
}
// [END storage_disable_bucket_lifecycle_management]
