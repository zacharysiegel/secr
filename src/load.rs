use crate::error::Error;
use crate::secret::{SecretBase64, SecretBase64Ref};
use std::collections::BTreeMap;
use std::fs;
use std::ops::{Deref, DerefMut};
use std::path::Path;

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

pub fn load_secrets_from_file(path: &Path) -> Result<SecretStore, Error> {
    let bytes: Vec<u8> = fs::read(&path)?;
    let mut store_owned: SecretStore = SecretStore::new();
    let store_ref: BTreeMap<&str, SecretBase64Ref> = serde_yaml_ng::from_slice(bytes.as_slice())?;

    for (key, value) in store_ref {
        _ = store_owned.insert(String::from(key), value.deep_clone());
    }

    Ok(store_owned)
}

pub fn list_secret_names(secrets: &SecretStore) -> Vec<String> {
    secrets.keys().into_iter().map(|key| key.to_string()).collect::<Vec<String>>()
}
