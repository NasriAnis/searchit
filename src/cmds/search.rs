use std::collections::HashMap;

use crate::config::TOP_RESULTS;
use crate::serialization::{DocTfIdf, deserialize_tfidf_to_word};
use crate::token;

pub fn run(args: Vec<String>) -> Vec<(DocTfIdf, f64)> {
    if args.len() < 3 {
        help();
    }
    let user_tokens = token::tokenize(args[2].clone());
    let deserialized_data = deserialize_tfidf_to_word();

    let mut scores: HashMap<usize, f64> = HashMap::new();

    for tok in user_tokens {
        for (idx, document) in deserialized_data.iter().enumerate() {
            if let Some(tfidf) = document.terms.get(&tok) {
                *scores.entry(idx).or_insert(0.0) += tfidf;
            }
        }
    }

    let mut relevant_documents: Vec<(usize, f64)> = scores.into_iter().collect();

    relevant_documents.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    relevant_documents.truncate(TOP_RESULTS);

    let top: Vec<(DocTfIdf, f64)> = relevant_documents
        .into_iter()
        .map(|(idx, score)| (deserialized_data[idx].clone(), score))
        .collect();

    top
}

fn help() {
    println!(
        "\
Help for search:
    - scan <'search query'>"
    );
}
