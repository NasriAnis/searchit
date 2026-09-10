#ifndef TXT_PARSING
#define TXT_PARSING

#include <stdio.h>

typedef struct {
    char *term;
    size_t number;
} term_count;

typedef struct {
    term_count *items;  // Word + its count
    size_t word_count;
    size_t count;       // Numbers of different items
    size_t capacity;
} term_table;

/*
 * this fhunction takes a pointer to
 * some space separated text and return
 * a term_table struct per text where is
 * each word and its count inside a term_count
 * Struct.
*/

term_table count_words(char *text);
void free_term_table(term_table *t);

#endif