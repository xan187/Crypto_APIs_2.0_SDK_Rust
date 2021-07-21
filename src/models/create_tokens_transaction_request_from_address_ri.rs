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
pub struct CreateTokensTransactionRequestFromAddressRi {
    /// Represents the fee priority of the automation, whether it is \"slow\", \"standard\" or \"fast\".
    #[serde(rename = "feePriority")]
    pub fee_priority: FeePriority,
    /// Defines the destination for the transaction, i.e. the recipient(s).
    #[serde(rename = "recipients")]
    pub recipients: Vec<crate::models::CreateTokensTransactionRequestFromAddressRiRecipients>,
    #[serde(rename = "senders")]
    pub senders: Box<crate::models::CreateTokensTransactionRequestFromAddressRiSenders>,
    #[serde(rename = "tokenTypeSpecificData")]
    pub token_type_specific_data: Box<crate::models::CreateTokensTransactionRequestFromAddressRiTokenTypeSpecificData>,
}

impl CreateTokensTransactionRequestFromAddressRi {
    pub fn new(fee_priority: FeePriority, recipients: Vec<crate::models::CreateTokensTransactionRequestFromAddressRiRecipients>, senders: crate::models::CreateTokensTransactionRequestFromAddressRiSenders, token_type_specific_data: crate::models::CreateTokensTransactionRequestFromAddressRiTokenTypeSpecificData) -> CreateTokensTransactionRequestFromAddressRi {
        CreateTokensTransactionRequestFromAddressRi {
            fee_priority,
            recipients,
            senders: Box::new(senders),
            token_type_specific_data: Box::new(token_type_specific_data),
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

