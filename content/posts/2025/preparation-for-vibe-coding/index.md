---
title: 无法回避的 Vibe Coding - 相关工具与资源
url: /preparation-for-vibe-coding/
date: 2025-12-29T15:21:00-05:00
featured: false
type: post
draft: true
toc: false
# menu: main
usePageBundles: true
thumbnail: "../images/logos/vibe-coding-logo.png"
categories:
  - ai
tags:
  - vibe coding
comment: true
codeMaxLines: 50
---

2022 年 11 月 ChatGPT 横空出世, 史称 ChatGPT 时刻, 从那一刻起, 不管你接不接受, 事情正在迅速起变化. 如果写代码从记事本, 一边查文档开始, 
到 IDE 的智能提示, 再到 Google 搜索代码, 从 StackOverflow 拷贝代码, 甚至是用 ChatGPT 对话抄写代码这些阶段, 软件工程方面并没有发生太大的变化.

在去年面对 AI 还犹豫做什么的时候, 今年毫无疑问就是 AI Agent. 就目前 AI 最大的成就莫过于解决掉了很多程序员的工作问题. 程序员们在面临 AI 
应该作出什么变化的话, 那 Vibe Coding 就不得不认真去看待. Vibe Coding 给我们带来某种快感的同时, 也伴随着焦虑.

Vibe Coding 最早由 OpenAI 联合创始人, 前特斯拉 AI 负责人 Andrej Karpathy 于 2025 年 2 月提出的一个新型编码方式. Vibe Coding 
给人一种最直白的感觉就是只与大语言模型对话的形式生成软件, 代码完全是个黑盒, 不直接修改代码, 基本都不看代码, 有编译等问题继续与 LLM 对话.
这种软件生产方式还要像传统方式来 Review 代码就很难了, AI 不辞辛苦生成的大量代码, 可能也不适于人类进行审核了.

Vibe Coding 成就了不少一人一公司, 但也不必过于相信有些人在网络上过份吹嘘的那样--零编程经验, 不写一行代码. 小白确实能用 Vibe Coding
做出一个东西来, 但真零编程经验, 技术框架选型就描述不清, 例如用 Vue.js, React.js, Next.js, 或者什么编程语言适合做什么事情等.

有编程经验搭配上了 Vibe Coding 一定能做的更好. 也别信什么 90% 代码是 AI 写的, Vibe Coding 就是一个黑盒. 那 Vibe Coding 试个多人协作的大型项目.
或者 Vibe Coding 做个银行, 政府, 航空航天项目? 这种关键领域的项目我想每一行代码都必须由人工审核. 所以远古的仍然稳定运行着的 COBOL
代码一直无法升级替换, 换成 AI 也别想简单的就能重写它们, 如果不需要重新测试的话, 那没问题. <!--more-->

当前唱重头戏的大语言模型下面几家

1. OpenAI 的 ChatGPT
2. Anthropic 的 Claude
3. Google 的 Gemini
4. X 的 Grok

其中只要有任何一家有重大版本发布, 其他几家都要争先恐后动作一番. 曾经 Meta 的 Llama 在开源模型界也风骚过, 但 Llama 4 却败下阵来. 在编程界
ChatGPT, Claude, 和 Gemini 影响更深远. Grok 似乎更热衷于政府项目, 针对个人终端用户没那么关注. 国内有几个知名的大语言模型有时候也值关注,
像 Qwen, DeepSeek, Kimi, GLM.

说到具体的 AI 辅助编程工具的话, 有以下几个

1. OpenAI 的 [Codex](https://openai.com/codex/), 需要开通 ChatGPT Plus 每月 $20 或以上会员, 可用 Codex CLI, 或 VS Code 插件
2. [Cursor](https://cursor.com/), Pro 会员每月 $20, 有 [Cursor CLI](https://cursor.com/docs/cli/overview), 或使用 VS Code 定制版 Cursor
3. [Windsurf](https://windsurf.com/), Pro 会员每月 $15, 也是通过使用 VS Code 定制版 Windsurf
4. [Claude Code](https://www.claude.com/product/claude-code), Pro 会员每月 $20, 可用 Claude Code CLI 或 JetBrains IDE/VS Code 等插件
5. [Copilot](https://github.com/features/copilot), Pro 会员每月 $10, 可用 Copilot CLI 或 JetBrains IDE/VS Code 等插件
6. [Gemini](https://geminicli.com/), Pro 会员每月 $19.99, 可用 Gemini CLI 或 JetBrains IDE/VS Code 等插件. 还有 VS Code 定制版 [Antigravity](https://antigravity.google/)
7. AWS 的 [KIRO](https://kiro.dev/), Pro 会员每月 $20, 可用 [KIRO CLI](https://kiro.dev/cli/) 或使用 VS Code 定制版 KIRO
8. Byte Dance 的 [TRAE](https://www.trae.ai/), Pro 会员每月 $10, 可用 VS Code 定制版 TRAE, 也有 JetBrains IDE/VS Code 等插件

对于我个人而言, 会更多的关注 OpenAI 的 Codex, Cursor, Claude Code 和 Copilot, 喜欢它们理由主要是它们提供了相应的 CLI 工具, 
因为本人是 IntelliJ IDEA 的重度使用者, 本机安装一个 Visual Studio Code 纯粹是为养着定期升级用. 临时打开文本也是选择 Vim.

Cursor 的口碑不错, 有点缺憾的是目前没有给 JetBrains 提供官方插件. Windsurf 曾与 OpenAI 闹过绯闻, 后来不了了之, 于是有了 Codex,
用了一小段时间 Claude Code, 感觉产出的质量很高, 也很聪明. Copilot 虽有更便宜的 $10 每月的 Pro 价格, 但要好点的体验还得上一个档次.

上面每家的 AI 辅助编程工具并非都是只用自家的模型, 可以交叉使用别家模型, 比如微软的 Copilot 下也能用 Claude Haiku Sonnet, Gemini, 
有时候更要选对了模型. 基于对 Claude Code 的正面反馈, 所以在很多时候都会选择 Claude 的大语言模型.

下面对 Codex, Cursor, Claude Code, Copilot 快速介绍体验介绍一下

### OpenAI Codex
Codex 提供了 VS Code, Cursor, Windsurf 插件, 或以 Codex web 方式使用. 还提供了 Codex CLI. 详细的 Codex 开发指南请参考官方的
[OpenAI Developers - Codex](https://developers.openai.com/codex/)

#### Codex CLI 的安装

```shell
npm i -g @openai/codex
```

安装后命令是 `codex`, 终端下进到项目目录, 使用 `codex` 命令加参数的方式可以运行一次性命令, 或者只输入 `codex` 进入交互模式

{{< bundle-image codex.png 610 >}}

现在可以在这个界面上进行交互, 进行自然语言的对话, 或输入 `/` 使用 Codex 的各种命令, 或输入 `@` 选择一个名多个文件(同一输入中多次使用 @), 
输入 `!` 后面可输入执行系统命令

下面是几个主要的 Codex 命令

1. /mode 选择大语言模型, 目前我的 Plus 用户只看到只个可选模型, gpt-5.2-codex, gpt-5.1-codex-max, gtp-5.1-codex-min, 和 gpt-t.2
2. /init 如果没有的话会生成 Codex 的  `[AGENTS.md](https://developers.openai.com/codex/guides/agents-md)` 文件, 这是 Codex 的指导文件, 子目录中也可以自己的 `AGENTS.md` 文件
3. /approvals 进行 `Full Access`  授权后就不会问次执行命令前都询问确认
4. /new  新开启一个新的会话, 节约上下文, Token
5. /compact 压缩上下文
6. /mcp 查看 MCP servers, 如添加一个 MCP server `/codex mcp add context7 -- npx -y @upstash/context7-mcp`

粘贴图片, 尽管你看到是 CLI 交互界面, Codex CLI 还是很方便的粘贴图片到输入行, 有两种方式
1. 从文件浏览中直接用鼠标拖一张图片到输入的位置上, Codex CLI 将会显示 
```shell
› [aws-lambda-rust-1.png 546x96]
```
2.图或复制图片到剪贴板, 然后在 Codex CLI 中按下 `ctrl + v`, 也会看到类似的显示
```shell
› [codex-clipboard-DitT5h.png 822x504]
```

你可以输入更多的问话, 回车后 Codex 就能收到这张图片. 文件拖拽的方式也支持其他格式的文件.

添加 MCP server, 可参考 Context7 [Local Server Connection](https://github.com/upstash/context7?tab=readme-ov-file#local-server-connection-1),
添加

```toml
[mcp_servers.context7]
args = ["-y", "@upstash/context7-mcp", "--api-key", "YOUR_API_KEY"]
command = "npx"
startup_timeout_ms = 20_000
```

内容到 `~/.codex/config.toml` 文件中.

### Cursor

### Claude Code

### Copilot
