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

// [Jules: Rust] Imports.
use crate::sample::Scenario;

use anyhow::bail;
use clap::Parser;
use humantime::parse_duration;
use std::time::Duration;

// [Jules: SDK] Defines the command-line arguments for the scenario-based integration tests.
/// Configuration options for the benchmark.
// [Jules: Rust] `#[derive(Parser)]` automatically implements the logic to parse command-line arguments into this struct.
// [Jules: Rust] `#[command(...)]` provides metadata for the help message.
#[derive(Clone, Debug, Parser)]
#[command(version, about, long_about = super::DESCRIPTION)]
pub struct Args {
    /// The name of the bucket used by the benchmark.
    ///
    /// You should use a regional bucket in the same region as the VM running
    /// the benchmark.
    // [Jules: Rust] `#[arg(long)]` means this argument is passed as `--bucket-name`.
    #[arg(long)]
    pub bucket_name: String,

    /// Use existing objects for the test.
    ///
    /// When true, the benchmark uses existing objects in the bucket.
    /// It ignores objects that are too small.
    // [Jules: Rust] `default_value_t = false` means if the flag is missing, it defaults to false.
    #[arg(long, default_value_t = false)]
    pub use_existing_dataset: bool,

    /// The number of concurrent tasks running the test.
    #[arg(long, default_value_t = 1)]
    pub task_count: usize,

    /// The rampup period between new tasks.
    // [Jules: Rust] `value_parser = parse_duration` uses the `humantime` crate to parse strings like "500ms" into `Duration`.
    #[arg(long, value_parser = parse_duration, default_value = "500ms")]
    pub rampup_period: Duration,

    /// The rampup period between new tasks to write data.
    #[arg(long, value_parser = parse_duration, default_value = "10ms")]
    pub dataset_rampup_period: Duration,

    /// The number of iterations for the test.
    #[arg(long, default_value_t = 1)]
    pub iterations: u64,

    /// The number of gRPC subchannels.
    #[arg(long)]
    pub grpc_subchannel_count: Option<usize>,

    /// The scenarios for this test.
    // [Jules: Rust] A positional argument list.
    pub scenarios: Vec<Scenario>,
}

impl Args {
    /// Validates the arguments after parsing.
    pub fn validate(&self) -> anyhow::Result<()> {
        // [Jules: Rust] Check if `grpc_subchannel_count` is set and is 0 (which is invalid).
        if self.grpc_subchannel_count.is_some_and(|v| v == 0) {
            bail!("invalid number of gRPC subchannels, should be > 0")
        }
        Ok(())
    }

    /// The fully qualified bucket name.
    // [Jules: SDK] Helper to construct the canonical resource name for the bucket.
    pub fn full_bucket_name(&self) -> String {
        format!("projects/_/buckets/{}", self.bucket_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn validate_success() -> anyhow::Result<()> {
        // [Jules: Rust] Parse arguments from a list of strings (simulating argv).
        let args = Args::try_parse_from(["program", "--bucket-name=bucket"])?;
        assert_eq!(args.bucket_name, "bucket");
        args.validate()?;
        Ok(())
    }

    #[test_case(&["program", "--bucket-name=bucket", "--grpc-subchannel-count=0"])]
    fn validate(input: &[&str]) -> anyhow::Result<()> {
        let args = Args::try_parse_from(input)?;
        assert_eq!(args.bucket_name, "bucket");
        let got = args.validate();
        assert!(got.is_err(), "{got:?} {args:?}");
        Ok(())
    }
}
