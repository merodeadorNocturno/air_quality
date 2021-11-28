pub mod air_data {
  use serde::{Deserialize, Serialize};

  #[derive(Serialize, Deserialize)]
  pub struct AirData {
    pub sid: String,
    pub id: String,
    pub position: u16,
    pub created_at: u32,
    pub created_meta: serde_json::Value,
    pub updated_at: u32,
    pub updated_meta: serde_json::Value,
    pub meta: String,
    pub measure_id: String,
    pub measure_name: String,
    pub measure_type: String,
    pub stratification_level: String,
    pub state_fips: String,
    pub state_name: String,
    pub county_fips: String,
    pub county_name: String,
    pub report_year: String,
    pub value: String,
    pub unit: String,
    pub unit_name: String,
    pub data_origin: String,
    pub monitor_only: String,
  }

  #[derive(Debug)]
  pub struct StateUsefulData {
    pub state: String,
    pub useful_data: Vec<UsefulData>,
  }

  #[derive(Debug)]
  pub struct UsefulData {
    pub measure_id: u16,
    pub measure_name: String,
    pub measure_type: String,
    pub stratification_level: String,
    pub state_fips: u16,
    pub state_name: String,
    pub county_fips: u16,
    pub county_name: String,
    pub report_year: u16,
    pub value: f64,
    pub unit: String,
    pub unit_name: String,
    pub data_origin: String,
    pub monitor_only: u8,
  }
}
