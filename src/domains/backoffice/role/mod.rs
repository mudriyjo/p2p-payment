pub mod model;
pub mod repository;
pub mod entity;
pub mod repository_impl;

// Re-export commonly used items
pub use model::{Role, ADMIN_ROLE_ID, SUPPORT_ROLE_ID, RISK_ROLE_ID, FINANCE_ROLE_ID, USER_ROLE_ID};
pub use model::{admin_role_id, support_role_id, risk_role_id, finance_role_id, user_role_id};
pub use repository::RoleRepository;
pub use repository_impl::PostgresRoleRepository;
