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

// [Jules: SDK] Start region tag for documentation.
// [START storage_rotate_encryption_key]
// [Jules: Rust] `RewriteObjectExt` is an extension trait that adds methods to the rewrite builder.
// [Jules: SDK] It provides `rewrite_until_done` which simplifies the multi-step rewrite process.
use google_cloud_storage::builder_ext::RewriteObjectExt;
use google_cloud_storage::client::StorageControl;
use google_cloud_storage::model::CommonObjectRequestParams;
// [Jules: SDK] `KeyAes256` is a helper for managing Customer-Supplied Encryption Keys (CSEK).
use google_cloud_storage::model_ext::KeyAes256;

// [Jules: SDK] This function rotates the encryption key of an object.
// [Jules: SDK] Key rotation is implemented by rewriting the object to itself, decrypting with the old key and encrypting with the new key.
pub async fn sample(
    client: &StorageControl,
    bucket_id: &str,
    object_id: &str,
    old_key: KeyAes256,
    new_key: KeyAes256,
) -> anyhow::Result<()> {
    // [Jules: Rust] Convert the `old_key` (KeyAes256) into `CommonObjectRequestParams`.
    // [Jules: Rust] `into()` works because `KeyAes256` implements `Into<CommonObjectRequestParams>`.
    let old: CommonObjectRequestParams = old_key.into();
    // [Jules: SDK] Initiate the rewrite operation.
    let updated = client
        .rewrite_object()
        // [Jules: SDK] Set the source object (the object to be rotated).
        .set_source_bucket(format!("projects/_/buckets/{bucket_id}"))
        .set_source_object(object_id)
        // [Jules: SDK] Provide the encryption parameters to decrypt the source object.
        .set_copy_source_encryption_algorithm(old.encryption_algorithm)
        .set_copy_source_encryption_key_bytes(old.encryption_key_bytes)
        .set_copy_source_encryption_key_sha256_bytes(old.encryption_key_sha256_bytes)
        // [Jules: SDK] Set the destination object (the same object, to overwrite it).
        .set_destination_bucket(format!("projects/_/buckets/{bucket_id}"))
        .set_destination_name(object_id)
        // [Jules: SDK] Provide the new encryption key to encrypt the destination object.
        // [Jules: SDK] `set_common_object_request_params` is a convenience method to set standard object parameters like encryption.
        .set_common_object_request_params(new_key)
        // [Jules: SDK] `rewrite_until_done` executes the rewrite loop.
        // [Jules: SDK] Large objects might require multiple RPC calls to rewrite completely. This helper handles that complexity.
        .rewrite_until_done()
        .await?;

    println!(
        "successfully rotated encryption key for object {object_id} in bucket {bucket_id}: {updated:?}"
    );
    Ok(())
}
// [Jules: SDK] End region tag.
// [END storage_rotate_encryption_key]
