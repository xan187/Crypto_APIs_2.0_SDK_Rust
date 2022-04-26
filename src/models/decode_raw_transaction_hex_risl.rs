/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// DecodeRawTransactionHexRisl : Litecoin



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DecodeRawTransactionHexRisl {
    /// Represents the time at which a particular transaction can be added to the blockchain
    #[serde(rename = "locktime")]
    pub locktime: i32,
    /// Represents the same as transactionId for account-based protocols like Ethereum, while it could be different in UTXO-based protocols like Bitcoin. E.g., in UTXO-based protocols hash is different from transactionId for SegWit transactions.
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    /// Represents the virtual size of this transaction.
    #[serde(rename = "vSize")]
    pub v_size: i32,
    /// Represents transaction version number.
    #[serde(rename = "version")]
    pub version: i32,
    /// Represents the transaction inputs.
    #[serde(rename = "vin")]
    pub vin: Vec<crate::models::DecodeRawTransactionHexRislVin>,
    /// Represents the transaction outputs.
    #[serde(rename = "vout")]
    pub vout: Vec<crate::models::DecodeRawTransactionHexRislVout>,
    /// Represents the size of a block, measured in weight units and including the segwit discount.
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

impl DecodeRawTransactionHexRisl {
    /// Litecoin
    pub fn new(locktime: i32, transaction_hash: String, v_size: i32, version: i32, vin: Vec<crate::models::DecodeRawTransactionHexRislVin>, vout: Vec<crate::models::DecodeRawTransactionHexRislVout>) -> DecodeRawTransactionHexRisl {
        DecodeRawTransactionHexRisl {
            locktime,
            transaction_hash,
            v_size,
            version,
            vin,
            vout,
            weight: None,
        }
    }
}


