#ifndef PDF_PARSING
#define PDF_PARSING

typedef struct s_page_text {
    int page;
    int full_page_count;
    char *text;
} s_page_text;

/*
 * this function takes a path to a pdf file
 * and returns an heap allocated array of
 * s_page_text structures
 * which can be accessed one by one.
*/

s_page_text *get_page_text_from_pdf(char *path_pdf);

#endif