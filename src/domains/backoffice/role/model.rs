use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const ADMIN_ROLE_ID: &str = "878c19c6-643b-4a57-98f1-a60786a38a92";
pub const SUPPORT_ROLE_ID: &str = "e79d6652-5efb-43ae-9565-04b3d3fcfc0f";
pub const RISK_ROLE_ID: &str = "48cd5981-0e75-4329-8e1d-57681e8715db";
pub const FINANCE_ROLE_ID: &str = "2e457833-9393-4a8f-9c0e-4314e1425312";
pub const USER_ROLE_ID: &str = "eec86d00-495c-490c-b151-b9d33672a681";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Role {
    pub role_id: Uuid,
    pub role_name: String,
    pub role_description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Role {
    pub fn new(
        role_id: Uuid,
        role_name: String,
        role_description: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            role_id,
            role_name,
            role_description,
            created_at,
            updated_at,
        }
    }

    // Helper methods to check role type
    pub fn is_admin(&self) -> bool {
        self.role_id == Uuid::parse_str(ADMIN_ROLE_ID).unwrap()
    }

    pub fn is_support(&self) -> bool {
        self.role_id == Uuid::parse_str(SUPPORT_ROLE_ID).unwrap()
    }

    pub fn is_risk(&self) -> bool {
        self.role_id == Uuid::parse_str(RISK_ROLE_ID).unwrap()
    }

    pub fn is_finance(&self) -> bool {
        self.role_id == Uuid::parse_str(FINANCE_ROLE_ID).unwrap()
    }

    pub fn is_user(&self) -> bool {
        self.role_id == Uuid::parse_str(USER_ROLE_ID).unwrap()
    }
}

// Helper functions to create role UUID constants
pub fn admin_role_id() -> Uuid {
    Uuid::parse_str(ADMIN_ROLE_ID).unwrap()
}

pub fn support_role_id() -> Uuid {
    Uuid::parse_str(SUPPORT_ROLE_ID).unwrap()
}

pub fn risk_role_id() -> Uuid {
    Uuid::parse_str(RISK_ROLE_ID).unwrap()
}

pub fn finance_role_id() -> Uuid {
    Uuid::parse_str(FINANCE_ROLE_ID).unwrap()
}

pub fn user_role_id() -> Uuid {
    Uuid::parse_str(USER_ROLE_ID).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_type_checkers() {
        let admin_role = Role::new(
            admin_role_id(),
            "Admin".to_string(),
            Some("Full system access".to_string()),
            Utc::now(),
            Utc::now(),
        );

        assert!(admin_role.is_admin());
        assert!(!admin_role.is_user());
        assert!(!admin_role.is_support());

        let user_role = Role::new(
            user_role_id(),
            "User".to_string(),
            Some("Standard user access".to_string()),
            Utc::now(),
            Utc::now(),
        );

        assert!(user_role.is_user());
        assert!(!user_role.is_admin());
    }

    #[test]
    fn test_role_constants() {
        assert_eq!(
            admin_role_id().to_string(),
            "878c19c6-643b-4a57-98f1-a60786a38a92"
        );
        assert_eq!(
            user_role_id().to_string(),
            "eec86d00-495c-490c-b151-b9d33672a681"
        );
    }
}
