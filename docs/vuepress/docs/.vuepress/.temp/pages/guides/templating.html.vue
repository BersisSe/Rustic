<template><div><h2 id="how-use-templates" tabindex="-1"><a class="header-anchor" href="#how-use-templates"><span>How use Templates</span></a></h2>
<h3 id="introduction" tabindex="-1"><a class="header-anchor" href="#introduction"><span>Introduction</span></a></h3>
<ul>
<li>What are templates in Rustic?</li>
<li>Overview of how <code v-pre>base.html</code> works as the default layout.</li>
</ul>
<h3 id="template-structure" tabindex="-1"><a class="header-anchor" href="#template-structure"><span>Template Structure</span></a></h3>
<ul>
<li><strong>Placeholders</strong>:
<ul>
<li><code v-pre>{content}</code>: Main content.</li>
<li><code v-pre>{title}</code>: Page title (from <code v-pre>meta.yaml</code>).</li>
</ul>
</li>
<li>Default folder structure:<div class="language-text line-numbers-mode" data-highlighter="prismjs" data-ext="text" data-title="text"><pre v-pre><code><span class="line">templates/</span>
<span class="line">  base.html</span>
<span class="line">  custom.html</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div></li>
</ul>
<h3 id="modifying-the-default-template" tabindex="-1"><a class="header-anchor" href="#modifying-the-default-template"><span>Modifying the Default Template</span></a></h3>
<ul>
<li>Explain how to find <code v-pre>base.html</code>.</li>
<li>Walk through adding custom styles or scripts.<div class="language-html line-numbers-mode" data-highlighter="prismjs" data-ext="html" data-title="html"><pre v-pre><code><span class="line"><span class="token tag"><span class="token tag"><span class="token punctuation">&lt;</span>head</span><span class="token punctuation">></span></span></span>
<span class="line">    <span class="token tag"><span class="token tag"><span class="token punctuation">&lt;</span>title</span><span class="token punctuation">></span></span>{title}<span class="token tag"><span class="token tag"><span class="token punctuation">&lt;/</span>title</span><span class="token punctuation">></span></span></span>
<span class="line">    <span class="token tag"><span class="token tag"><span class="token punctuation">&lt;</span>link</span> <span class="token attr-name">rel</span><span class="token attr-value"><span class="token punctuation attr-equals">=</span><span class="token punctuation">"</span>stylesheet<span class="token punctuation">"</span></span> <span class="token attr-name">href</span><span class="token attr-value"><span class="token punctuation attr-equals">=</span><span class="token punctuation">"</span>/static/styles.css<span class="token punctuation">"</span></span><span class="token punctuation">></span></span></span>
<span class="line"><span class="token tag"><span class="token tag"><span class="token punctuation">&lt;/</span>head</span><span class="token punctuation">></span></span></span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div></li>
</ul>
<h3 id="creating-a-new-template" tabindex="-1"><a class="header-anchor" href="#creating-a-new-template"><span>Creating a New Template</span></a></h3>
<ul>
<li>Steps to define a new template (e.g., <code v-pre>blog_post.html</code>).</li>
<li>How to assign a template to specific pages using metadata:<div class="language-json line-numbers-mode" data-highlighter="prismjs" data-ext="json" data-title="json"><pre v-pre><code><span class="line"><span class="token punctuation">{</span></span>
<span class="line">  <span class="token property">"template"</span><span class="token operator">:</span> <span class="token string">"blog_post.html"</span><span class="token punctuation">,</span></span>
<span class="line">  <span class="token property">"title"</span><span class="token operator">:</span> <span class="token string">"My Blog Post"</span></span>
<span class="line"><span class="token punctuation">}</span></span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div></li>
</ul>
<h3 id="advanced-tips" tabindex="-1"><a class="header-anchor" href="#advanced-tips"><span>Advanced Tips</span></a></h3>
<ul>
<li>Include dynamic elements like navigation bars.</li>
<li>Using conditionals for flexibility (<code v-pre>if</code>, <code v-pre>else</code> statements).</li>
</ul>
</div></template>


