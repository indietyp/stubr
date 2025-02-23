use serde_json::Value;
use wiremock::{Match, Request};

use super::super::super::{
    helpers::RequestAuthExtension,
    super::json::{json_path_eq::JsonPathEqMatcher, JsonMatcher},
};

pub struct JsonPayloadPathEqMatcher(pub String, pub Value);

impl Match for JsonPayloadPathEqMatcher {
    fn matches(&self, req: &Request) -> bool {
        req.jwt_payload()
            .map(|p| JsonPathEqMatcher(&self.0, &self.1).matches(&p))
            .unwrap_or_default()
    }
}