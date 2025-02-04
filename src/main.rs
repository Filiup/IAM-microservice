mod common;
mod iam;
mod tags;

use common::{database::Database, helper::get_env_variable};
use iam::iam_controller::IamController;

use poem::{listener::TcpListener, EndpointExt, Result, Route};
use poem_openapi::OpenApiService;
use tokio::signal::unix::{signal, SignalKind};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let port = get_env_variable("MS_IAM_REST_PORT");

    let api_service = OpenApiService::new(IamController, "ms-iam", "1.0")
        .server(format!("http://localhost:{}/ms-iam", port));
    let ui = api_service.swagger_ui();
    let server_key = get_env_variable("JWT_ACCESS_PUBLIC_KEY");

    let app = Route::new()
        .nest("/ms-iam", api_service)
        .nest("/ms-iam/docs", ui)
        .data(server_key);
    poem::Server::new(TcpListener::bind(format!("0.0.0.0:{}", port)))
        .run_with_graceful_shutdown(app, sigint(), None)
        .await
}

async fn sigint() {
    let mut sigterm = signal(SignalKind::terminate()).unwrap();
    let mut sigint = signal(SignalKind::interrupt()).unwrap();

    tokio::select! {
        _ = sigterm.recv() => {},
        _ = sigint.recv() => {}
    }

    // Letting PostgreSQL know that to close connections created by the pool
    Database::pg().await.close().await;
}
