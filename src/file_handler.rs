use crate::config::Config;
use crate::constants::*;
use rayon::prelude::*;
use serde_json::to_writer_pretty;




use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::io::Write;



pub fn read_folder(path: &str) -> io::Result<Vec<PathBuf>>{
    let mut output = Vec::new();
    let dir = fs::read_dir(path)?;
    for paths in dir{
        let entry = paths?;
        output.push(entry.path());
        
    }
    Ok(output)
}

pub fn write_file(path: &str, contents: String) -> io::Result<()> {
    let path = Path::new(path);
    if path.is_dir() {
        eprint!("This path Is pointing to a Directory?");
        std::process::exit(0);
    }
    fs::write(path, contents)?;
    Ok(())
}

pub fn init_directories(base_path: &str, title: String) -> io::Result<()> {
    let content_dir = Path::new(base_path).join("content");
    let output_dir = Path::new(base_path).join("output");
    let static_dir = Path::new(base_path).join("static");
    let themes_dir = Path::new(base_path).join("themes");
    let templates_dir = Path::new(base_path).join("templates");
    let config_file = Path::new(base_path).join("rustic.config.json");
    let light_css = themes_dir.join("light.css");
    let dark_css = themes_dir.join("dark.css");
    let default_template_dir = templates_dir.join("default");
    let meta_file = content_dir.join("meta.json");

    fs::create_dir_all(&content_dir)?;
    fs::create_dir_all(&output_dir)?;
    fs::create_dir_all(&static_dir)?;
    fs::create_dir_all(&templates_dir)?;
    fs::create_dir_all(&themes_dir)?;
    fs::create_dir_all(&default_template_dir)?;

    // Create config file if it doesn't exist
    if !config_file.exists() {
        let mut file = fs::File::create(&config_file)?;
        let config = Config::new(
            "127.168.1.1:8080".to_string(),
            title,
            "light".to_string(),
            "default".to_string()
        );
        to_writer_pretty(&file, &config).unwrap();
        file.flush()?; // Yazma işlemini tamamla
    }
    if !meta_file.exists(){
        fs::File::create(&meta_file)?;
        write_file(&meta_file.to_str().unwrap(), META.to_string())?;
    }


    if !dark_css.exists() {
        fs::File::create(&dark_css)?;
        write_file(&dark_css.to_str().unwrap(), DARK_CSS.to_string())?;
    }
    if !light_css.exists() {
        fs::File::create(&light_css)?;
        write_file(&light_css.to_str().unwrap(), LIGHT_CSS.to_string())?;
    }

    // Write Templates
    write_file(
        default_template_dir.join("base.html").to_str().unwrap(),
        BASE_HTML_CONTENT.to_string(),
    )?;
    

    // Write content index file
    write_file(
        content_dir.join("index.md").to_str().unwrap(),
        INDEX_MD_CONTENT.to_string(),
    )?;

    Ok(())
}

pub fn clear_output(dir: &str) -> io::Result<()> {
    fs::remove_dir_all(dir)?;
    fs::create_dir(dir)?;
    Ok(())
}

pub fn copy_themes(output: &Path, config: &Config) -> io::Result<()>{
    let theme = format!("themes/{}.css" ,&config.theme);
    let file = output.join(format!("{}.css", &config.theme));
    fs::copy(&theme, file)?;
    
    Ok(())
}

pub fn copy_static(static_dir : &Path, dest: &Path) -> io::Result<()>{
    if !static_dir.is_dir() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Source directory not found"));
    }
    fs::create_dir_all(dest)?;
    let dir = fs::read_dir(static_dir)?;

    dir.par_bridge().for_each(|entry|{
        let entry = entry.unwrap();
        let entry_path = entry.path();

        let relative_path = entry_path.strip_prefix(static_dir).unwrap();
        let target_path = dest.join(relative_path);

        if entry_path.is_dir() {
            // Recursively copy subdirectories
            copy_static(&entry_path, &target_path).expect("Recurse Error");
        } else if entry_path.is_file() {
            // Copy files
            fs::copy(&entry_path, &target_path).expect("Copy Error");
        }
    });
    Ok(())
}