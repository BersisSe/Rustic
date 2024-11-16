# **Rustic - Alpha Version** 🚧
> *This is an early alpha version and is not recommended for production use.*  
> *This version may be unstable and is primarily intended for testing and gathering feedback on features.*

---
![Untitled design](https://github.com/user-attachments/assets/48f636ef-9121-4e15-9841-52565a4d9264)

# Rustic - A Lightweight, High-Performance Static Site Generator in Rust 🦀

Rustic is a fast, flexible, and developer-friendly static site generator (SSG) written in Rust. Designed to simplify the process of building static websites and documentation sites, Rustic combines speed and extensibility with the power of Rust’s performance and safety. Build rich, responsive sites effortlessly with a focus on efficiency and simplicity.

### Key Features

- **Blazing-Fast Build Speeds**  
   Leverages Rust's concurrent processing and efficient file handling to quickly generate large sites.

- **Markdown to HTML Conversion**  
   Supports seamless Markdown conversion to HTML, making content creation straightforward and enjoyable.

- **Flexible Templating with Tera**  
   Tera templates provide full control over site layout, allowing developers to create reusable themes and apply custom designs across pages.

- **Automatic Asset Management**  
   Automatically manages static assets like CSS, JavaScript, and images, copying them to the output directory for organized deployment.

- **Cross-Platform Compatibility**  
   Designed to work seamlessly across Windows, macOS, and Linux, allowing teams to work in their preferred environments.

### Why Choose Rustic?

Rustic is built for developers who want a powerful, Rust-based alternative to traditional static site generators like Hugo. By leveraging Rust’s speed, safety, and ecosystem, Rustic is designed for unmatched performance and ease of use. Whether you’re building a blog, documentation, or portfolio, Rustic simplifies the creation of static websites with intuitive commands and fast builds.

---

### Getting Started
#### Installation via Cargo
1. **Install Rust**:  
   If you don’t have Rust installed, download it from [rust-lang.org](https://www.rust-lang.org/tools/install).
   
2. **Install Rustic**:  
   Use Cargo to install Rustic:  
   ```bash
   cargo install rustic-ssg
   ```
   
3. **Initialize a New Project**:  
   Run `rustic init` to set up a new project. **Do not rename the auto-generated folders** for compatibility.

4. **Build the Project**:  
   Run `rustic build` to generate your static website quickly.

#### Installation via Releases
1. **Download Latest Release**:  
   Visit the [Releases page](https://github.com/BersisSe/Rustic/releases) and download the latest executable for your OS.

2. **Add to PATH**:  
   Add the executable to your OS’s PATH to use `rustic` commands globally.

3. **Initialize and Build**:  
   Follow steps 3 and 4 from the Cargo instructions above.

---

### Roadmap

Rustic is under active development, with new features and performance improvements planned. Key upcoming features include:

- **Theming and CSS Support** (Completed✅)
   Pre-built themes for quick styling options and flexibility to add custom CSS.

- **Performance Optimization** (Ongoing..😬)
   Multi-threaded processing and caching mechanisms to further speed up build times.

- **Scripting Integration** 
   Support for JavaScript, Lua, and Python to add dynamic functionality like API data fetching.

---

### Contributing

We’re actively seeking contributions! Whether it’s fixing bugs, suggesting features, improving documentation, or optimizing performance, we appreciate all contributions.  
Please check the [Contributing Guidelines](https://github.com/BersisSe/Rustic/blob/alpha/CONTRIBUTING.md) for details on how to get started.

---

### *Note*

Rustic is in heavy development, and many features (around 60%) are still in progress. Your feedback and bug reports will help shape the future of Rustic—thank you for being a part of our journey!
