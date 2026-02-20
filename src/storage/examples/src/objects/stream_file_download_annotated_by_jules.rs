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
// [START storage_stream_file_download]
// [Jules: SDK] `Storage` is the client for object data operations.
use google_cloud_storage::client::Storage;

// [Jules: SDK] Downloads an object as a stream of bytes.
// [Jules: SDK] This is the most efficient way to read large objects, as it doesn't load the entire object into memory.
pub async fn sample(client: &Storage, bucket_id: &str) -> anyhow::Result<()> {
    const NAME: &str = "object-to-download.txt";
    // [Jules: SDK] Initiate the read request.
    let mut reader = client
        .read_object(format!("projects/_/buckets/{bucket_id}"), NAME)
        .send()
        .await?;
    println!("counting newlines {NAME} in bucket {bucket_id}");
    let mut count = 0;
    // [Jules: Rust] Consume the stream chunk by chunk.
    // [Jules: Rust] `reader.next()` returns a `Future` yielding `Option<Result<Bytes>>`.
    while let Some(buffer) = reader.next().await.transpose()? {
        // [Jules: Rust] Process the chunk immediately.
        // [Jules: Rust] Here we count the number of newline characters in the buffer.
        // [Jules: Rust] `into_iter()` creates an iterator over the bytes.
        count += buffer.into_iter().filter(|c| *c == b'\n').count();
    }
    println!("object {NAME} in bucket {bucket_id} contains {count} newlines");
    Ok(())
}
// [END storage_stream_file_download]
