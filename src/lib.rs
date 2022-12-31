extern "C" {
    fn ige256(data: *const u8, length: u32, key: *const u8, iv: *const u8, encrypt: u8) -> *mut u8;
}

#[no_mangle]
pub fn ige256_encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> *mut u8 {
    unsafe {
        return ige256(
            data.as_ptr(),
            data.len() as u32,
            key.as_ptr(),
            iv.as_ptr(),
            1,
        );
    };
}

#[no_mangle]
pub fn ige256_decrypt(ige_encrypted: &[u8], key: &[u8], iv: &[u8]) -> *const u8 {
    unsafe {
        return ige256(
            ige_encrypted.as_ptr(),
            ige_encrypted.len() as u32,
            key.as_ptr(),
            iv.as_ptr(),
            0,
        );
    };
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_ige256_encrypt_decrypt() {
        let data = Vec::with_capacity(1024 * 1024 + 7);
        let key = [8; 32];
        let iv = [16; 32];
        let ige_encrypted = ige256_encrypt(&data, &key, &iv);
        unsafe {
            let ige_encrypted = *ige_encrypted;
            println!("ige_encrypted: {}", ige_encrypted);
            let ige_decrypted = ige256_decrypt(&ige_encrypted.to_be_bytes(), &key, &iv);
            let ige_decrypted = *ige_decrypted;
            println!("ige_decrypted: {}", ige_decrypted);
            assert_eq!(ige_encrypted, ige_decrypted)
        };
    }
}
