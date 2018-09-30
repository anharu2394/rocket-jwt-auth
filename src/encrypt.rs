extern crate fernet;

use self::fernet::Fernet;
use self::fernet::DecryptionError;

fn fernet() -> Fernet {
    let key = fernet::Fernet::generate_key();
    fernet::Fernet::new(&key).unwrap()
}
pub fn encrypt(secret_phrase: &String) -> String {
    fernet().encrypt(secret_phrase.as_bytes())  
}
pub fn decrypt(token: &String) -> Result<Vec<u8>, DecryptionError> {
    fernet().decrypt(token)
}
