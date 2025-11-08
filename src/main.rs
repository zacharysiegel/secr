use base64::Engine;
use clap::error::ErrorKind;
use clap::{Arg, ArgAction, ArgGroup, ArgMatches, Command};
use secr::cryptography;
use secr::cryptography::{decrypt, encrypt, generate_key};
use secr::secret::{BASE64, SecretBase64, list_secret_names};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut command: Command = Command::new("secr")
        .version("1.0")
        .about("Manage encrypted secrets for the shop application. Uses the ChaCha20Poly1305 algorithm.")
        .author("Zachary Siegel")
        .flatten_help(true)
        .subcommand(Command::new("encrypt")
            .about("Encrypt a secret with a key")
            .arg(Arg::new("key")
                .help("The base64-encoded secret key")
                .short('k')
                .long("key"))
            .arg(Arg::new("generate_key")
                .help("Generate a new key during encryption rather than accepting an existing key")
                .short('g')
                .long("generate-key")
                .action(ArgAction::SetTrue))
            .group(ArgGroup::new("keys")
                .args(["key", "generate_key"])
                .required(true)
                .multiple(false))
            .arg(Arg::new("plaintext")
                .help("The plaintext to encrypt")
                .required(true)))
        .subcommand(Command::new("decrypt")
            .about("Decrypt a secret by name")
            .arg(Arg::new("key")
                .help("The base64-encoded secret key")
                .short('k')
                .long("key")
                .required(true))
            .arg(Arg::new("name")
                .help("The name of the encrypted secret")
                .required(true))
            .arg(Arg::new("utf8")
                .long("utf8")
                .conflicts_with("base64")
                .action(ArgAction::SetTrue)
                .help("Print only the UTF-8-encoded plaintext to standard output"))
            .arg(Arg::new("base64")
                .long("base64")
                .conflicts_with("utf8")
                .action(ArgAction::SetTrue)
                .help("Print only the Base64-encoded plaintext to standard output")))
        .subcommand(Command::new("key")
            .about("Generate a new encryption key"))
        .subcommand(Command::new("list")
            .about("List all available secrets"))
        ;
    let matches: ArgMatches = command.clone().get_matches();

    if let Some(sub_matches) = matches.subcommand_matches("encrypt") {
        let plaintext: &String = sub_matches
            .get_one("plaintext")
            .expect("plaintext is required");

        let provided_key: Option<&String> = sub_matches.get_one("key");
        let generate_key: bool = sub_matches.get_flag("generate_key");

        if provided_key.is_some() && generate_key {
            command
                .error(
                    ErrorKind::DisplayHelp,
                    "key and generate_key are mutually exclusive",
                )
                .exit();
        }

        let key: Vec<u8> = match provided_key {
            Some(key) => BASE64.decode(key)?,
            None => cryptography::generate_key(),
        };

        let secret: SecretBase64 = encrypt(&key, plaintext.as_bytes())?;

        if generate_key {
            println!("Generated key (base64):\n\t{}", BASE64.encode(&key));
        }
        println!("{}", secret);
        return Ok(());
    }

    if let Some(sub_matches) = matches.subcommand_matches("decrypt") {
        let secret_name: &String = sub_matches
            .get_one("name")
            .expect("secret name is required");
        let key: Vec<u8> = match sub_matches.get_one::<String>("key") {
            Some(key) => BASE64.decode(key.as_bytes())?,
            None => panic!("key is required"),
        };
        let only_utf8: bool = sub_matches.get_flag("utf8");
        let only_base64: bool = sub_matches.get_flag("base64");

        let plaintext: Vec<u8> = decrypt(&key, secret_name)?;

        if only_utf8 {
            let plaintext_utf8: String = String::from_utf8(plaintext.clone())?;
            println!("{}", plaintext_utf8);
            return Ok(());
        }

        if only_base64 {
            let plaintext_base64: String = BASE64.encode(&plaintext);
            println!("{}", plaintext_base64);
            return Ok(());
        }

        println!(
            "UTF-8 encoding:\n\t{}",
            String::from_utf8(plaintext.clone())?
        );
        println!("Base64 encoding:\n\t{}", BASE64.encode(&plaintext));
        return Ok(());
    }

    if let Some(_) = matches.subcommand_matches("key") {
        let key: Vec<u8> = generate_key();
        println!("Generated key (base64):\n\t{}", BASE64.encode(key));
        return Ok(());
    }

    if let Some(_) = matches.subcommand_matches("list") {
        let text: String = list_secret_names().join("\n");
        println!("{}", text);
        return Ok(());
    }

    command
        .error(ErrorKind::DisplayHelp, "Invalid invocation")
        .exit();
}
