#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
    char *term;
    size_t number;
} term_count;

typedef struct {
    term_count *items;
    size_t count;
    size_t capacity;
} term_table;

term_table count_words(char *text) {
    term_table table = { NULL, 0, 0 };
    char delim[] = " \t\r\n\v\f,./:;()[]{}`'\"!@#$%&*?<>";
    char *token = strtok(text, delim);

    while (token) {
        size_t i;
        for (i = 0; i < table.count; i++) {
            if (strcmp(table.items[i].term, token) == 0) {
                table.items[i].number++;
                break;
            }
        }

        if (i == table.count) {
            if (table.count == table.capacity) {
                table.capacity = table.capacity ? table.capacity * 2 : 8;
                term_count *tmp = realloc(table.items, table.capacity * sizeof(term_count));
                if (tmp == NULL) {
                    fprintf(stderr, "ERROR: realloc failed in count_words()\n");
                    exit(1);
                }
                table.items = tmp;
            }
            table.items[table.count].term = strdup(token);
            table.items[table.count].number = 1;
            table.count++;
        }

        token = strtok(NULL, delim);
    }

    return table;
}

void free_term_table(term_table *t) {
    for (size_t i = 0; i < t->count; i++)
        free(t->items[i].term);
    free(t->items);
}