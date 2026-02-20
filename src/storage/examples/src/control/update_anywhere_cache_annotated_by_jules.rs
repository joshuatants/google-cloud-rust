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
// [START storage_control_update_anywhere_cache]
// [Jules: SDK] `Poller` is a trait that provides methods to wait for Long Running Operations (LROs) to complete.
use google_cloud_lro::Poller;
use google_cloud_storage::client::StorageControl;
use google_cloud_storage::model::AnywhereCache;
use google_cloud_wkt::FieldMask;

// [Jules: SDK] Sample to update an Anywhere Cache instance.
// [Jules: SDK] Anywhere Cache is a feature that allows caching GCS data in other zones/regions.
pub async fn sample(
    client: &StorageControl,
    bucket_id: &str,
    cache_id: &str,
) -> anyhow::Result<()> {
    // [Jules: SDK] Prepare the `AnywhereCache` resource with the fields we want to update.
    let anywhere_cache = AnywhereCache::new()
        // [Jules: SDK] Set the resource name. The format is `projects/_/buckets/{bucket}/anywhereCaches/{cache}`.
        .set_name(format!(
            "projects/_/buckets/{}/anywhereCaches/{}",
            bucket_id, cache_id
        ))
        // [Jules: SDK] Set the new admission policy.
        .set_admission_policy("ADMIT_ON_SECOND_MISS".to_string());
    // [Jules: SDK] Initiate the update operation.
    // [Jules: SDK] Updating an Anywhere Cache is a Long Running Operation (LRO) because it might involve infrastructure changes.
    let operation = client
        .update_anywhere_cache()
        .set_anywhere_cache(anywhere_cache)
        // [Jules: SDK] Use a FieldMask to specify that we only want to update the "admission_policy" field.
        .set_update_mask(FieldMask::default().set_paths(["admission_policy"]))
        // [Jules: SDK] `poller()` returns a helper to poll the operation status.
        .poller()
        // [Jules: SDK] `until_done()` polls the operation until it completes or fails.
        .until_done()
        .await?;
    println!("Updated anywhere cache: {:?}", operation);
    Ok(())
}
// [Jules: SDK] End region tag.
// [END storage_control_update_anywhere_cache]
