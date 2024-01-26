use anyhow::Result;
#[cfg(not(debug_assertions))]
use minerva_server::LambdaLayer;
use minerva_service::{get_router, AppConfig};
#[cfg(debug_assertions)]
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
pub async fn main() -> Result<()> {
    let app = get_router(AppConfig::default()).await;

    // debug build
    #[cfg(debug_assertions)]
    {
        setup_tracing(false);
        let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
        info!("Listening on: {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;
    }

    // release build
    #[cfg(not(debug_assertions))]
    {
        setup_tracing(false);

        let app = tower::ServiceBuilder::new()
            .layer(LambdaLayer::default())
            .service(app);

        info!("Starting lambda server");
        lambda_http::run(app).await.unwrap();
    }
    Ok(())
}

fn setup_tracing(json: bool) {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into());

    if json {
        tracing_subscriber::registry()
            .with(filter)
            .with(tracing_subscriber::fmt::layer().json())
            .init();
    } else {
        tracing_subscriber::registry()
            .with(filter)
            .with(tracing_subscriber::fmt::layer())
            .init();
    }
}
