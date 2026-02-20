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

// [Jules: SDK] Start region tag.
// [START storage_add_bucket_label]
// [Jules: SDK] `StorageControl` is the client for interacting with GCS.
use google_cloud_storage::client::StorageControl;
// [Jules: SDK] `FieldMask` is used to specify which fields to update in a PATCH request.
use google_cloud_wkt::FieldMask;

// [Jules: SDK] Adds a label to an existing bucket.
pub async fn sample(
    client: &StorageControl,
    bucket_id: &str,
    label_key: &str,
    label_value: &str,
) -> anyhow::Result<()> {
    // [Jules: SDK] First, we retrieve the current bucket metadata.
    // [Jules: SDK] This is necessary to get the current `metageneration` for optimistic concurrency control,
    // [Jules: SDK] and to get the existing labels so we can append to them.
    let bucket = client
        .get_bucket()
        .set_name(format!("projects/_/buckets/{bucket_id}"))
        .send()
        .await?;
    // [Jules: SDK] Capture the metageneration.
    let metageneration = bucket.metageneration;
    // [Jules: Rust] Clone the existing labels map.
    // [Jules: Rust] `bucket.labels` is a `HashMap<String, String>`.
    let mut labels = bucket.labels.clone();
    // [Jules: Rust] Insert the new label. If the key already exists, the value is updated.
    labels.insert(label_key.to_string(), label_value.to_string());

    // [Jules: SDK] Perform the update.
    let bucket = client
        .update_bucket()
        // [Jules: SDK] Provide the updated bucket metadata with the new labels.
        .set_bucket(bucket.set_labels(labels))
        // [Jules: SDK] Set the precondition to prevent race conditions.
        // [Jules: SDK] The update will fail if the bucket has been modified since we retrieved it.
        .set_if_metageneration_match(metageneration)
        // [Jules: SDK] Use a FieldMask to indicate that we only want to update the "labels" field.
        // [Jules: SDK] This ensures we don't accidentally overwrite other fields if the `bucket` object contains other changes.
        .set_update_mask(FieldMask::default().set_paths(["labels"]))
        .send()
        .await?;
    println!("Successfully added label {label_key}={label_value} to bucket {bucket_id}");
    println!("{:?}", bucket.labels);
    Ok(())
}
// [Jules: SDK] End region tag.
// [END storage_add_bucket_label]
