#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Merchant {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: MerchantStatus,
    pub sites: Vec<Site>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MerchantStatus {
    Active,
    Onboarding,
    Inactive,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Site {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub name: String,
    pub url: String,
    pub status: SiteStatus,
    pub redirectSuccessUrl: Option<String>,
    pub redirectFailureUrl: Option<String>,
    pub credentials: Option<SiteCredentials>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SiteStatus {
    Active,
    Inactive,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SiteCredentials {
    pub id: Uuid,
    pub site_id: Uuid,
    pub api_key: String,
    pub api_secret: String,
    pub AllowedIps: Vec<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}