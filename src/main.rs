use actix_web::{web, App, HttpRequest, HttpServer, Responder, http::StatusCode};
use std::process::Command;

async fn index(_req: HttpRequest) -> impl Responder {
    "<h1>This is the Index Page from the CAIPORA Service!</h1><br><p>Please read the docs to know how to run all the commands.</p>"
    .with_status(StatusCode::OK)
    .with_header("content-type", "text/html; charset=utf-8")
}

async fn run_node_cmd(cmd: web::Path<String>) -> impl Responder {
    let output = Command::new("cardano-cli")
        .arg(&cmd.as_str())
        .arg("--testnet-magic")
        .arg("1097911063")
        .output()
        .expect("failed to execute process");

    String::from_utf8(output.stdout).unwrap()
}

async fn run_node_query_tip(_req: HttpRequest) -> impl Responder {
    let output = Command::new("cardano-cli")
        .arg("query")
        .arg("tip")
        .arg("--testnet-magic")
        .arg("1097911063")
        .output()
        .expect("failed to execute process");

    String::from_utf8(output.stdout).unwrap()
}

async fn run_node_query_utxo(_req: HttpRequest, address: web::Path<String>) -> impl Responder {
    let output = Command::new("cardano-cli")
        .arg("query")
        .arg("utxo")
        .arg("address")
        .arg(&address.as_str())
        .arg("--testnet-magic")
        .arg("1097911063")
        .output()
        .expect("failed to execute process");

    String::from_utf8(output.stdout).unwrap()
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/tip", web::get().to(run_node_query_tip))
            .route("/utxo/{address}", web::get().to(run_node_query_utxo))
            .route("/cmd/{cmd}", web::get().to(run_node_cmd))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
