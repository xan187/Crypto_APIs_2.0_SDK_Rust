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
pub struct ListAllAssetsByWalletIdri {
    #[serde(rename = "coins")]
    pub coins: Vec<crate::models::ListAllAssetsFromAllWalletsRiCoins>,
    /// Represents fungible tokens'es detailed information
    #[serde(rename = "fungibleTokens")]
    pub fungible_tokens: Vec<crate::models::ListAllAssetsFromAllWalletsRiFungibleTokens>,
    /// Represents non-fungible tokens'es detailed information.
    #[serde(rename = "nonFungibleTokens")]
    pub non_fungible_tokens: Vec<crate::models::ListAllAssetsFromAllWalletsRiNonFungibleTokens>,
    /// Defines the unique ID of the Wallet.
    #[serde(rename = "walletId")]
    pub wallet_id: String,
    /// Represents the name of the wallet.
    #[serde(rename = "walletName")]
    pub wallet_name: String,
}

impl ListAllAssetsByWalletIdri {
    pub fn new(coins: Vec<crate::models::ListAllAssetsFromAllWalletsRiCoins>, fungible_tokens: Vec<crate::models::ListAllAssetsFromAllWalletsRiFungibleTokens>, non_fungible_tokens: Vec<crate::models::ListAllAssetsFromAllWalletsRiNonFungibleTokens>, wallet_id: String, wallet_name: String) -> ListAllAssetsByWalletIdri {
        ListAllAssetsByWalletIdri {
            coins,
            fungible_tokens,
            non_fungible_tokens,
            wallet_id,
            wallet_name,
        }
    }
}


