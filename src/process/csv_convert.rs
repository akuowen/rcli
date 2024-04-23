use std::fs;

use crate::cli::Format;
use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct Data {
    data: Vec<Value>,
}

pub fn process_csv(input: &str, output: &str, output_format: Format) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut result_vec = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    // let mut ret = Vec::with_capacity(128);
    for ele in reader.records() {
        let record = ele?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        result_vec.push(json_value)
    }

    let result_content = match output_format {
        Format::Json => serde_json::to_string_pretty(&result_vec)?,
        Format::Yaml => serde_yaml::to_string(&result_vec)?,
        Format::Toml => {
            let data = Data { data: result_vec };
            toml::to_string_pretty(&data)?
        }
    };
    fs::write(output, result_content)?;
    Ok(())
}
