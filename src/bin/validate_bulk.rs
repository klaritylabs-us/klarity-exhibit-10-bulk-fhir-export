use std::{env, fs, process::ExitCode};
use bulk_export::validate_manifest_json;

fn main() -> ExitCode {
    let path = env::args().nth(1).expect("usage: validate-bulk <file>");
    let json = fs::read_to_string(path).unwrap();
    let report = validate_manifest_json(&json).unwrap();
    println!("{}", serde_json::to_string_pretty(&report.issues).unwrap());
    ExitCode::from(if report.is_valid() { 0 } else { 1 })
}
