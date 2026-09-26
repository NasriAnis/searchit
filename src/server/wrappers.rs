use std::fs::File;
use std::io;
use std::path::Path;

use tiny_http::{Request, Response, StatusCode};


pub fn serve_file(request: Request, path: &str, content_type: &str) {
    let _ = request.respond(
        Response::from_file(File::open(Path::new(path)).expect("File to server not found"))
            .with_header(
                tiny_http::Header::from_bytes("Content-Type", content_type)
                    .expect("Uncorrect header"),
            ),
    );
}

pub fn serve_pdf_inline(request: Request, url: &str) -> Result<(), io::Error> {
    // url is like "/files/some%2Fnested%2Fpath.pdf"
    let raw_path = &url["/files/".len()..];
    let decoded = percent_decode(raw_path);

    // reject path traversal
    if decoded.contains("..") {
        let _ = request.respond(Response::empty(StatusCode::from(400)));
        return Ok(());
    }

    let full_path = Path::new(&decoded);
    let file = File::open(full_path)?;

    let filename = full_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "file.pdf".to_string());

    let response = Response::from_file(file)
        .with_header(tiny_http::Header::from_bytes("Content-Type", "application/pdf").unwrap())
        .with_header(
            tiny_http::Header::from_bytes(
                "Content-Disposition",
                format!("inline; filename=\"{}\"", filename).as_bytes(),
            )
            .unwrap(),
        )
        .with_header(tiny_http::Header::from_bytes("X-Content-Type-Options", "nosniff").unwrap());

    let _ = request.respond(response);
    Ok(())
}

fn percent_decode(s: &str) -> String {
    urlencoding::decode(s)
        .map(|c| c.into_owned())
        .unwrap_or_else(|_| s.to_string())
}
