# Rustic 🌲  
**Rustic** is a fast and minimalistic static site generator built with Rust. Create stunning websites with simplicity, flexibility, and lightning-fast performance.

---

## 🚀 Features
- **Dynamic Markdown to HTML conversion** with templating support.  
- **Lightning-fast builds** leveraging Rust's performance.  
- **Live preview server** with **hot reload**.  
- **Customizable themes and templates**.  
- **Simple CLI for seamless workflows**.

---

## 📥 Installation

### **1. Install via Cargo**
Ensure you have [Rust installed](https://rustup.rs/), then run:
```bash
cargo install rustic-ssg
```

### **2. Download Prebuilt Binaries**
Download prebuilt binaries from the [Releases Page](https://github.com/BersisSe/rustic-core/releases).  
Add the binary to your system PATH to use it globally.

### **3. Build from Source**
Clone the repository and build using Cargo:
```bash
git clone https://github.com/BersisSe/rustic-core.git
cd rustic-core
cargo build --release
```
Move the resulting binary (`target/release/rustic`) to a directory in your PATH.

---

## 🛠️ Usage

### **Commands**
| Command         | Description                                   |
|------------------|-----------------------------------------------|
| `rustic init`   | Initialize a new Rustic project.             |
| `rustic build`  | Generate the static site in the `output/` folder. |
| `rustic serve`  | Start a local server with hot reload.         |
| `rustic clean`  | Remove the `output/` directory.               |
| `rustic help`   | Display command usage and options.            |

### **Example Workflow**
1. **Initialize a project**:
   ```bash
   rustic init
   cd my-rustic-site
   ```
2. **Build the site**:
   ```bash
   rustic build
   ```
3. **Preview locally**:
   ```bash
   rustic serve
   ```
4. **Clean the build**:
   ```bash
   rustic clean
   ```

---

## ✨ Documentation
Detailed guides and examples are available in the [Documentation](https://bersisse.github.io/rustic-core/).

---

## 🌍 Community
- **Contributions** are welcome! Check the [Contributing Guide](CONTRIBUTING.md).  
- Report bugs or suggest features via [GitHub Issues](https://github.com/BersisSe/rustic-core/issues).

---

## 📜 License
Rustic is open-source software, licensed under the [Apache-2.0 License](LICENSE).


