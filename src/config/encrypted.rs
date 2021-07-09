use aes::{Aes128, BlockDecrypt};
use aes::cipher::{BlockEncrypt, NewBlockCipher,
    generic_array::GenericArray,
};
use simple_error::SimpleError;

pub fn aes_128_ecb_encrypt(key: &str, value: &str) -> Result<String, simple_error::SimpleError> {
    let size  = 16;
    if key.len() < size {
        return Err(SimpleError::new("Expected an encryption key of at least 16 bytes"))
    }

    let part_of_key = &key.as_bytes()[..size];
    let cipher = Aes128::new(&GenericArray::from_slice(part_of_key));
    let plaintext_bytes = pkcs5_padding(value.as_bytes(), size);
    let mut encrypted: Vec<u8> = vec![0; plaintext_bytes.len()];

    let mut block_start = 0;
    let mut block_end = size;

    let mut block;
    loop {
        block = GenericArray::clone_from_slice(&plaintext_bytes[block_start..block_end]);
        cipher.encrypt_block(&mut block);

        encrypted.splice(block_start..block_end, block.iter().cloned());

        block_start += size;
        block_end += size;

        // Check if we can exit the loop
        if block_start >= encrypted.len() {
            break;
        }
    }

    let encrypted_string = base64::encode(encrypted);
    Ok(encrypted_string)
}

pub fn aes_128_ecb_decrypt(key: &str, encrypted_string: &str) -> Result<String, simple_error::SimpleError> {
    let size  = 16;
    if key.len() < size {
        return Err(SimpleError::new("Expected an encryption key of at least 16 bytes"))
    }

    let encrypted = base64::decode(encrypted_string).map_err(|e| {
        let error_message = format!("Could not base64 decode encrypted string: {}", e);
        return SimpleError::new(error_message)
    })?;

    let part_of_key = &key.as_bytes()[..size];
    let cipher = Aes128::new(&GenericArray::from_slice(part_of_key));
    let mut decrypted: Vec<u8> = vec![0; encrypted.len()];

    let mut block_start = 0;
    let mut block_end = size;

    let mut block;
    loop {
        block = GenericArray::clone_from_slice(&encrypted[block_start..block_end]);
        cipher.decrypt_block(&mut block);

        decrypted.splice(block_start..block_end, block.iter().cloned());

        block_start += size;
        block_end += size;

        // Check if we can exit the loop
        if block_start >= encrypted.len() {
            break;
        }
    }

    let decrypted_bytes = pkcs5_unpadding(&decrypted[..]);
    let decrypted_string = String::from_utf8(decrypted_bytes).map_err(|e| {
        let error_message = format!("Could not parse decrypted value into a string: {}", e);
        return SimpleError::new(error_message)
    })?;

    Ok(decrypted_string)
}

fn pkcs5_padding(data: &[u8], block_size: usize) -> Vec<u8> {
    let padding = block_size - (data.len() % block_size);
    let mut data_vec = Vec::from(data);
    let padding_vec: Vec<u8> = vec![padding as u8; padding];
    data_vec.extend(padding_vec);

    data_vec
}

fn pkcs5_unpadding(data: &[u8]) -> Vec<u8> {
    let length = data.len();
    let padding = data.last().unwrap().clone() as usize;

    let data_vec = Vec::from(&data[..(length-padding)]);
    data_vec
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_if_decryption_works() {
        use super::aes_128_ecb_decrypt;

        let key = "rNu7RWwJGnBN7GgI1TlyVvysAG6ELBeb";
        let encrypted_str = "6deMja63wuUxVeDncdUBog==";

        let decrypted = aes_128_ecb_decrypt(key, encrypted_str);
        assert_eq!(decrypted.is_ok(), true);
        let decrypted_value = decrypted.unwrap();
        assert_eq!(decrypted_value, "testing");
    }

    #[test]
    fn test_if_encryption_works() {
        use super::aes_128_ecb_encrypt;

        let key = "rNu7RWwJGnBN7GgI1TlyVvysAG6ELBeb";
        let plain_str = "testing";

        let encrypted = aes_128_ecb_encrypt(key, plain_str);
        assert_eq!(encrypted.is_ok(), true);
        let encrypted_value = encrypted.unwrap();
        assert_eq!(encrypted_value, "6deMja63wuUxVeDncdUBog==");
    }
}
