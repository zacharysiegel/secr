use crate::secret::SecretBase64;
use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};
pub struct SecretStore(BTreeMap<String, SecretBase64>);

impl Deref for SecretStore {
    type Target = BTreeMap<String, SecretBase64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SecretStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl SecretStore {
    pub fn new() -> Self {
        Self(BTreeMap::default())
    }

    pub fn into_inner(self) -> BTreeMap<String, SecretBase64> {
        self.0
    }
}

pub fn list_secret_names(secrets: &SecretStore) -> Vec<String> {
    secrets.keys().into_iter().map(|key| key.to_string()).collect::<Vec<String>>()
}
