pub use clap::{Arg, Command, Parser};
pub use num_cpus;
pub use once_cell::sync::{Lazy, OnceCell};
pub use std::collections::HashMap;
pub use std::str::FromStr;
pub use std::thread;
pub use std::time::Instant;

pub use bip0039::{Count, Mnemonic};
pub use libsecp256k1::{PublicKey, SecretKey};
pub use std::sync::Arc;
pub use tiny_hderive::bip32::ExtendedPrivKey;
pub use tiny_hderive::bip44::ChildNumber;
pub use tiny_keccak::{Hasher, Keccak};
pub use tokio::sync::mpsc;
pub use trompt::Trompt;

pub use crypto2::blockcipher::Aes128;

pub fn generate_address(
    words: Count,
    sender: Arc<mpsc::UnboundedSender<(Mnemonic, String)>>,
    leading_chars: &crate::LeadingChars,
) {
    match words {
        Count::Words12 => {}
        Count::Words24 => {}
        _ => panic!(),
    }
    let leading_chars = leading_chars.0.clone();
    // mnemonic
    loop {
        let mnemonic = Mnemonic::generate(words);
        let seed = mnemonic.to_seed("");

        let hdwallet = ExtendedPrivKey::derive(&seed, "m/44'/60'/0'/0").unwrap();
        let account0 = hdwallet.child(ChildNumber::from_str("0").unwrap()).unwrap();

        let secret_key = SecretKey::parse(&account0.secret()).unwrap();

        let public_key = PublicKey::from_secret_key(&secret_key);
        let public_key = &public_key.serialize()[1..65];

        let addr = &keccak_hash(public_key);

        if &addr[24..24 + leading_chars.len()] == &leading_chars {
            let addr = &addr[24..];
            let addr = addr.to_string();
            sender.send((mnemonic, addr)).unwrap();
        }
    }
}

fn keccak_hash(input: &[u8]) -> String {
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];

    hasher.update(input);
    hasher.finalize(&mut output);

    hex::encode(output)
}
