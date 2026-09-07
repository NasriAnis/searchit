#ifndef PDF_PARSING
#define PDF_PARSING

typedef struct s_page_text {
    int page;
    int full_page_count;
    const char *text;
} s_page_text;

s_page_text *get_page_text_from_pdf(char *path_pdf);

#endif