// [Jules: Rust] The double slash `//` starts a single-line comment.
// [Jules: Rust] These specific comments are part of the file header, explaining the license under which this code is distributed.
// [Jules: Rust] It is common practice in open-source projects to include license information at the top of every source file.
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

// [Jules: SDK] This comment is a "region tag". It is used by the Google Cloud documentation system to automatically
// [Jules: SDK] extract and display this code snippet in the official documentation.
// [START storage_set_object_retention_policy]
// [Jules: Rust] The `use` keyword is used to bring items (like structs, enums, traits) from other modules into the current scope.
// [Jules: Rust] This allows you to use them by their short name (e.g., `StorageControl`) instead of the full path (e.g., `google_cloud_storage::client::StorageControl`).
// [Jules: SDK] `StorageControl` is the main client for interacting with the Google Cloud Storage Control API.
// [Jules: SDK] `Retention` is a model struct representing the retention policy of an object.
use google_cloud_storage::{client::StorageControl, model::object::Retention};
// [Jules: SDK] `FieldMask` is a well-known type used to specify which fields of a resource should be updated.
// [Jules: SDK] `Timestamp` is a well-known type representing a point in time, independent of any time zone.
use google_cloud_wkt::{FieldMask, Timestamp};
// [Jules: Rust] `std::time::SystemTime` is part of the Rust standard library and provides access to the system clock.
use std::time::SystemTime;

// [Jules: Rust] `pub` makes this function public, allowing it to be called from other modules.
// [Jules: Rust] `async` marks this function as asynchronous. It will return a `Future` that needs to be `await`ed to run.
// [Jules: Rust] `fn` is the keyword to define a function.
// [Jules: Rust] `sample` is the name of the function.
// [Jules: Rust] `client: &StorageControl` means the function takes a reference to a `StorageControl` instance. Using a reference avoids taking ownership of the client.
// [Jules: Rust] `bucket_id: &str` means the function takes a string slice as the bucket identifier. `&str` is preferred over `String` for function arguments when you just need to read the string.
// [Jules: Rust] `-> anyhow::Result<()>` specifies the return type. `anyhow::Result` is a convenient alias for `Result<T, anyhow::Error>`, allowing the function to return any error type.
// [Jules: Rust] The `()` unit type indicates that the function returns no meaningful value on success (like `void` in other languages).
pub async fn sample(client: &StorageControl, bucket_id: &str) -> anyhow::Result<()> {
    // [Jules: Rust] `const` defines a constant value. The type must be explicitly specified (`&str` in this case).
    // [Jules: Rust] Constants are evaluated at compile time and are usually named in UPPER_CASE.
    const NAME: &str = "object-to-update";
    // [Jules: SDK] This block retrieves the current metadata of the object. We need this to get the current `metageneration`.
    // [Jules: SDK] `client.get_object()` initiates a request builder for the GetObject API.
    let object = client
        .get_object()
        // [Jules: SDK] `set_bucket` specifies the bucket containing the object. The format `projects/_/buckets/{bucket_id}` is the resource name format for buckets.
        // [Jules: Rust] `format!` is a macro that creates a `String` by interpolating values. It works like `printf` in C or f-strings in Python.
        .set_bucket(format!("projects/_/buckets/{bucket_id}"))
        // [Jules: SDK] `set_object` specifies the name of the object to retrieve.
        .set_object(NAME)
        // [Jules: SDK] `send()` finalizes the builder and sends the request to the API. It returns a future.
        .send()
        // [Jules: Rust] `.await` suspends execution of the function until the future completes (i.e., the request finishes).
        // [Jules: Rust] `?` is the error propagation operator. If the result is `Ok`, it unwraps the value. If it is `Err`, it returns the error from the function immediately.
        .await?;

    // [Jules: Rust] `Timestamp::try_from` attempts to convert a `SystemTime` into a protobuf `Timestamp`.
    // [Jules: Rust] This conversion can fail (e.g., if the time is out of range), so it returns a `Result`, which we handle with `?`.
    let now = Timestamp::try_from(SystemTime::now())?;
    // [Jules: Rust] We calculate a future time (24 hours from now) for the retention period.
    // [Jules: Rust] `clamp` ensures the nanoseconds are within the valid range for a protobuf timestamp.
    // [Jules: Rust] `24 * 60 * 60` calculates the number of seconds in a day.
    let then = Timestamp::clamp(now.seconds() + 24 * 60 * 60, now.nanos());
    // [Jules: SDK] `metageneration` is a version number for the object's metadata. It is used for optimistic concurrency control.
    let metageneration = object.metageneration;
    // [Jules: SDK] This block updates the object's retention policy.
    // [Jules: SDK] `client.update_object()` initiates a request builder for the UpdateObject API.
    let updated = client
        .update_object()
        // [Jules: SDK] `set_if_metageneration_match` sets a precondition: the update will only succeed if the object's current metageneration matches the one we retrieved earlier.
        // [Jules: SDK] This prevents us from overwriting changes made by someone else in the meantime (race condition).
        .set_if_metageneration_match(metageneration)
        // [Jules: SDK] `set_object` provides the new object metadata.
        // [Jules: SDK] We take the existing `object` and modify it using the `set_retention` method.
        .set_object(
            object.set_retention(
                // [Jules: SDK] `Retention::new()` creates a new `Retention` object.
                Retention::new()
                    // [Jules: SDK] `set_mode` sets the retention mode. "UNLOCKED" means the retention policy can be removed or shortened.
                    .set_mode("UNLOCKED")
                    // [Jules: SDK] `set_retain_until_time` sets the expiration time for the retention.
                    .set_retain_until_time(then),
            ),
        )
        // [Jules: SDK] `set_override_unlocked_retention(true)` is required when changing a retention policy from "UNLOCKED" or setting a new "UNLOCKED" policy.
        .set_override_unlocked_retention(true)
        // [Jules: SDK] `set_update_mask` tells the server which fields we intend to update.
        // [Jules: SDK] Here we only want to update the "retention" field. Any other fields in the object metadata will be ignored by the server.
        // [Jules: Rust] `FieldMask::default()` creates an empty field mask. `.set_paths(...)` adds the paths to it.
        // [Jules: Rust] `["retention"]` creates an array slice containing one string literal.
        .set_update_mask(FieldMask::default().set_paths(["retention"]))
        // [Jules: SDK] Send the update request.
        .send()
        // [Jules: Rust] Await the result and propagate any errors.
        .await?;
    // [Jules: Rust] `println!` is a macro that prints text to the standard output (console).
    // [Jules: Rust] `{NAME}` and `{bucket_id}` are captured from the surrounding scope and inserted into the string.
    // [Jules: Rust] `{updated:?}` prints the `updated` variable using its `Debug` implementation, which gives a developer-friendly text representation of the struct.
    println!("successfully set retention for object {NAME} in bucket {bucket_id}: {updated:?}");
    // [Jules: Rust] `Ok(())` creates a `Result::Ok` variant containing the unit value `()`.
    // [Jules: Rust] This indicates that the function completed successfully.
    Ok(())
}
// [Jules: SDK] This tag marks the end of the region that will be extracted for documentation.
// [END storage_set_object_retention_policy]
