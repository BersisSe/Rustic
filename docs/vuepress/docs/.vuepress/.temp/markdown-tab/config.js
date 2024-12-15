import { CodeTabs } from "D:/Projects/Rustic/docs/vuepress-starter/node_modules/.pnpm/@vuepress+plugin-markdown-tab@2.0.0-rc.66_markdown-it@14.1.0_vuepress@2.0.0-rc.19_@vuepress+b_xvk4fijdpsw4odx5zmrkyk6fb4/node_modules/@vuepress/plugin-markdown-tab/lib/client/components/CodeTabs.js";
import { Tabs } from "D:/Projects/Rustic/docs/vuepress-starter/node_modules/.pnpm/@vuepress+plugin-markdown-tab@2.0.0-rc.66_markdown-it@14.1.0_vuepress@2.0.0-rc.19_@vuepress+b_xvk4fijdpsw4odx5zmrkyk6fb4/node_modules/@vuepress/plugin-markdown-tab/lib/client/components/Tabs.js";
import "D:/Projects/Rustic/docs/vuepress-starter/node_modules/.pnpm/@vuepress+plugin-markdown-tab@2.0.0-rc.66_markdown-it@14.1.0_vuepress@2.0.0-rc.19_@vuepress+b_xvk4fijdpsw4odx5zmrkyk6fb4/node_modules/@vuepress/plugin-markdown-tab/lib/client/styles/vars.css";

export default {
  enhance: ({ app }) => {
    app.component("CodeTabs", CodeTabs);
    app.component("Tabs", Tabs);
  },
};
