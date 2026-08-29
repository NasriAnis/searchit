#include <mupdf/fitz.h>

#include "include/pdf_parsing.h"

s_page_text *get_page_text_from_pdf(char *path_pdf) {
    fz_context *ctx = fz_new_context(NULL, NULL, FZ_STORE_UNLIMITED);
    fz_register_document_handlers(ctx);

    fz_document *doc = fz_open_document(ctx, path_pdf);
    int page_count = fz_count_pages(ctx, doc);

    s_page_text *buffer = malloc(page_count * sizeof(s_page_text));
    if (!buffer) {
        fz_drop_document(ctx, doc);
        fz_drop_context(ctx);
        return NULL;
    }

    for (int i = 0; i < page_count; i++) {
        fz_page *page = fz_load_page(ctx, doc, i);
        fz_stext_page *text = fz_new_stext_page_from_page(ctx, page, NULL);

        fz_buffer *buf = fz_new_buffer_from_stext_page(ctx, text);
        const char *page_text = fz_string_from_buffer(ctx, buf);

        buffer[i].page = i + 1;
        buffer[i].full_page_count = page_count;
        buffer[i].text = strdup(page_text);

        fz_drop_buffer(ctx, buf);
        fz_drop_stext_page(ctx, text);
        fz_drop_page(ctx, page);
    }

    fz_drop_document(ctx, doc);
    fz_drop_context(ctx);
    return buffer;
}