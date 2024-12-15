<template><div><h1 id="commands-overview-🚀" tabindex="-1"><a class="header-anchor" href="#commands-overview-🚀"><span>Commands Overview 🚀</span></a></h1>
<p>Rustic provides a simple, yet powerful CLI to manage your static site. This guide outlines all available commands to help you get the most out of Rustic.</p>
<hr>
<h2 id="_1-init" tabindex="-1"><a class="header-anchor" href="#_1-init"><span><strong>1. <code v-pre>init</code></strong></span></a></h2>
<p><strong>Description</strong>: Initializes a new Rustic site in the current directory.</p>
<h3 id="usage" tabindex="-1"><a class="header-anchor" href="#usage"><span>Usage:</span></a></h3>
<div class="language-text line-numbers-mode" data-highlighter="prismjs" data-ext="text" data-title="text"><pre v-pre><code><span class="line">rustic init</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div></div></div><h3 id="what-it-does" tabindex="-1"><a class="header-anchor" href="#what-it-does"><span>What It Does:</span></a></h3>
<ul>
<li>
<p>Creates the following site structure:</p>
<div class="language-text line-numbers-mode" data-highlighter="prismjs" data-ext="text" data-title="text"><pre v-pre><code><span class="line">your-site/</span>
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
<p><strong>content/</strong>: Stores your Markdown content and <code v-pre>meta.yaml</code>.</p>
</li>
<li>
<p><strong>themes/</strong>: Contains built-in and custom themes.</p>
</li>
<li>
<p><strong>static/</strong>: Holds static assets like images and styles.</p>
</li>
<li>
<p><strong>templates/</strong>: Contains HTML templates for your site.</p>
</li>
<li>
<p><strong>rustic.config.json</strong>: Configuration file for site-wide settings like the title and theme.</p>
</li>
</ul>
<hr>
<h2 id="_2-build" tabindex="-1"><a class="header-anchor" href="#_2-build"><span><strong>2. <code v-pre>build</code></strong></span></a></h2>
<p><strong>Description</strong>: Builds your static site from the content and templates.</p>
<h3 id="usage-1" tabindex="-1"><a class="header-anchor" href="#usage-1"><span>Usage:</span></a></h3>
<div class="language-text line-numbers-mode" data-highlighter="prismjs" data-ext="text" data-title="text"><pre v-pre><code><span class="line">rustic build</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div></div></div><h3 id="what-it-does-1" tabindex="-1"><a class="header-anchor" href="#what-it-does-1"><span>What It Does:</span></a></h3>
<ul>
<li>Reads the Markdown files from the <code v-pre>content/</code> directory.</li>
<li>Converts them into HTML using templates from the <code v-pre>templates/</code> directory.</li>
<li>Outputs the resulting files to the <code v-pre>output/</code> directory.</li>
</ul>
<h3 id="generated-structure" tabindex="-1"><a class="header-anchor" href="#generated-structure"><span>Generated structure:</span></a></h3>
<div class="language-text line-numbers-mode" data-highlighter="prismjs" data-ext="text" data-title="text"><pre v-pre><code><span class="line">output/</span>
<span class="line">├── index.html</span>
<span class="line">└── static/</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div><hr>
<h2 id="_3-serve" tabindex="-1"><a class="header-anchor" href="#_3-serve"><span><strong>3. <code v-pre>serve</code></strong></span></a></h2>
<p><strong>Description</strong>: Starts a local development server with live-reload functionality.</p>
<h3 id="usage-2" tabindex="-1"><a class="header-anchor" href="#usage-2"><span>Usage:</span></a></h3>
<div class="language-text line-numbers-mode" data-highlighter="prismjs" data-ext="text" data-title="text"><pre v-pre><code><span class="line">rustic serve</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div></div></div><h3 id="what-it-does-2" tabindex="-1"><a class="header-anchor" href="#what-it-does-2"><span>What It Does:</span></a></h3>
<ul>
<li>Serves the site from the <code v-pre>output/</code> directory at <code v-pre>http://localhost:8080</code>.</li>
<li>Watches the <code v-pre>content/</code> directory for changes and automatically rebuilds the site.</li>
<li>Reloads the browser when changes are detected.</li>
</ul>
<h3 id="example-output" tabindex="-1"><a class="header-anchor" href="#example-output"><span>Example Output:</span></a></h3>
<div class="language-text line-numbers-mode" data-highlighter="prismjs" data-ext="text" data-title="text"><pre v-pre><code><span class="line">Server running at http://localhost:8080</span>
<span class="line">Use Ctrl+C to stop the server</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div><div class="line-number"></div></div></div><hr>
<h2 id="_4-clean" tabindex="-1"><a class="header-anchor" href="#_4-clean"><span><strong>4. <code v-pre>clean</code></strong></span></a></h2>
<p><strong>Description</strong>: Removes the <code v-pre>output/</code> directory.</p>
<h3 id="usage-3" tabindex="-1"><a class="header-anchor" href="#usage-3"><span>Usage:</span></a></h3>
<div class="language-text line-numbers-mode" data-highlighter="prismjs" data-ext="text" data-title="text"><pre v-pre><code><span class="line">rustic clean</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div></div></div><h3 id="what-it-does-3" tabindex="-1"><a class="header-anchor" href="#what-it-does-3"><span>What It Does:</span></a></h3>
<ul>
<li>Deletes the entire <code v-pre>output/</code> directory to provide a clean slate for the next build.</li>
</ul>
<hr>
<h2 id="_5-help" tabindex="-1"><a class="header-anchor" href="#_5-help"><span><strong>5. <code v-pre>help</code></strong></span></a></h2>
<p><strong>Description</strong>: Displays a list of available commands or detailed information about a specific command.</p>
<h3 id="usage-4" tabindex="-1"><a class="header-anchor" href="#usage-4"><span>Usage:</span></a></h3>
<div class="language-bash line-numbers-mode" data-highlighter="prismjs" data-ext="sh" data-title="sh"><pre v-pre><code><span class="line">rustic <span class="token builtin class-name">help</span></span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div></div></div><p>Or for specific command help:</p>
<div class="language-bash line-numbers-mode" data-highlighter="prismjs" data-ext="sh" data-title="sh"><pre v-pre><code><span class="line">rustic <span class="token builtin class-name">help</span> <span class="token operator">&lt;</span>command<span class="token operator">></span></span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div></div></div><h3 id="example" tabindex="-1"><a class="header-anchor" href="#example"><span>Example:</span></a></h3>
<div class="language-bash line-numbers-mode" data-highlighter="prismjs" data-ext="sh" data-title="sh"><pre v-pre><code><span class="line">rustic <span class="token builtin class-name">help</span> build</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div></div></div><p>Output:</p>
<div class="language-text line-numbers-mode" data-highlighter="prismjs" data-ext="text" data-title="text"><pre v-pre><code><span class="line">Usage: rustic build</span>
<span class="line">Description: Builds your static site from content and templates.</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div><div class="line-number"></div></div></div><hr>
<p>Explore Rustic and unleash the power of static site generation! 🚀✨</p>
</div></template>


