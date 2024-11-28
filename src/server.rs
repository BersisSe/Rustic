use notify::{RecommendedWatcher, RecursiveMode, Watcher, EventKind, Config};
use std::sync::mpsc;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use colored::*;


/// Starts the preview server with hot reload functionality.
pub fn run_server_with_hot_reload(
    address: &str,
    output_dir: &str,
    content_dir: &str,
    build_site: impl Fn() + Send + 'static,
) -> std::io::Result<()> {
    // Channel for file-watching events
    let (tx, rx) = mpsc::channel();

    // Initialize the notify watcher
    let mut watcher = RecommendedWatcher::new(
        move |res| {
            if let Ok(event) = res {
                tx.send(event).expect("Failed to send file system event");
            }
        },
        Config::default(),
    )
    .unwrap();

    // Watch the content directory
    watcher
        .watch(Path::new(content_dir), RecursiveMode::Recursive)
        .expect("Failed to watch content directory");

    // Start the HTTP server
    let listener = TcpListener::bind(address)?;
    println!(
        "{}, {}",
        "Server running at".cyan(),
        format!("http://{}", address).green()
    );
    println!("Use Ctrl+C to stop the server");

    // Flag to track rebuild state
    let mut reload_flag = false;

    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_clone = Arc::clone(&shutdown);

    ctrlc::set_handler(move || {
        shutdown_clone.store(true, Ordering::SeqCst);
        println!("{}", "Shutting down gracefully...".yellow());
    })
    .expect("Error setting Ctrl+C handler");

    loop {
        if shutdown.load(Ordering::SeqCst) {
            break; // Exit the loop to shut down
        }
        
        // Handle file changes
        while let Ok(event) = rx.try_recv() {
            if matches!(
                event.kind,
                EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_)
            ) {
                println!("{}","Change detected. Rebuilding site...".yellow());
                build_site();
                reload_flag = true; // Mark for reload
            }
        }

        // Handle HTTP requests
        if let Ok((stream, _)) = listener.accept() {
            handle_connection(stream, output_dir, &mut reload_flag)?;
        }
    }
    println!("{}", "Server stopped.".green());
    Ok(())
}

/// Handles an incoming HTTP connection.
fn handle_connection(
    mut stream: TcpStream,
    output_dir: &str,
    reload_flag: &mut bool,
) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;
    // ! Dev Mode Setting
    let development_mode = true;
    // Parse the HTTP request line
    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap_or("");
    let mut parts = request_line.split_whitespace();

    // Parse the request method and path
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("/");

    // Serve only GET requests
    if method == "GET" {
        if path == "/reload" {
            // Simple polling endpoint for reload check
            let response = if *reload_flag {
                *reload_flag = false; // Reset flag after notifying the client
                "HTTP/1.1 200 OK\r\nContent-Length: 7\r\n\r\nRELOAD"
            } else {
                "HTTP/1.1 204 No Content\r\n\r\n"
            };
            stream.write_all(response.as_bytes())?;
        } else {
            // Map "/" to "index.html" by default
            let file_path = if path == "/" {
                Path::new(output_dir).join("index.html")
            } else {
                // Clean the path to prevent directory traversal
                Path::new(output_dir).join(sanitize_path(path))
            };

            // Attempt to read and serve the file
            match fs::read_to_string(&file_path) {
                Ok(mut contents) => {
                    if development_mode {
                        contents.push_str(r#"
                        <!--Hot Reload Code İnjection Do not Delete!-->
                        <script>
                        async function reloadOnChange() {
                            try {
                                const res = await fetch('/reload');
                                if (res.status === 200) {
                                    location.reload();
                                }
                            } 
                            catch (err) {
                            console.error('Live reload failed', err);
                            }
                        }
                        setInterval(reloadOnChange, 1000);
                        </script> "#);
                    }
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                        contents.len(),
                        contents
                    );
                    stream.write_all(response.as_bytes())?;
                }
                Err(_) => {
                    let response =
                        "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 13\r\n\r\n404 Not Found";
                    stream.write_all(response.as_bytes())?;
                }
            }
        }
    } else {
        let response =
            "HTTP/1.1 405 METHOD NOT ALLOWED\r\nContent-Length: 23\r\n\r\n405 Method Not Allowed";
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
