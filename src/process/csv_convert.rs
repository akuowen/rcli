use std::fs;

use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use anyhow::Result;

use crate::cli::Format;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
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
    };
    fs::write(output, result_content)?;
    Ok(())
}
