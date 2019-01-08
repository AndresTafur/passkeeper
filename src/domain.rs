use base64::decode;
use base64::encode;

use util::{derive_key, do_decrypt, do_encrypt};
use std::vec::Vec;

#[derive(Clone)]
pub struct KeeperCipher {
    password: String,
}

impl KeeperCipher {
    pub fn new(password: String) -> KeeperCipher {
        Self { password: password }
    }

    pub fn encrypt(&self, resource: &String) -> Result<String, String> {
        let mut buffer = KeeperCipher::gen_salt();
        let (key, ivector) = derive_key(&buffer, &self.password);
        let result = do_encrypt(resource.as_bytes(), &key, &ivector);

        result
            .map(move |mut v| {
                buffer.append(&mut v);
                encode(&buffer)
            })
            .map_err(|_| "Error encrypting".to_string())
    }

    pub fn decrypt(&self, resource: &String) -> Result<String, String> {
        let decoded = decode(resource).unwrap();
        let salt = &decoded[..8];
        let data = &decoded[8..];
        let (key, ivector) = derive_key(&salt.to_vec(), &self.password);

        do_decrypt(&data, &key, &ivector)
            .map(|v| String::from_utf8(v).unwrap())
            .map_err(|_| "Error decrypting".to_string())
    }

    fn gen_salt() -> Vec<u8> {
        b"somesalt".to_vec()
    }
}

#[cfg(test)]
mod tests {

    use domain::KeeperCipher;

    #[test]
    fn encrypt_test() {
        let cipher = KeeperCipher::new("Test_AES_PASS".to_string());
        let encrypted = cipher.encrypt(&"1234".to_string()).unwrap();
        assert_eq!(encrypted, "c29tZXNhbHTkdipHPoe4wIJsjERyH2yg");
    }

    #[test]
    fn decrypt_test() {
        let cipher = KeeperCipher::new("Test_AES_PASS".to_string());
        let decrypted = cipher
            .decrypt(&"c29tZXNhbHTkdipHPoe4wIJsjERyH2yg".to_string())
            .unwrap();
        assert_eq!(decrypted, "1234");
    }

}
