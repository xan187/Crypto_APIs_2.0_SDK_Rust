/*
 * CryptoAPIs
 *
 * Crypto APIs is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2021-03-20
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockHeightReachedRi {
    /// Represents the specified value of block height for which the callback will be received.
    #[serde(rename = "blockHeightReached")]
    pub block_height_reached: i64,
    /// Represents the Secret Key value provided by the customer. This field is used for security purposes during the callback notification, in order to prove the sender of the callback as Crypto APIs. For more information please see our [Documentation](https://developers.cryptoapis.io/technical-documentation/general-information/callbacks#callback-security).
    #[serde(rename = "callbackSecretKey")]
    pub callback_secret_key: String,
    /// Represents the URL that is set by the customer where the callback will be received at. The callback notification will be received only if and when the event occurs. We support ONLY httpS type of protocol.
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
    /// Defines the specific time/date when the subscription was created in Unix Timestamp.
    #[serde(rename = "createdTimestamp")]
    pub created_timestamp: i32,
    /// Defines whether the subscription is active or not. Set as boolean.
    #[serde(rename = "isActive")]
    pub is_active: bool,
    /// Represents a unique ID used to reference the specific callback subscription.
    #[serde(rename = "referenceId")]
    pub reference_id: String,
}

impl BlockHeightReachedRi {
    pub fn new(block_height_reached: i64, callback_secret_key: String, callback_url: String, created_timestamp: i32, is_active: bool, reference_id: String) -> BlockHeightReachedRi {
        BlockHeightReachedRi {
            block_height_reached,
            callback_secret_key,
            callback_url,
            created_timestamp,
            is_active,
            reference_id,
        }
    }
}

