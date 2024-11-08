# Rustic - A Lightweight, High-Performance Static Site Generator in Rust 🦀

Rustic is a fast, powerful, and developer-friendly static site generator (SSG) written in Rust, designed to streamline the process of building static websites and documentation sites. Built with a focus on simplicity, speed, and extensibility, Rustic empowers developers to create robust websites with ease—while leveraging the performance and safety benefits of Rust.

### Key Features

- **Blazing Fast Build Speeds**: Thanks to Rust's concurrency and efficient handling of file operations, Rustic is optimized to build even large sites quickly.
- **Markdown to HTML Conversion**: Easily convert Markdown files to HTML, making it simple to author content in a clean, readable format.
- **Customizable Templating with Tera**: Rustic integrates Tera templates, allowing you to design your site layout with full flexibility and apply reusable themes across pages.
- **Automatic Asset Management**: Automatically copies static files (CSS, JS, images) to the output directory for efficient asset handling.
- **Cross-Platform Support**: Developed to work on major platforms (Windows, macOS, and Linux), enabling teams to work seamlessly across environments.
- **Parallelized File Processing**: Uses Rayon to ensure that file handling and content processing are optimized for speed and scale.

### Why Choose Rustic?

Rustic is designed with Rust developers and the Rust community in mind, offering a lightweight yet robust tool for building static websites. It aims to provide an alternative to other static site generators like Hugo by leveraging Rust’s ecosystem for unmatched performance, reliability, and ease of use. Whether you're building a blog, documentation site, or personal portfolio, Rustic makes static site generation intuitive and enjoyable.

### Getting Started
1. **Install Rust**: You can install rust from `https://www.rust-lang.org/tools/install` here.
1. **Installation**: Install via `cargo install rustic-ssg`.
2. **Usage**: Configure your input in the content directory and create a output directory, add content in Markdown, and customize your Tera templates in templates folder.
3. **Build**: Run `rustic build` to generate a complete, static website in seconds.

### Contributing

We welcome contributions! Whether it’s reporting bugs, adding features, or improving documentation, every contribution is valuable to us and to the Rust ecosystem.

## *Note*
This Project is Still on heavy development so %80 of the features hasn't implamented yet.
