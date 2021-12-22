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
pub struct ListDepositAddressesRiConfirmedBalance {
    /// Defines the total balance of the address that is confirmed. It doesn't include unconfirmed transactions.
    #[serde(rename = "amount")]
    pub amount: String,
    /// Represents the unit of the confirmed balance.
    #[serde(rename = "unit")]
    pub unit: String,
}

impl ListDepositAddressesRiConfirmedBalance {
    pub fn new(amount: String, unit: String) -> ListDepositAddressesRiConfirmedBalance {
        ListDepositAddressesRiConfirmedBalance {
            amount,
            unit,
        }
    }
}

