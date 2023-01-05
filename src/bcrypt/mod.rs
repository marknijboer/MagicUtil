use std::process::exit;

use pwhash::bcrypt::{self, BcryptSetup, BcryptVariant};
use clap::ArgMatches;

use crate::utils::print_error;

/// Handles all system related commands.
pub fn handle_bcrypt_command(submatches: &ArgMatches) {
    if let Some(subsubmatches) = submatches.subcommand_matches("hash") {
        hash_plaintext(subsubmatches);
        return;
    }
    
    unreachable!("No valid subcommand found")
}

/// Prints the bcrypt hashed version of the plaintext
fn hash_plaintext(submatches: &ArgMatches) {
    let key: &String = submatches.get_one("PLAINTEXT").unwrap();
    if key.is_empty() {
        print_error("Expected a plaintext to hash");
        exit(1);
    }

    let hash_res = bcrypt::hash_with(BcryptSetup{
        variant: Some(BcryptVariant::V2a),
        salt: None,
        cost: None,
    }, key);

    if let Err(hash_err) = hash_res {
        let error_message = format!("Could not hash plaintext: {hash_err:?}");
        print_error(error_message);
        exit(1);
    }

    println!("{}", hash_res.unwrap());
}