#include <stdio.h>
#include <stdlib.h>
#include "include/pdf_parsing.h"

#define PATH_PDF_TEST "test/test.pdf"

int main() {
    s_page_text *buffer = get_page_text_from_pdf(PATH_PDF_TEST);
    return EXIT_SUCCESS;
}