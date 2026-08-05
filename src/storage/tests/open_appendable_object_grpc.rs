// Copyright 2026 Google LLC
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

#![cfg(google_cloud_unstable_storage_bidi)]

use bytes::Bytes;
use gaxi::grpc::tonic::{Response as TonicResponse, Status as TonicStatus};
use google_cloud_auth::credentials::anonymous::Builder as Anonymous;
use google_cloud_storage::client::Storage;
use pretty_assertions::assert_eq;
use storage_grpc_mock::google::storage::v2::{
    BidiWriteHandle, BidiWriteObjectRedirectedError, BidiWriteObjectRequest,
    BidiWriteObjectResponse, Object as ProtoObject, bidi_write_object_request,
    bidi_write_object_response::WriteStatus,
};
use storage_grpc_mock::{MockStorage, start};

const BIND_ADDRESS: &str = "127.0.0.1:0";
const BUCKET_NAME: &str = "projects/_/buckets/test-bucket";
const OBJECT_NAME: &str = "test-object";
const OBJECT_GENERATION: i64 = 123456;
const OBJECT_CONTENT: &[u8] = b"hello appendable world";

const ERR_STREAM_CLOSED_PREMATURELY: &str = "gRPC stream closed before the request was received";
const ERR_RECV_ERROR: &str = "error while reading the request";

#[tokio::test]
async fn open_appendable_object_single_block_success() -> anyhow::Result<()> {
    // Arrange
    const USER_AGENT: &str = "open_appendable_object_grpc/1.0";
    const QUOTA_PROJECT: &str = "appendable-object-quota-project";

    let (observed_tx, observed_rx) = tokio::sync::oneshot::channel::<BidiWriteObjectRequest>();

    let mut mock = MockStorage::new();
    mock.expect_bidi_write_object().return_once(move |request| {
        assert_request_metadata(request.metadata(), USER_AGENT, QUOTA_PROJECT);
        let (_, _, mut requests) = request.into_parts();
        let (tx, rx) = tokio::sync::mpsc::channel(2);
        tokio::spawn(async move {
            let first = requests
                .recv()
                .await
                .expect(ERR_STREAM_CLOSED_PREMATURELY)
                .expect(ERR_RECV_ERROR);
            observed_tx
                .send(first)
                .expect("failed to send recorded request");
            tx.send(Ok(initial_resource_response(0)))
                .await
                .expect("send initial");
            while let Some(Ok(_req)) = requests.recv().await {}
            tx.send(Ok(final_resource_response(OBJECT_CONTENT.len() as i64)))
                .await
                .expect("send final");
        });

        Ok(TonicResponse::from(rx))
    });
    let (endpoint, _server) = start(BIND_ADDRESS, mock).await?;
    let client = make_client(endpoint).await?;

    // Act
    let mut writer = client
        .open_appendable_object(BUCKET_NAME, OBJECT_NAME)
        .with_user_agent(USER_AGENT)
        .with_quota_project(QUOTA_PROJECT)
        .send()
        .await?;
    writer.append(Bytes::from_static(OBJECT_CONTENT)).await?;
    let metadata = writer.finalize().await?;

    // Assert
    let first_request = observed_rx.await?;
    let first_message = first_request
        .first_message
        .expect("first request should contain first_message");
    let spec = match first_message {
        bidi_write_object_request::FirstMessage::WriteObjectSpec(s) => s,
        _ => panic!("expected WriteObjectSpec"),
    };
    assert_eq!(
        spec.resource.as_ref().map(|r| r.bucket.as_str()),
        Some(BUCKET_NAME)
    );
    assert_eq!(
        spec.resource.as_ref().map(|r| r.name.as_str()),
        Some(OBJECT_NAME)
    );

    assert_eq!(metadata.size, OBJECT_CONTENT.len() as i64);
    assert_eq!(metadata.name, OBJECT_NAME);

    Ok(())
}

#[tokio::test]
async fn open_appendable_object_chunked_appends_flush_and_finalize() -> anyhow::Result<()> {
    // Arrange
    let mut mock = MockStorage::new();
    mock.expect_bidi_write_object().return_once(move |request| {
        let (_, _, mut requests) = request.into_parts();
        let (tx, rx) = tokio::sync::mpsc::channel(3);
        tokio::spawn(async move {
            let _first = requests.recv().await;
            tx.send(Ok(initial_resource_response(0)))
                .await
                .expect("send initial");
            let _append1 = requests.recv().await;
            let _flush = requests.recv().await;
            tx.send(Ok(persisted_size_response(6)))
                .await
                .expect("send flush");
            while let Some(Ok(_req)) = requests.recv().await {}
            tx.send(Ok(final_resource_response(11)))
                .await
                .expect("send final");
        });

        Ok(TonicResponse::from(rx))
    });
    let (endpoint, _server) = start(BIND_ADDRESS, mock).await?;
    let client = make_client(endpoint).await?;

    // Act
    let mut writer = client
        .open_appendable_object(BUCKET_NAME, OBJECT_NAME)
        .send()
        .await?;
    writer.append(Bytes::from_static(b"hello ")).await?;
    let flushed_size = writer.flush().await?;
    writer.append(Bytes::from_static(b"world")).await?;
    let metadata = writer.finalize().await?;

    // Assert
    assert_eq!(flushed_size, 6);
    assert_eq!(metadata.size, 11);

    Ok(())
}

#[tokio::test]
async fn reopen_appendable_object_success() -> anyhow::Result<()> {
    // Arrange
    let (observed_tx, observed_rx) = tokio::sync::oneshot::channel::<BidiWriteObjectRequest>();

    let mut mock = MockStorage::new();
    mock.expect_bidi_write_object().return_once(move |request| {
        let (_, _, mut requests) = request.into_parts();
        let (tx, rx) = tokio::sync::mpsc::channel(2);
        tokio::spawn(async move {
            let first = requests
                .recv()
                .await
                .expect(ERR_STREAM_CLOSED_PREMATURELY)
                .expect(ERR_RECV_ERROR);
            observed_tx.send(first).expect("send recorded");
            tx.send(Ok(initial_resource_response(6)))
                .await
                .expect("send initial");
            while let Some(Ok(_req)) = requests.recv().await {}
            tx.send(Ok(final_resource_response(11)))
                .await
                .expect("send final");
        });

        Ok(TonicResponse::from(rx))
    });
    let (endpoint, _server) = start(BIND_ADDRESS, mock).await?;
    let client = make_client(endpoint).await?;

    // Act
    let mut writer = client
        .reopen_appendable_object(BUCKET_NAME, OBJECT_NAME, OBJECT_GENERATION)
        .send()
        .await?;
    assert_eq!(writer.persisted_size(), 6);
    writer.append(Bytes::from_static(b"world")).await?;
    let metadata = writer.finalize().await?;

    // Assert
    let first_req = observed_rx.await?;
    let first_message = first_req
        .first_message
        .expect("first request should contain first_message");
    let spec = match first_message {
        bidi_write_object_request::FirstMessage::AppendObjectSpec(s) => s,
        _ => panic!("expected AppendObjectSpec"),
    };
    assert_eq!(spec.bucket, BUCKET_NAME);
    assert_eq!(spec.object, OBJECT_NAME);
    assert_eq!(spec.generation, OBJECT_GENERATION);
    assert_eq!(metadata.size, 11);

    Ok(())
}

#[tokio::test]
async fn open_appendable_object_redirect_resumes_with_routing_token_and_write_handle()
-> anyhow::Result<()> {
    // Arrange
    let (attempt1_tx, attempt1_rx) = tokio::sync::oneshot::channel::<BidiWriteObjectRequest>();
    let (attempt2_tx, attempt2_rx) = tokio::sync::oneshot::channel::<BidiWriteObjectRequest>();
    let attempt1_tx = std::sync::Arc::new(std::sync::Mutex::new(Some(attempt1_tx)));
    let attempt2_tx = std::sync::Arc::new(std::sync::Mutex::new(Some(attempt2_tx)));

    let mut mock = MockStorage::new();
    mock.expect_bidi_write_object()
        .times(2)
        .returning(move |request| {
            let (_, _, mut requests) = request.into_parts();
            if let Some(tx) = attempt1_tx.lock().expect("mutex").take() {
                tokio::spawn(async move {
                    let first = requests
                        .recv()
                        .await
                        .expect(ERR_STREAM_CLOSED_PREMATURELY)
                        .expect(ERR_RECV_ERROR);
                    tx.send(first).expect("send attempt 1");
                });
                return Err(redirect_status("test-routing-token", b"test-write-handle"));
            }
            if let Some(tx) = attempt2_tx.lock().expect("mutex").take() {
                let (tx_resp, rx_resp) = tokio::sync::mpsc::channel(2);
                tokio::spawn(async move {
                    let first = requests
                        .recv()
                        .await
                        .expect(ERR_STREAM_CLOSED_PREMATURELY)
                        .expect(ERR_RECV_ERROR);
                    tx.send(first).expect("send attempt 2");
                    tx_resp
                        .send(Ok(initial_resource_response(0)))
                        .await
                        .expect("send initial");
                    while let Some(Ok(_req)) = requests.recv().await {}
                    tx_resp
                        .send(Ok(final_resource_response(OBJECT_CONTENT.len() as i64)))
                        .await
                        .expect("send final");
                });
                return Ok(TonicResponse::from(rx_resp));
            }
            Err(TonicStatus::internal("unexpected attempt"))
        });

    let (endpoint, _server) = start(BIND_ADDRESS, mock).await?;
    let client = make_client(endpoint).await?;

    // Act
    let mut writer = client
        .open_appendable_object(BUCKET_NAME, OBJECT_NAME)
        .send()
        .await?;
    writer.append(Bytes::from_static(OBJECT_CONTENT)).await?;
    let metadata = writer.finalize().await?;

    // Assert
    let req1 = attempt1_rx.await?;
    let first1 = req1.first_message.expect("first_message 1");
    let spec1 = match first1 {
        bidi_write_object_request::FirstMessage::WriteObjectSpec(s) => s,
        _ => panic!("expected WriteObjectSpec"),
    };
    assert_eq!(
        spec1.resource.as_ref().map(|r| r.bucket.as_str()),
        Some(BUCKET_NAME)
    );

    let req2 = attempt2_rx.await?;
    let first2 = req2.first_message.expect("first_message 2");
    let spec2 = match first2 {
        bidi_write_object_request::FirstMessage::AppendObjectSpec(s) => s,
        _ => panic!("expected AppendObjectSpec"),
    };
    assert_eq!(spec2.routing_token, Some("test-routing-token".to_string()));
    assert_eq!(
        spec2.write_handle,
        Some(BidiWriteHandle {
            handle: b"test-write-handle".to_vec(),
        })
    );
    assert_eq!(spec2.generation, OBJECT_GENERATION);

    assert_eq!(metadata.size, OBJECT_CONTENT.len() as i64);

    Ok(())
}

fn redirect_status(routing: &str, handle: &'static [u8]) -> TonicStatus {
    use prost::Message as _;
    let redirect = BidiWriteObjectRedirectedError {
        routing_token: Some(routing.to_string()),
        write_handle: Some(BidiWriteHandle {
            handle: handle.to_vec(),
        }),
        generation: Some(OBJECT_GENERATION),
    };
    let redirect = prost_types::Any::from_msg(&redirect).expect("encode redirect any");
    let status = storage_grpc_mock::google::rpc::Status {
        code: gaxi::grpc::tonic::Code::Aborted as i32,
        message: "redirected".into(),
        details: vec![redirect],
    };
    let mut buf = bytes::BytesMut::with_capacity(256);
    status.encode(&mut buf).expect("encode rpc status");
    TonicStatus::with_details(gaxi::grpc::tonic::Code::Aborted, "redirected", buf.freeze())
}

fn test_metadata(size: i64) -> ProtoObject {
    ProtoObject {
        bucket: BUCKET_NAME.to_string(),
        name: OBJECT_NAME.to_string(),
        generation: OBJECT_GENERATION,
        size,
        ..Default::default()
    }
}

fn initial_resource_response(size: i64) -> BidiWriteObjectResponse {
    BidiWriteObjectResponse {
        write_status: Some(WriteStatus::Resource(test_metadata(size))),
        ..Default::default()
    }
}

fn final_resource_response(size: i64) -> BidiWriteObjectResponse {
    let mut obj = test_metadata(size);
    obj.finalize_time = Some(prost_types::Timestamp {
        seconds: 1234567890,
        nanos: 0,
    });
    BidiWriteObjectResponse {
        write_status: Some(WriteStatus::Resource(obj)),
        ..Default::default()
    }
}

fn persisted_size_response(size: i64) -> BidiWriteObjectResponse {
    BidiWriteObjectResponse {
        write_status: Some(WriteStatus::PersistedSize(size)),
        ..Default::default()
    }
}

async fn make_client(endpoint: impl Into<String>) -> anyhow::Result<Storage> {
    let client = Storage::builder()
        .with_credentials(Anonymous::new().build())
        .with_endpoint(endpoint)
        .build()
        .await?;
    Ok(client)
}

fn assert_request_metadata(
    metadata: &gaxi::grpc::tonic::MetadataMap,
    expected_user_agent: &str,
    expected_quota_project: &str,
) {
    let user_agent = metadata
        .get(http::header::USER_AGENT.as_str())
        .and_then(|value| value.to_str().ok())
        .expect("user-agent should be set");
    assert!(
        user_agent
            .split(' ')
            .any(|value| value == expected_user_agent),
        "{user_agent}"
    );
    let quota_project = metadata
        .get("x-goog-user-project")
        .and_then(|value| value.to_str().ok())
        .expect("quota-project should be set");
    assert_eq!(quota_project, expected_quota_project);
}
