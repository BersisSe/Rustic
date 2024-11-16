use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rustic")]
#[command(about = "Rustic - A simple static site generator", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
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

    ///Inits a new project
    Init,
}
