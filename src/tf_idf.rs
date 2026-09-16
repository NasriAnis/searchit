pub fn tf_idf(n_t_appr_in_d: f64, n_t_words_in_d: f64, n_d: f64, n_d_where_t: f64) -> f64 {
    let tf: f64 = n_t_appr_in_d / n_t_words_in_d;
    let idf: f64 = (n_d / n_d_where_t).log(10.0);
    tf * idf
}