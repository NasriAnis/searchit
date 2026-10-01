pub fn compute(n_t_appr_in_d: f64, n_t_words_in_d: f64, n_d: f64, n_d_where_t: f64) -> f64 {
    let tf: f64 = n_t_appr_in_d / n_t_words_in_d;
    let idf: f64 = (1.0 + n_d / n_d_where_t).log10();
    tf * idf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_tfidf() {
        let got = compute(10.0, 100.0, 1000.0, 2.0);
        let expected = 0.1 * (501.0_f64).log10();
        assert!((got - expected).abs() < 1e-12);
    }

    #[test]
    fn single_doc_is_not_zero() {
        assert!(compute(1.0, 10.0, 1.0, 1.0) > 0.0);
    }
}