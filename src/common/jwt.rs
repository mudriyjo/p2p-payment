use crate::common::error::AppError;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub user_id: Uuid,
    pub role_id: Uuid,
    pub role_name: String,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(user_id: Uuid, role_id: Uuid, role_name: String, expiration_hours: i64) -> Self {
        let now = Utc::now();
        let exp = now + Duration::hours(expiration_hours);

        Self {
            sub: user_id.to_string(),
            user_id,
            role_id,
            role_name: role_name.clone(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.exp
    }

    pub fn is_admin(&self) -> bool {
        self.role_name == "Admin"
    }

    pub fn has_role(&self, role_name: &str) -> bool {
        self.role_name == role_name
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

    pub fn encode_token(
        &self,
        user_id: Uuid,
        role_id: Uuid,
        role_name: String,
    ) -> Result<String, AppError> {
        let claims = Claims::new(user_id, role_id, role_name, 24);
        let token = encode(&Header::default(), &claims, &self.encoding_key)?;
        Ok(token)
    }

    pub fn encode_token_with_expiration(
        &self,
        user_id: Uuid,
        role_id: Uuid,
        role_name: String,
        expiration_hours: i64,
    ) -> Result<String, AppError> {
        let claims = Claims::new(user_id, role_id, role_name, expiration_hours);
        let token = encode(&Header::default(), &claims, &self.encoding_key)?;
        Ok(token)
    }

    pub fn decode_token(&self, token: &str) -> Result<Claims, AppError> {
        let token_data = decode::<Claims>(token, &self.decoding_key, &self.validation)?;

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

pub struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};

            let mut security_scheme = SecurityScheme::ApiKey(
                ApiKey::Header(ApiKeyValue::new("X-JWT-Token"))
            );

            // Add description to the security scheme
            if let SecurityScheme::ApiKey(api_key) = &mut security_scheme {
                *api_key = ApiKey::Header(
                    ApiKeyValue::with_description("X-JWT-Token", "JWT token for authentication. Click 'Authorize' button above to add your token.")
                );
            }

            components.add_security_scheme("jwt_token", security_scheme);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_role_id() -> Uuid {
        Uuid::parse_str("eec86d00-495c-490c-b151-b9d33672a681").unwrap()
    }

    #[test]
    fn test_encode_and_decode_token() {
        let jwt_service = JwtService::new("test_secret_key");
        let user_id = Uuid::new_v4();
        let role_id = test_role_id();

        let token = jwt_service
            .encode_token(user_id, role_id, "User".to_string())
            .unwrap();
        assert!(!token.is_empty());

        let claims = jwt_service.decode_token(&token).unwrap();
        assert_eq!(claims.user_id, user_id);
        assert_eq!(claims.role_id, role_id);
        assert_eq!(claims.role_name, "User");
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
        let role_id = test_role_id();
        let token = jwt_service1
            .encode_token(user_id, role_id, "User".to_string())
            .unwrap();

        let result = jwt_service2.decode_token(&token);
        assert!(result.is_err());
    }

    #[test]
    fn test_claims_expiration() {
        let user_id = Uuid::new_v4();
        let role_id = test_role_id();

        let mut claims = Claims::new(user_id, role_id, "User".to_string(), -1);
        assert!(claims.is_expired());

        claims = Claims::new(user_id, role_id, "User".to_string(), 24);
        assert!(!claims.is_expired());
    }

    #[test]
    fn test_claims_is_admin() {
        let user_id = Uuid::new_v4();
        let admin_role_id = Uuid::parse_str("878c19c6-643b-4a57-98f1-a60786a38a92").unwrap();

        let admin_claims = Claims::new(user_id, admin_role_id, "Admin".to_string(), 24);
        assert!(admin_claims.is_admin());

        let user_claims = Claims::new(user_id, test_role_id(), "User".to_string(), 24);
        assert!(!user_claims.is_admin());
    }

    #[test]
    fn test_claims_has_role() {
        let user_id = Uuid::new_v4();
        let role_id = test_role_id();

        let claims = Claims::new(user_id, role_id, "Support".to_string(), 24);
        assert!(claims.has_role("Support"));
        assert!(!claims.has_role("Admin"));
    }
}
