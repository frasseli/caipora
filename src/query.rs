use crate::configuration::get_configuration;
use actix_web::{web, HttpRequest, Responder};
use std::process::Command;

pub async fn node_query_tip(_req: HttpRequest) -> impl Responder {
    let config = get_configuration().expect("Failed to load configuration");

    let mut arguments = vec!["query", "tip"];
    if config.node.environment.as_str() == "testnet" {
        arguments.push("--testnet-magic");
        arguments.push(&config.node.magic_number);
    }

    let output = Command::new("cardano-cli")
        .args(arguments)
        .output()
        .expect("failed to execute process");

    String::from_utf8(output.stdout).unwrap()
}

pub async fn node_query_utxo(_req: HttpRequest, address: web::Path<String>) -> impl Responder {
    let config = get_configuration().expect("Failed to load configuration");

    let mut arguments = vec!["query", "utxo", "--address", &address.as_str(), "--out-file=/dev/stdout"];
    if config.node.environment.as_str() == "testnet" {
        arguments.push("--testnet-magic");
        arguments.push(&config.node.magic_number);
    }

    let output = Command::new("cardano-cli")
        .args(arguments)
        .output()
        .expect("failed to execute process");

    String::from_utf8(output.stdout).unwrap()
}
