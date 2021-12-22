/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// GetBlockDetailsByBlockHashFromCallbackRibsz : Zilliqa



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetBlockDetailsByBlockHashFromCallbackRibsz {
    /// Defines how difficult it is for a specific miner to mine the block.
    #[serde(rename = "difficulty")]
    pub difficulty: String,
    /// Represents the Directory Service block which contains metadata about the miners who participate in the consensus protocol.
    #[serde(rename = "dsBlock")]
    pub ds_block: i32,
    /// Defines how difficult it is to mine the dsBlocks.
    #[serde(rename = "dsDifficulty")]
    pub ds_difficulty: String,
    /// Represents a part of the DS Committee which leads the consensus protocol for the epoch.
    #[serde(rename = "dsLeader")]
    pub ds_leader: String,
    /// Represents the maximum amount of gas allowed in the block in order to determine how many transactions it can fit.
    #[serde(rename = "gasLimit")]
    pub gas_limit: i32,
    /// Defines how much of the gas for the block has been used.
    #[serde(rename = "gasUsed")]
    pub gas_used: i32,
    #[serde(rename = "microBlocks")]
    pub micro_blocks: Vec<String>,
}

impl GetBlockDetailsByBlockHashFromCallbackRibsz {
    /// Zilliqa
    pub fn new(difficulty: String, ds_block: i32, ds_difficulty: String, ds_leader: String, gas_limit: i32, gas_used: i32, micro_blocks: Vec<String>) -> GetBlockDetailsByBlockHashFromCallbackRibsz {
        GetBlockDetailsByBlockHashFromCallbackRibsz {
            difficulty,
            ds_block,
            ds_difficulty,
            ds_leader,
            gas_limit,
            gas_used,
            micro_blocks,
        }
    }
}

