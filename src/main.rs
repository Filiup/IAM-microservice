mod common;
mod iam;
mod tags;

use common::{database::Database, helper::get_env_variable};
use iam::iam_controller::IamController;
use iam::iam_grpc_controller::{IamGrpcController, IamServiceServer};
use poem::middleware::Tracing;
use poem::{listener::TcpListener, EndpointExt, Result, Route};
use poem_grpc::RouteGrpc;
use poem_openapi::OpenApiService;
use tokio::signal::unix::{signal, SignalKind};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let rest_port = get_env_variable("MS_IAM_REST_PORT");
    let grpc_port = get_env_variable("MS_IAM_GRPC_PORT");
    let public_key_base64 = get_env_variable("JWT_ACCESS_PUBLIC_KEY");

    let api_service = OpenApiService::new(IamController, "ms-iam", "1.0")
        .server(format!("http://localhost:{}/ms-iam", rest_port));
    let ui = api_service.swagger_ui();

    let routes_rest = Route::new()
        .nest("/ms-iam", api_service)
        .nest("/ms-iam/docs", ui)
        .data(public_key_base64.clone());

    let routes_rpc = RouteGrpc::new()
        .add_service(IamServiceServer::new(IamGrpcController))
        .with(Tracing)
        .data(public_key_base64.clone());

    let server_rest = poem::Server::new(TcpListener::bind(format!("0.0.0.0:{}", rest_port)))
        .run_with_graceful_shutdown(routes_rest, sigint(), None);

    let server_grpc = poem::Server::new(TcpListener::bind(format!("0.0.0.0:{}", grpc_port)))
        .run_with_graceful_shutdown(routes_rpc, sigint(), None);

    let handle_rest = tokio::spawn(server_rest);
    let handle_grpc = tokio::spawn(server_grpc);

    let _ = tokio::try_join!(handle_rest, handle_grpc)?;

    Ok(())
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
