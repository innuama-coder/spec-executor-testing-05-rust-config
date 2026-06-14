use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigParseError {
    pub line: usize,
    pub message: String,
}

pub fn parse_config(_input: &str) -> Result<BTreeMap<String, String>, ConfigParseError> {
    todo!("implement according to docs/PRD.md, docs/HLD.md, and docs/LLD.md")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_single_pair() {
        let parsed = parse_config("name=spec-executor").unwrap();
        assert_eq!(parsed.get("name").map(String::as_str), Some("spec-executor"));
    }
}

