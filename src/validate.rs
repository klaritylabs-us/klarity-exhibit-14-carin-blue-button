use serde_json::Value;
use crate::outcome::{ValidationIssue, ValidationReport};

pub fn validate_carin_json(json: &str) -> Result<ValidationReport, String> {
    let v: Value = serde_json::from_str(json).map_err(|e| format!("JSON: {e}"))?;
    let mut r = ValidationReport::ok();
    if v.get("resourceType").and_then(|x| x.as_str()) != Some("ExplanationOfBenefit") {
        r.push(ValidationIssue::error("resourceType", "CARIN consumer flows often use ExplanationOfBenefit", "carin-eob"));
    }
    if v.pointer("/patient/reference").is_none() {
        r.push(ValidationIssue::error("patient", "patient reference required", "patient-ref"));
    }
    Ok(r)
}
