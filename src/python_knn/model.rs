use crate::route_request::BLEReading;
use diesel::IntoSql;
use serde::{Deserialize, Serialize};
use serde_json::{Error, Value};

use std::{process::Command};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RawCoordinate {
    pub x: i32,
    pub y: i32,
}

pub fn classify(source: Vec<BLEReading>, dataset: &str) -> Result<RawCoordinate, Error> {
     let py_exe = dotenv::var("PY_EXE").expect("Python path not set");
     let py_train = dotenv::var("PY_TRAIN").expect("Python  script not set");
     let dataset_dir = dotenv::var("DATASET_FOLDER").expect("Dataset folders path not set");

     let full_dataset = dataset_dir.as_str().to_owned() + dataset;

    let output = Command::new("cmd")
        .args(&[
            "/C",
            &py_exe,
            &py_train,
            serde_json::to_string(&source).unwrap().as_str(),
            &full_dataset
        ])
        .output()
        .expect("failed to execute process");
    let response = String::from_utf8(output.stdout).unwrap();
    let j: Vec<&str> = response.split("\r\n").collect::<Vec<&str>>();
    return match serde_json::from_str(j[1]) {
        Ok(val) => {
            return match serde_json::from_value(val) {
                Ok(predicted) => Ok(predicted),
                Err(e) => Err(e),
            }
            
        }
        Err(e) => Err(e),
    };
}
