/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// ListTransactionsByBlockHeightRibsz : Zcash



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListTransactionsByBlockHeightRibsz {
    /// It is used to enforce balance of Spend and Output transfers, in order to prevent their replay across transactions.
    #[serde(rename = "bindingSig")]
    pub binding_sig: String,
    /// Represents a block height after which the transaction will expire.
    #[serde(rename = "expiryHeight")]
    pub expiry_height: i32,
    /// Represents an encoding of a JoinSplitSig public validating key.
    #[serde(rename = "joinSplitPubKey")]
    pub join_split_pub_key: String,
    /// Is used to sign transactions that contain at least one JoinSplit description.
    #[serde(rename = "joinSplitSig")]
    pub join_split_sig: String,
    /// Represents the time at which a particular transaction can be added to the blockchain.
    #[serde(rename = "locktime")]
    pub locktime: i64,
    /// \"Overwinter\" is the network upgrade for the Zcash blockchain.
    #[serde(rename = "overwintered")]
    pub overwintered: bool,
    /// Represents the total size of this transaction.
    #[serde(rename = "size")]
    pub size: i32,
    /// Represents a sequence of JoinSplit descriptions using BCTV14 proofs.
    #[serde(rename = "vJoinSplit")]
    pub v_join_split: Vec<crate::models::ListTransactionsByBlockHeightRibszVJoinSplit>,
    /// Object Array representation of transaction output descriptions
    #[serde(rename = "vShieldedOutput")]
    pub v_shielded_output: Vec<crate::models::ListTransactionsByBlockHeightRibszVShieldedOutput>,
    /// Object Array representation of transaction spend descriptions
    #[serde(rename = "vShieldedSpend")]
    pub v_shielded_spend: Vec<crate::models::GetTransactionDetailsByTransactionIdribszVShieldedSpend>,
    /// Defines the transaction value balance.
    #[serde(rename = "valueBalance")]
    pub value_balance: String,
    /// Represents the transaction version number.
    #[serde(rename = "version")]
    pub version: i32,
    /// Represents the transaction version group ID.
    #[serde(rename = "versionGroupId")]
    pub version_group_id: String,
    /// Object Array representation of transaction inputs
    #[serde(rename = "vin")]
    pub vin: Vec<crate::models::ListTransactionsByBlockHeightRibszVin>,
    /// Object Array representation of transaction outputs
    #[serde(rename = "vout")]
    pub vout: Vec<crate::models::ListTransactionsByBlockHeightRibszVout>,
}

impl ListTransactionsByBlockHeightRibsz {
    /// Zcash
    pub fn new(binding_sig: String, expiry_height: i32, join_split_pub_key: String, join_split_sig: String, locktime: i64, overwintered: bool, size: i32, v_join_split: Vec<crate::models::ListTransactionsByBlockHeightRibszVJoinSplit>, v_shielded_output: Vec<crate::models::ListTransactionsByBlockHeightRibszVShieldedOutput>, v_shielded_spend: Vec<crate::models::GetTransactionDetailsByTransactionIdribszVShieldedSpend>, value_balance: String, version: i32, version_group_id: String, vin: Vec<crate::models::ListTransactionsByBlockHeightRibszVin>, vout: Vec<crate::models::ListTransactionsByBlockHeightRibszVout>) -> ListTransactionsByBlockHeightRibsz {
        ListTransactionsByBlockHeightRibsz {
            binding_sig,
            expiry_height,
            join_split_pub_key,
            join_split_sig,
            locktime,
            overwintered,
            size,
            v_join_split,
            v_shielded_output,
            v_shielded_spend,
            value_balance,
            version,
            version_group_id,
            vin,
            vout,
        }
    }
}


