pub const INDEX_MD_CONTENT: &str = r#"
# Welcome to Rustic

This is the main content for the home page. You can add more content here.
"#;
pub const BASE_HTML_CONTENT: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="description" content="{{ description  }}">
    <meta name="keywords" content="{{ keywords }}">
    <meta property="og:type" content="website">
    <meta property="og:title" content="{{ config.title }}">
    <meta property="og:description" content="{{ description }}">
    <title>{{ config.title }}</title>
    <link rel="stylesheet" href="{{config.theme}}.css">

</head>
<body>
    <header>
        <h1>{{ config.title }}</h1>
    </header>
    <main>
        {{ content | safe }}
    </main>
    
</body>
</html>
"#;
pub const DARK_CSS: &str = r#"
:root {
    --background-color: #121212;
    --text-color: #e0e0e0;
    --link-color: #1e90ff;
    --header-bg: #1c1c1c;
    --header-text-color: #ffffff;
    --button-bg: #1e90ff;
    --button-text: #ffffff;
}

body {
    background-color: var(--background-color);
    color: var(--text-color);
    font-family: Arial, sans-serif;
    margin: 0;
    padding: 0;
    line-height: 1.6;
}

header {
    background-color: var(--header-bg);
    padding: 1rem;
    text-align: center;
}

header h1 {
    color: var(--header-text-color);
    margin: 0;
}

a {
    color: var(--link-color);
    text-decoration: none;
}

a:hover {
    text-decoration: underline;
}

button {
    background-color: var(--button-bg);
    color: var(--button-text);
    border: none;
    padding: 0.5rem 1rem;
    cursor: pointer;
    font-size: 1rem;
}

button:hover {
    background-color: #007acc;
}

.container {
    max-width: 800px;
    margin: 0 auto;
    padding: 1rem;
}

footer {
    text-align: center;
    padding: 1rem;
    color: var(--text-color);
    background-color: #333333;
}
"#;
pub const LIGHT_CSS: &str = r#"
:root {
    --background-color: #ffffff;
    --text-color: #333333;
    --link-color: #0066cc;
    --header-bg: #f4f4f9;
    --header-text-color: #333333;
    --button-bg: #0066cc;
    --button-text: #ffffff;
}

body {
    background-color: var(--background-color);
    color: var(--text-color);
    font-family: Arial, sans-serif;
    margin: 0;
    padding: 0;
    line-height: 1.6;
}

header {
    background-color: var(--header-bg);
    padding: 1rem;
    text-align: center;
}

header h1 {
    color: var(--header-text-color);
    margin: 0;
}

a {
    color: var(--link-color);
    text-decoration: none;
}

a:hover {
    text-decoration: underline;
}

button {
    background-color: var(--button-bg);
    color: var(--button-text);
    border: none;
    padding: 0.5rem 1rem;
    cursor: pointer;
    font-size: 1rem;
}

button:hover {
    background-color: #005bb5;
}

.container {
    max-width: 800px;
    margin: 0 auto;
    padding: 1rem;
}

footer {
    text-align: center;
    padding: 1rem;
    color: var(--text-color);
    background-color: #eeeeee;
}
"#;
pub const META: &str = r#"
{
  "pages": {
    "index": {
      "title": "Welcome to Rustic",
      "description": "A blazing fast, simple static site generator.",
      "keywords": ["rustic", "static site generator", "rust"],
      "canonical": "/"
            }
    }
}
"#;

