use regex::Regex;

pub fn extract_mentions(content: &str) -> Vec<String> {
    let re = Regex::new(r"@(\w+)").unwrap();
    re.captures_iter(content)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_mentions() {
        assert_eq!(extract_mentions("Hello @john and @jane!"), vec!["john", "jane"]);
        assert_eq!(extract_mentions("No mentions here"), Vec::<String>::new());
        assert_eq!(extract_mentions("@single"), vec!["single"]);
    }
}
