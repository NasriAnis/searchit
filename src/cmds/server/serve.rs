use std::io;
use tiny_http::{Method, Response, Server, StatusCode};

use crate::cmds::search;
use crate::serialization;
use crate::cmds::server::wrappers;

#[derive(serde::Serialize)]
struct SearchResult {
    path: String,
    page: u32,
    extension: String,
    score: f64,
}

pub fn run() -> Result<(), io::Error> {
    let server = Server::http("0.0.0.0:8000").unwrap();

    loop {
        let mut request = server.recv()?;

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
                wrappers::serve_file(request, "templates/index.html", "text/html; charset=utf-8");
            }
            (Method::Post, "/search") => {
                let mut query = String::new();
                let _ = request.as_reader().read_to_string(&mut query);

                let documents: Vec<(serialization::DocTfIdf, f64)> =
                    search::run(vec!["".to_string(), "".to_string(), query.clone()])?;

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

                let response = Response::from_string(json).with_header(
                    tiny_http::Header::from_bytes(
                        "Content-Type",
                        "application/json; charset=utf-8",
                    )
                    .expect("incorrect header"),
                );
                let _ = request.respond(response);
            }
            (Method::Get, url) if url.starts_with("/files/") => {
                wrappers::serve_pdf_inline(request, url)?;
            }
            (Method::Get, "/index.js") => {
                wrappers::serve_file(
                    request,
                    "templates/index.js",
                    "text/javascript; charset=utf-8",
                );
            }
            _ => {
                let not_found = StatusCode::from(404);
                let response = Response::empty(not_found);
                let _ = request.respond(response);
            }
        }
    }
}
