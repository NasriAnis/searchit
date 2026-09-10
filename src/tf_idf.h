#ifndef TF_IDF
#define TF_IDF

#include <stddef.h>

/* 
 * a  = numbers of t apprearence in page
 * b  = numbers of words in page
 * c  = numbers of pages
 * d  = numbers of pages where t
*/
size_t tf_idf(size_t a, size_t b, size_t c, size_t d);

#endif
