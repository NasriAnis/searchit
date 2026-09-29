pub fn compute(n_t_appr_in_d: f64, n_t_words_in_d: f64, n_d: f64, n_d_where_t: f64) -> f64 {
    let tf: f64 = n_t_appr_in_d / n_t_words_in_d;
    let idf: f64 = (n_d / n_d_where_t).log(10.0);
    tf * idf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_tfidf() {
        assert_eq!(
            compute(10 as f64, 100 as f64, 1000 as f64, 2 as f64),
            0.2698970004336018
        );
    }
}
