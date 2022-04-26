/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateAutomaticTokensForwardingRi {
    /// Represents the URL that is set by the customer where the callback will be received at. The callback notification will be received only if and when the event occurs. `We support ONLY httpS type of protocol`.
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
    /// Represents the number of confirmations, i.e. the amount of blocks that have been built on top of this block.
    #[serde(rename = "confirmationsCount")]
    pub confirmations_count: i32,
    /// Defines the specific time/date when the automatic forwarding was created in Unix Timestamp.
    #[serde(rename = "createdTimestamp")]
    pub created_timestamp: i32,
    /// Represents the specific fee address, which is always automatically generated. Users must fund it.
    #[serde(rename = "feeAddress")]
    pub fee_address: String,
    /// Represents the fee priority of the automation, whether it is \"SLOW\", \"STANDARD\" or \"FAST\".
    #[serde(rename = "feePriority")]
    pub fee_priority: FeePriority,
    /// Represents the hash of the address that forwards the tokens.
    #[serde(rename = "fromAddress")]
    pub from_address: String,
    /// Represents the minimum transfer amount of the tokens in the `fromAddress` that can be allowed for an automatic forwarding.
    #[serde(rename = "minimumTransferAmount")]
    pub minimum_transfer_amount: String,
    /// Represents a unique ID used to reference the specific callback subscription.
    #[serde(rename = "referenceId")]
    pub reference_id: String,
    /// Represents the hash of the address the tokens are forwarded to.
    #[serde(rename = "toAddress")]
    pub to_address: String,
    #[serde(rename = "tokenData")]
    pub token_data: Box<crate::models::CreateAutomaticTokensForwardingRits>,
}

impl CreateAutomaticTokensForwardingRi {
    pub fn new(callback_url: String, confirmations_count: i32, created_timestamp: i32, fee_address: String, fee_priority: FeePriority, from_address: String, minimum_transfer_amount: String, reference_id: String, to_address: String, token_data: crate::models::CreateAutomaticTokensForwardingRits) -> CreateAutomaticTokensForwardingRi {
        CreateAutomaticTokensForwardingRi {
            callback_url,
            confirmations_count,
            created_timestamp,
            fee_address,
            fee_priority,
            from_address,
            minimum_transfer_amount,
            reference_id,
            to_address,
            token_data: Box::new(token_data),
        }
    }
}

/// Represents the fee priority of the automation, whether it is \"SLOW\", \"STANDARD\" or \"FAST\".
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FeePriority {
    #[serde(rename = "slow")]
    Slow,
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "fast")]
    Fast,
}

