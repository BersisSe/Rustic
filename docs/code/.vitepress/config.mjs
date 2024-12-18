import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "Rustic Hub",
  description: "The central hub for all things Rustic. Learn, Discover, Collaborate.",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Guide', link: '/guides/' },
      { text: 'Marketplace', link: '/market/' },
    ],
  
    
    socialLinks: [
      { icon: 'github', link: 'https://github.com/BersisSe/rustic-core' }
    ]
  }
})
