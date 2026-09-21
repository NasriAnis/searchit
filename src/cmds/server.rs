use tiny_http::{Method, Request, Response, Server, StatusCode};
use std::path::Path;
use std::fs::File;

use crate::cmds::search;
use crate::serialization;

#[derive(serde::Serialize)]
struct SearchResult {
    path: String,
    page: u32,
    extension: String,
    score: f64,
}

pub fn run(){
    let server = Server::http("0.0.0.0:8000").unwrap();

    loop {
        let mut request = match server.recv() {
            Ok(rq) => rq,
            Err(e) => { println!("error: {}", e); break }
        };

        let method = request.method().clone();
        let url = request.url().to_string();

        // log connection
        println!(
            "RECEIVED: from {remote_address} with {method} at {url}",
            remote_address = match request.remote_addr() {
                Some(t) => t.to_string(),
                None => "unknown".to_string(),
            },
        );

        match (&method, url.as_str()) {
            (Method::Get, "/") => {
                serve_file(request, "templates/index.html", "text/html; charset=utf-8");
            }
            (Method::Post, "/search") => {
                let mut query = String::new();
                let _ = request.as_reader().read_to_string(&mut query);

                let documents: Vec<(serialization::DocTfIdf, f64)> = search::run(vec!["".to_string(), "".to_string(), query.clone()]);

                let results: Vec<SearchResult> = documents
                    .into_iter()
                    .map(|(doc, score)| SearchResult {
                        path: doc.path,
                        page: doc.page,
                        extension: doc.extension,
                        score,
                    })
                .collect();

                let json = serde_json::to_string(&results).unwrap_or_else(|_| "[]".to_string());

                let response = Response::from_string(json)
                    .with_header(
                        tiny_http::Header::from_bytes("Content-Type", "application/json; charset=utf-8")
                        .expect("incorrect header"),
                    );
                let _ = request.respond(response);
            }
            (Method::Get, url) if url.starts_with("/files/") => {
                serve_pdf_inline(request, url);
            }
            (Method::Get, "/index.js") => {
                serve_file(request, "templates/index.js", "text/javascript; charset=utf-8");
            }
            _ => {
                let not_found = StatusCode::from(404);
                let response = Response::empty(not_found);
                let _ = request.respond(response);
            }
        }
    }
}

fn serve_file(request: Request, path: &str, content_type: &str){
    let _ = request.respond(Response::from_file(File::open(Path::new(path)).expect("File to server not found"))
        .with_header(
            tiny_http::Header::from_bytes("Content-Type", content_type).expect("Uncorrect header"),
        ));
}

fn serve_pdf_inline(request: Request, url: &str) {
    // url is like "/files/some%2Fnested%2Fpath.pdf"
    let raw_path = &url["/files/".len()..];
    let decoded = percent_decode(raw_path);

    // reject path traversal
    if decoded.contains("..") {
        let _ = request.respond(Response::empty(StatusCode::from(400)));
        return;
    }

    let full_path = Path::new(&decoded);

    let file = match File::open(full_path) {
        Ok(f) => f,
        Err(_) => {
            let _ = request.respond(Response::empty(StatusCode::from(404)));
            return;
        }
    };

    let filename = full_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "file.pdf".to_string());

    let response = Response::from_file(file)
        .with_header(
            tiny_http::Header::from_bytes("Content-Type", "application/pdf").unwrap(),
        )
        .with_header(
            tiny_http::Header::from_bytes(
                "Content-Disposition",
                format!("inline; filename=\"{}\"", filename).as_bytes(),
            )
            .unwrap(),
        )
        .with_header(
            tiny_http::Header::from_bytes("X-Content-Type-Options", "nosniff").unwrap(),
        );

    let _ = request.respond(response);
}

fn percent_decode(s: &str) -> String {
    urlencoding::decode(s).map(|c| c.into_owned()).unwrap_or_else(|_| s.to_string())
}