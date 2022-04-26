/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// GetTransactionDetailsByTransactionIdribsbc : Bitcoin Cash



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetTransactionDetailsByTransactionIdribsbc {
    /// Represents the time at which a particular transaction can be added to the blockchain.
    #[serde(rename = "locktime")]
    pub locktime: i64,
    /// Represents the total size of this transaction.
    #[serde(rename = "size")]
    pub size: i32,
    /// Represents transaction version number.
    #[serde(rename = "version")]
    pub version: i32,
    /// Represents the transaction inputs.
    #[serde(rename = "vin")]
    pub vin: Vec<crate::models::GetTransactionDetailsByTransactionIdribsbcVin>,
    /// Represents the transaction outputs.
    #[serde(rename = "vout")]
    pub vout: Vec<crate::models::GetTransactionDetailsByTransactionIdribsbcVout>,
}

impl GetTransactionDetailsByTransactionIdribsbc {
    /// Bitcoin Cash
    pub fn new(locktime: i64, size: i32, version: i32, vin: Vec<crate::models::GetTransactionDetailsByTransactionIdribsbcVin>, vout: Vec<crate::models::GetTransactionDetailsByTransactionIdribsbcVout>) -> GetTransactionDetailsByTransactionIdribsbc {
        GetTransactionDetailsByTransactionIdribsbc {
            locktime,
            size,
            version,
            vin,
            vout,
        }
    }
}


