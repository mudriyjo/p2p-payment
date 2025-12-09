use bcrypt::{hash, verify, DEFAULT_COST};
use crate::common::error::AppError;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let hashed = hash(password, DEFAULT_COST)?;
    Ok(hashed)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let is_valid = verify(password, hash)?;
    Ok(is_valid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_password() {
        let password = "my_secure_password123";
        let hashed = hash_password(password).unwrap();

        assert_ne!(hashed, password);
        assert!(verify_password(password, &hashed).unwrap());
        assert!(!verify_password("wrong_password", &hashed).unwrap());
    }

    #[test]
    fn test_different_hashes_for_same_password() {
        let password = "test123";
        
        let hash1 = hash_password(password).unwrap();
        let hash2 = hash_password(password).unwrap();
        
        assert_ne!(hash1, hash2);
        assert!(verify_password(password, &hash1).unwrap());
        assert!(verify_password(password, &hash2).unwrap());
    }
}