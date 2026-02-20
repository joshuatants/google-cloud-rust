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
// [START storage_set_bucket_public_iam]
// [Jules: SDK] `Binding` represents an IAM binding, which binds a role to a set of members.
use google_cloud_iam_v1::model::Binding;
use google_cloud_storage::client::StorageControl;

// [Jules: SDK] Sets a bucket to be publicly accessible by granting `roles/storage.objectViewer` to `allUsers`.
// [Jules: SDK] WARNING: This makes all objects in the bucket readable by anyone on the internet.
pub async fn sample(client: &StorageControl, bucket_id: &str) -> anyhow::Result<()> {
    // [Jules: SDK] Get the current IAM policy.
    // [Jules: SDK] It's important to get the existing policy first so we don't overwrite existing bindings.
    let mut policy = client
        .get_iam_policy()
        .set_resource(format!("projects/_/buckets/{bucket_id}"))
        .send()
        .await?;
    // [Jules: Rust] Add a new binding to the policy.
    policy.bindings.push(
        Binding::new()
            // [Jules: SDK] The role to grant. `objectViewer` allows listing and reading objects.
            .set_role("roles/storage.objectViewer")
            // [Jules: SDK] The members to grant the role to. `allUsers` is a special identifier for public access.
            .set_members(vec!["allUsers".to_string()]),
    );
    // [Jules: SDK] Update the IAM policy.
    let updated_policy = client
        .set_iam_policy()
        .set_resource(format!("projects/_/buckets/{bucket_id}"))
        .set_policy(policy)
        .send()
        .await?;
    println!(
        "Successfully set public IAM policy for bucket {}",
        bucket_id
    );
    println!("The updated policy is: {:?}", updated_policy);
    Ok(())
}
// [END storage_set_bucket_public_iam]
