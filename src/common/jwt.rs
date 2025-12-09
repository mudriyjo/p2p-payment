use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::common::error::AppError;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub user_id: Uuid,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(user_id: Uuid, expiration_hours: i64) -> Self {
        let now = Utc::now();
        let exp = now + Duration::hours(expiration_hours);

        Self {
            sub: user_id.to_string(),
            user_id,
            exp: exp.timestamp(),
            iat: now.timestamp(),
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.exp
    }
}

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation: Validation,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        let encoding_key = EncodingKey::from_secret(secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());
        let validation = Validation::default();

        Self {
            encoding_key,
            decoding_key,
            validation,
        }
    }

    pub fn encode_token(&self, user_id: Uuid) -> Result<String, AppError> {
        let claims = Claims::new(user_id, 24);       
        let token = encode(&Header::default(), &claims, &self.encoding_key)?;
        Ok(token)
    }

    pub fn encode_token_with_expiration(
        &self,
        user_id: Uuid,
        expiration_hours: i64,
    ) -> Result<String, AppError> {
        let claims = Claims::new(user_id, expiration_hours);
        let token = encode(&Header::default(), &claims, &self.encoding_key)?;
        Ok(token)
    }

    pub fn decode_token(&self, token: &str) -> Result<Claims, AppError> {
        let token_data = decode::<Claims>(
            token,
            &self.decoding_key,
            &self.validation,
        )?;

        if token_data.claims.is_expired() {
            return Err(AppError::Unauthorized("Token has expired".to_string()));
        }

        Ok(token_data.claims)
    }

    pub fn validate_token(&self, token: &str) -> Result<bool, AppError> {
        match self.decode_token(token) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_and_decode_token() {
        let jwt_service = JwtService::new("test_secret_key");
        let user_id = Uuid::new_v4();

        let token = jwt_service.encode_token(user_id).unwrap();
        assert!(!token.is_empty());

        let claims = jwt_service.decode_token(&token).unwrap();
        assert_eq!(claims.user_id, user_id);
        assert!(!claims.is_expired());
    }

    #[test]
    fn test_invalid_token() {
        let jwt_service = JwtService::new("test_secret_key");
        let invalid_token = "invalid.token.here";

        let result = jwt_service.decode_token(invalid_token);
        assert!(result.is_err());
    }

    #[test]
    fn test_token_with_different_secret() {
        let jwt_service1 = JwtService::new("secret1");
        let jwt_service2 = JwtService::new("secret2");
        
        let user_id = Uuid::new_v4();
        let token = jwt_service1.encode_token(user_id).unwrap();

        let result = jwt_service2.decode_token(&token);
        assert!(result.is_err());
    }

    #[test]
    fn test_claims_expiration() {
        let user_id = Uuid::new_v4();
        
        let mut claims = Claims::new(user_id, -1);
        assert!(claims.is_expired());

        claims = Claims::new(user_id, 24);
        assert!(!claims.is_expired());
    }
}