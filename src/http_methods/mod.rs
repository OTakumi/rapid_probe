pub mod get_handler;

use crate::test_result::{AssertionResult, TestResult};

pub fn validate_status_code(result: &mut TestResult, actual: u16, expected: u16) {
    if actual != expected {
        result.add_assertion(AssertionResult {
            assertion_type: "status_code".to_string(),
            passed: false,
            expected: expected.to_string(),
            actual: actual.to_string(),
            message: Some(format!("Expected status {} but got {}", expected, actual)),
        });
    } else {
        result.add_assertion(AssertionResult {
            assertion_type: "status_code".to_string(),
            passed: true,
            expected: expected.to_string(),
            actual: actual.to_string(),
            message: None,
        });
    }
}
