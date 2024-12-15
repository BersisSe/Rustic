import comp from "D:/Projects/Rustic/docs/vuepress-starter/docs/.vuepress/.temp/pages/guides/docs-index.html.vue"
const data = JSON.parse("{\"path\":\"/guides/docs-index.html\",\"title\":\"Welcome to Rustic Documentation 🌲\",\"lang\":\"en-US\",\"frontmatter\":{},\"headers\":[{\"level\":2,\"title\":\"📖 Documentation Overview\",\"slug\":\"📖-documentation-overview\",\"link\":\"#📖-documentation-overview\",\"children\":[]},{\"level\":2,\"title\":\"🚀 Key Features\",\"slug\":\"🚀-key-features\",\"link\":\"#🚀-key-features\",\"children\":[]},{\"level\":2,\"title\":\"🌟 Why Rustic?\",\"slug\":\"🌟-why-rustic\",\"link\":\"#🌟-why-rustic\",\"children\":[]},{\"level\":2,\"title\":\"🌐 Get Involved\",\"slug\":\"🌐-get-involved\",\"link\":\"#🌐-get-involved\",\"children\":[]}],\"git\":{},\"filePathRelative\":\"guides/docs-index.md\"}")
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
