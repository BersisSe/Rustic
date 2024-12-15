import comp from "D:/Projects/Rustic/docs/vuepress-starter/docs/.vuepress/.temp/pages/index.html.vue"
const data = JSON.parse("{\"path\":\"/\",\"title\":\"\",\"lang\":\"en-US\",\"frontmatter\":{\"home\":true,\"heroImage\":\"/images/rustic-logo.png\",\"tagline\":\"The next-generation Static Site Generator with simplicity and customization at its core.\",\"actions\":[{\"text\":\"Get Started →\",\"link\":\"/guides/getting-started\",\"type\":\"primary\"},{\"text\":\"Learn More\",\"link\":\"/about/\",\"type\":\"secondary\"}],\"features\":[{\"title\":\"Powerful CLI\",\"details\":\"A clean and intuitive CLI for building, previewing, and managing your static site with ease.\"},{\"title\":\"Flexible Layout System\",\"details\":\"Leverage a hybrid layout system that combines pre-built templates and user-defined Markdown layouts.\"},{\"title\":\"Community-Driven Marketplace\",\"details\":\"Explore themes and plugins shared by the community, or contribute your own creations.\"}],\"footer\":\"Crafted with care by the Bersis Sevimli | Open Source under Apache 2.0 License\"},\"headers\":[{\"level\":2,\"title\":\"Why Choose Rustic?\",\"slug\":\"why-choose-rustic\",\"link\":\"#why-choose-rustic\",\"children\":[{\"level\":3,\"title\":\"What Can You Build?\",\"slug\":\"what-can-you-build\",\"link\":\"#what-can-you-build\",\"children\":[]}]},{\"level\":2,\"title\":\"Explore the Rustic Hub 🚀\",\"slug\":\"explore-the-rustic-hub-🚀\",\"link\":\"#explore-the-rustic-hub-🚀\",\"children\":[{\"level\":3,\"title\":\"Get Involved\",\"slug\":\"get-involved\",\"link\":\"#get-involved\",\"children\":[]}]}],\"git\":{},\"filePathRelative\":\"index.md\"}")
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
