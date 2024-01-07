mod handle_clients;
mod handle_data;
mod service;

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use warp::Filter;

use crate::service::Server;
use autometrics::prometheus_exporter;
use client_manager::ClientManager;
use common::prelude::ServiceID;
use config_manager::ConfigManager;
use service_utils::{print_utils, shutdown_utils};

const SVC_ID: ServiceID = ServiceID::QDGW;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the metrics exporter.
    prometheus_exporter::init();

    // Setup ConfigManager instance for contextual autoconfiguration.
    let cfg_manager = async { ConfigManager::new(SVC_ID) }.await;

    // Configure the metrics endpoint.
    let metric_config = cfg_manager.get_svc_metric_config();
    let metrics_host = metric_config.metric_host();
    let metrics_port = metric_config.metric_port();
    let metrics_uri = metric_config.metric_uri();
    let metrics_addr = format!("{}:{}", metrics_host, metrics_port);

    //Creates a SocketAddr instance from the metrics address string.
    let web_addr: SocketAddr = metrics_addr
        .parse()
        .expect("[QDGW]/main: Failed to parse metric host to address");

    //Creates a new Warp filter for the metrics endpoint.
    let routes = warp::get()
        .and(warp::path(metrics_uri.clone()))
        .map(prometheus_exporter::encode_http_response);

    //Creates a new Warp filter for the metrics endpoint with a graceful shutdown handler.
    let signal = shutdown_utils::signal_handler("Http web server");
    let (_, web_server) = warp::serve(routes).bind_with_graceful_shutdown(web_addr, signal);

    //Creates a new Tokio task for the HTTP web server.
    let web_handle = tokio::spawn(web_server);

    // Autoconfigures message channel
    let msg_config = cfg_manager.get_message_client_config();
    let service_topic = msg_config.control_channel();

    // creates a new consumer for the topic
    let consumer = fluvio::consumer(&service_topic, 0)
        .await
        .expect("[QDGW]/main: Failed to create a message consumer");


    // We have to use Arc/Mutex here to allow multi-threaded access those manager instances.
    let client_manager = Arc::new(Mutex::new(ClientManager::new()));

    // let db_config = cfg_manager.get_db_config();

    // NEEDS FIX: PG connection needs to be fully tokio async.
    //

    // let query_manager =
    //     async { Arc::new(Mutex::new(QueryDBManager::new(db_config.clone()))) }.await;
    //
    // // SymbolManager is fallible because it reads from the DB during instantiation.
    // let symbol_manager = async {
    //     Arc::new(Mutex::new(
    //         SymbolManager::new(db_config.clone())
    //             .expect("[QDGW]/main: Failed to create SymbolManager instance."),
    //     ))
    // }.await;

    //Creates a new server
    let server = Server::new(
        consumer,
        client_manager,
        // query_manager,
        // symbol_manager
    );

    //Creates a new Tokio task for the server.
    let signal = shutdown_utils::signal_handler("Fluvio connector");
    let service_handle = tokio::spawn(server.run(signal));

    // Prints the start header for the service
    print_utils::print_start_header_message_service(
        &SVC_ID,
        &service_topic,
        &metrics_addr,
        &metrics_uri,
    );

    //Starts both servers concurrently.
    match tokio::try_join!(web_handle, service_handle) {
        Ok(_) => {}
        Err(e) => {
            println!("[QDGW]/main: Failed to start gRPC and HTTP server: {:?}", e);
        }
    }

    //Prints the stop headers for the current service.
    print_utils::print_stop_header(&SVC_ID);

    Ok(())
}
