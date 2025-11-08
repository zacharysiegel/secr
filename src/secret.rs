use crate::data::SECRETS;
use base64::Engine;
use base64::engine::DecodePaddingMode;
use std::fmt::{Display, Formatter};

pub const BASE64: base64::engine::general_purpose::GeneralPurpose =
    base64::engine::GeneralPurpose::new(
        &base64::alphabet::STANDARD,
        base64::engine::GeneralPurposeConfig::new()
            .with_decode_padding_mode(DecodePaddingMode::Indifferent)
            .with_encode_padding(true),
    );

#[derive(Debug)]
pub struct Secret<T> {
    pub nonce: T,
    pub ciphertext: T,
}

pub type SecretBase64 = Secret<String>;
pub type SecretBytes = Secret<Vec<u8>>;

impl SecretBase64 {
    pub fn base64_decode(&self) -> Result<SecretBytes, base64::DecodeError> {
        Ok(SecretBytes {
            nonce: BASE64.decode(&self.nonce)?,
            ciphertext: BASE64.decode(&self.ciphertext)?,
        })
    }
}

impl SecretBytes {
    pub fn base64_encode(&self) -> SecretBase64 {
        SecretBase64 {
            nonce: BASE64.encode(&self.nonce),
            ciphertext: BASE64.encode(&self.ciphertext),
        }
    }
}

impl Display for SecretBase64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut display: String = String::from("SecretBase64 {\n");
        display.push_str(&format!("    nonce: String::from(\"{}\"),\n", self.nonce));
        display.push_str(&format!(
            "    ciphertext: String::from(\"{}\"),\n",
            self.ciphertext
        ));
        display.push_str("},");
        write!(f, "{}", display)
    }
}

pub fn list_secret_names() -> Vec<String> {
    SECRETS
        .keys()
        .into_iter()
        .map(|key| key.to_string())
        .collect::<Vec<String>>()
}
