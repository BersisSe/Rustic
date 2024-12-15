import comp from "D:/Projects/Rustic/docs/vuepress-starter/docs/.vuepress/.temp/pages/guides/writing.html.vue"
const data = JSON.parse("{\"path\":\"/guides/writing.html\",\"title\":\"How to Write content in Markdown\",\"lang\":\"en-US\",\"frontmatter\":{},\"headers\":[{\"level\":3,\"title\":\"Introduction\",\"slug\":\"introduction\",\"link\":\"#introduction\",\"children\":[]},{\"level\":3,\"title\":\"Supported Markdown Syntax\",\"slug\":\"supported-markdown-syntax\",\"link\":\"#supported-markdown-syntax\",\"children\":[]},{\"level\":3,\"title\":\"Metadata Files\",\"slug\":\"metadata-files\",\"link\":\"#metadata-files\",\"children\":[]},{\"level\":3,\"title\":\"Advanced Features\",\"slug\":\"advanced-features\",\"link\":\"#advanced-features\",\"children\":[]},{\"level\":3,\"title\":\"Example Markdown File\",\"slug\":\"example-markdown-file\",\"link\":\"#example-markdown-file\",\"children\":[]}],\"git\":{},\"filePathRelative\":\"guides/writing.md\"}")
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
