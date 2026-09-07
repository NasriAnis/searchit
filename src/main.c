#include <stddef.h>
#include <stdlib.h>

#include "pdf_parsing.h"
#include "txt_parsing.h"

#define PATH_PDF_TEST "test/test.pdf"

int main() {
    s_page_text *buffer = get_page_text_from_pdf(PATH_PDF_TEST);

    char *world_list = per_worlds_count_in(buffer[100].text);

    return EXIT_SUCCESS;
}