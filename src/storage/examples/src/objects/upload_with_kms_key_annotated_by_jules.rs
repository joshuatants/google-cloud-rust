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
// [START storage_upload_with_kms_key]
// [Jules: SDK] `Storage` is the client for object data operations (uploads/downloads).
use google_cloud_storage::client::Storage;

// [Jules: SDK] Uploads a file using a Cloud KMS key for encryption.
pub async fn sample(
    client: &Storage,
    bucket_id: &str,
    file_path: &str,
    kms_key: &str,
) -> Result<(), anyhow::Error> {
    const NAME: &str = "object-to-upload.txt";
    // [Jules: Rust] Open the local file asynchronously.
    let payload = tokio::fs::File::open(file_path).await?;
    // [Jules: SDK] Initiate the upload.
    let _result = client
        .write_object(format!("projects/_/buckets/{bucket_id}"), NAME, payload)
        // [Jules: SDK] Specify the Cloud KMS key to use for server-side encryption.
        .set_kms_key(kms_key)
        // [Jules: SDK] `send_unbuffered` performs a streaming upload without buffering the entire file in memory.
        // [Jules: SDK] This is efficient for large files.
        .send_unbuffered()
        .await?;

    println!("Uploaded {NAME} to bucket {bucket_id} with KMS key {kms_key}.");
    Ok(())
}
// [END storage_upload_with_kms_key]
