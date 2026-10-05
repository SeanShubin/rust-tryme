//! Derive macros, using serde as the example.
//!
//! `#[derive(...)]` runs a procedural macro at compile time that writes the
//! implementation for you -- the nearest thing Rust has to annotation
//! processing, except it is checked like ordinary code.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Experiment {
    pub name: String,
    pub iterations: u32,

    /// Renaming shows the wire format is independent of the field name.
    #[serde(rename = "isComplete")]
    pub complete: bool,

    /// An absent value falls back to the default rather than failing.
    #[serde(default)]
    pub notes: Vec<String>,
}

pub fn to_json(experiment: &Experiment) -> Result<String, serde_json::Error> {
    serde_json::to_string(experiment)
}

pub fn from_json(json: &str) -> Result<Experiment, serde_json::Error> {
    serde_json::from_str(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Experiment {
        Experiment {
            name: "ownership".to_string(),
            iterations: 3,
            complete: true,
            notes: vec!["borrow checker".to_string()],
        }
    }

    #[test]
    fn round_trips() {
        let original = sample();
        let json = to_json(&original).expect("serialize");
        let parsed = from_json(&json).expect("deserialize");
        assert_eq!(original, parsed);
    }

    #[test]
    fn renames_the_field_on_the_wire() {
        let json = to_json(&sample()).expect("serialize");
        assert!(json.contains("\"isComplete\":true"), "unexpected json: {json}");
        assert!(!json.contains("\"complete\""), "unexpected json: {json}");
    }

    #[test]
    fn missing_optional_field_uses_the_default() {
        let json = r#"{"name":"traits","iterations":1,"isComplete":false}"#;
        let parsed = from_json(json).expect("deserialize");
        assert_eq!(Vec::<String>::new(), parsed.notes);
    }

    #[test]
    fn missing_required_field_is_an_error() {
        let json = r#"{"name":"traits"}"#;
        assert!(from_json(json).is_err());
    }
}
