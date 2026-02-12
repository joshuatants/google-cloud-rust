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

// [START storage_upload_file]
use google_cloud_storage::client::Storage;

pub async fn sample(
    client: &Storage,
    bucket: &str,
    object: &str,
    file_path: &str,
) -> Result<(), anyhow::Error> {
    // [Jules: Rust]
    // Open the file asynchronously using `tokio::fs`.
    // We get a `File` handle which implements `AsyncRead`.
    let payload = tokio::fs::File::open(file_path).await?;

    // [Jules: SDK]
    // `write_object` initiates an upload.
    // The arguments are: parent bucket (formatted string), object name, and the data payload.
    // `client.write_object` accepts anything that implements `Into<Payload>`. `tokio::fs::File` is one such type.
    let _result = client
        .write_object(format!("projects/_/buckets/{bucket}"), object, payload)
        // [Jules: SDK]
        // `send_unbuffered()` performs a "simple upload".
        // The entire request body is sent in a single HTTP request.
        // This is efficient for small files but not recommended for very large files (where resumable upload `send_buffered()` is better).
        .send_unbuffered()
        .await?;

    println!("Uploaded {file_path} to {object} in bucket {bucket}.");
    Ok(())
}
// [END storage_upload_file]
