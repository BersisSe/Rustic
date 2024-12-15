import comp from "D:/Projects/Rustic/docs/vuepress-starter/docs/.vuepress/.temp/pages/market/market-index.html.vue"
const data = JSON.parse("{\"path\":\"/market/market-index.html\",\"title\":\"Under Construction 🚧\",\"lang\":\"en-US\",\"frontmatter\":{},\"headers\":[],\"git\":{},\"filePathRelative\":\"market/market-index.md\"}")
export { comp, data }

if (import.meta.webpackHot) {
  import.meta.webpackHot.accept()
  if (__VUE_HMR_RUNTIME__.updatePageData) {
    __VUE_HMR_RUNTIME__.updatePageData(data)
  }
}

if (import.meta.hot) {
  import.meta.hot.accept(({ data }) => {
    __VUE_HMR_RUNTIME__.updatePageData(data)
  })
}
