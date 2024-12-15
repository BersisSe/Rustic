import { defaultTheme } from '@vuepress/theme-default'
import { defineUserConfig } from 'vuepress/cli'
import { viteBundler } from '@vuepress/bundler-vite'

export default defineUserConfig({
  lang: 'en-US',

  title: 'Rustic Hub',
  description: 'Where All assets and docs about Rustic live',

  theme: defaultTheme({
    logo: 'images/rustic-logo.png',
  
    navbar: [
      {
        text: "Docs",
        link: "/guides/docs-index",
      },
      {
        text: "Marketplace",
        link: "/market/market-index.md",
      }
      
    ],
  }),

  bundler: viteBundler(),
})
