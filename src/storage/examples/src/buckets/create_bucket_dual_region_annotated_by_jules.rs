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

// [Jules: SDK] Region tag for documentation.
// [START storage_create_bucket_dual_region]
// [Jules: SDK] `StorageControl` is the client.
// [Jules: SDK] `Bucket` is the model for the bucket resource.
// [Jules: SDK] `CustomPlacementConfig` allows specifying custom locations for dual-region or multi-region buckets.
use google_cloud_storage::{
    client::StorageControl, model::Bucket, model::bucket::CustomPlacementConfig,
};

// [Jules: SDK] This sample demonstrates creating a dual-region bucket.
// [Jules: SDK] A dual-region bucket replicates data across two specific regions for high availability and low latency.
pub async fn sample(
    client: &StorageControl,
    project_id: &str,
    bucket_id: &str,
) -> anyhow::Result<()> {
    let bucket = client
        .create_bucket()
        // [Jules: SDK] The parent resource for buckets is the project. "projects/_" is a special alias for the project associated with the credentials.
        // [Jules: SDK] However, here we are using the global collection alias.
        .set_parent("projects/_")
        // [Jules: SDK] Set the ID of the bucket to create.
        .set_bucket_id(bucket_id)
        // [Jules: SDK] Provide the bucket metadata.
        .set_bucket(
            Bucket::new()
                // [Jules: SDK] Explicitly set the project that will own the bucket.
                .set_project(format!("projects/{project_id}"))
                // [Jules: SDK] Configure custom placement.
                .set_custom_placement_config(
                    // [Jules: SDK] Specify the two regions where data should be stored.
                    // [Jules: Rust] `["US-EAST4", "US-CENTRAL1"]` creates an array slice of string literals.
                    CustomPlacementConfig::new().set_data_locations(["US-EAST4", "US-CENTRAL1"]),
                ),
        )
        // [Jules: SDK] Send the request.
        .send()
        .await?;
    println!("successfully created bucket {bucket:?}");
    Ok(())
}
// [Jules: SDK] End region tag.
// [END storage_create_bucket_dual_region]
