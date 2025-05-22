pub fn init_tracing() {
    _ = tracing_subscriber::fmt()
        .with_env_filter("rusmppc=trace,rusmpp=debug")
        .try_init();
}

#[tokio::test]
async fn bind() {
    init_tracing();
}
