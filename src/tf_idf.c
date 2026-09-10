#include <stddef.h>
#include <stdio.h>
#include <math.h>

double tf(size_t nmbr_of_t_appearence, size_t nmbr_of_wrds){
    return (double)nmbr_of_t_appearence/(double)nmbr_of_wrds;
}

double idf(size_t nmbr_of_pgs, size_t nmbr_of_pgs_where_t){
    return log((double)nmbr_of_pgs/(double)nmbr_of_pgs_where_t);
}

double tf_idf(size_t a, size_t b, size_t c, size_t d){
    double calc_tf = tf(a, b);
    double calc_idf = idf(c, d);
    return calc_tf * calc_idf;
}

