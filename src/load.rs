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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptography;
    use crate::secret::SecretBase64;
    use std::path::PathBuf;

    #[test]
    fn load_secrets_from_file_reads_to_yaml_output() {
        let key: Vec<u8> = cryptography::generate_key();
        let secret: SecretBase64 = cryptography::encrypt(&key, b"mysecret").unwrap();
        let path: PathBuf = std::env::temp_dir().join(format!("secr-load-test-{}.yaml", std::process::id()));
        fs::write(&path, secret.to_yaml("hello_world")).unwrap();

        let store: Result<SecretStore, Error> = load_secrets_from_file(&path);
        fs::remove_file(&path).unwrap();

        let decrypted: Vec<u8> = cryptography::decrypt(&store.unwrap(), &key, "hello_world").unwrap();
        assert_eq!(decrypted, b"mysecret");
    }
}
