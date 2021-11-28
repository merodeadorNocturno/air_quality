extern crate serde_json;
mod air_objects;
mod my_fs;

use my_fs::load_file::read_file;

use serde_json::Value as JsonValue;

use std::time::Instant;

use air_objects::air_data::{AirData, UsefulData};

use itertools::Itertools;

mod rust_modules;

use rust_modules::my_path::get_dir_path;
use std::path::PathBuf;


fn get_air_values(filepath: &PathBuf) -> JsonValue {
  let my_json: String = read_file(filepath);
  let air_quality = serde_json::from_str(&my_json).unwrap();

  air_quality
}

fn create_data_vector(data: JsonValue) -> Vec<UsefulData> {
  let air_data_vector: Vec<AirData> = serde_json::from_value(data).unwrap();
  
  let data_vector: Vec<UsefulData> = air_data_vector
    .into_iter()
    .map(|adv| -> UsefulData {
      let useful_data = UsefulData {
        measure_id: adv.measure_id.to_string().parse().unwrap(),
        measure_name: adv.measure_name,
        measure_type: adv.measure_type,
        stratification_level: adv.stratification_level,
        state_fips: adv.state_fips.to_string().parse().unwrap(),
        state_name: adv.state_name,
        county_fips: adv.county_fips.to_string().parse().unwrap(),
        county_name: adv.county_name,
        report_year: adv.report_year.to_string().parse().unwrap(),
        value: adv.value.to_string().parse().unwrap(),
        unit: adv.unit,
        unit_name: adv.unit_name,
        data_origin: adv.data_origin,
        monitor_only: adv.monitor_only.to_string().parse().unwrap(),
      };

      useful_data
    })
    .collect();

  data_vector
}

fn reducer(my_data: &Vec<UsefulData>) -> f64 {
  let sum_value: f64 = my_data.iter().fold(0.0, |sum, x| sum + x.value);
  let _len = my_data.len();

  let _average_value: f64 = sum_value / _len as f64;
  _average_value
}

fn create_measure_vector(data: &Vec<UsefulData>) -> Vec<u16> {

  let unique_measure_ids = data
    .into_iter()
    .map(|item| item.measure_id)
    .unique()
    .collect();

  unique_measure_ids
}

fn create_types_vector(data: &Vec<UsefulData>) -> Vec<String> {
  let my_unique_types = data
    .into_iter()
    .map(|item| item.measure_type.to_string())
    .unique()
    .collect();

  my_unique_types
}

fn filter_by_type(data: &Vec<UsefulData>, my_type: String) -> Vec<UsefulData> {

  let my_reference = data
    .into_iter()
    .filter(|item| item.measure_type.to_string() == my_type)
    .map(|item| -> UsefulData {
      let useful_data = UsefulData {
        measure_id: item.measure_id.to_string().parse().unwrap(),
        measure_name: item.measure_name.to_string(),
        measure_type: item.measure_type.to_string(),
        stratification_level: item.stratification_level.to_string(),
        state_fips: item.state_fips.to_string().parse().unwrap(),
        state_name: item.state_name.to_string(),
        county_fips: item.county_fips.to_string().parse().unwrap(),
        county_name: item.county_name.to_string(),
        report_year: item.report_year.to_string().parse().unwrap(),
        value: item.value.to_string().parse().unwrap(),
        unit: item.unit.to_string(),
        unit_name: item.unit_name.to_string(),
        data_origin: item.data_origin.to_string(),
        monitor_only: item.monitor_only.to_string().parse().unwrap(),
      };

      useful_data
    })
    .collect();

  my_reference
}

fn filter_by_county_name(data: &Vec<UsefulData>, county_name: String) -> Vec<UsefulData> {
  let location = data
    .into_iter()
    .filter(|item| item.county_name.to_string() == county_name)
    .map(|item| -> UsefulData {
      let useful_data = UsefulData {
        measure_id: item.measure_id.to_string().parse().unwrap(),
        measure_name: item.measure_name.to_string(),
        measure_type: item.measure_type.to_string(),
        stratification_level: item.stratification_level.to_string(),
        state_fips: item.state_fips.to_string().parse().unwrap(),
        state_name: item.state_name.to_string(),
        county_fips: item.county_fips.to_string().parse().unwrap(),
        county_name: item.county_name.to_string(),
        report_year: item.report_year.to_string().parse().unwrap(),
        value: item.value.to_string().parse().unwrap(),
        unit: item.unit.to_string(),
        unit_name: item.unit_name.to_string(),
        data_origin: item.data_origin.to_string(),
        monitor_only: item.monitor_only.to_string().parse().unwrap(),
      };
      useful_data
    })
    .collect();

  location
}

fn filter_by_state_name(data: &Vec<UsefulData>, state_name: String) -> Vec<UsefulData> {
  let location = data
    .into_iter()
    .filter(|item| item.state_name.to_string() == state_name && item.unit.to_string() == "µg/m³")
    .map(|item| -> UsefulData {
      let useful_data = UsefulData {
        measure_id: item.measure_id.to_string().parse().unwrap(),
        measure_name: item.measure_name.to_string(),
        measure_type: item.measure_type.to_string(),
        stratification_level: item.stratification_level.to_string(),
        state_fips: item.state_fips.to_string().parse().unwrap(),
        state_name: item.state_name.to_string(),
        county_fips: item.county_fips.to_string().parse().unwrap(),
        county_name: item.county_name.to_string(),
        report_year: item.report_year.to_string().parse().unwrap(),
        value: item.value.to_string().parse().unwrap(),
        unit: item.unit.to_string(),
        unit_name: item.unit_name.to_string(),
        data_origin: item.data_origin.to_string(),
        monitor_only: item.monitor_only.to_string().parse().unwrap(),
      };
      useful_data
    })
    .collect();

  location
}

fn main() {
  let init = Instant::now();
  let filepath = get_dir_path();
  println!("PATH {:?}", filepath);
  let air_json: JsonValue = get_air_values(&filepath);

  let _my_data: Vec<UsefulData> = create_data_vector(air_json["data"].clone());

  let _measure_vector: Vec<u16> = create_measure_vector(&_my_data);
  let _types_vector: Vec<String> = create_types_vector(&_my_data);

  let _counts_vector: Vec<UsefulData> = filter_by_type(&_my_data, "Counts".to_string());
  let _avg_counts: f64 = reducer(&_counts_vector);

  let _pct_vector: Vec<UsefulData> = filter_by_type(&_my_data, "Percent".to_string());
  let _avg_pct: f64 = reducer(&_pct_vector);

  let _avg_vector: Vec<UsefulData> = filter_by_type(&_my_data, "Average".to_string());
  let _avg_avg: f64 = reducer(&_avg_vector);

  let _connecticut: Vec<UsefulData> = filter_by_state_name(&_my_data, "Connecticut".to_string());
  let _avg_connecticut: f64 = reducer(&_connecticut);

  let ended = Instant::now();

  println!("Rust:: {:?}", ended.duration_since(init));
  println!("measure_ids {:?}", &_measure_vector);
  println!("measure_types {:?}", &_types_vector);
  println!(" ");
  println!("\tTotal Counts registers: {}", &_counts_vector.len());
  println!("\tAverage in Counts: {}", &_avg_counts);
  println!(" ");
  println!("\tTotal Percent registers: {}", &_pct_vector.len());
  println!("\tAverage in Percent: {}", &_avg_pct);
  println!(" ");
  println!("\tTotal Average registers: {}", &_avg_vector.len());
  println!("\tAverage in Average: {}", &_avg_avg);
  println!(" ");

  println!("\tAverage in Connecticut: {} µg/m³", &_avg_connecticut);

  println!(" ");
}
