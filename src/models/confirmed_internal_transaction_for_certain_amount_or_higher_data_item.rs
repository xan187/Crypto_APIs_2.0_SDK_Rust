/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// ConfirmedInternalTransactionForCertainAmountOrHigherDataItem : Defines an `item` as one result.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfirmedInternalTransactionForCertainAmountOrHigherDataItem {
    /// Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc.
    #[serde(rename = "blockchain")]
    pub blockchain: String,
    /// Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\",\"mordor\" are test networks.
    #[serde(rename = "network")]
    pub network: String,
    /// Defines the specific address of the internal transaction.
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "minedInBlock")]
    pub mined_in_block: Box<crate::models::AddressInternalTransactionConfirmedDataItemMinedInBlock>,
    /// Defines the Parent Transaction's unique ID.
    #[serde(rename = "parentTransactionId")]
    pub parent_transaction_id: String,
    /// Defines the specific operation's unique ID.
    #[serde(rename = "operationId")]
    pub operation_id: String,
    /// Defines the amount of coins sent with the confirmed transaction.
    #[serde(rename = "amount")]
    pub amount: String,
    /// Defines the unit of the transaction, e.g. Gwei.
    #[serde(rename = "unit")]
    pub unit: String,
}

impl ConfirmedInternalTransactionForCertainAmountOrHigherDataItem {
    /// Defines an `item` as one result.
    pub fn new(blockchain: String, network: String, address: String, mined_in_block: crate::models::AddressInternalTransactionConfirmedDataItemMinedInBlock, parent_transaction_id: String, operation_id: String, amount: String, unit: String) -> ConfirmedInternalTransactionForCertainAmountOrHigherDataItem {
        ConfirmedInternalTransactionForCertainAmountOrHigherDataItem {
            blockchain,
            network,
            address,
            mined_in_block: Box::new(mined_in_block),
            parent_transaction_id,
            operation_id,
            amount,
            unit,
        }
    }
}


