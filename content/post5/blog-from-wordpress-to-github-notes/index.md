---
title: 博客从 WordPress 迁移至 GitHub 手记
url: /blog-from-wordpress-to-github-notes/
date: 2025-12-17T21:28:00-06:00
featured: false
draft: false
type: post
toc: false
# menu: main
usePageBundles: true
codeMaxLines: 50
categories:
  - blog
tags: 
  - notes
comment: true
---

经过足足 3 周的时间终于把博客 https://yanbin.blog 从 AWS Lightsail 主机的 WordPress 迁移到了 Hugo 搭建的 GitHub Pages 上。
从动态页面转换成纯静态页，访问时确实是飞快，至少是从北美访问每个半秒以下开。

本来一个博客网站就没有必要搞成动态的，之前网站由于操作系统，WordPress, 和数据库的升级，一段时间以来常出现网站无法访问，1G 内存都顶不住，
后经 一番 Apache, MySQL, 系统调优才得以解决，不过 WordPress 再如何使用文件缓存性能都比静态页面差许多。

快速回顾一下本博客的历史，2006 年前 QQ 空间，后来 blogcn.com, 再到 blogjava.net，2010 年始声请了 unmi.cc 域名, 租 VPS 自搭 WordPress 
服务，后面就不断的换 VPS 提供商，也出现过数据少量丢失的现象，所以有些图片或附件不可考。由于 unmi.cc 无法顺利迁出才有了新的 yanbin.blog 域名,
所以博文中还有不少 unmi.cc 的影子，以至于 unmi.cc 被人注册了，并且还堂而皇之的建立了一个李鬼网站，其中很多标题是盗用我的，内容全是 AI 生成。
<!--more-->

考虑使用静态页面博客的优点有

1. 省钱, 不用租用 VPS 来搭建 WordPress. 可免费托管静态页面的地方有 Cloudflare Pages, GitHub Page, 和 Netlify
2. 借助于 GitHub, 有着天然的版本管理功能, 不需要 WordPress 的 Revision 功能. 还不用担心备份的问题
3. 速度极快, 这就是静态页面直接带来的好处, 如果必要的话还能免费使用 Cloudflare, Netlify 的全球 CDN 功能
4. Markdown 书写日志比在 WordPress 后台编辑方便, 可随时随地, 完成后只要提交到 GitHub 使会自动发布. 而且嵌入程序代码时更方便, 只要三个撇号
5. Markdown 文档既适合人阅读也适合机器解析, 而 HTML 格式的内容就不是给人看的, 所以在从 WordPress 转换为静态页面时不少日志内容全在一行
6. 用 Hugo 这样的工具管理静态页面, 定制性能 WordPress 更强, 再也不用管理那些插件了, 有定制需求直接编辑 layout, shortcode 等文件
7. GitHub Pages 或 Cloudflare Pages 可绑定自己的域名, 和自动管理 HTTPS 证书, 不用自己 Let's Encrypt

在使用 GitHub Pages 静态面面网站有以下流行的工具可供选择

1. [Jekyll](https://jekyllrb.com/), Ruby 写的, GitHub Pages 原生支持, 不用配置 Action
2. [Astro](https://astro.build/), Node.js, TypeScript 编写的
3. [Hexo](https://hexo.io/), Node.js, JavaScript 编写的
4. [VitePress](https://vitepress.dev/) 和 [VuePress](https://vuepress.vuejs.org/), Node.js, 依托 Vue.js 生态, 新项目建议用 VitePress
5. [Hugo](https://gohugo.io/), Go 语言编写的, 编译型, 所以比以上解释型工具要快
6. [Zola](https://www.getzola.org/), 刚刚得知的一款 Rust 编写的工具, 比 Hugo 还快

具体选用哪个工具根据自己喜欢的语言生态, 提供的主题是否丰富, 以及可定制性, 本地调试的方便性. 基于此我选择了 Go 写的 Hugo, 因为它的快, 
有着丰富的主题, 关键是找到我喜爱的 [Clarity 主题](https://themes.gohugo.io/themes/hugo-clarity/). 并且在后期使用当中感觉不错.

#### 创建一个 Hugo 站点并用 Clarity 主题

下面相应的命令

```bash
brew install hugo   # 在 macOS 下安装 Hugo
hugo new site my-static-site
cd my-static-site
git init
git submodule add https://github.com/chipzoller/hugo-clarity.git themes/hugo-clarity
cp -R themes/hugo-clarity/exampleSite/* ./    # 使用主题中的示例站点内容
hugo server
```

这样就在 1313 端口上启动了一个站点, 访问 http://localhost:1313 就能看到与 Clarity 主题示例站点内容. Hugo 生成的所有内容都在 `public`
目录下. 自己的页面在 content/post 下创建. 

例如用 hugo 命令

```bash
hugo new post/my-first-post.md         # 会创建 /content/post/my-first-post.md
hugo new post/my-second-post/index.md  # 会创建 /content/post/my-second-post/index.md
```

修改以上两个 *.md 文件中的 `draft: true` 为 `draft: false` 就能浏览 http://localhost:1313/post/my-first-post 和
http://localhost:1313/post/my-second-post/.

把 post/my-second-post/index.md 中的 `usePageBundles: false` 改为 `usePageBundles: true` 就是 Page bundles.

#### WordPress 逐篇迁移时的过程记录.

以上静态页面工具多是推荐使用 Markdown 格式的文档, Hugo 也不例外, 于是一开始想像着能否用 WorkPress 的某个插件直接导出所有的日志为静态页面,
找到了类似于 Jekyll Exporter 的插件 [wordpress-to-hugo-exporter](https://themes.gohugo.io/themes/hugo-clarity/), 
导出后与想像相差甚远, 一劳永逸还是别想了. 但由它导出的图片等文件还是用上了.

干脆自己来, 顺便熟悉一下 Rust, 所以用 Rust 直接访问 WordPress 的数据库导出全部日志包括特征图片, 页面访问到一个 JSON 文件中. 使用的 SQL 为

```sql
select a.id, a.post_status, nullif(a.post_date_gmt, '0000-00-00 00:00:00'),
  a.post_title, a.post_name, a.post_content, nullif(a.post_modified_gmt, '0000-00-00 00:00:00'),
  b.meta_value as views, d.guid as feature_image
        from wp_posts a
        left join wp_postmeta b on a.id=b.post_id and b.meta_key='views'
        left join wp_postmeta c on a.id=c.post_id and c.meta_key ='_thumbnail_id'
        left join wp_posts d on d.id=c.meta_value
  where a.post_type in ('post') and a.post_status in ('publish', 'draft', 'pending', 'private')

```

再基于这个 `all_posts.json` 文件用 Rust 生成一个一个名为 `a.post_name`(即 url) 的目录, 目录中包含 `page.html`, 其中有 FrontMatter
和日志的 HTML 内容. 生成 `page.html` 对许多 HTML 标签作了 `<br>`, `\r\n` 等替换处理.

由 Rust 生成像 `aws-python-lambda-layer/page.html` 文件时用到了模板 Crate `Tera`, 模板文件的内容为

```yaml
---
title: {{ post.title }}
url: /{{ post.name }}/
date: {{ post.post_date_gmt | date_format }}
featured: false
draft: true
toc: false
# menu: main
usePageBundles: true{% if post.feature_image %}
thumbnail: "../images/logos/{{ post.feature_image }}"{% endif %}
categories:{% for cat in post.categories %}
  - {{ cat }}{% endfor %}
tags: {% for tag in post.tags %}
  - {{ tag }}{% endfor %}
comment: true
codeMaxLines: 50
# additional
wpPostId: {{ post.id }} {# post id in Wordpress #}
wpStatus: {{ post.status }}
views: {{ post.views }}
lastmod: {{ post.last_modified_gmt | date_format }}
---

{{ post_content }}
```

这也将作为以后写作新日志的模板.

其间尝试过用 `html2md` 等 HTML To Markdown 的 Crates 转换 WordPress 的 HTML 内容为 Markdown 格式, 但始终都无法得到理想的内容, 
如表格转换有问题, 对众多的代码片段如 `<pre class="lang:java">...</pre>` 无法正确转换. 最后还是索性从产生的 1200 多个 `page.html` 
逐个理一遍. 

针对 1200 多个 `page.html` 中的每一篇, 几乎对每一行都要查看, 要做的事情基本是

1. 把 `draft` 由 `true` 改成 `false`
2. 把出现的 `<pre class="..."` 标签都替换成 `{{</* highlight java */>}}`. 在 WordPress 中多数时候没有指定语言, 但用 highlight 
Hugo 标签时必须确定语言, 如果在 HTML 中有 mark 行也须手工填过去, 如 `<pre class="lang:default mark:1,4-6 decode:true">...</pre>` 
就要变成  `{{</* highlight java "hl_lines=1 4-6" */>}}...{{</*/ highlight */>}}`
3. 对于新的 `{{</* highlight xxx */>}}` 中的所有 HTML 实体全部还原回去, 如 `&lt;`, `&gt;` 变回 `<` 和 `>`; `<br><br>` 会还原为 
`\r\n`, 每行后挂单的 `<br/>` 要删除掉
4. 从 WordPress 生成的 page.html 中有大量的 `<br>`, 在某些行上后的 `<br>` 要进行清除
5. 对于 `<img>` 图片链接, 转换为自定义的 Shortcode `{{</* bundle-image xxx.png 611 */>}}`, 并且从 WordPress to Hugo 
导出的文件中找到相应的图片拷贝到对应 `page.html` 所在的目录中
6. 把站内的链接由 `https://unmi.cc/xxx` 或 `https://yanbin.blog/yyy` 改成相对链接 `/xxx` 和 `/yyy`
7. 编辑 `page.html` 的过程中还发现少量的 WordPress 日志还被挂码了, 看到有这样的内容  
 `<div id="xunlei_com_thunder_helper_plugin_d462f475-c18e-46be-bd10-327458d045bd"> </div>`, 也作了相应的清理
8. `page.html` 中有大量的原本为 `&nbsp;` 对应的空格显示为 {{< bundle-image nbsps.png 210 inline >}}. 对于 `<blockquote>` 
中的 NBSP 保留, 在 `{{/* highligth xxx */>}}` 中的全部用 Vim `s/\%u00a0/ /g` 进行替换
9. 还有些日志在几经辗转之后居然产生了乱码, 像 `特性还�欢际谴悠渌镅阅嵌韫吹摹Ｏ喾碈#也从Ja`, 暂时难以恢复, 除非进行 encode/decode 尝试
10. 把有些整篇日志 HTML 内容全揉成了一行的内容进行简单的分行, 不然对以上替换操作不易进行, 分成多行后稍稍利于人工阅读
11. 把 `<!--more-->` 从标签内部移出, 例如有些日志 `<!--more-->` 在 `<strong>..</strong>` 之间, 造成列表页面其他日志概要全变粗.  
WordPress WYSIWYG 编辑器选择 More 按钮把不少的 `<!--more-->` 加到的标签中间

#### 为什么选择用 `page.html`

Hugo 不仅支持 Markdown 的文件格式, 还支持以下其他几种格式

1. index.adoc: [AsciiDoc](https://asciidoc.org/) 格式
2. index.org: [Emacs Org Mode](https://orgmode.org/) 格式文件
3. index.pandoc: [Pandoc](https://pandoc.org/) 格式
4. index.rst: [rsStructuredText](https://docutils.sourceforge.io/rst.html) 格式
5. index.html

除了 HTML Markup 格式外, 其余的格式都是广义上的 Markdown 格式.

存储在 WordPress `wp_posts` 表中的日志内容是 HTML 格式, 经过程序自动化转换 HTML 为 Markdown 的尝试均以失败告终. 幸好 Hugo 直接支持
HTML 格式, 所以对原来 WordPress 中的老日志就直接用 HTML 格式, 然后在此基础上进行修改, 以尽可能的减少内容失真.

为何不用 `index.html` 而用 `page.html` 文件名, 这要回到 Hugo 支持的 [Page bundles](https://gohugo.io/content-management/page-bundles/)
说起.

前面有提到 Page bundles, 就是创建的 `/post/my-second-post/index.md` 这样的目录层次, 并且在 `index.md` 中启用 `usePageBundles`.
这样的话与该 post 相关的资源就直接放在该 `/post/my-second-post` 中了, 因为基本上每个 post 相关资源是独立的. 修改或删除该 post
只需在该目录中进行, 这给 post 创造了十分好的隔离性. 如有共享内容就放到 `static/`目录中即可.

Hugo 对于 `/post/my-first-post.md` 文件编译后会生成 `/public/post/my-first-post/index.html`, 对于 `/post/my-fist-post.html`
格式也是一样的.

采用 Bundle 的话, 编译后在 `/public/post/my-second-post/` 目录中同时包含 `index.html` 和其他资源文件. 但对于 `/post/my-second-post/`
目录下文件命名为 `index.html`, `_index.html`, 或 `page.html` 时产生的内容稍有不同. 对应的访问方式, 前二者为
http://localhost:1313/post/my-second-post/, 后者为 http://localhost:1313/post/my-second-post/page/

如果在 `post/my-second-post/page.html` 的 FrontMatter 中配置 `url: /second-post/` 后, 访问方式就变成了
http://localhost:1313/second-post, 这时候在 `/public` 目录中 `index.html` 和资源文件分到两个目录中去了

```bash
public/second-post             # 跟随 url
└── index.html

public/post/my-second-post     # 跟随 bundle(目录) 名
└── wp-hugo-1.png
```

对于 Bundle 使用了自定义 url 之后图片等资源访问就稍有不同了.

使用 `index.html` 能让编译后的 `index.html` 和其他资源文件都在一起, 但实际运行时会产生许多的 `public/post/{1,2,3,...17} 这样奇怪的目录.
所以对于从 WordPress 迁移过来的文件还是选择了用 `page.html`, 而非 `index.html` 文件.

#### 关于 IntelliJ IDEA `Hugo fix` 插件

刚开始对于产生的每一个 `page.html` 文件真是纯手工打造, 用下面各种 Vim 命令替换

```Vim
'<,'>s/&lt;/</g
'<,'>s/&gt;/>/g
'<,'>s/&amp;/\&/g
'<,'>s/\%u00a0/ /g
'<,'>s#<br/><br/>#\r#g
'<,'>s#<br/>##
'<,'>s#<br />##
```
对选择区域用 `gv` 反复操作. 然后在 IntelliJ 中查找 `<pre>` 和 `</pre>` 进行相应的替换. 大约进行到 400 多篇后, 需要些更自动化且可靠的操作.
纯 AI 系统提示词让它对剩下的 800 多篇进行操作是信不过的, 但是可以做一个 IntelliJ 插件啊. 于是用 Claude Code CLI 以 Vibe Coding
的方式做一个 `Hugo fix` 插件, 在 IntelliJ IDEA 中使用上下文菜单和快捷键操作

{{< bundle-image wp-hugo-1.png 500 >}}

这是第一次完全用提示词创建的一个 IntelliJ IDEA 插件, 在这里不是吹嘘什么没写一行代码, 或者零 IntelliJ IDEA 插件开发经验就做成了一个可用的产品,
而是在使用 Claude Code CLI 过程中人自动懒惰的根本不想去看它生成的代码, 所以做完之后仍然是零编程经验, 学习到的知识也是零, 也没有经历过任何的思考,
只是每一步问 `process?` 的时候回签 `yes`. 但是经常人的一番调教之后, AI 变得更聪明了, 当前系统只有 JDK 25 的情况下, 它自行分析错误, 
然后主动下载 JDK 17 解决了构建问题, 然而使用 AI 的人却变得更愚蠢了.

下面的该插件完成之后, 用  `list the menu` 显示出来的功能

```text
> list the menu

⏺ Here's the complete menu structure for the Hugo fix plugin:

  Menu Structure

  ┌─────────────────────┐
  │ Hugo fix        ►   │──┬─► fix entities
  └─────────────────────┘  ├─► replace </pre>
                           ├─► select <pre>
                           ├─► remove last <br/>
                           ├─► rewrite <img>
                           ├─► toggle draft
                           └─► copy url

  All Menu Items with Shortcuts

  | #   | Menu Item    | Shortcut | Description                                                                          |
  |-----|--------------|----------|--------------------------------------------------------------------------------------|
  | 1   | fix entities | ⌃⌘⇧Z     | Decode HTML entities in entire file (&lt; → <, &gt; → >, &amp; → &, <br/><br/> → \n) |
  | 2   | replace      | ⌃⇧⌘R     | Replace </pre> tags with \n{{</*/ highlight */>}} in entire file                         |
  | 3   | select       | ⌃⌘⇧S     | Search and select next <pre[^>]*?> tag                                               |
  | 4   | remove last  | ⌃⇧⌘B     | Remove last <br/> from current line if it ends with <br/><br/>                       |
  | 5   | rewrite      | ⌃⇧⌘I     | Convert <img src="path/file.jpg" width="600"> to {{</* bundle-image file.jpg 600 */>}}   |
  | 6   | toggle draft | ⌃⇧⌘D     | Toggle first occurrence of draft: true ↔ draft: false                                |
  | 7   | copy url     | ⌃⇧⌘C     | Copy first url: <value> with prefix http://localhost:1313<value> to clipboard        |

  Keyboard Shortcut Quick Reference

  - ⌃⌘⇧Z - fix entities
  - ⌃⇧⌘R - replace
  - ⌃⌘⇧S - select
  - ⌃⇧⌘B - remove last
  - ⌃⇧⌘I - rewrite
  - ⌃⇧⌘D - toggle draft
  - ⌃⇧⌘C - copy url

───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
> test the plugin in RustRover                                                                                      ↵ send
───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
  ? for shortcuts
```

上面也是差不多用过的提示词, 每一个子菜单是经过多次迭代增加的.

后面使用该插件进行修改其余的 `page.html` 文件效率高了 10X 以上. 这才得已在昨天完成最后的收尾工作, 把域名管理移到 Cloudflare 上, 
绑定老域名到新的静态博客站点.

#### 启用 giscus 评论

即使是静态站点, 评论也还是必要的, 方便于各种交流. `Clarity` 主题内置支持的评论组件有 `giscus` 和 `utterances`. `giscus` 使用的是 GitHub
Repo 的 `Discussions` 功能来存储关连日志评论的.  启用 `giscu` 的步骤大致为

1. GitHub repo 启用 `Discussions` 功能
2. https://github.com/apps/giscus 由安装 `giscus` 到 GitHub Pages 对应的 Repo
3. https://giscus.app/ 配置得到参数, 并配置到 `/config/_default/params.toml` 中

#### 启用 Page view

在 WordPress 中每篇日志都有页面查看计数, 在导出 WordPress 数据时每篇日志的 Page view 也获取到了. 网站统计功能使用了
[GoatCounter](https://www.goatcounter.com/). GoatCounter 除了可统计每个页面被访问的次数, 还能由 API 获得相应 URL 被访问的次数.

```shell
curl <goatcounter-url>/counter/study-rust-workspace-package-crate-module.json
{
  "count": "8",
  "count_unique": "8"
}
```

基于这个功能, 就能实现每个页面显示访问计数, 加上在 WordPress 中的页面原始计数就是总是访问次数. 显示样式为
{{< bundle-image page-view.png 200 inline >}}, `135` 为 WordPress 和新页面访问总和.

#### 一些自定义的功能

基于主题 `Clarity` 自定义了一些 layouts 下的 partials, shortcodes 页面, 主要有

1. 去除了白色, 黑色主题的选择功能, 强制为白色主题. 现阶段各种应用至少还有白色主题可选, 不知是谁设定的很多地方默认主题为黑色
2. `归档` 页面按年显示日志条目, 通过自定义的 ShortCode `archive.html` 实现
3. 右边栏增加 `Blog Stats` 显示几个统计数据, 包括日志总数, 标签总计, 日志分类, 和最后构建时间
4. 列表页(如首页, 分类或标签列表页面) 中显示每篇日志 `<!--more-->` 之前的内容, 并且正确按格式分行显示
5. `/assets/sass/_custom.sass` 中定义代码白色主题显示, 和其他更多的样式定义
6. 原创日志后加上了 `CC BY-NC-SA 4.0` 许可声明, 不过基本是意思一下, 别人怎么用也拦不住
7. 两个重要自定义 ShortCode, `bundle-image` 和 `bundle-resource`

显示图片虽然在 Markdown 文件中可以用 

```markdown
![python](python3.14-new-features-1.png)  
```

或者用 [`figure`](https://gohugo.io/shortcodes/figure/) ShortCode, 它完整的参数是

```html
{{</* figure
    src="/images/examples/zion-national-park.jpg"
    alt="A photograph of Zion National Park"
    link="https://www.nps.gov/zion/index.htm"
    caption="Zion National Park"
    class="ma0 w-75"
*/>}}
```

参数是可选的, 所以能因地制宜的写上必须的项, 如

```html
{{</* figure src="wp-hugo-1.png" */>}}
{{</* figure src="wp-hugo-1.png" width="611px" */>}}
```

为了进一步简化, 定义了 `bundle-image`, 因为基本上日志都是在引用当前 Bundle 中的资源, 它的完整参数是

```html
{{</* bundle-image src="a.jpg"  width="800px" class="aligncenter" */>}}
```

也能基于位置来传入参数, 如

```html
{{</* bundle-image a.jpg */>}}
{{</* bundle-image a.jpg 800 */>}}
{{</* bundle-image a.jpg 800 inline */>}}
```

`bundle-resource` 与 `bundle-image` 的功能类似, 只是它用来引用 Bundle 目录中的其他资源, 如

```html
<a href="{{</* bundle-resource tools.jar */>}}">tools.jar</a>
```

迁移完所有的 WordPress 日志到 Hugo 之后, 这是写下的第一篇日志, 用 Markdown 写日志果然是清爽了许多. 再也不用打开网页来编辑文章了, 
也不会写作过程中提示该页无法响应了.