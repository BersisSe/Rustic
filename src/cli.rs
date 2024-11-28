use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rustic")]
#[command(about = "Rustic - A simple static site generator", long_about = None)]
pub struct Cli {
    /// Show the version of the application
    #[arg(short, long, global = true)]  // `global = true` makes it available globally
    pub version: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,  // We make this an `Option` to handle no subcommand
}

#[derive(Subcommand)]
pub enum Commands {
    /// Builds the static site
    Build {
        /// The input directory containing Markdown files
        #[arg(short, long, default_value = "content")]
        input: String,

        /// The output directory for generated HTML files
        #[arg(short, long, default_value = "output")]
        output: String,
    },

    /// Starts a local server to preview the site
    Serve,

    /// Cleans the output directory
    Clean,

    /// Inits a new project
    Init,
}
