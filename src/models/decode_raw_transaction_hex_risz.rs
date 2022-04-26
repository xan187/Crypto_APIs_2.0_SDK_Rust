/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// DecodeRawTransactionHexRisz : Zcash



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DecodeRawTransactionHexRisz {
    /// Represents a block height after which the transaction will expire.
    #[serde(rename = "expiryHeight")]
    pub expiry_height: i32,
    /// Represents the locktime on the transaction on the specific blockchain, i.e. the blockheight at which the transaction is valid.
    #[serde(rename = "locktime")]
    pub locktime: i32,
    /// \"Overwinter\" is the network upgrade for the Zcash blockchain.
    #[serde(rename = "overwintered")]
    pub overwintered: bool,
    /// Defines if the transaction includes sapling or not.
    #[serde(rename = "saplinged")]
    pub saplinged: bool,
    /// Represents the same as transactionId for account-based protocols like Ethereum, while it could be different in UTXO-based protocols like Bitcoin. E.g., in UTXO-based protocols hash is different from transactionId for SegWit transactions.
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    /// Defines the transaction value balance.
    #[serde(rename = "valueBalance")]
    pub value_balance: String,
    /// Represents the transaction version number.
    #[serde(rename = "version")]
    pub version: i32,
    /// Represents the transaction version group ID
    #[serde(rename = "versionGroupId")]
    pub version_group_id: String,
    /// Represents the Inputs of the transaction
    #[serde(rename = "vin")]
    pub vin: Vec<crate::models::DecodeRawTransactionHexRiszVin>,
    /// Represents the Inputs of the transaction
    #[serde(rename = "vout")]
    pub vout: Vec<crate::models::DecodeRawTransactionHexRiszVout>,
}

impl DecodeRawTransactionHexRisz {
    /// Zcash
    pub fn new(expiry_height: i32, locktime: i32, overwintered: bool, saplinged: bool, transaction_hash: String, value_balance: String, version: i32, version_group_id: String, vin: Vec<crate::models::DecodeRawTransactionHexRiszVin>, vout: Vec<crate::models::DecodeRawTransactionHexRiszVout>) -> DecodeRawTransactionHexRisz {
        DecodeRawTransactionHexRisz {
            expiry_height,
            locktime,
            overwintered,
            saplinged,
            transaction_hash,
            value_balance,
            version,
            version_group_id,
            vin,
            vout,
        }
    }
}


