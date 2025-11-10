use crate::error::Error;
use crate::secret::SecretBase64Ref;
use crate::store::SecretStore;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub fn load_secrets_from_file(path: &Path) -> Result<SecretStore, Error> {
    let bytes: Vec<u8> = fs::read(&path)?;
    let mut store_owned: SecretStore = SecretStore::new();
    let store_ref: BTreeMap<&str, SecretBase64Ref> = serde_yaml_ng::from_slice(bytes.as_slice())?;

    for (key, value) in store_ref {
        _ = store_owned.insert(String::from(key), value.deep_clone());
    }

    Ok(store_owned)
}
