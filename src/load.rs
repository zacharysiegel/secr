use crate::error::Error;
use crate::secret::SecretBase64Ref;
use crate::store::SecretStore;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub fn load_secrets_from_file(path: &Path) -> Result<SecretStore, Error> {
    let contents: String = fs::read_to_string(&path)?;
    let contents: String = normalize_leading_tabs_to_spaces(&contents);

    let store_ref: BTreeMap<&str, SecretBase64Ref> = serde_yaml_ng::from_str(&contents)?;

    let mut store_owned: SecretStore = SecretStore::new();
    for (key, value) in store_ref {
        _ = store_owned.insert(String::from(key), value.deep_clone());
    }

    Ok(store_owned)
}

/// Store files written by `secr` versions before 1.2.3 indented with tabs, which YAML forbids in indentation.
fn normalize_leading_tabs_to_spaces(contents: &str) -> String {
    let normalized_lines: Vec<String> = contents
        .lines()
        .map(|line| {
            let leading_whitespace_length: usize = line.len() - line.trim_start().len();
            let (leading_whitespace, rest): (&str, &str) = line.split_at(leading_whitespace_length);

            format!("{}{}", leading_whitespace.replace('\t', "  "), rest)
        })
        .collect();

    normalized_lines.join("\n")
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

    #[test]
    fn load_secrets_from_file_reads_legacy_tab_indented_file() {
        let key: Vec<u8> = cryptography::generate_key();
        let secret: SecretBase64 = cryptography::encrypt(&key, b"mysecret").unwrap();
        let tab_indented_contents: String = format!(
            "hello_world:\n\tnonce: '{}'\n\tciphertext: '{}'",
            secret.nonce, secret.ciphertext,
        );
        let path: PathBuf = std::env::temp_dir().join(format!("secr-load-legacy-tab-test-{}.yaml", std::process::id()));
        fs::write(&path, tab_indented_contents).unwrap();

        let store: Result<SecretStore, Error> = load_secrets_from_file(&path);
        fs::remove_file(&path).unwrap();

        let decrypted: Vec<u8> = cryptography::decrypt(&store.unwrap(), &key, "hello_world").unwrap();
        assert_eq!(decrypted, b"mysecret");
    }
}
