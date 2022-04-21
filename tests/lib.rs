use itertools::izip;
use itertools::Itertools;
use std::fs;
use std::process::Command;
use serde_json::from_str;
use serde_json::{Result, Value};

#[test]
fn test_report() {
    assert!(Command::new("bash")
        .arg("-c")
        .arg("target/debug/seqcomplexity -q tests/resources/example.fastqsanger > /tmp/report.json")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success());

    let result = fs::read_to_string("/tmp/report.json").unwrap();
    let expected = include_str!("expected/report.json");

    let v = serde_json::from_str(&result);
    let w = serde_json::from_str(&expected);

    //assert_eq!(v["complexity"]["value"], w["complexity"]["value"]);

}
