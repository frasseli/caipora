use actix_web::{http::StatusCode, web, App, HttpRequest, HttpServer, Responder};
use caipora::{configuration::get_configuration, protocol::update_protocol_parameters};
use caipora::query::{node_query_tip, node_query_utxo};

async fn index(_req: HttpRequest) -> impl Responder {
    "<h1>This is the Index Page from the CAIPORA Service!</h1><br><p>Please read the docs to know how to run all the commands.</p>"
    .with_status(StatusCode::OK)
    .with_header("content-type", "text/html; charset=utf-8")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");

    println!("Updating protocol parameters...");
    let protocol_update = update_protocol_parameters().await;
    if protocol_update {
        println!("Parameters updated in file protocol.json");
    }

    println!(
        "Caipora is watching over {}:{}",
        configuration.application.host, configuration.application.port
    );

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/tip", web::get().to(node_query_tip))
            .route("/utxo/{address}", web::get().to(node_query_utxo))
    })
    .bind((
        configuration.application.host,
        configuration.application.port,
    ))?
    .run()
    .await
}
