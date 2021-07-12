use std::process::exit;

use pwhash::bcrypt::{self, BcryptSetup, BcryptVariant};
use clap::ArgMatches;

/// Handles all system related commands.
pub fn handle_bcrypt_command(submatches: &ArgMatches) {
    if let Some(subsubmatches) = submatches.subcommand_matches("hash") {
        hash_plaintext(subsubmatches);
        return;
    }
    
    println!("{}", submatches.usage());
    exit(2);
}

/// Prints the bcrypt hashed version of the plaintext
fn hash_plaintext(submatches: &ArgMatches) {
    let key = submatches.value_of("PLAINTEXT").unwrap();
    if key.is_empty() {
        eprintln!("Expected a plaintext to hash");
        exit(1);
    }

    let hash_res = bcrypt::hash_with(BcryptSetup{
        variant: Some(BcryptVariant::V2a),
        salt: None,
        cost: None,
    }, key);

    if let Err(hash_err) = hash_res {
        eprintln!("Could not hash plaintext: {:?}", hash_err);
        exit(1);
    }

    println!("{}", hash_res.unwrap());
}