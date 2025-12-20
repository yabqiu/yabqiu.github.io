---
title: 博客从 WordPress 迁移至 GitHub 手记
url: /blog-from-wordpress-to-github-notes/
date: 2025-12-17T21:28:00-06:00
featured: false
draft: true
type: post
toc: false
# menu: main
usePageBundles: true
categories:
  - blog
tags: 
  - notes
comment: true
---

原来在 WordPress 数据库中还遭受过注入攻击
```html
<div id="xunlei_com_thunder_helper_plugin_d462f475-c18e-46be-bd10-327458d045bd"> </div>
```
大量的
```
  <br/><br/>
```
代码片段从 &lt;pre&gt; 转换成

<pre>
&#123;&#123;&lt; highlight java &gt;&#125;&#125;
</pre>

有很多 &lt; &gt; &amp; 要替换

想直接用 Rust 或 Python 组件转换 html 到 markdown, 但都不准确. 所以用 bundle/page.html, 
不能为 bundle/index.html 否则在 public/ 目录中会生成不一样的内容

引用 bundle 中的图片与其他次资源
引入评论，访问计数

第一次完全用 Claude Code CLI Vibe coding 了一个 'Hugo Fix' 插件

HTML 是设计为浏览器渲染的语法结构，不利于人工阅读，例如有些日志被无端的压缩成了一行，对浏览器没有任何压力，要人工修改就难了。
而且还有大量的 HTML entities，而 Markdown 是适于人工编写与阅读的，没有多余的标签，可用拆行增加阅读性，中间两行才是实际
显示的换行

Bundle 方便于组织与日志相关的资源

{{< bundle-image wp-hugo-1.png 611 >}}

{{< figure src="wp-hugo-1.png" width="611px" >}}

[hugo figure shortcode](https://gohugo.io/shortcodes/figure/)

<pre>
&#123;&#123;&lt; figure
    src="/images/examples/zion-national-park.jpg"
    alt="A photograph of Zion National Park"
    link="https://www.nps.gov/zion/index.htm"
    caption="Zion National Park"
    class="ma0 w-75"
&gt;&#125;&#125;
</pre>

{{< figure src="wp-hugo-1.png" width="611px" alt="Image Alt" caption="Image Caption" >}}

bundle/page.html 与 bundle/index.md(bundle/index.html) 时, Hugo 会编译 bundle 内资源到不同的目录中


use bundle-image
{{< bundle-image wp-hugo-1.png 611 >}}

还有很多 http://unmi.cc/xxx 遗留链接，图片链接