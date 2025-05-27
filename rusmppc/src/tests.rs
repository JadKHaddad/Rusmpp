use std::{str::FromStr, time::Duration};

use rusmpp::{
    CommandId,
    pdus::{DeliverSmResp, SubmitSm},
    session::SessionState,
};
use server::{Server, UnbindServer};
use tokio_stream::StreamExt;

use crate::{ConnectionBuilder, Event, error::Error};

mod server;

pub fn init_tracing() {
    _ = tracing_subscriber::fmt()
        .with_env_filter("rusmppc=trace")
        .try_init();
}

// cargo test --package rusmppc --lib -- tests::bind --exact --show-output --ignored
#[tokio::test]
#[ignore = "Integration test"]
async fn bind() {
    use rusmpp::{
        pdus::SubmitSm,
        types::{COctetString, OctetString},
        values::{EsmClass, Npi, RegisteredDelivery, ServiceType, Ton},
    };

    init_tracing();

    let (client, mut events) = ConnectionBuilder::new()
        .system_id(COctetString::from_str("NfDfddEKVI0NCxO").unwrap()) // cspell:disable-line
        .password(COctetString::from_str("rEZYMq5j").unwrap())
        .system_type(COctetString::empty())
        .addr_ton(Ton::Unknown)
        .addr_npi(Npi::Unknown)
        .address_range(COctetString::empty())
        .transceiver()
        .enquire_link_interval(Duration::from_secs(3))
        .response_timeout(Duration::from_secs(2))
        .max_command_length(1024)
        .connect("127.0.0.1:2775")
        .await
        .expect("Failed to connect");

    let client_clone = client.clone();
    let events = tokio::spawn(async move {
        while let Some(event) = events.next().await {
            tracing::debug!(?event, "Event",);

            if let Event::Command(command) = event {
                if command.id() == CommandId::DeliverSm {
                    tracing::debug!("Received DeliverSm");

                    let _ = client_clone
                        .deliver_sm_resp(command.sequence_number(), DeliverSmResp::default())
                        .await;
                }
            }
        }

        tracing::debug!("Event stream closed");
    });

    client
        .submit_sm(
            SubmitSm::builder()
                .service_type(ServiceType::default())
                .source_addr_ton(Ton::Unknown)
                .source_addr_npi(Npi::Unknown)
                .source_addr(COctetString::from_str("12345").unwrap())
                .destination_addr(COctetString::from_str("491701234567").unwrap())
                .esm_class(EsmClass::default())
                .registered_delivery(RegisteredDelivery::request_all())
                .short_message(OctetString::from_str("Hi, I am a short message.").unwrap())
                .build(),
        )
        .await
        .expect("Failed to submit_sm");

    // tokio::time::sleep(Duration::from_secs(2)).await;
    // drop(client);

    // or

    tokio::time::sleep(Duration::from_secs(20)).await;
    client.unbind().await.expect("Failed to unbind");
    let _ = client.terminated().await;

    // if the events task is done, this means that all tasks have terminated
    // if got end of stream in the reader task,
    // or all clients were dropped, so we closed the connection.
    //
    // To ensure graceful shutdown, drop all clients and await the events stream to finish.
    // Or client::unbind() then client::terminated().

    let _ = events.await;
}

#[tokio::test]
async fn bind_timeout() {
    init_tracing();

    let (_server, client) = tokio::io::duplex(1024);

    let error = ConnectionBuilder::new()
        .response_timeout(Duration::from_millis(500))
        .connected(client)
        .await
        .unwrap_err();

    assert!(matches!(error, Error::Timeout));

    // TODO: I have no idea how to check if all tasks are terminated.
    // See the logs to check if the tasks terminated.
    tokio::time::sleep(Duration::from_millis(500)).await;
}

/// Cancelling the request future should not cancel the request itself.
///
/// The response, if any, should be sent to the events stream.
/// The sequence number should also be removed from the pending responses
#[tokio::test]
async fn cancel_request_future() {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        Server::new().run(server).await;
    });

    let (client, mut events) = ConnectionBuilder::new()
        .response_timeout(Duration::from_millis(1000))
        .connected(client)
        .await
        .expect("Failed to connect");

    let future = client.submit_sm(SubmitSm::builder().build());

    tokio::select! {
        _ = tokio::time::sleep(Duration::from_millis(100)) => {
            tracing::debug!("Canceling request future");
        }
        _ = future => {}
    }

    // Checking the pending responses number is not accurate.

    // The submit sm response should be sent to the events stream

    let Some(event) = events.next().await else {
        panic!("No event received");
    };

    let Event::Command(command) = event else {
        panic!("Expected command event, got {:?}", event);
    };

    assert!(matches!(command.id(), CommandId::SubmitSmResp));
    assert_eq!(command.sequence_number(), 2);

    client.unbind().await.expect("Failed to unbind");

    let _ = client.terminated().await;
}

/// The response, if any, should be sent to the events stream.
/// The sequence number should also be removed from the pending responses
#[tokio::test]
async fn request_timeout() {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        Server::new()
            .bind_delay(Duration::from_millis(200))
            .response_delay(Duration::from_secs(1))
            .run(server)
            .await;
    });

    let (client, mut events) = ConnectionBuilder::new()
        .response_timeout(Duration::from_millis(500))
        .connected(client)
        .await
        .expect("Failed to connect");

    let error = client
        .submit_sm(SubmitSm::builder().build())
        .await
        .unwrap_err();

    assert!(matches!(error, Error::Timeout));

    // The submit sm response should be sent to the events stream

    let Some(event) = events.next().await else {
        panic!("No event received");
    };

    let Event::Command(command) = event else {
        panic!("Expected command event, got {:?}", event);
    };

    assert!(matches!(command.id(), CommandId::SubmitSmResp));
    assert_eq!(command.sequence_number(), 2);

    let error = client.unbind().await.unwrap_err();

    assert!(matches!(error, Error::Timeout));

    let _ = client.terminated().await;
}

#[tokio::test]
async fn unbind() {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        Server::new().run(server).await;
    });

    let (client, _events) = ConnectionBuilder::new()
        .connected(client)
        .await
        .expect("Failed to connect");

    client.unbind().await.expect("Failed to unbind");

    // Can't assert the state of the client to be unbound here.

    let _ = client.terminated().await;

    let session_state = client.session_state();
    assert!(matches!(session_state, SessionState::Closed));
}

#[tokio::test]
async fn drop_client() {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        Server::new().run(server).await;
    });

    let (client, events) = ConnectionBuilder::new()
        .connected(client)
        .await
        .expect("Failed to connect");

    drop(client);

    let _ = events.collect::<Vec<_>>().await;
}

#[tokio::test]
async fn cancel_unbind_future() {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        Server::new().run(server).await;
    });

    let (client, mut events) = ConnectionBuilder::new()
        .response_timeout(Duration::from_millis(1000))
        .connected(client)
        .await
        .expect("Failed to connect");

    let future = client.unbind();

    tokio::select! {
        _ = tokio::time::sleep(Duration::from_millis(100)) => {
            tracing::debug!("Canceling request future");
        }
        _ = future => {}
    }

    // The response should be sent to the events stream

    let Some(event) = events.next().await else {
        panic!("No event received");
    };

    let Event::Command(command) = event else {
        panic!("Expected command event, got {:?}", event);
    };

    assert!(matches!(command.id(), CommandId::UnbindResp));
    assert_eq!(command.sequence_number(), 2);

    let _ = client.terminated().await;
}

#[tokio::test]
async fn request_after_closing() {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        Server::new().run(server).await;
    });

    let (client, _) = ConnectionBuilder::new()
        .connected(client)
        .await
        .expect("Failed to connect");

    client.unbind().await.expect("Failed to unbind");

    let error = client
        .submit_sm(SubmitSm::builder().build())
        .await
        .unwrap_err();

    assert!(matches!(error, Error::ConnectionClosed));

    let _ = client.terminated().await;
}

/// Enquire link timeout should close the connection.
#[tokio::test]
async fn enquire_link_timeout() {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        Server::new()
            .enquire_link_delay(Duration::from_secs(5))
            .run(server)
            .await;
    });

    let (client, _) = ConnectionBuilder::new()
        // Send enquire link every 2 seconds
        .enquire_link_interval(Duration::from_secs(2))
        // Wait for 1 second for the response
        .response_timeout(Duration::from_secs(1))
        .connected(client)
        .await
        .expect("Failed to connect");

    let _ = client.terminated().await;
}

#[tokio::test]
async fn server_crash_on_request() {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        Server::new().run(server).await;
    });

    let (client, _) = ConnectionBuilder::new()
        .connected(client)
        .await
        .expect("Failed to connect");

    client
        .generic_nack(1)
        .await
        .expect("Failed to send generic_nack");

    let _ = client.terminated().await;
}

#[tokio::test]
async fn connection_lost() {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        Server::new()
            .close_connection_delay(Duration::from_secs(1))
            .run(server)
            .await;
    });

    let (client, _) = ConnectionBuilder::new()
        .connected(client)
        .await
        .expect("Failed to connect");

    tokio::time::sleep(Duration::from_secs(2)).await;

    let error = client
        .submit_sm(SubmitSm::builder().build())
        .await
        .unwrap_err();

    assert!(matches!(error, Error::ConnectionClosed));

    let _ = client.terminated().await;
}

#[tokio::test]
async fn server_wants_unbind() {
    init_tracing();

    let (server, client) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        UnbindServer::new(Duration::from_secs(1)).run(server).await;
    });

    let (client, _) = ConnectionBuilder::new()
        .connected(client)
        .await
        .expect("Failed to connect");

    let _ = client.terminated().await;
}

#[tokio::test]
#[ignore = "Just to see the connection ids"]
async fn multiple() {
    init_tracing();

    let mut tasks = vec![];

    for _ in 0..10 {
        let task = tokio::spawn(async move {
            let (server, client) = tokio::io::duplex(1024);

            tokio::spawn(async move {
                Server::new()
                    .bind_delay(Duration::from_millis(200))
                    .response_delay(Duration::from_secs(1))
                    .run(server)
                    .await;
            });

            let (client, _) = ConnectionBuilder::new()
                .connected(client)
                .await
                .expect("Failed to connect");

            client
                .submit_sm(SubmitSm::builder().build())
                .await
                .expect("Failed to submit_sm");

            client.unbind().await.expect("Failed to unbind");

            let _ = client.terminated().await;
        });

        tasks.push(task);
    }

    for task in tasks {
        let _ = task.await;
    }
}
