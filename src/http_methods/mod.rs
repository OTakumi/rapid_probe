pub mod get_handler;

use crate::test_case::TestCase;
use crate::test_result::{AssertionResult, TestResult};

pub trait HttpMethodHandler {
    async fn handle_request(&self, test_case: &TestCase, result: TestResult) -> TestResult;
}

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
