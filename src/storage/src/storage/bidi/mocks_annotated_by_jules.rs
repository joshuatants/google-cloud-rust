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

// [Jules: Rust] Imports from the parent module and other test helpers.
use super::connector::Connector;
use super::tests::test_options;
use super::{Client, Receiver, RequestOptions, TonicStreaming};
use crate::google::storage::v2::{
    BidiReadObjectRequest, BidiReadObjectResponse, BidiReadObjectSpec,
};
use gaxi::grpc::tonic::{Extensions, Response as TonicResponse, Result as TonicResult};
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

// [Jules: Rust] `mockall` mocks are not `Clone` by default because they contain state (expectations).
// [Jules: Rust] However, our `Connector` requires a `Clone`-able client.
// [Jules: Rust] The solution is to wrap the mock in an `Arc` (Atomic Reference Counted) pointer, which is `Clone`.
#[derive(Clone, Debug)]
pub struct SharedMockClient(pub(crate) Arc<MockTestClient>);

impl SharedMockClient {
    pub fn new(mock: MockTestClient) -> Self {
        Self(Arc::new(mock))
    }
}

// [Jules: Rust] Implement the `Client` trait for our shared wrapper.
impl Client for SharedMockClient {
    type Stream = MockStream;

    // [Jules: Rust] Forward the `start` call to the inner mock object.
    async fn start(
        &self,
        extensions: Extensions,
        path: http::uri::PathAndQuery,
        rx: Receiver<BidiReadObjectRequest>,
        options: &RequestOptions,
        api_client_header: &'static str,
        request_params: &str,
    ) -> crate::Result<TonicResult<TonicResponse<Self::Stream>>> {
        self.0.start(
            extensions,
            path,
            rx,
            options,
            api_client_header,
            request_params,
        )
    }
}

// [Jules: Rust] Implement `TonicStreaming` for a standard MPSC receiver.
// [Jules: Rust] This allows us to use a simple channel to simulate the stream of responses from the server.
impl TonicStreaming for Receiver<TonicResult<BidiReadObjectResponse>> {
    async fn next_message(&mut self) -> TonicResult<Option<BidiReadObjectResponse>> {
        // [Jules: Rust] `recv()` waits for the next message. `transpose()` handles the `Option<Result>` vs `Result<Option>` conversion.
        self.recv().await.transpose()
    }
}

// [Jules: Rust] Define a trait that mirrors the `start` method we need to mock.
// [Jules: Rust] `mockall::automock` generates a struct `MockTestClient` implementing this trait.
#[mockall::automock]
pub trait TestClient: std::fmt::Debug {
    fn start(
        &self,
        extensions: Extensions,
        path: http::uri::PathAndQuery,
        rx: Receiver<BidiReadObjectRequest>,
        options: &RequestOptions,
        api_client_header: &'static str,
        request_params: &str,
    ) -> crate::Result<TonicResult<TonicResponse<MockStream>>>;
}

// [Jules: Rust] Type aliases for the mock stream (channel).
pub type MockStream = Receiver<TonicResult<BidiReadObjectResponse>>;
pub type MockStreamSender = Sender<TonicResult<BidiReadObjectResponse>>;

// [Jules: SDK] Helper to create a `Connector` with a configured mock client.
pub fn mock_connector(mock: MockTestClient) -> Connector<SharedMockClient> {
    let client = SharedMockClient::new(mock);

    // [Jules: SDK] Default spec for testing.
    let spec = BidiReadObjectSpec {
        bucket: "projects/_/buckets/test-bucket".into(),
        object: "test-object".into(),
        ..BidiReadObjectSpec::default()
    };

    Connector::new(spec, test_options(), client.clone())
}

// [Jules: SDK] Helper to create the channel pair used for the mock stream.
pub fn mock_stream() -> (MockStreamSender, MockStream) {
    tokio::sync::mpsc::channel(10)
}
