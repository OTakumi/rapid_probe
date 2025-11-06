use chrono::{DateTime, Utc};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct TestResult {
    pub test_name: String,
    pub passed: bool,
    pub duration: Duration,
    pub started_at: DateTime<Utc>,
    pub status_code: Option<u16>,
    pub error_message: Option<String>,
    pub assertions: Vec<AssertionResult>,
}

#[derive(Debug, Clone)]
pub struct AssertionResult {
    pub assertion_type: String,
    pub passed: bool,
    pub expected: String,
    pub actual: String,
    pub message: Option<String>,
}

impl TestResult {
    pub fn new(test_name: String) -> Self {
        Self {
            test_name,
            passed: true,
            duration: Duration::default(),
            started_at: Utc::now(),
            status_code: None,
            error_message: None,
            assertions: Vec::new(),
        }
    }

    pub fn with_error(mut self, error: String) -> Self {
        self.passed = false;
        self.error_message = Some(error);
        self
    }

    pub fn with_status_code(mut self, status_code: u16) -> Self {
        self.status_code = Some(status_code);
        self
    }

    pub fn add_assertion(&mut self, assertion: AssertionResult) {
        if !assertion.passed {
            self.passed = false;
        }
        self.assertions.push(assertion);
    }

    pub fn finish(mut self) -> Self {
        self.duration = Utc::now().signed_duration_since(self.started_at).to_std().unwrap_or_default();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_test_result_is_initially_passed() {
        let result = TestResult::new("test_api_endpoint".to_string());
        
        assert_eq!(result.test_name, "test_api_endpoint");
        assert!(result.passed);
        assert_eq!(result.duration, Duration::default());
        assert!(result.status_code.is_none());
        assert!(result.error_message.is_none());
        assert!(result.assertions.is_empty());
    }

    #[test]
    fn test_with_error_marks_as_failed() {
        let result = TestResult::new("test_api".to_string())
            .with_error("Connection timeout".to_string());
        
        assert!(!result.passed);
        assert_eq!(result.error_message, Some("Connection timeout".to_string()));
    }

    #[test]
    fn test_with_status_code() {
        let result = TestResult::new("test_api".to_string())
            .with_status_code(200);
        
        assert_eq!(result.status_code, Some(200));
        assert!(result.passed);
    }

    #[test]
    fn test_add_passing_assertion() {
        let mut result = TestResult::new("test_api".to_string());
        
        let assertion = AssertionResult {
            assertion_type: "status_code".to_string(),
            passed: true,
            expected: "200".to_string(),
            actual: "200".to_string(),
            message: None,
        };
        
        result.add_assertion(assertion);
        
        assert!(result.passed);
        assert_eq!(result.assertions.len(), 1);
    }

    #[test]
    fn test_add_failing_assertion_marks_test_as_failed() {
        let mut result = TestResult::new("test_api".to_string());
        
        let assertion = AssertionResult {
            assertion_type: "status_code".to_string(),
            passed: false,
            expected: "200".to_string(),
            actual: "404".to_string(),
            message: Some("Expected status 200 but got 404".to_string()),
        };
        
        result.add_assertion(assertion);
        
        assert!(!result.passed);
        assert_eq!(result.assertions.len(), 1);
    }

    #[test]
    fn test_finish_calculates_duration() {
        let mut result = TestResult::new("test_api".to_string());
        
        // 少し待つ
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        result = result.finish();
        
        assert!(result.duration > Duration::from_millis(0));
    }

    #[test]
    fn test_multiple_assertions_with_one_failure() {
        let mut result = TestResult::new("test_api".to_string());
        
        // 成功するアサーション
        result.add_assertion(AssertionResult {
            assertion_type: "status_code".to_string(),
            passed: true,
            expected: "200".to_string(),
            actual: "200".to_string(),
            message: None,
        });
        
        // 失敗するアサーション
        result.add_assertion(AssertionResult {
            assertion_type: "header".to_string(),
            passed: false,
            expected: "application/json".to_string(),
            actual: "text/html".to_string(),
            message: Some("Content-Type mismatch".to_string()),
        });
        
        assert!(!result.passed);
        assert_eq!(result.assertions.len(), 2);
    }
}