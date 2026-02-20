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

// [Jules: Rust] Imports from sibling modules in the benchmark crate.
use super::args::Args;
use super::experiment::{Experiment, Range};
use super::sample::Attempt;
// [Jules: Rust] `anyhow` is used for flexible error handling. `bail!` is a macro to return an error early.
use anyhow::{Result, bail};
use google_cloud_storage::client::Storage;
use google_cloud_storage::model_ext::ReadRange;
use google_cloud_storage::object_descriptor::ObjectDescriptor;
use std::collections::HashMap;
use std::time::Instant;

// [Jules: SDK] The `Runner` struct manages the benchmark execution state.
// [Jules: SDK] It caches `ObjectDescriptor`s to avoid reopening objects repeatedly.
pub struct Runner {
    descriptors: HashMap<String, ObjectDescriptor>,
}

impl Runner {
    // [Jules: Rust] Async constructor to initialize the runner and pre-open objects.
    pub async fn new(args: &Args, objects: Vec<String>, client: Storage) -> Result<Self> {
        let bucket_name = format!("projects/_/buckets/{}", args.bucket_name);
        let mut descriptors = HashMap::new();
        // [Jules: Rust] Iterate over the object names provided.
        for name in objects {
            // [Jules: SDK] `open_object` prepares a handle to the object.
            // [Jules: SDK] This allows performing multiple operations (like reads) on the same object efficiently.
            let descriptor = client
                .open_object(bucket_name.clone(), name.clone())
                .send()
                .await?;
            descriptors.insert(name, descriptor);
        }
        Ok(Self { descriptors })
    }

    // [Jules: SDK] Runs a single iteration of the benchmark experiment.
    // [Jules: SDK] An iteration consists of multiple read operations defined in `experiment.ranges`.
    pub async fn iteration(&self, experiment: &Experiment) -> Vec<Result<Attempt>> {
        // [Jules: Rust] Map each range in the experiment to an `attempt` future.
        let running = experiment
            .ranges
            .iter()
            .map(|r| self.attempt(r))
            .collect::<Vec<_>>();

        // [Jules: Rust] `join_all` executes all the futures concurrently and waits for them to complete.
        // [Jules: Rust] This simulates concurrent load on the client.
        futures::future::join_all(running).await
    }

    // [Jules: SDK] Performs a single read attempt for a specific range.
    async fn attempt(&self, range: &Range) -> Result<Attempt> {
        // [Jules: Rust] Start the timer.
        let start = Instant::now();
        // [Jules: Rust] `let Some(...) = ... else { ... }` is a let-else statement.
        // [Jules: Rust] It attempts to match the pattern. If it fails, the `else` block is executed (which must diverge, e.g., return or panic).
        let Some(descriptor) = self.descriptors.get(&range.object_name) else {
            bail!(
                "cannot find object {} in available descriptors",
                range.object_name
            );
        };
        // [Jules: SDK] `read_range` initiates a read for a specific byte range.
        // [Jules: SDK] `ReadRange::segment` specifies the offset and length.
        let mut reader = descriptor
            .read_range(ReadRange::segment(range.read_offset, range.read_length))
            .await;
        let mut ttfb = None;
        let mut size = 0;
        // [Jules: Rust] Consume the stream of data chunks.
        while let Some(b) = reader.next().await.transpose()? {
            // [Jules: SDK] Record Time To First Byte (TTFB) on the first chunk.
            let _ = ttfb.get_or_insert(start.elapsed());
            size += b.len();
        }
        // [Jules: SDK] Verify we received the expected amount of data.
        if size != range.read_length as usize {
            bail!("mismatched requested vs. received size");
        }
        // [Jules: SDK] Record Time To Last Byte (TTLB).
        let ttlb = start.elapsed();
        // [Jules: SDK] If we got no data (size 0), TTFB is same as TTLB.
        let ttfb = ttfb.unwrap_or(ttlb);
        Ok(Attempt { size, ttfb, ttlb })
    }
}
