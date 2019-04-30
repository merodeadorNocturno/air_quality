extern crate serde_json;
mod my_fs;
mod air_objects;

use my_fs::load_file::read_file;

use serde_json::Value as JsonValue;

use std::time::Instant;

use air_objects::air_data::AirData;
use air_objects::air_data::UsefulData;

pub fn get_air_values(filepath: &str) -> JsonValue {
    let my_json: String = read_file(filepath);
    let air_quality = serde_json::from_str(&my_json).unwrap();

    air_quality
}

pub fn create_data_vector(data: JsonValue) -> Vec<UsefulData> {
    let mut data_vector: Vec<UsefulData> = Vec::new();
    let air_data_array: Vec<AirData> = serde_json::from_value(data).unwrap();

        for air_data in air_data_array {
        let ad = UsefulData {
            measure_id: air_data.measure_id.to_string().parse().unwrap(),
            measure_name: air_data.measure_name,
            measure_type: air_data.measure_type,
            stratification_level: air_data.stratification_level,
            state_fips: air_data.state_fips.to_string().parse().unwrap(),
            state_name: air_data.state_name,
            county_fips: air_data.county_fips.to_string().parse().unwrap(),
            county_name: air_data.county_name,
            report_year: air_data.report_year.to_string().parse().unwrap(),
            value: air_data.value.to_string().parse().unwrap(),
            unit: air_data.unit,
            unit_name: air_data.unit_name,
            data_origin: air_data.data_origin,
            monitor_only: air_data.monitor_only.to_string().parse().unwrap(),
        };
        data_vector.push(ad);
    }

    data_vector
}

pub fn reducer(my_data: &Vec<UsefulData>) -> f64 {
    let sum_value: f64 = my_data.iter().fold(0.0, |sum, x| sum + x.value);
    let _len = my_data.len();

    let _average_value:f64 = sum_value / _len as f64;

    _average_value
} 

fn main() {
    let init = Instant::now();
    let filepath = "/home/tonatiuh.martinez/Development/rust/air_quality/json_files/airQuality.json";
    let air_json: JsonValue = get_air_values(&filepath);

    // let _county_name: String = "El Paso".to_string();
    // let _report_year: u16 = 2012;
    let _measure_id: u16 = 292;

    let _my_data: Vec<UsefulData> = create_data_vector(air_json["data"].clone())
        .into_iter()
        // .filter(|mi| mi.value > 1.0 && mi.measure_id == _measure_id)
        .filter(|mi| mi.measure_id == _measure_id)
        .collect();

    let _average_value: f64 = reducer(&_my_data);

    println!("Average: {}", _average_value);

    let ended = Instant::now();
    println!("Rust: {:?}", ended.duration_since(init));
    println!("{}", _my_data.len());
}
