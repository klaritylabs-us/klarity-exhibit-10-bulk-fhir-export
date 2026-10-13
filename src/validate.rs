use serde_json::Value;
use crate::outcome::{ValidationIssue, ValidationReport};

pub fn validate_manifest_json(json: &str) -> Result<ValidationReport, String> {
    let v: Value = serde_json::from_str(json).map_err(|e| format!("JSON: {e}"))?;
    let mut r = ValidationReport::ok();
    if v.get("resourceType").and_then(|x| x.as_str()) != Some("Bundle") {
        r.push(ValidationIssue::error("resourceType", "export manifest should be a Bundle", "manifest-bundle"));
    }
    if v.get("type").and_then(|x| x.as_str()) != Some("document") {
        r.push(ValidationIssue::warning("type", "bulk export manifest often uses type document", "manifest-type"));
    }
    let has_output = v.get("link").and_then(|l| l.as_array()).is_some_and(|links| {
        links.iter().any(|l| l.get("relation").and_then(|x| x.as_str()) == Some("output"))
    });
    if !has_output {
        r.push(ValidationIssue::error("link", "manifest needs output link to NDJSON files", "output-link"));
    }
    Ok(r)
}

pub fn validate_ndjson_line(line: &str) -> ValidationReport {
    let mut r = ValidationReport::ok();
    let line = line.trim();
    if line.is_empty() { r.push(ValidationIssue::error("line", "empty NDJSON line", "empty-line")); return r; }
    if serde_json::from_str::<serde_json::Value>(line).is_err() {
        r.push(ValidationIssue::error("line", "NDJSON line must be one JSON object", "ndjson-parse"));
    }
    r
}
