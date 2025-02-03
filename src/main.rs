mod app_args;
mod lua;
mod run;
mod server;
mod server_config;
mod test;

use crate::app_args::Commands;
use app_args::AppArgs;
use clap::Parser;
use glob::glob;
use log::{debug, info};
use opentelemetry::KeyValue;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::{ExportConfig, WithExportConfig};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::{RandomIdGenerator, Sampler, TracerProvider};
use tracing::span;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

#[actix_web::main]
async fn main() {
    let resource = Resource::new(vec![
        KeyValue::new("service.name", "redox"), // Set your service name here
        KeyValue::new("service.version", "1.0.0"),
    ]);
    
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_export_config(ExportConfig::default())
        .build()
        .expect("Failed to create OTLP exporter");

    let tracer = TracerProvider::builder()
        .with_batch_exporter(exporter, opentelemetry_sdk::runtime::Tokio)
        .with_id_generator(RandomIdGenerator::default())
        .with_sampler(Sampler::AlwaysOn)
        .with_resource(resource)
        .build();

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer.tracer("redox"));
    let subscriber = Registry::default().with(telemetry);

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let args = AppArgs::parse();

    log4rs::init_file("logger.yml", Default::default()).unwrap();

    match args.command {
        Commands::Serve { port } => {
            let config = server_config::ServerConfig::new().expect("Failed to load server config");
            debug!("App Config: {:?}", &config);
            server::serve(
                config.entrypoint,
                config.shutdown_timeout,
                config.address,
                port.unwrap_or(config.port),
            )
            .await;
        }
        Commands::Run { entrypoint } => {
            info!("Running Lua script: {}", &entrypoint);
            span!(tracing::Level::INFO, "run", entrypoint = &entrypoint)
                .in_scope(|| async {
                    run::run(&entrypoint).await;
                })
                .await;
        }
        Commands::Test { include } => {
            info!("Running tests: {}", &include);

            let files = glob(&include).expect("Failed to read glob pattern");

            for file in files {
                match file {
                    Ok(path) => {
                        let path = path.to_str().unwrap().to_string();
                        span!(tracing::Level::INFO, "test", path = &path)
                            .in_scope(|| async {
                                info!("Running test: {}", &path);
                                test::run(&path).await;
                            })
                            .await;
                    }
                    Err(e) => {
                        info!("Failed to read file: {}", e);
                    }
                }
            }
        }
    }

    debug!("Finished...");
}
