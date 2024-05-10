use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub castka: f64,
    pub mena: String,
    pub datum: DateTime<Utc>,
    pub typ_platby: String,
    pub seznam_polozek: Vec<String>,
}