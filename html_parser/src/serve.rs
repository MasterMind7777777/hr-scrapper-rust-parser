use std::fs;
use std::path::Path;
use tiny_http::{Response, Server};

fn main() {
    // Define the address and port to serve the content
    let server = Server::http("0.0.0.0:8000").unwrap();
    println!("Serving on http://localhost:8000");

    for request in server.incoming_requests() {
        // Define the path to the HTML file
        let file_path = Path::new("html_pages/test.html");

        // Check if the file exists and read its content
        if file_path.exists() {
            let html_content = fs::read_to_string(file_path).expect("Error reading HTML file");

            // Create a response with the HTML content and send it back to the client
            let response = {
                let mut this = Response::from_string(html_content);
                let header: tiny_http::Header =
                    "Content-Type: text/html; charset=UTF-8".parse().unwrap();
                this.add_header(header);
                this
            };
            request.respond(response).expect("Failed to respond");
        } else {
            // Respond with a 404 error if the file is not found
            let response = Response::from_string("404 - File Not Found")
                .with_status_code(404)
                .with_header(
                    "Content-Type: text/plain; charset=UTF-8"
                        .parse::<tiny_http::Header>()
                        .unwrap(),
                );
            request.respond(response).expect("Failed to respond");
        }
    }
}
