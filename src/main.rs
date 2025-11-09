mod error;

use crate::error::AppError;
use base64::Engine;
use clap::error::ErrorKind;
use clap::{Arg, ArgAction, ArgGroup, ArgMatches, Command};
use secr::cryptography;
use secr::cryptography::{decrypt, encrypt, generate_key};
use secr::secret::{list_secret_names, SecretBase64, BASE64};
use std::env;
use std::error::Error;
use std::fmt::Display;
use std::ops::Deref;
use std::path::PathBuf;

const STORE_ENV_KEY: &'static str = "SECR__STORE_PATH";

enum SubCommand {
    Encrypt,
    Decrypt,
    Key,
    List,
}

impl From<SubCommand> for clap::builder::Str {
    fn from(value: SubCommand) -> Self {
        Self::from(value.public_string())
    }
}

impl SubCommand {
    pub fn public_string(&self) -> &'static str {
        match self {
            SubCommand::Encrypt => "encrypt",
            SubCommand::Decrypt => "decrypt",
            SubCommand::Key => "key",
            SubCommand::List => "list",
        }
    }

    pub fn subcommand_matches<'a>(&self, matches: &'a ArgMatches) -> Option<&'a ArgMatches> {
        matches.subcommand_matches(self.public_string())
    }
}

fn main() -> Result<(), AppError> {
    let mut command: Command = create_command();
    let matches: ArgMatches = command.get_matches_mut();
    route(command, matches)
}

fn create_command() -> Command {
    let arg_file: Arg = Arg::new("file")
        .help(format!("Path to the file storing the encrypted secrets. If unset, tries to read from the path specified by '{}'", STORE_ENV_KEY))
        .long("file")
        .short("f");
    Command::new("secr")
        .version("1.0")
        .about("Manage encrypted secrets for the shop application. Uses the ChaCha20Poly1305 algorithm.")
        .author("Zachary Siegel")
        .flatten_help(true)
        .subcommand(Command::new(SubCommand::Encrypt)
            .about("Encrypt a secret with a key")
            .arg(Arg::new("key")
                .help("The base64-encoded secret key")
                .long("key")
                .short('k')
            )
            .arg(Arg::new("generate_key")
                .help("Generate a new key during encryption rather than accepting an existing key")
                .long("generate-key")
                .short('g')
                .action(ArgAction::SetTrue)
            )
            .group(ArgGroup::new("keys")
                .args(["key", "generate_key"])
                .required(true)
                .multiple(false)
            )
            .arg(Arg::new("plaintext")
                .help("The plaintext to encrypt")
                .required(true)
            )
        )
        .subcommand(Command::new(SubCommand::Decrypt)
            .about("Decrypt a secret by name")
            .arg(Arg::new("key")
                .help("The base64-encoded secret key")
                .short('k')
                .long("key")
                .required(true)
            )
            .arg(Arg::new("name")
                .help("The name of the encrypted secret")
                .required(true)
            )
            .arg(Arg::new("utf8")
                .long("utf8")
                .conflicts_with("base64")
                .action(ArgAction::SetTrue)
                .help("Print only the UTF-8-encoded plaintext to standard output")
            )
            .arg(Arg::new("base64")
                .help("Print only the Base64-encoded plaintext to standard output")
                .long("base64")
                .conflicts_with("utf8")
                .action(ArgAction::SetTrue)
            )
            .arg(&arg_file)
        )
        .subcommand(Command::new(SubCommand::Key)
            .about("Generate a new encryption key")
        )
        .subcommand(Command::new(SubCommand::List)
            .about("List all available secrets")
            .arg(&arg_file)
        )
}

fn route(mut command: Command, matches: ArgMatches) -> Result<(), AppError> {
    if let Some(sub_matches) = SubCommand::Encrypt.subcommand_matches(&matches) {
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

    if let Some(sub_matches) = SubCommand::Decrypt.subcommand_matches(&matches) {
        let secret_name: &String = sub_matches
            .get_one("name")
            .expect("secret name is required");
        let key: Vec<u8> = match sub_matches.get_one::<String>("key") {
            Some(key) => BASE64.decode(key.as_bytes())?,
            None => panic!("key is required"),
        };
        let only_utf8: bool = sub_matches.get_flag("utf8");
        let only_base64: bool = sub_matches.get_flag("base64");
        let store_path: PathBuf = PathBuf::from(match sub_matches.get_one::<String>("file") {
            Some(path) => path.clone(),
            None => env::var(STORE_ENV_KEY)?,
        });
        //todo!(store_path);

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

    if let Some(_) = SubCommand::Key.subcommand_matches(&matches) {
        let key: Vec<u8> = generate_key();
        println!("Generated key (base64):\n\t{}", BASE64.encode(key));
        return Ok(());
    }

    if let Some(_) = SubCommand::List.subcommand_matches(&matches) {
        let text: String = list_secret_names().join("\n");
        println!("{}", text);
        return Ok(());
    }

    command
        .error(ErrorKind::DisplayHelp, "Invalid invocation")
        .exit();
}
