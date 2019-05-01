extern crate serde_json;
mod my_fs;
mod air_objects;

use my_fs::load_file::read_file;

use serde_json::Value as JsonValue;

use std::time::Instant;

use air_objects::air_data::AirData;
use air_objects::air_data::UsefulData;

use itertools::Itertools;

fn get_air_values(filepath: &str) -> JsonValue {
    let my_json: String = read_file(filepath);
    let air_quality = serde_json::from_str(&my_json).unwrap();

    air_quality
}

fn create_data_vector(data: JsonValue) -> Vec<UsefulData> {
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

fn reducer(my_data: &Vec<UsefulData>) -> f64 {
    let sum_value: f64 = my_data.iter().fold(0.0, |sum, x| sum + x.value);
    let _len = my_data.len();

    let _average_value:f64 = sum_value / _len as f64;
    _average_value
}

fn create_measure_vector(data: &Vec<UsefulData>) -> Vec<u16> {
    let mut measure_ids = Vec::new();

    for item in data {
        measure_ids.push(item.measure_id);
    }

    let unique_measure_ids = measure_ids.into_iter().unique().collect();

    unique_measure_ids
}

fn create_types_vector(data: &Vec<UsefulData>) -> Vec<String> {
    let mut types = Vec::new();

    for item in data {
        types.push(item.measure_type.to_string());
    }

    let my_unique_types = types.into_iter().unique().collect();

    my_unique_types
}

fn filter_by_type(data: &Vec<UsefulData>, my_type: String) -> Vec<UsefulData> {
    let mut my_reference = Vec::new();

    for air_data in data {
        if air_data.measure_type.to_string() == my_type {
            let ad = UsefulData {
                measure_id: air_data.measure_id.to_string().parse().unwrap(),
                measure_name: air_data.measure_name.to_string(),
                measure_type: air_data.measure_type.to_string(),
                stratification_level: air_data.stratification_level.to_string(),
                state_fips: air_data.state_fips.to_string().parse().unwrap(),
                state_name: air_data.state_name.to_string(),
                county_fips: air_data.county_fips.to_string().parse().unwrap(),
                county_name: air_data.county_name.to_string(),
                report_year: air_data.report_year.to_string().parse().unwrap(),
                value: air_data.value.to_string().parse().unwrap(),
                unit: air_data.unit.to_string(),
                unit_name: air_data.unit_name.to_string(),
                data_origin: air_data.data_origin.to_string(),
                monitor_only: air_data.monitor_only.to_string().parse().unwrap(),
            };
            my_reference.push(ad);
        }
    }

    my_reference

}

fn main() {
    let init = Instant::now();
    let filepath = "/home/tonatiuh.martinez/Development/rust/air_quality/json_files/airQuality.json";
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
    println!("\tTotal Average registers {}:", &_avg_vector.len());
    println!("\tAverage in Average: {}", &_avg_avg);
    println!(" ");


}