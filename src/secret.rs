use base64::Engine;
use base64::engine::DecodePaddingMode;
use serde::Deserialize;

pub const BASE64: base64::engine::general_purpose::GeneralPurpose = base64::engine::GeneralPurpose::new(
    &base64::alphabet::STANDARD,
    base64::engine::GeneralPurposeConfig::new()
        .with_decode_padding_mode(DecodePaddingMode::Indifferent)
        .with_encode_padding(true),
);

#[derive(Debug, Deserialize)]
pub struct SecretBase64Ref<'a> {
    pub nonce: &'a str,
    pub ciphertext: &'a str,
}

impl<'a> SecretBase64Ref<'a> {
    pub fn deep_clone(&self) -> SecretBase64 {
        SecretBase64 {
            nonce: self.nonce.to_owned(),
            ciphertext: self.ciphertext.to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct SecretBase64 {
    pub nonce: String,
    pub ciphertext: String,
}

impl SecretBase64 {
    pub fn base64_decode(&self) -> Result<SecretBytes, base64::DecodeError> {
        Ok(SecretBytes {
            nonce: BASE64.decode(&self.nonce)?,
            ciphertext: BASE64.decode(&self.ciphertext)?,
        })
    }

    pub fn to_yaml(&self, name: &str) -> String {
        format!(
            concat!("{}:\n", "\tnonce: '{}'\n", "\tciphertext: '{}'"),
            name, self.nonce, self.ciphertext
        )
    }
}

#[derive(Debug)]
pub struct SecretBytes {
    pub nonce: Vec<u8>,
    pub ciphertext: Vec<u8>,
}

impl SecretBytes {
    pub fn base64_encode(&self) -> SecretBase64 {
        SecretBase64 {
            nonce: BASE64.encode(&self.nonce),
            ciphertext: BASE64.encode(&self.ciphertext),
        }
    }
}
