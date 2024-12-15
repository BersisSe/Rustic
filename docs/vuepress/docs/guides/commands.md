# Commands Overview 🚀

Rustic provides a simple, yet powerful CLI to manage your static site. This guide outlines all available commands to help you get the most out of Rustic.

---

## **1. `init`**
**Description**: Initializes a new Rustic site in the current directory.

### Usage:
```
rustic init
```

### What It Does:
- Creates the following site structure:
  ```
  your-site/
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

- **content/**: Stores your Markdown content and `meta.yaml`.
- **themes/**: Contains built-in and custom themes.
- **static/**: Holds static assets like images and styles.
- **templates/**: Contains HTML templates for your site.
- **rustic.config.json**: Configuration file for site-wide settings like the title and theme.

---

## **2. `build`**
**Description**: Builds your static site from the content and templates.

### Usage:
```
rustic build
```

### What It Does:
- Reads the Markdown files from the `content/` directory.
- Converts them into HTML using templates from the `templates/` directory.
- Outputs the resulting files to the `output/` directory.

### Generated structure:
```
output/
├── index.html
└── static/
```

---

## **3. `serve`**
**Description**: Starts a local development server with live-reload functionality.

### Usage:
```
rustic serve
```

### What It Does:
- Serves the site from the `output/` directory at `http://localhost:8080`.
- Watches the `content/` directory for changes and automatically rebuilds the site.
- Reloads the browser when changes are detected.

### Example Output:
```
Server running at http://localhost:8080
Use Ctrl+C to stop the server
```

---

## **4. `clean`**
**Description**: Removes the `output/` directory.

### Usage:
```
rustic clean
```

### What It Does:
- Deletes the entire `output/` directory to provide a clean slate for the next build.

---

## **5. `help`**
**Description**: Displays a list of available commands or detailed information about a specific command.

### Usage:
```bash
rustic help
```
Or for specific command help:
```bash
rustic help <command>
```

### Example:
```bash
rustic help build
```
Output:
```
Usage: rustic build
Description: Builds your static site from content and templates.
```

---

Explore Rustic and unleash the power of static site generation! 🚀✨

