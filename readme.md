# secr &emsp; [![Latest Version]][crates.io]

[Latest Version]: https://img.shields.io/crates/v/secr.svg

[crates.io]: https://crates.io/crates/secr

_A simple secret manager._

Manage encrypted secrets stored in a file. Uses the [ChaCha20Poly1305](https://en.wikipedia.org/wiki/ChaCha20-Poly1305) algorithm.

## Usage

### Install `secr` to your system via Cargo

	cargo install secr

### Generate a symmetric encryption key:

```bash
secr key
```

Example:

```
$ secr key
Generated key (base64):
2Af8Ty6PG9ICZppnY5cB8GMPdNg4NubxU4HyPEx1h0E=
```

### Encrypt a secret

```bash
secr encrypt --key '<symmetric_key>' '<plaintext>'
```

Output is preformatted in YAML. Example:

```
$ secr encrypt --key '2Af8Ty6PG9ICZppnY5cB8GMPdNg4NubxU4HyPEx1h0E=' 'mysecret'
__untitled__:
    nonce: 'qt+WBOWfII63uI9q'
    ciphertext: 'qTEMbZ5SZVox7Mmsc61cXbEbzYuKVO1u'
```

### Add the encrypted secret to a data store

Example:

```bash
cat >> secrets.yaml << 'EOF'
hello_world:
  nonce: 'qt+WBOWfII63uI9q'
  ciphertext: 'qTEMbZ5SZVox7Mmsc61cXbEbzYuKVO1u'
EOF
```

### Decrypt the secret

```bash
secr decrypt --file '<store_path>' --key '<symmetric_key>' '<secret_name>'
```

Example:

```text
$ secr decrypt --file secrets.yaml --key '2Af8Ty6PG9ICZppnY5cB8GMPdNg4NubxU4HyPEx1h0E=' 'hello_world'
UTF-8 encoding:
mysecret
Base64 encoding:
bXlzZWNyZXQ=
```

## Versioning

This package follows the [Semantic Versioning](https://semver.org/) convention.

## License

Licensed under the [MIT license](./LICENSE).
