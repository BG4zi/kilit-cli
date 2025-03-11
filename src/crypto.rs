use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_padding::Pkcs7;
use base64::{encode, decode};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::str;
use rand::Rng; // To generate random IV


type Aes128Cbc = Cbc<Aes128, Pkcs7>;

fn generate_key(password: &str, key_size: usize) -> Vec<u8> {
    let mut key = vec![0u8; key_size];
    let password_bytes = password.as_bytes();
    for (i, byte) in password_bytes.iter().cycle().enumerate() {
        if i >= key_size {
            break;
        }
        key[i] = *byte;
    }
    key
}

pub fn encrypt_data(data: &[u8], password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let key = generate_key(password, 16);

    // Generate a random IV
    let mut iv = [0u8; 16];
    rand::rng().fill(&mut iv); // Fill IV with random bytes

    let cipher = Aes128Cbc::new_from_slices(&key, &iv)?;
    let encrypted_data = cipher.encrypt_vec(data);

    // Combine IV and encrypted data
    let mut combined_data = Vec::new();
    combined_data.extend_from_slice(&iv); // Store IV first
    combined_data.extend_from_slice(&encrypted_data); // Store encrypted bytes

    // Return Base64-encoded (IV + Ciphertext)
    Ok(encode(&combined_data))
}

pub fn decrypt_data(encrypted_data: &str, password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let key = generate_key(password, 16);
    let combined_data = decode(encrypted_data).expect("Base64 decode failed");

    if combined_data.len() < 16 {
        return Err("Ciphertext too short to contain an IV".into());
    }

    // Extract IV (first 16 bytes) and ciphertext (remaining bytes)
    let iv = &combined_data[0..16];
    let encrypted_bytes = &combined_data[16..];


    let cipher = Aes128Cbc::new_from_slices(&key, iv)?;
    let decrypted_data = cipher.decrypt_vec(encrypted_bytes)?;

    Ok(str::from_utf8(&decrypted_data).expect("UTF-8 conversion failed").to_string())
}

pub fn save_encrypted_to_file(encrypted_data: &str, filename: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(filename)?;
    writeln!(file, "{}", encrypted_data)?;
    Ok(())
}

pub fn load_encrypted_from_file(filename: &str) -> std::io::Result<Vec<String>> {
    let mut file = File::open(filename)?;
    let mut encrypted_data = String::new();
    file.read_to_string(&mut encrypted_data)?;

    // Split into lines, filter out empty ones
    let lines: Vec<String> = encrypted_data.split('\n')
        .filter(|s| !s.is_empty()) // Corrected filter
        .map(|s| s.to_string())
        .collect();

    Ok(lines)
}

pub fn new_data(data: &str, password: &str, filename: &str) {
    // Encrypt the data
    match encrypt_data(data.as_bytes(), password) {
        Ok(encrypted_data) => {

            // Save the encrypted data to a file
            if let Err(e) = save_encrypted_to_file(&encrypted_data, filename) {
                eprintln!("Failed to save encrypted data: {}", e);
            }
        }
        Err(e) => {
            eprintln!("ERROR: {e}");
        }
    }
}

/*
fn main() {
    let password = "my_secret_password";
    let data = "This.!!!{}{}{()()xx is the data to encrypt.";

    new_data(data, password, "kilit");
    // Load previously saved encrypted data and decrypt it
    let lines = load_encrypted_from_file("kilit").unwrap();
    for enc in lines {
		  println!("DEC: {datum}", datum = decrypt_data(&enc, password).unwrap());
    }

    // Encrypt and save new data
}

*/
