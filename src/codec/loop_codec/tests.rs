use std::net::TcpStream;

use crate::{codec::loop_codec::CommandLoopCodec, ende::decode::DecodeError};

// cargo test do_loop_codec --features tracing -- --ignored --nocapture
#[test]
#[ignore = "integration test"]
fn do_loop_codec() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var(
            "RUST_LOG",
            "rusmpp::codec::encode=trace,rusmpp::codec::decode=trace",
        );
    }

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect");
    stream
        .set_read_timeout(Some(std::time::Duration::from_millis(50)))
        .expect("Failed to set read timeout");
    stream
        .set_nonblocking(true)
        .expect("Failed to set non-blocking");

    let mut loop_codec = CommandLoopCodec::<4>::new();

    loop {
        match loop_codec.try_decode(&mut stream) {
            Ok(Some(command)) => {
                println!("Received command: {:?}", command);
            }
            Ok(None) => {}
            Err(DecodeError::IoError(err)) if err.kind() == std::io::ErrorKind::TimedOut => {}
            Err(DecodeError::IoError(err)) if err.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
