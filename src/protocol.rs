use crate::configuration::get_configuration;
use std::process::Command;

/// Update protocol parameters
///
/// cardano-cli query protocol-parameters \
/// --testnet-magic 1097911063 \
/// --out-file protocol.json
pub async fn update_protocol_parameters() -> bool {
    let config = get_configuration().expect("Failed to load configuration");

    let mut arguments = vec!["query", "protocol-parameters"];

    if config.node.environment.as_str() == "testnet" {
        arguments.push("--testnet-magic");
        arguments.push(&config.node.magic_number);
    }

    arguments.push("--out-file");
    arguments.push("protocol.json");

    let output = Command::new("cardano-cli")
        .args(arguments)
        .output()
        .expect("failed to execute process");

    output.status.success()
}

/// Generate verification key and signning key
/// cardano-cli address key-gen \
/// --verification-key-file policy/policy.vkey \
/// --signing-key-file policy/policy.skey
pub async fn generate_verification_key_and_signing_key() -> bool {
    let mut arguments = vec!["address", "key-gen"];

    arguments.push("--verification-key-file");
    arguments.push("policy/policy.vkey");

    arguments.push("--signing-key-file");
    arguments.push("policy/policy.skey");

    let output = Command::new("cardano-cli")
        .args(arguments)
        .output()
        .expect("failed to execute process");

    output.status.success()
}

/// Generate key hash
/// cardano-cli address key-hash --payment-verification-key-file policy/policy.vkey
pub async fn generate_key_hash() -> bool {
    let mut arguments = vec!["address", "key-hash"];

    arguments.push("--payment-verification-key-file");
    arguments.push("policy/policy.vkey");

    let output = Command::new("cardano-cli")
        .args(arguments)
        .output()
        .expect("failed to execute process");

    output.status.success()
}

/// Generate Policy ID
/// cardano-cli transaction policyid \
/// --script-file ./policy/policy.script >> policy/policyID
pub async fn generate_policy_id() -> bool {
    let mut arguments = vec!["transaction", "policyid"];

    arguments.push("--script-file");
    arguments.push("policy/policy.script");

    arguments.push(">>");
    arguments.push("policy/policyID");

    let output = Command::new("cardano-cli")
        .args(arguments)
        .output()
        .expect("failed to execute process");

    output.status.success()
}
