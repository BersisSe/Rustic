<template><div><h1 id="how-to-write-content-in-markdown" tabindex="-1"><a class="header-anchor" href="#how-to-write-content-in-markdown"><span>How to Write content in Markdown</span></a></h1>
<h3 id="introduction" tabindex="-1"><a class="header-anchor" href="#introduction"><span>Introduction</span></a></h3>
<ul>
<li>Brief overview of Rustic's Markdown support.</li>
<li>Emphasize simplicity and compatibility with standard Markdown.</li>
</ul>
<h3 id="supported-markdown-syntax" tabindex="-1"><a class="header-anchor" href="#supported-markdown-syntax"><span>Supported Markdown Syntax</span></a></h3>
<ul>
<li><strong>Headers</strong>: <code v-pre>#</code>, <code v-pre>##</code>, <code v-pre>###</code> for heading levels.</li>
<li><strong>Emphasis</strong>: <code v-pre>*italic*</code>, <code v-pre>**bold**</code>, <code v-pre>~~strikethrough~~</code>.</li>
<li><strong>Lists</strong>: Ordered (<code v-pre>1. Item</code>) and unordered (<code v-pre>- Item</code>).</li>
<li><strong>Links and Images</strong>: <code v-pre>[Link](url)</code> and <code v-pre>![Image](url)</code>.</li>
<li><strong>Code Blocks</strong>: ```rust`, ```html`, etc.</li>
</ul>
<h3 id="metadata-files" tabindex="-1"><a class="header-anchor" href="#metadata-files"><span>Metadata Files</span></a></h3>
<ul>
<li>Explain the role of <code v-pre>meta.yaml</code> in organizing content.</li>
<li>Example <code v-pre>meta.yaml</code> file:<div class="language-json line-numbers-mode" data-highlighter="prismjs" data-ext="json" data-title="json"><pre v-pre><code><span class="line"><span class="token punctuation">{</span></span>
<span class="line">  <span class="token property">"title"</span><span class="token operator">:</span> <span class="token string">"My Blog Post"</span><span class="token punctuation">,</span></span>
<span class="line">  <span class="token property">"author"</span><span class="token operator">:</span> <span class="token string">"Jane Doe"</span><span class="token punctuation">,</span></span>
<span class="line">  <span class="token property">"date"</span><span class="token operator">:</span> <span class="token string">"2024-11-24"</span></span>
<span class="line"><span class="token punctuation">}</span></span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div></li>
<li>How <code v-pre>meta.yaml</code> enhances content generation (e.g., automatic title tags).</li>
</ul>
<h3 id="advanced-features" tabindex="-1"><a class="header-anchor" href="#advanced-features"><span>Advanced Features</span></a></h3>
<ul>
<li>Adding inline HTML for custom styling.</li>
<li>Tips for managing media (e.g., embedding images or videos).</li>
</ul>
<h3 id="example-markdown-file" tabindex="-1"><a class="header-anchor" href="#example-markdown-file"><span>Example Markdown File</span></a></h3>
<div class="language-markdown line-numbers-mode" data-highlighter="prismjs" data-ext="md" data-title="md"><pre v-pre><code><span class="line"><span class="token title important"><span class="token punctuation">#</span> Welcome to My Rustic Site</span></span>
<span class="line">This is a <span class="token bold"><span class="token punctuation">**</span><span class="token content">bold</span><span class="token punctuation">**</span></span> statement, and this is <span class="token italic"><span class="token punctuation">*</span><span class="token content">italicized</span><span class="token punctuation">*</span></span> text.</span>
<span class="line"></span>
<span class="line"><span class="token title important"><span class="token punctuation">##</span> Things I Love</span></span>
<span class="line"><span class="token list punctuation">-</span> Coding</span>
<span class="line"><span class="token list punctuation">-</span> Writing</span>
<span class="line"><span class="token list punctuation">-</span> Rustic!</span>
<span class="line"></span>
<span class="line"><span class="token url">[<span class="token content">Learn more about Rustic</span>](<span class="token url">https://github.com/your-repo/rustic</span>)</span></span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div><hr>
</div></template>


