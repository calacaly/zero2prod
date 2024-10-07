use super::router;
use tracing_subscriber::EnvFilter;
pub async fn run() -> std::io::Result<()> {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("debug"))
        .init();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, router::new()).await?;
    Ok(())
}
