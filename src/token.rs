use std::collections::HashMap;

pub fn tokenize(text: String) -> Vec<String> {
    let mut tokens: Vec<String> = vec![];
    let parts: Vec<&str> = text.split_whitespace().collect();
    for part in parts {
        let cleaned = part.trim_matches(|c: char| !c.is_alphanumeric());
        if cleaned.is_empty() {
            continue;
        }
        tokens.push(cleaned.to_ascii_lowercase());
    }
    tokens
}

pub fn count_individual_token(tokens: Vec<String>) -> HashMap<String, usize> {
    let mut hashmap_terms: HashMap<String, usize> = HashMap::new();
    for tok in tokens {
        if !hashmap_terms.contains_key(tok.as_str()) {
            hashmap_terms.insert(tok, 1);
        } else {
            if let Some(i) = hashmap_terms.get(tok.as_str()) {
                hashmap_terms.insert(tok, i + 1);
            }
        }
    }
    hashmap_terms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_text() {
        let text = "hello, how are you doing? hello".to_string();
        let tokens = tokenize(text);
        assert_eq!(tokens, vec!["hello", "how", "are", "you", "doing", "hello"]);
    }

    #[test]
    fn count_token() {
        let tokens = vec![
            "hello".to_string(),
            "how".to_string(),
            "are".to_string(),
            "you".to_string(),
            "doing".to_string(),
            "hello".to_string(),
        ];
        let mut expect: HashMap<String, usize> = HashMap::new();
        expect.insert("hello".to_string(), 2);
        expect.insert("how".to_string(), 1);
        expect.insert("are".to_string(), 1);
        expect.insert("you".to_string(), 1);
        expect.insert("doing".to_string(), 1);
        assert_eq!(count_individual_token(tokens), expect)
    }
}
