use std::fs;
use std::io;
use std::fs::*;
use std::io::Write;
use std::path::Path;
use crate::config::Config;
use serde_json::to_writer_pretty;


const  INDEX_MD_CONTENT : &str= r#"
    # Welcome to My Rustic Site
    
    This is the homepage of your new static site.
    "#;

const BASE_HTML_CONTENT : &str = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>{{ config.title }}</title>
    </head>
    <body>
        {% block content %}{% endblock %}
    </body>
    </html>
    "#;
    
const INDEX_HTML_CONTENT : &str = r#"
    {% extends "base.html" %}

    {% block content %}
    <h1>Welcome to {{ config.title }}</h1>
    <p>This is your homepage.</p>
    {% endblock %}
    "#;

pub fn read_folder(path: &str) -> io::Result<Vec<String>>{
    let mut file_paths = Vec::new();

    for entry in read_dir(path)?{
        let entry = entry?;
        let path = entry.path();
        if path.is_file(){
            let path_str = path.to_str().unwrap();
            file_paths.push(path_str.to_string());
        }
    }
    Ok(file_paths)
}

pub fn write_file(path: &str, contents: String) -> io::Result<()>{
    let path = Path::new(path);
    if path.is_dir(){
        eprint!("This path Is pointing to a Directory?");
        std::process::exit(0);
    }
    fs::write(path, contents)?;
    Ok(())
}

pub fn init_directories(base_path: &str , title: String) -> io::Result<()>{
    let content_dir = Path::new(base_path).join("content");
    let output_dir = Path::new(base_path).join("output");
    let static_dir = Path::new(base_path).join("static");
    let templates_dir = Path::new(base_path).join("templates");
    let config_file = Path::new(base_path).join("rustic.config.json");
    
    fs::create_dir_all(&content_dir)?;
    fs::create_dir_all(&output_dir)?;
    fs::create_dir_all(&static_dir)?;
    fs::create_dir_all(&templates_dir)?;

    if !config_file.exists() {
        let file = fs::File::create(&config_file)?;
        let config = Config::new("http://127.168.1.1:8080".to_string(), title,);
        to_writer_pretty(file, &config)?;
    }
    
    write_file(templates_dir.join("base.html").to_str().unwrap(), BASE_HTML_CONTENT.to_string())
        .expect("Error While Creating Files");
    write_file(templates_dir.join("index.html").to_str().unwrap(), INDEX_HTML_CONTENT.to_string())
        .expect("Error While Creating Files");
    write_file(content_dir.join("index.md").to_str().unwrap(), INDEX_MD_CONTENT.to_string())
        .expect("Error While Creating Files");
    Ok(())

}

pub fn clear_output(dir : &str) -> io::Result<()>{
    fs::remove_dir_all(dir)?;
    fs::create_dir(dir)?;
    Ok(())
}