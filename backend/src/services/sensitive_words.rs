use std::collections::HashSet;

pub struct SensitiveWords {
    words: HashSet<String>,
}

impl SensitiveWords {
    pub fn new() -> Self {
        let words: HashSet<String> = ["spam", "advertisement", "hate", "violence", "汝", "瘙痒", "牛逼", "傻逼", "废物", "智障"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        SensitiveWords { words }
    }

    pub fn filter(&self, content: &str) -> String {
        let mut result = content.to_string();
        for word in &self.words {
            let replacement = "*".repeat(word.len());
            result = result.replace(word, &replacement);
        }
        result
    }

    pub fn contains_sensitive(&self, content: &str) -> bool {
        let lower = content.to_lowercase();
        for word in &self.words {
            if lower.contains(&word.to_lowercase()) {
                return true;
            }
        }
        false
    }

    pub fn find_sensitive_words(&self, content: &str) -> Vec<String> {
        let lower = content.to_lowercase();
        let mut found = Vec::new();
        for word in &self.words {
            if lower.contains(&word.to_lowercase()) {
                found.push(word.clone());
            }
        }
        found
    }
}

impl Default for SensitiveWords {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter() {
        let sw = SensitiveWords::new();
        let result = sw.filter("This is spam content");
        assert!(result.contains("****"));
    }
}