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
pub struct CreateCoinsTransactionRequestFromWalletRi {
    /// Represents the fee priority of the automation, whether it is \"slow\", \"standard\" or \"fast\".
    #[serde(rename = "feePriority")]
    pub fee_priority: FeePriority,
    /// Defines the destination of the transaction, whether it is incoming or outgoing.
    #[serde(rename = "recipients")]
    pub recipients: Vec<crate::models::CreateCoinsTransactionRequestFromWalletRiRecipients>,
    /// Represents the specific amount of the transaction.
    #[serde(rename = "totalTransactionAmount")]
    pub total_transaction_amount: String,
    /// Defines the status of the transaction, e.g. \"created, \"await_approval\", \"pending\", \"prepared\", \"signed\", \"broadcasted\", \"success\", \"failed\", \"rejected\", mined\".
    #[serde(rename = "transactionRequestStatus")]
    pub transaction_request_status: TransactionRequestStatus,
}

impl CreateCoinsTransactionRequestFromWalletRi {
    pub fn new(fee_priority: FeePriority, recipients: Vec<crate::models::CreateCoinsTransactionRequestFromWalletRiRecipients>, total_transaction_amount: String, transaction_request_status: TransactionRequestStatus) -> CreateCoinsTransactionRequestFromWalletRi {
        CreateCoinsTransactionRequestFromWalletRi {
            fee_priority,
            recipients,
            total_transaction_amount,
            transaction_request_status,
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
/// Defines the status of the transaction, e.g. \"created, \"await_approval\", \"pending\", \"prepared\", \"signed\", \"broadcasted\", \"success\", \"failed\", \"rejected\", mined\".
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransactionRequestStatus {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "await-approval")]
    AwaitApproval,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "prepared")]
    Prepared,
    #[serde(rename = "signed")]
    Signed,
    #[serde(rename = "broadcasted")]
    Broadcasted,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "mined")]
    Mined,
}

