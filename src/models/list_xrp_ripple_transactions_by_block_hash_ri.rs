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
pub struct ListXrpRippleTransactionsByBlockHashRi {
    /// Represents any additional data that may be needed.
    #[serde(rename = "additionalData", skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<String>,
    #[serde(rename = "destinationTag", skip_serializing_if = "Option::is_none")]
    pub destination_tag: Option<i64>,
    /// Represents the index position of the transaction in the specific block.
    #[serde(rename = "index")]
    pub index: i32,
    /// Represents the hight of the block where this transaction was mined/confirmed for first time. The height is defined as the number of blocks in the blockchain preceding this specific block.
    #[serde(rename = "minedInBlockHeight")]
    pub mined_in_block_height: i32,
    /// Represents an object of addresses that receive the transactions.
    #[serde(rename = "recipients")]
    pub recipients: Vec<crate::models::ListXrpRippleTransactionsByBlockHashRiRecipients>,
    /// Represents an object of addresses that provide the funds.
    #[serde(rename = "senders")]
    pub senders: Vec<crate::models::ListXrpRippleTransactionsByBlockHashRiSenders>,
    /// Defines the transaction input's sequence as an integer, which is is used when transactions are replaced with newer versions before LockTime.
    #[serde(rename = "sequence")]
    pub sequence: i64,
    /// Defines the status of the transaction.
    #[serde(rename = "status")]
    pub status: String,
    /// Defines the exact date/time in Unix Timestamp when this transaction was mined, confirmed or first seen in Mempool, if it is unconfirmed.
    #[serde(rename = "timestamp")]
    pub timestamp: i32,
    /// Represents the same as `transactionId` for account-based protocols like Ethereum, while it could be different in UTXO-based protocols like Bitcoin. E.g., in UTXO-based protocols `hash` is different from `transactionId` for SegWit transactions.
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    /// Defines the type of the transaction.
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "fee")]
    pub fee: Box<crate::models::ListXrpRippleTransactionsByBlockHashRiFee>,
    #[serde(rename = "offer")]
    pub offer: Box<crate::models::ListXrpRippleTransactionsByBlockHashRiOffer>,
    #[serde(rename = "receive")]
    pub receive: Box<crate::models::ListXrpRippleTransactionsByBlockHashRiReceive>,
    #[serde(rename = "value")]
    pub value: Box<crate::models::ListXrpRippleTransactionsByBlockHashRiValue>,
}

impl ListXrpRippleTransactionsByBlockHashRi {
    pub fn new(index: i32, mined_in_block_height: i32, recipients: Vec<crate::models::ListXrpRippleTransactionsByBlockHashRiRecipients>, senders: Vec<crate::models::ListXrpRippleTransactionsByBlockHashRiSenders>, sequence: i64, status: String, timestamp: i32, transaction_hash: String, _type: String, fee: crate::models::ListXrpRippleTransactionsByBlockHashRiFee, offer: crate::models::ListXrpRippleTransactionsByBlockHashRiOffer, receive: crate::models::ListXrpRippleTransactionsByBlockHashRiReceive, value: crate::models::ListXrpRippleTransactionsByBlockHashRiValue) -> ListXrpRippleTransactionsByBlockHashRi {
        ListXrpRippleTransactionsByBlockHashRi {
            additional_data: None,
            destination_tag: None,
            index,
            mined_in_block_height,
            recipients,
            senders,
            sequence,
            status,
            timestamp,
            transaction_hash,
            _type,
            fee: Box::new(fee),
            offer: Box::new(offer),
            receive: Box::new(receive),
            value: Box::new(value),
        }
    }
}


