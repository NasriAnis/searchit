use std::path::Path;

use pdf_extract::extract_text_by_pages;

fn text_per_page_from(path: &Path) -> Option<Vec<String>> {
    let text_per_page = extract_text_by_pages(path);
    match text_per_page {
        Ok(t) => Some(t),
        Err(e) => {
            eprint!("ERROR in extracting text from pdf {path:?}: {e}");
            None
        }
    }
}
