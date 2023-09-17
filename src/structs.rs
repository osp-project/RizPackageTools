use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
pub struct TargetStrings {
    pub area_test_target: String,
    pub area_verify_target: String,
    pub aes256_key_target: String,
    pub aes256_iv_target: String,
    pub server_host_target: String,
    pub game_config_address_target: String,
    pub xsolla_purchase_address_target: String,
    pub rsa_public_key_target: String
}