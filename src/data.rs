use crate::secret::SecretBase64;
use std::collections::BTreeMap;
use std::sync::LazyLock;

pub const SECRETS: LazyLock<BTreeMap<&'static str, SecretBase64>> = LazyLock::new(|| {
    let mut map: BTreeMap<&'static str, SecretBase64> = BTreeMap::new();
    map.insert(
        "ebay__zach.sandbox.cert_id",
        SecretBase64 {
            nonce: String::from("vzB3tLkRz0xnv4Ud"),
            ciphertext: String::from(
                "BZjNxWZfKjDhbsuB87qPtW1pDGgUd7yI6iHXGDK04TYgTKfLDlbgaGyFKOVlnaBF7YRt7Q==",
            ),
        },
    );
    map.insert(
        "authelia__identity_validation.reset_password.jwt_secret",
        SecretBase64 {
            nonce: String::from("itPCR0zyhoX9prEn"),
            ciphertext: String::from("648vVA5gA/DH5f+ipxJFUjZYssu5hudOA8t2DiTkNLfgyo7yC8gbuXUKVbPKtp/OzCpVqpNTSOuHJfXU7s7QZTYmafKar1PwL663V6wlc2Q="),
        },
    );
    map.insert(
        "authelia__session.secret",
        SecretBase64 {
            nonce: String::from("c72hP+mWcw8lBW1N"),
            ciphertext: String::from("Zwz06lw20bV0wUjqR8wL5immtGofakPaPJETRtOBKguaocbIlnjGXjRrtfSvWho14CfK5eJk+YRAHy5bApkdapZ/YLmw8EX6gWam0/W05YE="),
        },
    );
    map.insert(
        "authelia__storage.encryption_key",
        SecretBase64 {
            nonce: String::from("sMZkqdpKs5XxNC5q"),
            ciphertext: String::from("tUoAtBedRLKweHrXcN8JA3zFvp1AZBmiZvQ3bxD+mJPBcj8uWZLphpA7ojtqhdb8LoXIGXnaS4D8v2veIu/4AX8y2clyBkooIKLs1EW886I="),
        },
    );
    map.insert(
        "authelia__storage.postgres.password",
        SecretBase64 {
            nonce: String::from("kB3iY32TYjwXNLbV"),
            ciphertext: String::from("VRWQCyroStd3wtAfbwOrVzXWi5ukAnDfsg6ewPfdcJ9HACo6bHifRHBKUin4B/jH0p0HaRXCWdMMTZafnTidEkFN2f++Z3rCj5krq7ByubU="),
        },
    );
    map.insert(
        "postgres__user.shop.password.local",
        SecretBase64 {
            nonce: String::from("mBwYxQkQv1e7itjC"),
            ciphertext: String::from("mmkNZyyYI+YWwhIJ2OQSqv9Hd+G0FiVlUYnXx53nfoMBtqKFcDfV1TEu380NzSMBjPNfG0Ca2SM4LobpjG4pSbtNmlJoWfsPuqhMqVQRT/0="),
        },
    );
    map.insert(
        "postgres__user.shop.password.stage",
        SecretBase64 {
            nonce: String::from("mBwYxQkQv1e7itjC"),
            ciphertext: String::from("mmkNZyyYI+YWwhIJ2OQSqv9Hd+G0FiVlUYnXx53nfoMBtqKFcDfV1TEu380NzSMBjPNfG0Ca2SM4LobpjG4pSbtNmlJoWfsPuqhMqVQRT/0="),
        },
    );
    map.insert(
        "postgres__user.shop.password.production",
        SecretBase64 {
            nonce: String::from("mBwYxQkQv1e7itjC"),
            ciphertext: String::from("mmkNZyyYI+YWwhIJ2OQSqv9Hd+G0FiVlUYnXx53nfoMBtqKFcDfV1TEu380NzSMBjPNfG0Ca2SM4LobpjG4pSbtNmlJoWfsPuqhMqVQRT/0="),
        },
    );
    map.insert(
        "proxy__cert.shop.certificate",
        SecretBase64 {
            nonce: String::from("TTSLi702vFELTUcR"),
            ciphertext: String::from("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"),
        },
    );
    map.insert(
        "proxy__cert.shop.key",
        SecretBase64 {
            nonce: String::from("Q3ODT0f6Se5X6wV2"),
            ciphertext: String::from("kw0cWgDY83Iku3ytBeihpO21+qChnoAPgfWvkcLXX/hIlyAcZeKZZfMmPHFNyHbUDAVDKf5tyca9duiPgMj8PYgdAdUllT8ogpPtXvJdVwBpHAGqCO0I2E/pNr/vYDqb08DeHFCSLEi9wKT5V7rSu81YIjszh1m084gDq+/IysYnccFu/14j1WGJXbBGPqouIDEQPQCQ6GSo15WbTj+ItGzUkLX4mhqTMkiOqVw7tlOx5TdrqyBTzSsWYBNZcKociCox0ysN7WSi5aADSliUb7NdCeMcidbEq2PlISWw+YLwT18ags8dbUi42yOElyFj+ou5YukC0i9xyvPsZG1xhQ=="),
        },
    );
    map
});
