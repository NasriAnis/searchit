use std::collections::HashMap;

pub fn tokenize(text: String) -> Vec<String> {
    let mut tokens: Vec<String> = vec![];
    let parts: Vec<&str> = text.split_whitespace().collect();
    for part in parts {
        tokens.push(part.to_string());
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
