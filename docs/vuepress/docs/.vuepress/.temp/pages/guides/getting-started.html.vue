<template><div><h1 id="getting-started-with-rustic-🌟" tabindex="-1"><a class="header-anchor" href="#getting-started-with-rustic-🌟"><span>Getting Started with Rustic 🌟</span></a></h1>
<p>Welcome to Rustic! This guide will help you install, set up, and create your first site with ease.</p>
<hr>
<h2 id="🛠️-prerequisites" tabindex="-1"><a class="header-anchor" href="#🛠️-prerequisites"><span>🛠️ Prerequisites</span></a></h2>
<p>Before you begin, make sure you have the following installed:</p>
<ol>
<li><strong>Rust</strong>: Install Rust via <a href="https://rustup.rs/" target="_blank" rel="noopener noreferrer">rustup.rs</a>.</li>
<li><strong>Git</strong> (Optional): For version control and cloning repositories.</li>
<li><strong>A Text Editor</strong>: Any editor of your choice (we recommend <a href="https://code.visualstudio.com/" target="_blank" rel="noopener noreferrer">VS Code</a>).</li>
</ol>
<hr>
<h2 id="📥-installing-rustic" tabindex="-1"><a class="header-anchor" href="#📥-installing-rustic"><span>📥 Installing Rustic</span></a></h2>
<p>You can install Rustic in three ways:</p>
<h3 id="_1-install-via-cargo" tabindex="-1"><a class="header-anchor" href="#_1-install-via-cargo"><span>1. Install via Cargo</span></a></h3>
<p>If you have Rust installed, run the following command:</p>
<div class="language-bash line-numbers-mode" data-highlighter="prismjs" data-ext="sh" data-title="sh"><pre v-pre><code><span class="line"><span class="token function">cargo</span> <span class="token function">install</span> rustic-ssg</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div></div></div><p>After installation, verify by running:</p>
<div class="language-bash line-numbers-mode" data-highlighter="prismjs" data-ext="sh" data-title="sh"><pre v-pre><code><span class="line">rustic <span class="token parameter variable">--version</span></span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div></div></div><h3 id="_2-download-prebuilt-binaries" tabindex="-1"><a class="header-anchor" href="#_2-download-prebuilt-binaries"><span>2. Download Prebuilt Binaries</span></a></h3>
<p>Rustic’s releases include prebuilt binaries for major platforms.</p>
<ol>
<li>Visit the <a href="https://github.com/your-username/rustic-core/releases" target="_blank" rel="noopener noreferrer">Releases Page</a>.</li>
<li>Download the appropriate binary for your OS.</li>
<li>Extract the downloaded file and add the binary to your system's PATH.</li>
</ol>
<p>Verify installation:</p>
<div class="language-bash line-numbers-mode" data-highlighter="prismjs" data-ext="sh" data-title="sh"><pre v-pre><code><span class="line">rustic <span class="token parameter variable">--version</span></span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div></div></div><h3 id="_3-build-from-source" tabindex="-1"><a class="header-anchor" href="#_3-build-from-source"><span>3. Build from Source</span></a></h3>
<p>To build from source:</p>
<ol>
<li>
<p>Clone the repository:</p>
<div class="language-bash line-numbers-mode" data-highlighter="prismjs" data-ext="sh" data-title="sh"><pre v-pre><code><span class="line"><span class="token function">git</span> clone https://github.com/your-username/rustic-core.git</span>
<span class="line"><span class="token builtin class-name">cd</span> rustic-core</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div><div class="line-number"></div></div></div></li>
<li>
<p>Build the project:</p>
<div class="language-bash line-numbers-mode" data-highlighter="prismjs" data-ext="sh" data-title="sh"><pre v-pre><code><span class="line"><span class="token function">cargo</span> build <span class="token parameter variable">--release</span></span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div></div></div></li>
<li>
<p>Add Rustic to your PATH:</p>
<div class="language-bash line-numbers-mode" data-highlighter="prismjs" data-ext="sh" data-title="sh"><pre v-pre><code><span class="line"><span class="token builtin class-name">export</span> <span class="token assign-left variable"><span class="token environment constant">PATH</span></span><span class="token operator">=</span><span class="token environment constant">$PATH</span>:/path/to/rustic-core/target/release</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div></div></div><p>Replace <code v-pre>/path/to/rustic-core</code> with the correct path to your cloned repository.</p>
</li>
<li>
<p>Verify installation:</p>
<div class="language-bash line-numbers-mode" data-highlighter="prismjs" data-ext="sh" data-title="sh"><pre v-pre><code><span class="line">rustic <span class="token parameter variable">--version</span></span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div></div></div></li>
</ol>
<hr>
<h2 id="✨-creating-your-first-site" tabindex="-1"><a class="header-anchor" href="#✨-creating-your-first-site"><span>✨ Creating Your First Site</span></a></h2>
<ol>
<li>
<p><strong>Initialize the site</strong>:</p>
<div class="language-bash line-numbers-mode" data-highlighter="prismjs" data-ext="sh" data-title="sh"><pre v-pre><code><span class="line">rustic init</span>
<span class="line"><span class="token builtin class-name">cd</span> my-rustic-site</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div><div class="line-number"></div></div></div><p>This sets up a basic site structure:</p>
<div class="language-text line-numbers-mode" data-highlighter="prismjs" data-ext="text" data-title="text"><pre v-pre><code><span class="line">my-rustic-site/</span>
<span class="line">├── static/</span>
<span class="line">├── output/</span>
<span class="line">├── themes/</span>
<span class="line">│   └── dark.css</span>
<span class="line">│   └── light.css</span>
<span class="line">├── content/</span>
<span class="line">│   └── index.md</span>
<span class="line">│   └── meta.yaml</span>
<span class="line">├── templates/default/</span>
<span class="line">│   └── base.html</span>
<span class="line">└── rustic.config.json</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div></li>
<li>
<p><strong>Edit your content</strong>:</p>
<ul>
<li>Add Markdown files to the <code v-pre>content/</code> directory.</li>
<li>Customize the <code v-pre>templates/base.html</code> layout.</li>
<li>Modify <code v-pre>rustic.config.json</code> for site-wide settings (e.g., title, theme).</li>
</ul>
</li>
</ol>
<hr>
<h2 id="⚙️-building-the-site" tabindex="-1"><a class="header-anchor" href="#⚙️-building-the-site"><span>⚙️ Building the Site</span></a></h2>
<p>Once you’ve added your content, build the static site:</p>
<div class="language-bash line-numbers-mode" data-highlighter="prismjs" data-ext="sh" data-title="sh"><pre v-pre><code><span class="line">rustic build</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div></div></div><p>This will generate an <code v-pre>output/</code> directory with the HTML files.</p>
<hr>
<h2 id="👀-previewing-your-site" tabindex="-1"><a class="header-anchor" href="#👀-previewing-your-site"><span>👀 Previewing Your Site</span></a></h2>
<p>Use the built-in development server with live reload:</p>
<div class="language-bash line-numbers-mode" data-highlighter="prismjs" data-ext="sh" data-title="sh"><pre v-pre><code><span class="line">rustic serve</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div></div></div><p>Visit the URL displayed (e.g., <code v-pre>http://localhost:8080</code>) to preview your site. Changes in <code v-pre>content/</code> will automatically reload in the browser.</p>
<hr>
<h2 id="🎉-next-steps" tabindex="-1"><a class="header-anchor" href="#🎉-next-steps"><span>🎉 Next Steps</span></a></h2>
<p>Congratulations! You’ve successfully created and previewed your first Rustic site. Explore more in the docs:</p>
<ul>
<li><RouteLink to="/guides/commands.html">Commands</RouteLink></li>
<li><RouteLink to="/guides/writing.html">Writing Content</RouteLink></li>
<li><RouteLink to="/guides/templating.html">How to Use Templates</RouteLink></li>
</ul>
<p>Happy building! 🚀✨</p>
</div></template>


