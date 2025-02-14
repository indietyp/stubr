use serde::Serialize;
use serde_json::Value;
use wiremock::ResponseTemplate;

use super::ResponseAppender;

#[derive(Serialize, Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct BodyFile {
    pub path_exists: bool,
    pub path: String,
    pub extension: Option<String>,
    pub content: String,
}

impl BodyFile {
    const JSON_EXT: &'static str = "json";
    const TEXT_EXT: &'static str = "txt";

    fn maybe_as_json(&self) -> Option<Value> {
        self.extension.as_deref()
            .filter(|&ext| ext == Self::JSON_EXT)
            .and_then(|_| serde_json::from_str::<Value>(self.content.as_str()).ok())
    }

    fn maybe_as_text(&self) -> Option<String> {
        self.extension.as_deref()
            .filter(|&ext| ext == Self::TEXT_EXT)
            .map(|_| self.content.to_owned())
    }

    fn is_json(&self) -> bool {
        self.extension.as_deref()
            .map(|ext| ext == Self::JSON_EXT)
            .unwrap_or_default()
    }

    fn is_text(&self) -> bool {
        self.extension.as_deref()
            .map(|ext| ext == Self::TEXT_EXT)
            .unwrap_or_default()
    }
}

impl BodyFile {
    pub fn render_templated(&self, mut resp: ResponseTemplate, content: String) -> ResponseTemplate {
        if !self.path_exists {
            resp = ResponseTemplate::new(500)
        } else if self.is_json() {
            let maybe_content: Option<Value> = serde_json::from_str(&content).ok();
            if let Some(content) = maybe_content {
                resp = resp.set_body_json(content);
            } else {
                resp = ResponseTemplate::new(500)
            }
        } else if self.is_text() {
            resp = resp.set_body_string(content);
        } else {
            resp = ResponseTemplate::new(500)
        }
        resp
    }
}

impl ResponseAppender for BodyFile {
    fn add(&self, mut resp: ResponseTemplate) -> ResponseTemplate {
        if !self.path_exists {
            resp = ResponseTemplate::new(500)
        } else if let Some(json) = self.maybe_as_json() {
            resp = resp.set_body_json(json);
        } else if let Some(text) = self.maybe_as_text() {
            resp = resp.set_body_string(text);
        } else {
            resp = ResponseTemplate::new(500)
        }
        resp
    }
}