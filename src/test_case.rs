use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct TestSuite {
    pub name: Option<String>,
    pub description: Option<String>,
    pub base_url: Option<String>,
    pub tests: Vec<TestCase>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TestCase {
    pub name: String,
    pub description: Option<String>,
    pub request: Request,
    pub expectations: Expectations,
    pub variables: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Request {
    pub method: String,
    pub url: String,
    pub headers: Option<HashMap<String, String>>,
    pub body_file: Option<String>,
    pub body: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Expectations {
    pub status_code: u16,
    pub headers: Option<HashMap<String, String>>,
    pub response: Option<ResponseExpectation>,
    pub performance: Option<PerformanceExpectation>,
    pub custom_assertions: Option<Vec<CustomAssertion>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResponseExpectation {
    pub schema_file: Option<String>,
    pub expected_file: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceExpectation {
    pub max_response_time_ms: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CustomAssertion {
    pub field: String,
    pub equals: serde_json::Value,
}
