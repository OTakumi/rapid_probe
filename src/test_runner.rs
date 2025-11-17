use crate::api_client::HttpClient;
use crate::http_methods::get_handler;
use crate::test_case::{TestCase, TestSuite};
use crate::test_result::TestResult;
use tracing::{debug, info, instrument, warn};

pub struct TestRunner<T: HttpClient> {
    client: T,
}

impl<T: HttpClient> TestRunner<T> {
    pub fn new(client: T) -> Self {
        Self { client }
    }

    #[instrument(skip(self, test_suite), fields(test_count = test_suite.tests.len()))]
    pub async fn run_test_suite(
        &self,
        test_suite: &TestSuite,
        base_url: Option<&str>,
    ) -> Vec<TestResult> {
        info!(
            "Starting test suite execution with {} tests",
            test_suite.tests.len()
        );
        if let Some(name) = &test_suite.name {
            info!("Test suite name: {}", name);
        }

        let mut results = Vec::new();

        for (index, test_case) in test_suite.tests.iter().enumerate() {
            info!(
                "Running test {}/{}: {}",
                index + 1,
                test_suite.tests.len(),
                test_case.name
            );
            let result = self.run_single_test(test_case, base_url).await;

            if result.passed {
                info!("Test passed: {}", test_case.name);
            } else {
                warn!("Test failed: {}", test_case.name);
                if let Some(err) = &result.error_message {
                    debug!("Error details: {}", err);
                }
            }

            results.push(result);
        }

        let passed = results.iter().filter(|r| r.passed).count();
        let failed = results.len() - passed;
        info!("Test suite completed: {} passed, {} failed", passed, failed);

        results
    }

    #[instrument(skip(self, test_case), fields(test_name = %test_case.name, method = %test_case.request.method))]
    pub async fn run_single_test(
        &self,
        test_case: &TestCase,
        _base_url: Option<&str>,
    ) -> TestResult {
        debug!(
            "Executing test: {} (method: {})",
            test_case.name, test_case.request.method
        );
        let result = TestResult::new(test_case.name.clone());

        // HTTPリクエストの実行
        let result = match test_case.request.method.to_uppercase().as_str() {
            "GET" => get_handler::handle_get_request(&self.client, test_case, result).await,
            _ => {
                warn!("Unsupported HTTP method: {}", test_case.request.method);
                result.with_error(format!(
                    "HTTP method '{}' is not supported yet",
                    test_case.request.method
                ))
            }
        };

        let finished_result = result.finish();
        debug!(
            "Test completed in {}ms",
            finished_result.duration.as_millis()
        );

        finished_result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api_client::MockHttpClient;
    use crate::test_case::{Expectations, Request};

    #[tokio::test]
    async fn test_run_test_suite() {
        // Arrange
        let test_suite = TestSuite {
            name: Some("Test Suite".to_string()),
            description: None,
            base_url: Some("https://api.example.com".to_string()),
            tests: vec![
                TestCase {
                    name: "Test 1".to_string(),
                    description: None,
                    request: Request {
                        method: "GET".to_string(),
                        url: "/test1".to_string(),
                        headers: None,
                        body_file: None,
                        body: None,
                    },
                    expectations: Expectations {
                        status_code: 200,
                        headers: None,
                        response: None,
                        performance: None,
                        custom_assertions: None,
                    },
                    variables: None,
                },
                TestCase {
                    name: "Test 2".to_string(),
                    description: None,
                    request: Request {
                        method: "GET".to_string(),
                        url: "/test2".to_string(),
                        headers: None,
                        body_file: None,
                        body: None,
                    },
                    expectations: Expectations {
                        status_code: 201,
                        headers: None,
                        response: None,
                        performance: None,
                        custom_assertions: None,
                    },
                    variables: None,
                },
            ],
        };

        let mut mock_client = MockHttpClient::new();
        mock_client
            .expect_get_with_headers()
            .withf(|url, _| url == "/test1")
            .times(1)
            .returning(|_, _| Ok((200, r#"{"result": "ok"}"#.to_string())));

        mock_client
            .expect_get_with_headers()
            .withf(|url, _| url == "/test2")
            .times(1)
            .returning(|_, _| Ok((201, r#"{"created": true}"#.to_string())));

        let runner = TestRunner::new(mock_client);

        // Act
        let results = runner.run_test_suite(&test_suite, None).await;

        // Assert
        assert_eq!(results.len(), 2);
        assert!(results[0].passed);
        assert!(results[1].passed);
        assert_eq!(results[0].test_name, "Test 1");
        assert_eq!(results[1].test_name, "Test 2");
    }

    #[tokio::test]
    async fn test_unsupported_method() {
        // Arrange
        let test_case = TestCase {
            name: "Unsupported Method Test".to_string(),
            description: None,
            request: Request {
                method: "POST".to_string(),
                url: "/users".to_string(),
                headers: None,
                body_file: None,
                body: None,
            },
            expectations: Expectations {
                status_code: 201,
                headers: None,
                response: None,
                performance: None,
                custom_assertions: None,
            },
            variables: None,
        };

        let mock_client = MockHttpClient::new();
        let runner = TestRunner::new(mock_client);

        // Act
        let result = runner.run_single_test(&test_case, None).await;

        // Assert
        assert!(!result.passed);
        assert!(result.error_message.is_some());
        assert!(result.error_message.unwrap().contains("not supported yet"));
    }
}
