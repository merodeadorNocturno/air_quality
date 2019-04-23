extern crate serde_json;

use serde::Deserialize;
use serde_json::Value as JsonValue;
// use serde_json::Value::Null;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use std::time::Instant;

#[derive(Deserialize)]
pub struct AirData {
    sid: String,
    id: String,
    position: u16,
    created_at: u32,
    created_meta: serde_json::Value,
    updated_at: u32,
    updated_meta: serde_json::Value,
    meta: String,
    measure_id: String,
    measure_name: String,
    measure_type: String,
    stratification_level: String,
    state_fips: String,
    state_name: String,
    county_fips: String,
    county_name: String,
    report_year: String,
    value: String,
    unit: String,
    unit_name: String,
    data_origin: String,
    monitor_only: String,
}

pub struct UsefulData {
    measure_id: String,
    measure_name: String,
    measure_type: String,
    stratification_level: String,
    state_fips: String,
    state_name: String,
    county_fips: String,
    county_name: String,
    report_year: String,
    value: String,
    unit: String,
    unit_name: String,
    data_origin: String,
    monitor_only: String,
}

pub fn read_file(filepath: &str) -> String {
    let file = File::open(filepath).expect("could not open file::");
    let mut buffered_reader = BufReader::new(file);
    let mut contents = String::new();
    let _number_of_bytes: usize = match buffered_reader.read_to_string(&mut contents) {
        Ok(_number_of_bytes) => _number_of_bytes,
        Err(_err) => 0,
    };

    contents
}

pub fn get_air_values(filepath: &str) -> JsonValue {
    let my_json = read_file(filepath);
    let air_quality = serde_json::from_str(&my_json).unwrap();

    air_quality
}

// pub fn create_column_vector(columns: JsonValue) -> Vec<String> {
//     let mut column_vector: Vec<String> = Vec::new();

//     for index in 0..21 {
//         column_vector.push(columns[index]["name"].to_string());
//     }

//     column_vector
// }

pub fn create_data_vector(data: JsonValue) -> Vec<UsefulData> {
    let mut data_vector: Vec<UsefulData> = Vec::new();
    let air_data_array: Vec<AirData> = serde_json::from_value(data).unwrap();

        for air_data in air_data_array {
        let ad = UsefulData {
            measure_id: air_data.measure_id,
            measure_name: air_data.measure_name,
            measure_type: air_data.measure_type,
            stratification_level: air_data.stratification_level,
            state_fips: air_data.state_fips.to_string().parse().unwrap(),
            state_name: air_data.state_name,
            county_fips: air_data.county_fips.to_string().parse().unwrap(),
            county_name: air_data.county_name,
            report_year: air_data.report_year.to_string().parse().unwrap(),
            value: air_data.value,
            unit: air_data.unit,
            unit_name: air_data.unit_name,
            data_origin: air_data.data_origin,
            monitor_only: air_data.monitor_only.to_string().parse().unwrap(),
        };
        data_vector.push(ad);
    }

    data_vector
}

pub fn join_data(my_data: Vec<AirData>) -> Vec<UsefulData> {
    let mut _my_data: Vec<UsefulData> = Vec::new();

    for _data in my_data {
        // println!("{}", _data.measure_name);
        let useful_data = UsefulData {
            measure_id: _data.measure_id,
            measure_name: _data.measure_name,
            measure_type: _data.measure_type,
            stratification_level: _data.stratification_level,
            state_fips: _data.state_fips,
            state_name: _data.state_name,
            county_fips: _data.county_fips,
            county_name: _data.county_name,
            report_year: _data.report_year,
            value: _data.value,
            unit: _data.unit,
            unit_name: _data.unit_name,
            data_origin: _data.data_origin,
            monitor_only: _data.monitor_only,
        };
        _my_data.push(useful_data);
    }

    _my_data
}

fn main() {
    let init = Instant::now();
    let filepath = "/home/tonatiuh.martinez/Development/rust/air_quality/json_files/airQuality.json";
    let air_json: JsonValue = get_air_values(&filepath);

    // let _columns = create_column_vector(air_json["meta"]["view"]["columns"].clone());
    let data = &air_json["data"];

    // println!("{}", _columns[0]);

    let _my_data = create_data_vector(data.clone());

    // join_data(_my_data);

    let ended = Instant::now();
    println!("Rust: {:?}", ended.duration_since(init));
}
