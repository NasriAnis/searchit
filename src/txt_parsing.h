#ifndef TXT_PARSING
#define TXT_PARSING

#include <stdio.h>

typedef struct {
    char *term;
    size_t number;
} term_count;

typedef struct {
    term_count *items;
    size_t count;
    size_t capacity;
} term_table;

term_table count_words(char *text);
void free_term_table(term_table *t);

#endif