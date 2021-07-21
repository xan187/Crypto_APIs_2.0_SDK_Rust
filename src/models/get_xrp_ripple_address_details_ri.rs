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
pub struct GetXrpRippleAddressDetailsRi {
    #[serde(rename = "balance")]
    pub balance: Box<crate::models::GetXrpRippleAddressDetailsRiBalance>,
    /// Defines the count of all confirmed incoming transactions from the address for coins. This applies to coins only, not to tokens transfers
    #[serde(rename = "incomingTransactionsCount")]
    pub incoming_transactions_count: i32,
    /// Defines the count of all confirmed outgoing transactions for coins. This applies to coins only, not to tokens transfers
    #[serde(rename = "outgoingTransactionsCount")]
    pub outgoing_transactions_count: i32,
    /// Defines the transaction input's sequence as an integer, which is is used when transactions are replaced with newer versions before LockTime.
    #[serde(rename = "sequence")]
    pub sequence: i32,
    /// Represents the total number of all transactions as part of this block.
    #[serde(rename = "transactionsCount")]
    pub transactions_count: i32,
}

impl GetXrpRippleAddressDetailsRi {
    pub fn new(balance: crate::models::GetXrpRippleAddressDetailsRiBalance, incoming_transactions_count: i32, outgoing_transactions_count: i32, sequence: i32, transactions_count: i32) -> GetXrpRippleAddressDetailsRi {
        GetXrpRippleAddressDetailsRi {
            balance: Box::new(balance),
            incoming_transactions_count,
            outgoing_transactions_count,
            sequence,
            transactions_count,
        }
    }
}

