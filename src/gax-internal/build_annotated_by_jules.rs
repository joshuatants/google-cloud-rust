// [Jules: Rust] Standard copyright header.
// [Jules: Rust] Apache 2.0 License.
// Copyright 2024 Google LLC
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

// [Jules: Rust] Imports for file I/O operations.
use std::fs::File;
use std::io::Write;
use std::path::Path;

// [Jules: Rust] This is a build script (`build.rs`).
// [Jules: Rust] It runs before the main compilation of the crate.
// [Jules: SDK] Its purpose here is to detect the Rust compiler version and make it available to the code.
// [Jules: SDK] This version info is used in the `User-Agent` header for telemetry.
fn main() {
    // [Jules: Rust] Cargo sets the `OUT_DIR` environment variable to the path where the build script should place generated files.
    let out_dir = std::env::var_os("OUT_DIR").expect("OUT_DIR not specified");
    let out_path = Path::new(&out_dir).to_owned();

    // [Jules: Rust] `rustc_version` is a build dependency that retrieves the version of the rust compiler being used.
    let rust_version = rustc_version::version().expect("Could not retrieve rustc version");
    // [Jules: Rust] Create a file named `build_env.rs` in the output directory.
    let mut f =
        File::create(out_path.join("build_env.rs")).expect("Could not create build environment");
    // [Jules: Rust] Write a Rust constant definition into the file.
    // [Jules: Rust] This generated file will be included in the main code using `include!(concat!(env!("OUT_DIR"), "/build_env.rs"));`.
    f.write_all(format!("pub(crate) const RUSTC_VERSION: &str = \"{rust_version}\";").as_bytes())
        .expect("Unable to write rust version");
    // [Jules: Rust] Flush the buffer to ensure the file is written to disk.
    f.flush().expect("failed to flush");
}
