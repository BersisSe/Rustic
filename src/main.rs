mod cli;
mod config;
mod constants;
mod file_handler;
mod parser;
mod server;
mod templating;
mod meta;

use clap::Parser;
use cli::{Cli, Commands};
use dialoguer::Input;
use templating::TemplateEngine;
use std::path::Path;
use std::time::SystemTime;
use rayon::prelude::*;
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    // Parse the CLI arguments
    let cli = Cli::parse();

    // Check if the --version flag is set
    if cli.version {
        // Print the version and exit
        println!("Rustic version: {}", VERSION);
        return;
    }

    // Proceed with subcommand logic if --version was not passed
    match cli.command {
        Some(Commands::Build { input, output }) => {
            println!("Building site...");
            build_site(input.as_str(), output.as_str());
        }
        Some(Commands::Serve) => {
            let config = config::Config::load("rustic.config.json").expect("Could not load the config file(Maybe The File is missing)");
            server::run_server_with_hot_reload(&config.base_url, "output", "content", || {
                build_site("content", "output");
            })
            .unwrap();
        }
        Some(Commands::Clean) => {
            println!("Cleaning output directory...");
            clean_output();
        }
        Some(Commands::Init) => {
            println!("Initializing project...");
            init_project();
        }
        None => {
            // If no subcommand is provided, display a message (or do something else)
            eprintln!("No subcommand provided. Use --help for more info.");
        }
    }
}

fn build_site(input: &str, output: &str) {
    let now = SystemTime::now();
    let config = config::Config::load("rustic.config.json")
        .expect("Could not load the config file (maybe the file is missing)");
    let engine = TemplateEngine::new("templates", "content/meta.json");

    // Clear output directory
    file_handler::clear_output(output).expect("Failed to clear output directory");

    let content_files = file_handler::read_folder(input).unwrap();
    content_files.par_iter().for_each(|file| {
        if file.file_name().map(|f| f == "meta.json").unwrap_or(false) {
            return;
        }
        // Read Markdown content
        let file_content = match std::fs::read_to_string(file) {
            Ok(content) => content,
            Err(_) => {
                eprintln!("Failed to read Markdown file: {:?}", file);
                return;
            }
        };

        // Convert Markdown to HTML
        let html = parser::markdown_to_html(&file_content);

        // Get file name for metadata lookup and output file naming
        if let Some(file_name) = file.file_stem() {
            let file_name_str = file_name.to_string_lossy();

            // Render the page
            match engine.render(file_name_str.as_ref(), &config, "base.html", &html) {
                Ok(rendered_html) => {
                    // Write rendered HTML to output folder
                    let output_path = format!("{}/{}.html", output, file_name_str);
                    if let Err(e) = file_handler::write_file(&output_path, rendered_html) {
                        eprintln!("Failed to write file {}: {}", output_path, e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to render page {}: {}", file_name_str, e);
                    std::process::exit(2)
                }
            }
        } else {
            eprintln!("Could not determine file stem for {:?}", file);
            std::process::exit(3)
        }
    });

    if let Err(e) = file_handler::copy_static(Path::new("static"), Path::new("output/static")) {
        eprintln!("Error copying static files: {}", e);
    } else {
        println!("Static files copied successfully!");
    }
    file_handler::copy_themes(Path::new(output), &config)
        .expect("Error While Copying Themes");
    let elapsed = now.elapsed().unwrap();
    println!("Build completed successfully! {:?}", elapsed);
}

fn init_project() {
    let project_name: String = Input::new()
        .with_prompt("Enter project name")
        .default("my_rustic_site".to_string())
        .interact_text()
        .unwrap();

    let project_path = Path::new(&project_name);
    if project_path.exists() {
        eprintln!("Error: A directory with the name '{}' already exists!", project_name);
        std::process::exit(0);
    }

    let site_title: String = Input::new()
        .with_prompt("Enter site title")
        .default("My Rustic Site".to_string())
        .interact_text()
        .unwrap();
    let now = SystemTime::now();
    if let Err(e) = file_handler::init_directories(project_path.to_str().unwrap(), site_title) {
        eprintln!("Error initializing project structure: {}", e);
        std::process::exit(3);
    }

    let elapsed = now.elapsed().unwrap();
    println!("Project initialized in {:?}", elapsed)
}

fn clean_output() {
    match file_handler::clear_output("output") {
        Ok(..) => println!("Output Cleared Successfully"),
        Err(..) => eprintln!("Cleaning Failed"),
    }
}
