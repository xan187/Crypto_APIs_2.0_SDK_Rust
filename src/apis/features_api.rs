/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `broadcast_locally_signed_transaction`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BroadcastLocallySignedTransactionError {
    Status400(crate::models::InlineResponse40099),
    Status401(crate::models::InlineResponse40199),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40399),
    Status404(crate::models::InlineResponse4041),
    Status409(crate::models::InlineResponse40917),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `decode_raw_transaction_hex`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DecodeRawTransactionHexError {
    Status400(crate::models::InlineResponse400102),
    Status401(crate::models::InlineResponse401102),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse403102),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `decode_x_address`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DecodeXAddressError {
    Status400(crate::models::InlineResponse400103),
    Status401(crate::models::InlineResponse401103),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse403103),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `derive_hd_wallet__x_pub_y_pub_z_pub_change_or_receiving_addresses`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeriveHdWalletXPubYPubZPubChangeOrReceivingAddressesError {
    Status400(crate::models::InlineResponse40070),
    Status401(crate::models::InlineResponse40170),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40370),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `encode_x_address`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EncodeXAddressError {
    Status400(crate::models::InlineResponse400104),
    Status401(crate::models::InlineResponse401104),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse403104),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `estimate_gas_limit`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EstimateGasLimitError {
    Status400(crate::models::InlineResponse400100),
    Status401(crate::models::InlineResponse401100),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse403100),
    Status404(crate::models::InlineResponse4041),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `estimate_token_gas_limit`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EstimateTokenGasLimitError {
    Status400(crate::models::InlineResponse400101),
    Status401(crate::models::InlineResponse401101),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse403101),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_eip_1559_fee_recommendations`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEip1559FeeRecommendationsError {
    Status400(crate::models::InlineResponse40098),
    Status401(crate::models::InlineResponse40198),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40398),
    Status404(crate::models::InlineResponse4041),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `validate_address`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValidateAddressError {
    Status400(crate::models::InlineResponse40097),
    Status401(crate::models::InlineResponse40197),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40397),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}


/// Through this endpoint customers can broadcast transactions that have been already signed locally. Instead of using a node for broadcasting a signed transaction users can use this endpoint. We then keep the user posted about the status by sending you a callback with a success or failure status.    {warning}This can be prepared and signed **only** locally, not through the API. We can provide support only for the process of broadcasting.{/warning}
pub async fn broadcast_locally_signed_transaction(configuration: &configuration::Configuration, blockchain: &str, network: &str, context: Option<&str>, broadcast_locally_signed_transaction_rb: Option<crate::models::BroadcastLocallySignedTransactionRb>) -> Result<crate::models::BroadcastLocallySignedTransactionR, Error<BroadcastLocallySignedTransactionError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-tools/{blockchain}/{network}/transactions/broadcast", configuration.base_path, blockchain=crate::apis::urlencode(blockchain), network=crate::apis::urlencode(network));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = context {
        local_var_req_builder = local_var_req_builder.query(&[("context", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&broadcast_locally_signed_transaction_rb);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BroadcastLocallySignedTransactionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint customers can decode a raw transaction hex and see the decoded transactions' details.
pub async fn decode_raw_transaction_hex(configuration: &configuration::Configuration, blockchain: &str, network: &str, context: Option<&str>, decode_raw_transaction_hex_rb: Option<crate::models::DecodeRawTransactionHexRb>) -> Result<crate::models::DecodeRawTransactionHexR, Error<DecodeRawTransactionHexError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-tools/{blockchain}/{network}/decode-raw-transaction", configuration.base_path, blockchain=crate::apis::urlencode(blockchain), network=crate::apis::urlencode(network));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = context {
        local_var_req_builder = local_var_req_builder.query(&[("context", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&decode_raw_transaction_hex_rb);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DecodeRawTransactionHexError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint, customers can decode an encoded XRP address with tag, by providing the specific x-address. The response includes the decoded classic address and the tag.
pub async fn decode_x_address(configuration: &configuration::Configuration, blockchain: &str, network: &str, x_address: &str, context: Option<&str>) -> Result<crate::models::DecodeXAddressR, Error<DecodeXAddressError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-tools/{blockchain}/{network}/decode-x-address/{xAddress}", configuration.base_path, blockchain=crate::apis::urlencode(blockchain), network=crate::apis::urlencode(network), xAddress=crate::apis::urlencode(x_address));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = context {
        local_var_req_builder = local_var_req_builder.query(&[("context", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DecodeXAddressError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint, customers can derive up to 10 addresses - both change and receive, from a certain HD Wallet (xPub, yPub, zPub), by providing an extended public key. By default the system creates a receiving/deposit address, unless the isChange attribute is set to 'true'. In that case the system derives a 'change' address. The change address can be derived only for UTXO based blockchains, for all the rest, this endpoint always creates a deposit/receiving address.
pub async fn derive_hd_wallet__x_pub_y_pub_z_pub_change_or_receiving_addresses(configuration: &configuration::Configuration, blockchain: &str, extended_public_key: &str, network: &str, context: Option<&str>, address_format: Option<&str>, addresses_count: Option<i32>, is_change: Option<bool>, start_index: Option<i32>) -> Result<crate::models::DeriveHdWalletXPubYPubZPubChangeOrReceivingAddressesR, Error<DeriveHdWalletXPubYPubZPubChangeOrReceivingAddressesError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-tools/{blockchain}/{network}/hd/{extendedPublicKey}/addresses/derive-address", configuration.base_path, blockchain=crate::apis::urlencode(blockchain), extendedPublicKey=crate::apis::urlencode(extended_public_key), network=crate::apis::urlencode(network));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = context {
        local_var_req_builder = local_var_req_builder.query(&[("context", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = address_format {
        local_var_req_builder = local_var_req_builder.query(&[("addressFormat", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = addresses_count {
        local_var_req_builder = local_var_req_builder.query(&[("addressesCount", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_change {
        local_var_req_builder = local_var_req_builder.query(&[("isChange", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start_index {
        local_var_req_builder = local_var_req_builder.query(&[("startIndex", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeriveHdWalletXPubYPubZPubChangeOrReceivingAddressesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint, customers can encode an encoded XRP address with tag, by providing the specific x-address. The response includes the encoded classic address and the tag.
pub async fn encode_x_address(configuration: &configuration::Configuration, address_tag: i32, blockchain: &str, classic_address: &str, network: &str, context: Option<&str>) -> Result<crate::models::EncodeXAddressR, Error<EncodeXAddressError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-tools/{blockchain}/{network}/encode-x-address/{classicAddress}/{addressTag}", configuration.base_path, addressTag=address_tag, blockchain=crate::apis::urlencode(blockchain), classicAddress=crate::apis::urlencode(classic_address), network=crate::apis::urlencode(network));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = context {
        local_var_req_builder = local_var_req_builder.query(&[("context", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<EncodeXAddressError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This endpoint helps customer in estimating the gas limit needed for a transaction. It gives information for gas expenses when sending ether to contracts or making a transaction with additional data in it.
pub async fn estimate_gas_limit(configuration: &configuration::Configuration, blockchain: &str, network: &str, context: Option<&str>, estimate_gas_limit_rb: Option<crate::models::EstimateGasLimitRb>) -> Result<crate::models::EstimateGasLimitR, Error<EstimateGasLimitError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-tools/{blockchain}/{network}/gas-limit", configuration.base_path, blockchain=crate::apis::urlencode(blockchain), network=crate::apis::urlencode(network));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = context {
        local_var_req_builder = local_var_req_builder.query(&[("context", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&estimate_gas_limit_rb);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<EstimateGasLimitError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This endpoint helps customer in estimating the Contract Gas Limit needed for a transaction. It gives information for gas expenses for a specific contract when sending ethers or making a transaction with additional data in it.
pub async fn estimate_token_gas_limit(configuration: &configuration::Configuration, blockchain: &str, network: &str, context: Option<&str>, estimate_token_gas_limit_rb: Option<crate::models::EstimateTokenGasLimitRb>) -> Result<crate::models::EstimateTokenGasLimitR, Error<EstimateTokenGasLimitError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-tools/{blockchain}/{network}/gas-limit/contract", configuration.base_path, blockchain=crate::apis::urlencode(blockchain), network=crate::apis::urlencode(network));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = context {
        local_var_req_builder = local_var_req_builder.query(&[("context", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&estimate_token_gas_limit_rb);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<EstimateTokenGasLimitError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint customers can obtain fee recommendations specifically for EIP 1559.
pub async fn get_eip_1559_fee_recommendations(configuration: &configuration::Configuration, network: &str, blockchain: &str, context: Option<&str>) -> Result<crate::models::GetEip1559FeeRecommendationsR, Error<GetEip1559FeeRecommendationsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-tools/{blockchain}/{network}/fees/eip1559", configuration.base_path, network=crate::apis::urlencode(network), blockchain=crate::apis::urlencode(blockchain));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = context {
        local_var_req_builder = local_var_req_builder.query(&[("context", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetEip1559FeeRecommendationsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This endpoint checks user public addresses whether they are valid or not.
pub async fn validate_address(configuration: &configuration::Configuration, blockchain: &str, network: &str, context: Option<&str>, validate_address_rb: Option<crate::models::ValidateAddressRb>) -> Result<crate::models::ValidateAddressR, Error<ValidateAddressError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-tools/{blockchain}/{network}/addresses/validate", configuration.base_path, blockchain=crate::apis::urlencode(blockchain), network=crate::apis::urlencode(network));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = context {
        local_var_req_builder = local_var_req_builder.query(&[("context", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&validate_address_rb);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ValidateAddressError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

