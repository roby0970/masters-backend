use crate::{route_request::BLEReading};
use serde::{Deserialize, Serialize};
use serde_json::Error;

use std::process::Command;

const PYEXE: &str = "C:/Users/Robi/AppData/Local/Programs/Python/Python38/python.exe";
const PYSCRIPT: &str =
    "c:/diplomski/blefingerprinting android/convert data to fluter class/train.py";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RawCoordinate {
    pub x: i32,
    pub y: i32,
}

pub fn classify(source: Vec<BLEReading>) -> Result<RawCoordinate, Error> {
    let output = Command::new("cmd")
        .args(&[
            "/C",
            &PYEXE,
            &PYSCRIPT,
            serde_json::to_string(&source).unwrap().as_str(),
        ])
        .output()
        .expect("failed to execute process");
    let response = String::from_utf8(output.stdout).unwrap();
    let j: Vec<&str> = response.split("\r\n").collect::<Vec<&str>>();
    let predicted_coordinate: Result<RawCoordinate, Error> =
        serde_json::from_value(serde_json::from_str(j[1]).unwrap());
    predicted_coordinate
}
