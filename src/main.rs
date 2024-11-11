mod cli;
mod parser;
mod templating;
mod server;
mod config;
mod file_handler;

use clap::Parser;
use cli::{ Cli, Commands };
use std::time::SystemTime;
use config::Config;
use dialoguer::Input;
use std::path::Path;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build { input, output } => {
            println!("Building site...");
            build_site(input, output);
        }
        Commands::Serve => {
            let config = config::Config::load("rustic.config.json");
            server::run_server(&config.base_url, "output").unwrap();
        }
        Commands::Clean => {
            println!("Cleaning output directory...");
            clean_dir();
        }
        Commands::Init => {
            println!("Initialzing");
            init_project();
        }
    }
}

fn build_site(input: &str, output: &str) {
    let now = SystemTime::now();
    let conf = Config::load("./rustic.config.json");
    let mut parsed_html: String = "null".to_string();
    let files = file_handler::read_folder(input).expect("Failed The Read input directory");
    for file in files {
        parsed_html = parser::markdown_to_html(&file);
    }
    let mut context = tera::Context::new();
    context.insert("config", &conf);
    context.insert("content", &parsed_html);
    let engine = templating::TemplateEngine::new("templates");
    let render_result = engine.render("index.html", &context).unwrap();
    let output = Path::new(output);
    file_handler::write_file(output.join("index.html").to_str().unwrap(), render_result).expect("Error While Writing to Output");
    let elapsed = now.elapsed().unwrap();
    println!("Project Built in {:?} !", elapsed)
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
    }
    let site_title: String = Input::new()
        .with_prompt("Enter site title")
        .default("My Rustic Site".to_string())
        .interact_text()
        .unwrap();
    let now = SystemTime::now();
    if let Err(e) = file_handler::init_directories(project_path.to_str().unwrap(), site_title) {
        eprintln!("Error initializing project structure: {}", e);
    }

    let elapsed = now.elapsed().unwrap();
    println!("Project initialized in {:?}", elapsed)
}


fn clean_dir(){
    match file_handler::clear_output("output") {
        Ok(..) => println!("Output Cleared Successfully"),
        Err(..) => eprintln!("Cleaning Failed" )
    }
}