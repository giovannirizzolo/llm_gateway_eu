use super::dsl::{Action, FieldMatcher, RulePack};
use serde_json::Value;

/// Evaluate payload against a rule pack.
/// Mutates the payload in place if redaction is applied.
pub fn evaluate(payload: &mut Value, pack: &RulePack) -> Result<(), String> {
    for rule in &pack.rules {
        let mut matched = false;

        for matcher in &rule.matchers {
            match matcher {
                FieldMatcher::Regex { target, pattern, flags } => {
                    let re = {
                        let mut builder = regex::RegexBuilder::new(pattern);
                        if let Some(v) = flags {
                            if v.iter().any(|s| s.eq_ignore_ascii_case("ignoreCase")) {
                                builder.case_insensitive(true);
                            }
                        }
                        builder.build().map_err(|e| e.to_string())?
                    };

                    if let Some(field) = payload.pointer_mut(target) {
                        if let Some(text) = field.as_str() {
                            if re.is_match(text) {
                                matched = true;
                                match &rule.action {
                                    Action::Redact { replace_with } => {
                                        *field = Value::String(
                                            re.replace_all(text, replace_with.as_str()).into(),
                                        );
                                    }
                                    Action::Block => {
                                        return Err(format!("Blocked by rule '{}'", rule.id));
                                    }
                                    Action::Allow => {}
                                }
                            }
                        }
                    }
                }
                // Detector branch stub – implement your ONNX/LLM detectors here
                FieldMatcher::Detector { .. } => {}
            }
        }

        // Short-circuit: once a rule hits, stop evaluating
        if matched {
            break;
        }
    }
    Ok(())
}