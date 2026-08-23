#include <mupdf/fitz.h>

#define PATH_PDF_TEST "test/test.pdf"

typedef struct {
  int page;
  int full_page_count;
  const char *text;
} s_page_text;

void get_page_text_from_pdf(char *path_pdf, s_page_text *buffer) {
  fz_context *ctx = fz_new_context(NULL, NULL, FZ_STORE_UNLIMITED);
  fz_register_document_handlers(ctx);

  fz_document *doc = fz_open_document(ctx, path_pdf);
  int page_count = fz_count_pages(ctx, doc);

  for (int i = 0; i < page_count; i++) {
    fz_page *page = fz_load_page(ctx, doc, i);
    fz_stext_page *text = fz_new_stext_page_from_page(ctx, page, NULL);

    fz_buffer *buf = fz_new_buffer_from_stext_page(ctx, text);
    const char *page_text = fz_string_from_buffer(ctx, buf);
    // printf("--- page %d ---\n%s\n", i + 1, page_text);

    buffer[i].page = i + 1;
    buffer[i].full_page_count = page_count;
    buffer[i].text = strdup(page_text);

    fz_drop_buffer(ctx, buf);
    fz_drop_stext_page(ctx, text);
    fz_drop_page(ctx, page);
  }
}

int main() {
  s_page_text buffer[1000];

  get_page_text_from_pdf(PATH_PDF_TEST, buffer);
  printf("%d: %s", buffer[100].page, buffer[100].text);

  return EXIT_SUCCESS;
}
