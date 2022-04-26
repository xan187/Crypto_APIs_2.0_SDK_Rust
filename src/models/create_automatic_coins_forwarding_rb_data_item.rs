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
pub struct CreateAutomaticCoinsForwardingRbDataItem {
    /// Represents the Secret Key value provided by the customer. This field is used for security purposes during the callback notification, in order to prove the sender of the callback as Crypto APIs. For more information please see our [Documentation](https://developers.cryptoapis.io/technical-documentation/general-information/callbacks#callback-security).
    #[serde(rename = "callbackSecretKey")]
    pub callback_secret_key: String,
    /// Represents the URL that is set by the customer where the callback will be received at. The callback notification will be received only if and when the event occurs. `We support ONLY httpS type of protocol`.
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
    /// Represents the number of confirmations, i.e. the amount of blocks that have been built on top of this block.
    #[serde(rename = "confirmationsCount")]
    pub confirmations_count: i32,
    /// Represents the fee priority of the automation, whether it is \"slow\", \"standard\" or \"fast\".
    #[serde(rename = "feePriority")]
    pub fee_priority: FeePriority,
    /// Represents the minimum transfer amount of the currency in the `fromAddress` that can be allowed for an automatic forwarding.
    #[serde(rename = "minimumTransferAmount")]
    pub minimum_transfer_amount: String,
    /// Represents the hash of the address the currency is forwarded to.
    #[serde(rename = "toAddress")]
    pub to_address: String,
}

impl CreateAutomaticCoinsForwardingRbDataItem {
    pub fn new(callback_secret_key: String, callback_url: String, confirmations_count: i32, fee_priority: FeePriority, minimum_transfer_amount: String, to_address: String) -> CreateAutomaticCoinsForwardingRbDataItem {
        CreateAutomaticCoinsForwardingRbDataItem {
            callback_secret_key,
            callback_url,
            confirmations_count,
            fee_priority,
            minimum_transfer_amount,
            to_address,
        }
    }
}

/// Represents the fee priority of the automation, whether it is \"slow\", \"standard\" or \"fast\".
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FeePriority {
    #[serde(rename = "slow")]
    Slow,
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "fast")]
    Fast,
}

