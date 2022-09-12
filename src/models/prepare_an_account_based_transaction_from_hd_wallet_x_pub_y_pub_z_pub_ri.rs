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
pub struct PrepareAnAccountBasedTransactionFromHdWalletXPubYPubZPubRi {
    /// Representation of the amount of the transaction
    #[serde(rename = "amount")]
    pub amount: String,
    /// Representation of the data in hex value
    #[serde(rename = "dataHex")]
    pub data_hex: String,
    /// Representation of the derivation index of the xpub address
    #[serde(rename = "derivationIndex")]
    pub derivation_index: i64,
    #[serde(rename = "fee")]
    pub fee: Box<crate::models::PrepareAnAccountBasedTransactionFromHdWalletXPubYPubZPubRiFee>,
    /// Represents the sequential running number for an address, starting from 0 for the first transaction. E.g., if the nonce of a transaction is 10, it would be the 11th transaction sent from the sender's address.
    #[serde(rename = "nonce")]
    pub nonce: String,
    /// Represents a recipient addresses. In account-based protocols like Ethereum there is only one address in this list.
    #[serde(rename = "recipient")]
    pub recipient: String,
    /// Represents a sender address. In account-based protocols like Ethereum there is only one address in this list.
    #[serde(rename = "sender")]
    pub sender: String,
    /// Representation of the hash that should be signed.
    #[serde(rename = "sigHash")]
    pub sig_hash: String,
    /// Representation of the transaction type
    #[serde(rename = "transactionType")]
    pub transaction_type: TransactionType,
}

impl PrepareAnAccountBasedTransactionFromHdWalletXPubYPubZPubRi {
    pub fn new(amount: String, data_hex: String, derivation_index: i64, fee: crate::models::PrepareAnAccountBasedTransactionFromHdWalletXPubYPubZPubRiFee, nonce: String, recipient: String, sender: String, sig_hash: String, transaction_type: TransactionType) -> PrepareAnAccountBasedTransactionFromHdWalletXPubYPubZPubRi {
        PrepareAnAccountBasedTransactionFromHdWalletXPubYPubZPubRi {
            amount,
            data_hex,
            derivation_index,
            fee: Box::new(fee),
            nonce,
            recipient,
            sender,
            sig_hash,
            transaction_type,
        }
    }
}

/// Representation of the transaction type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransactionType {
    #[serde(rename = "legacy-transaction")]
    LegacyTransaction,
}
