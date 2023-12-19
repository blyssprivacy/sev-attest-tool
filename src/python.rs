use pyo3::prelude::*;

use crate::verify_attestation::verify_attestation_report as verify_attestation_report_raw;
use crate::verify_attestation::{SAMPLE_ATTESTATION, SAMPLE_VCEK};

/// Verifies an attestation report, using the provided report JSON string and VCEK bytes.
#[pyfunction]
pub fn verify_attestation_report(
    report_json: &str,
    vcek_bytes: &[u8],
    fail_on_purpose: Option<bool>,
) -> PyResult<()> {
    verify_attestation_report_raw(report_json, vcek_bytes, fail_on_purpose.unwrap_or(false));
    Ok(())
}

#[pymodule]
fn sev_attest_tool(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(verify_attestation_report, m)?)?;
    m.add("SAMPLE_ATTESTATION", SAMPLE_ATTESTATION)?;
    m.add("SAMPLE_VCEK", SAMPLE_VCEK)?;

    Ok(())
}
