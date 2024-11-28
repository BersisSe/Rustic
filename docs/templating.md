## How use Templates
### Introduction
- What are templates in Rustic?
- Overview of how `base.html` works as the default layout.

### Template Structure
- **Placeholders**:
  - `{content}`: Main content.
  - `{title}`: Page title (from `meta.json`).
- Default folder structure:
  ```
  templates/
    base.html
    custom.html
  ```

### Modifying the Default Template
- Explain how to find `base.html`.
- Walk through adding custom styles or scripts.
  ```html
  <head>
      <title>{title}</title>
      <link rel="stylesheet" href="/static/styles.css">
  </head>
  ```

### Creating a New Template
- Steps to define a new template (e.g., `blog_post.html`).
- How to assign a template to specific pages using metadata:
  ```json
  {
    "template": "blog_post.html",
    "title": "My Blog Post"
  }
  ```

### Advanced Tips
- Include dynamic elements like navigation bars.
- Using conditionals for flexibility (`if`, `else` statements).

