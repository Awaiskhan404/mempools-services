#![allow(clippy::await_holding_refcell_ref)]

use alert_service::AlertService;
use auth_service::AuthService;
use chain_service::ChainService;
use crawler_service::CrawlerService;
use db_migration::{Migrator, MigratorTrait};
use destination_service::DestinationService;
use filter_service::FilterService;
use gateway_service::GatewayService;
use mempools_api::api::{
    gateway_admin_server::GatewayAdminServer, gateway_public_server::GatewayPublicServer,
    gateway_server::GatewayServer,
};
use notification_service::NotificationService;
use sea_orm::{ConnectOptions, Database};
use server::{config::Config, interceptors::AdminInterceptor};
use std::time::Duration;

use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tower_http::catch_panic::CatchPanicLayer;
use util::{
    service_registry::{RegisteryServices, ServiceRegistery},
    Result,
};

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let config = Config::from_env_or_default()?;
    let registery = ServiceRegistery::new();
    let db = Database::connect(
        ConnectOptions::new(config.application_config.db_url.clone())
            .sqlx_logging(false)
            .clone(),
    )
    .await?;

    // Set up database
    Migrator::up(&db, None).await?;
    #[cfg(feature = "dev")]
    server::test_data::add_test_data(&db).await?;

    let gateway_service = GatewayService::new(registery.clone());
    let crawler_service = CrawlerService::new(db.clone(), registery.clone());
    let filter_service = FilterService::new(registery.clone())?;
    let alert_service = AlertService::new(db.clone());
    let chain_service = ChainService::new(db.clone());
    let auth_service = AuthService::new(config.auth_config.clone());
    let destination_service = DestinationService::new(registery.clone(), db.clone());
    let notification_service =
        NotificationService::new(config.notification_config, registery.clone(), db.clone())?;

    let svcs = RegisteryServices {
        filter_service: Box::new(filter_service.clone()),
        alert_service: Box::new(alert_service.clone()),
        notification_service: Box::new(notification_service.clone()),
        destination_service: Box::new(destination_service.clone()),
        chain_service: Box::new(chain_service.clone()),
        auth_service: Box::new(auth_service.clone()),
    };
    registery.register_services(svcs).await;

    // Daemons
    notification_service.spawn_daemons();
    crawler_service.spawn_daemons();

    // Create server
    let server = Server::builder()
        .accept_http1(true)
        .timeout(Duration::from_secs(7));

    // Add Layers

    #[cfg(not(feature = "cors"))]
    let server = server.layer(tower_http::cors::CorsLayer::very_permissive().allow_origin(
        tower_http::cors::AllowOrigin::predicate(|origin, _| {
            origin.as_bytes().ends_with(b".mempools.com")
        }),
    ));

    #[cfg(feature = "cors")]
    let server = server.layer(tower_http::cors::CorsLayer::very_permissive());

    let server = server.layer(CatchPanicLayer::new());
    let mut server = server.layer(GrpcWebLayer::new());

    // Add Services

    #[cfg(feature = "reflection")]
    server.add_service(
        tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(mempools_api::FILE_DESCRIPTOR_SET)
            .build()?,
    );

    let auth_interceptor;
    #[cfg(feature = "auth")]
    {
        auth_interceptor = server::interceptors::AuthInterceptor::new(
            config
                .auth_config
                .cognito_config
                .ok_or("need cognito config for auth feature")?,
        )
        .await?;
    }
    #[cfg(not(feature = "auth"))]
    {
        auth_interceptor = server::interceptors::MockIdTokenSetter::new("1");
    }

    let gateway_public_server = GatewayPublicServer::new(gateway_service.clone());
    let gateway_server = tower::ServiceBuilder::new()
        .layer(tonic::service::interceptor(auth_interceptor.clone()))
        .service(GatewayServer::new(gateway_service.clone()));
    let gateway_admin_server = tower::ServiceBuilder::new()
        .layer(tonic::service::interceptor(auth_interceptor))
        .layer(tonic::service::interceptor(AdminInterceptor::new(
            config.application_config.admins,
        )))
        .service(GatewayAdminServer::new(gateway_service.clone()));

    let server = server
        .add_service(gateway_server)
        .add_service(gateway_public_server)
        .add_service(gateway_admin_server);

    let addr = "0.0.0.0:8123".parse()?;
    tokio::task::spawn(async move { server.serve(addr).await }).await??;
    info!("started mempools-server on port 8123...");

    Ok(())
}
