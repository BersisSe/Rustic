use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};

/// Starts the server, listening on the specified address and serving files from the output directory.
pub fn run_server(address: &str, output_dir: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(address)?;
    println!("Server running at http://{}", address);
    println!("Use Ctrl+C to Stop the server");

    // Listen for incoming connections
    for stream in listener.incoming() {
        let stream = stream?;
        handle_connection(stream, output_dir)?;
    }

    Ok(())
}

/// Handles an incoming TCP connection and serves files from the output directory.
fn handle_connection(mut stream: TcpStream, output_dir: &str) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    // Parse the HTTP request line
    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap_or("");
    let mut parts = request_line.split_whitespace();

    // Parse the request method and path
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("/");

    // Serve only GET requests
    if method == "GET" {
        // Map "/" to "index.html" by default
        let file_path = if path == "/" {
            Path::new(output_dir).join("index.html")
        } else {
            // Clean the path to prevent directory traversal
            Path::new(output_dir).join(sanitize_path(path))
        };

        // Attempt to read and serve the file
        match fs::read_to_string(&file_path) {
            Ok(contents) => {
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                    contents.len(),
                    contents
                );
                stream.write_all(response.as_bytes())?;
            }
            Err(_) => {
                let response = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 13\r\n\r\n404 Not Found";
                stream.write_all(response.as_bytes())?;
            }
        }
    } else {
        let response = "HTTP/1.1 405 METHOD NOT ALLOWED\r\nContent-Length: 23\r\n\r\n405 Method Not Allowed";
        stream.write_all(response.as_bytes())?;
    }

    stream.flush()?;
    Ok(())
}

/// Sanitizes the requested path to prevent directory traversal (e.g., "/../").
fn sanitize_path(requested_path: &str) -> PathBuf {
    // Remove leading slashes and dot-dot (..) components to prevent directory traversal
    let mut clean_path = PathBuf::new();
    for component in Path::new(requested_path).components() {
        match component {
            std::path::Component::Normal(part) => clean_path.push(part),
            _ => continue,
        }
    }
    clean_path
}
