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
pub struct CreateFungibleTokensTransactionRequestFromAddressRi {
    /// Represents the Secret Key value provided by the customer. This field is used for security purposes during the callback notification, in order to prove the sender of the callback as Crypto APIs. For more information please see our [Documentation](https://developers.cryptoapis.io/technical-documentation/general-information/callbacks#callback-security).
    #[serde(rename = "callbackSecretKey")]
    pub callback_secret_key: String,
    /// Represents the URL that is set by the customer where the callback will be received at. The callback notification will be received only if and when the event occurs.
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
    /// Represents the fee priority of the automation, whether it is \"slow\", \"standard\" or \"fast\".
    #[serde(rename = "feePriority")]
    pub fee_priority: FeePriority,
    /// Represents an optional note to add a free text in, explaining or providing additional detail on the transaction request.
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Defines the destination for the transaction, i.e. the recipient(s).
    #[serde(rename = "recipients")]
    pub recipients: Vec<crate::models::CreateFungibleTokensTransactionRequestFromAddressRiRecipients>,
    #[serde(rename = "senders")]
    pub senders: Box<crate::models::CreateFungibleTokensTransactionRequestFromAddressRiSenders>,
    #[serde(rename = "tokenTypeSpecificData")]
    pub token_type_specific_data: Box<crate::models::CreateFungibleTokensTransactionRequestFromAddressRis>,
    /// Represents a unique identifier of the transaction request (the request sent to make a transaction), which helps in identifying which callback and which `referenceId` concern that specific transaction request.
    #[serde(rename = "transactionRequestId")]
    pub transaction_request_id: String,
}

impl CreateFungibleTokensTransactionRequestFromAddressRi {
    pub fn new(callback_secret_key: String, callback_url: String, fee_priority: FeePriority, recipients: Vec<crate::models::CreateFungibleTokensTransactionRequestFromAddressRiRecipients>, senders: crate::models::CreateFungibleTokensTransactionRequestFromAddressRiSenders, token_type_specific_data: crate::models::CreateFungibleTokensTransactionRequestFromAddressRis, transaction_request_id: String) -> CreateFungibleTokensTransactionRequestFromAddressRi {
        CreateFungibleTokensTransactionRequestFromAddressRi {
            callback_secret_key,
            callback_url,
            fee_priority,
            note: None,
            recipients,
            senders: Box::new(senders),
            token_type_specific_data: Box::new(token_type_specific_data),
            transaction_request_id,
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
