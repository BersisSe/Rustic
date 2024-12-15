
# Getting Started with Rustic 🌟

Welcome to Rustic! This guide will help you install, set up, and create your first site with ease.

---

## 🛠️ Prerequisites
Before you begin, make sure you have the following installed:

1. **Rust**: Install Rust via [rustup.rs](https://rustup.rs/).
2. **Git** (Optional): For version control and cloning repositories.
3. **A Text Editor**: Any editor of your choice (we recommend [VS Code](https://code.visualstudio.com/)).

---

## 📥 Installing Rustic

You can install Rustic in three ways:

### 1. Install via Cargo
If you have Rust installed, run the following command:
```bash
cargo install rustic-ssg
```
After installation, verify by running:
```bash
rustic --version
```

### 2. Download Prebuilt Binaries
Rustic’s releases include prebuilt binaries for major platforms.

1. Visit the [Releases Page](https://github.com/your-username/rustic-core/releases).
2. Download the appropriate binary for your OS.
3. Extract the downloaded file and add the binary to your system's PATH.

Verify installation:
```bash
rustic --version
```

### 3. Build from Source
To build from source:

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/rustic-core.git
   cd rustic-core
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Add Rustic to your PATH:
   ```bash
   export PATH=$PATH:/path/to/rustic-core/target/release
   ```
   Replace `/path/to/rustic-core` with the correct path to your cloned repository.

4. Verify installation:
   ```bash
   rustic --version
   ```

---

## ✨ Creating Your First Site

1. **Initialize the site**:
   ```bash
   rustic init
   cd my-rustic-site
   ```
   This sets up a basic site structure:
   ```
   my-rustic-site/
   ├── static/
   ├── output/
   ├── themes/
   │   └── dark.css
   │   └── light.css
   ├── content/
   │   └── index.md
   │   └── meta.yaml
   ├── templates/default/
   │   └── base.html
   └── rustic.config.json
   ```

2. **Edit your content**:
   - Add Markdown files to the `content/` directory.
   - Customize the `templates/base.html` layout.
   - Modify `rustic.config.json` for site-wide settings (e.g., title, theme).

---

## ⚙️ Building the Site

Once you’ve added your content, build the static site:
```bash
rustic build
```
This will generate an `output/` directory with the HTML files.

---

## 👀 Previewing Your Site

Use the built-in development server with live reload:
```bash
rustic serve
```
Visit the URL displayed (e.g., `http://localhost:8080`) to preview your site. Changes in `content/` will automatically reload in the browser.

---

## 🎉 Next Steps

Congratulations! You’ve successfully created and previewed your first Rustic site. Explore more in the docs:

- [Commands](commands.md)
- [Writing Content](writing.md)
- [How to Use Templates](templating.md)

Happy building! 🚀✨
