use crate::test_case::TestSuite;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub struct TestCaseLoader;

impl TestCaseLoader {
    pub fn load_from_file(file_path: &Path) -> Result<TestSuite> {
        let content = fs::read_to_string(file_path)
            .with_context(|| format!("failed to read file: {}", file_path.display()))?;

        let test_suite: TestSuite =
            serde_yaml::from_str(&content).context("failed to parse YAML")?;

        Ok(test_suite)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_simple_yaml_test_case() {
        let yaml_content = r#"
name: "Simple API Test Suite"
description: "Basic API tests"
base_url: "https://api.example.com"
tests:
  - name: "Get User Test"
    description: "Test getting a user by ID"
    request:
      method: "GET"
      url: "/users/1"
    expectations:
      status_code: 200
"#;

        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", yaml_content).unwrap();

        let result = TestCaseLoader::load_from_file(file.path());

        assert!(result.is_ok());
        let test_suite = result.unwrap();

        assert_eq!(test_suite.name, Some("Simple API Test Suite".to_string()));
        assert_eq!(test_suite.description, Some("Basic API tests".to_string()));
        assert_eq!(
            test_suite.base_url,
            Some("https://api.example.com".to_string())
        );
        assert_eq!(test_suite.tests.len(), 1);

        let test_case = &test_suite.tests[0];
        assert_eq!(test_case.name, "Get User Test");
        assert_eq!(test_case.request.method, "GET");
        assert_eq!(test_case.request.url, "/users/1");
        assert_eq!(test_case.expectations.status_code, 200);
    }

    #[test]
    fn test_load_yaml_with_headers() {
        let yaml_content = r#"
tests:
  - name: "Auth Test"
    request:
      method: "GET"
      url: "/protected"
      headers:
        Authorization: "Bearer token123"
        Content-Type: "application/json"
    expectations:
      status_code: 200
"#;

        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", yaml_content).unwrap();

        let result = TestCaseLoader::load_from_file(file.path());

        assert!(result.is_ok());
        let test_suite = result.unwrap();

        let test_case = &test_suite.tests[0];
        assert!(test_case.request.headers.is_some());

        let headers = test_case.request.headers.as_ref().unwrap();
        assert_eq!(
            headers.get("Authorization"),
            Some(&"Bearer token123".to_string())
        );
        assert_eq!(
            headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );
    }

    #[test]
    fn test_load_yaml_with_variables() {
        let yaml_content = r#"
tests:
  - name: "Variable Test"
    request:
      method: "GET"
      url: "/users/{{user_id}}"
    expectations:
      status_code: 200
    variables:
      user_id: "123"
      api_key: "secret"
"#;

        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", yaml_content).unwrap();

        let result = TestCaseLoader::load_from_file(file.path());

        assert!(result.is_ok());
        let test_suite = result.unwrap();

        let test_case = &test_suite.tests[0];
        assert!(test_case.variables.is_some());

        let variables = test_case.variables.as_ref().unwrap();
        assert_eq!(variables.get("user_id"), Some(&"123".to_string()));
        assert_eq!(variables.get("api_key"), Some(&"secret".to_string()));
    }

    #[test]
    fn test_load_multiple_test_cases() {
        let yaml_content = r#"
tests:
  - name: "Test 1"
    request:
      method: "GET"
      url: "/test1"
    expectations:
      status_code: 200
  - name: "Test 2"
    request:
      method: "POST"
      url: "/test2"
    expectations:
      status_code: 201
"#;

        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", yaml_content).unwrap();

        let result = TestCaseLoader::load_from_file(file.path());

        assert!(result.is_ok());
        let test_suite = result.unwrap();

        assert_eq!(test_suite.tests.len(), 2);
        assert_eq!(test_suite.tests[0].name, "Test 1");
        assert_eq!(test_suite.tests[1].name, "Test 2");
    }

    #[test]
    fn test_file_not_found() {
        let result = TestCaseLoader::load_from_file(Path::new("non_existent_file.yaml"));
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_yaml() {
        let yaml_content = r#"
tests:
  - name: "Test"
    invalid_yaml: [
"#;

        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", yaml_content).unwrap();

        let result = TestCaseLoader::load_from_file(file.path());
        assert!(result.is_err());
    }
}
