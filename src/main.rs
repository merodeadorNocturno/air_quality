extern crate serde_json;

use serde::Deserialize;
use serde_json::Value as JsonValue;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use std::time::Instant;

// #[derive(Deserialize)]
// struct AirDataArray {
//     data: Vec<AirData>,
// }

#[derive(Deserialize)]
pub struct AirData {
    measure_id: String,
    measure_name: String,
    measure_type: String,
    stratification_level: String,
    state_fips: u16,
    state_name: String,
    county_fips: u16,
    county_name: String,
    report_year: u16,
    value: String,
    unit: String,
    unit_name: String,
    data_origin: String,
    monitor_only: u16,
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

pub fn create_column_vector(columns: JsonValue) -> Vec<String> {
    let mut column_vector: Vec<String> = Vec::new();

    for index in 0..21 {
        column_vector.push(columns[index]["name"].to_string());
    }

    column_vector
}

pub fn create_data_vector(data: JsonValue) -> Vec<AirData> {
    let mut data_vector: Vec<AirData> = Vec::new();
    let air_data_array: Vec<AirData> = serde_json::from_value(data).unwrap();

    for air_data in air_data_array {
        let ad = AirData {
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

fn main() {
    let init = Instant::now();
    let filepath = "/Users/pills/Development/rust/air_quality/json_files/sample.json";
    let air_json: JsonValue = get_air_values(&filepath);

    let columns = create_column_vector(air_json["meta"]["view"]["columns"].clone());
    let data = &air_json["data"];

    // for column in columns {
    //     println!("{}", column);
    // }

    let my_data = create_data_vector(data.clone());

    for data in my_data {
        println!("{}", data.county_name);
    }

    let ended = Instant::now();
    println!("Rust: {:?}", ended.duration_since(init));
}