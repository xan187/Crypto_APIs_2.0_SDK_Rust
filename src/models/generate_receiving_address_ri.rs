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
pub struct GenerateReceivingAddressRi {
    /// Specifies the specific address's unique string value.
    #[serde(rename = "address")]
    pub address: String,
    /// Defines the specific UNIX time when the deposit address was created.
    #[serde(rename = "createdTimestamp")]
    pub created_timestamp: i32,
    /// Represents a custom tag that customers can set up for their Wallets and addresses. E.g. custom label named \"Special addresses\".
    #[serde(rename = "label")]
    pub label: String,
}

impl GenerateReceivingAddressRi {
    pub fn new(address: String, created_timestamp: i32, label: String) -> GenerateReceivingAddressRi {
        GenerateReceivingAddressRi {
            address,
            created_timestamp,
            label,
        }
    }
}


