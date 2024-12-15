export const redirects = JSON.parse("{}")

export const routes = Object.fromEntries([
  ["/", { loader: () => import(/* webpackChunkName: "index.html" */"D:/Projects/Rustic/docs/vuepress-starter/docs/.vuepress/.temp/pages/index.html.js"), meta: {"title":""} }],
  ["/guides/commands.html", { loader: () => import(/* webpackChunkName: "guides_commands.html" */"D:/Projects/Rustic/docs/vuepress-starter/docs/.vuepress/.temp/pages/guides/commands.html.js"), meta: {"title":"Commands Overview 🚀"} }],
  ["/guides/docs-index.html", { loader: () => import(/* webpackChunkName: "guides_docs-index.html" */"D:/Projects/Rustic/docs/vuepress-starter/docs/.vuepress/.temp/pages/guides/docs-index.html.js"), meta: {"title":"Welcome to Rustic Documentation 🌲"} }],
  ["/guides/getting-started.html", { loader: () => import(/* webpackChunkName: "guides_getting-started.html" */"D:/Projects/Rustic/docs/vuepress-starter/docs/.vuepress/.temp/pages/guides/getting-started.html.js"), meta: {"title":"Getting Started with Rustic 🌟"} }],
  ["/guides/templating.html", { loader: () => import(/* webpackChunkName: "guides_templating.html" */"D:/Projects/Rustic/docs/vuepress-starter/docs/.vuepress/.temp/pages/guides/templating.html.js"), meta: {"title":""} }],
  ["/guides/writing.html", { loader: () => import(/* webpackChunkName: "guides_writing.html" */"D:/Projects/Rustic/docs/vuepress-starter/docs/.vuepress/.temp/pages/guides/writing.html.js"), meta: {"title":"How to Write content in Markdown"} }],
  ["/market/market-index.html", { loader: () => import(/* webpackChunkName: "market_market-index.html" */"D:/Projects/Rustic/docs/vuepress-starter/docs/.vuepress/.temp/pages/market/market-index.html.js"), meta: {"title":"Under Construction 🚧"} }],
  ["/404.html", { loader: () => import(/* webpackChunkName: "404.html" */"D:/Projects/Rustic/docs/vuepress-starter/docs/.vuepress/.temp/pages/404.html.js"), meta: {"title":""} }],
]);

if (import.meta.webpackHot) {
  import.meta.webpackHot.accept()
  if (__VUE_HMR_RUNTIME__.updateRoutes) {
    __VUE_HMR_RUNTIME__.updateRoutes(routes)
  }
  if (__VUE_HMR_RUNTIME__.updateRedirects) {
    __VUE_HMR_RUNTIME__.updateRedirects(redirects)
  }
}

if (import.meta.hot) {
  import.meta.hot.accept(({ routes, redirects }) => {
    __VUE_HMR_RUNTIME__.updateRoutes(routes)
    __VUE_HMR_RUNTIME__.updateRedirects(redirects)
  })
}
