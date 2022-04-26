/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// DecodeRawTransactionHexRisd2ScriptSig : Specifies the required signatures.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DecodeRawTransactionHexRisd2ScriptSig {
    /// The asm strands for assembly, which is the symbolic representation of the Bitcoin's Script language op-codes.
    #[serde(rename = "asm", skip_serializing_if = "Option::is_none")]
    pub asm: Option<String>,
    /// Represents the hex of the public key of the address.
    #[serde(rename = "hex", skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    /// Represents the script type of the reference transaction identifier.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl DecodeRawTransactionHexRisd2ScriptSig {
    /// Specifies the required signatures.
    pub fn new() -> DecodeRawTransactionHexRisd2ScriptSig {
        DecodeRawTransactionHexRisd2ScriptSig {
            asm: None,
            hex: None,
            _type: None,
        }
    }
}


