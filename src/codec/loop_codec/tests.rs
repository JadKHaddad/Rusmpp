use std::net::TcpStream;

use crate::codec::loop_codec::LoopCodec;

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

    let stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect");

    println!("Connected to server");

    let mut loop_codec = LoopCodec::<2, _>::new(&stream);

    loop {
        match loop_codec.try_decode() {
            Ok(Some(command)) => {
                println!("Received command: {:?}", command);
            }
            Ok(None) => {}
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
