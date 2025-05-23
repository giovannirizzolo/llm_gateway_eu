use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RulePack {
    pub version: String,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub description: Option<String>,
    pub matchers: Vec<FieldMatcher>,
    pub action: Action,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FieldMatcher {
    Regex {
        target: String,   // JSON pointer – e.g. "/messages/0/content"
        pattern: String,
        flags: Option<Vec<String>>,
    },
    Detector {
        target: String,
        detector: String, // e.g. "biometrics"
        confidence_gt: f32,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Action {
    Redact { replace_with: String },
    Block,
    Allow,
}