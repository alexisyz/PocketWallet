use wasm_bindgen::prelude::*;
use argon2::{Argon2, Algorithm, Version, Params};
use bip39::{Mnemonic, Language};
use zeroize::Zeroize;

#[wasm_bindgen]
pub fn process_keys(password: &str, salt: &str, mem_mb: u32) -> String {
    let mem_kb = mem_mb * 1024;

    // Config: Argon2id, 256-bit output
    let params = Params::new(mem_kb, 3, 1, Some(32)).expect("Invalid parameters");
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut hash = [0u8; 32]; 
    
    // Hashing process
    if argon2.hash_password_into(password.as_bytes(), salt.as_bytes(), &mut hash).is_err() {
        return "Error: Hashing failed".to_string();
    }

    // BIP-39 Mnemonic derivation
    let mnemonic = Mnemonic::from_entropy(&hash, Language::English).expect("Entropy error");
    let result = mnemonic.phrase().to_string();

    // Memory cleanup
    hash.zeroize();

    result
}
