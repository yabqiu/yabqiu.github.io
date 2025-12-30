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
ChatGPT, Claude, 和 Gemini 影响更深远. Grok 似乎更热衷于政府项目, 针对个人终端用户没那么关注.

说到具体的 AI 辅助编程工具的话, 有以下几个

1. OpenAI 的 [Codex](https://openai.com/codex/), 需要开通 ChatGPT Plus 每月 $20 或以上会员, 可用 Codex CLI
2. [Cursor](https://cursor.com/), Pro 会员每月 $20, 有 [Cursor CLI](https://cursor.com/docs/cli/overview), 或使用 VS Code 定制版 Cursor
3. [Windsurf](https://windsurf.com/), Pro 会员每月 $15, 也是通过使用 VS Code 定制版 Windsurf
4. [Claude Code](https://www.claude.com/product/claude-code), Pro 会员每月 $20, 可用 Claude Code CLI 或 JetBrains IDE/VS Code 等插件
5. [Copilot](https://github.com/features/copilot), Pro 会员每月 $10, 可用 Copilot CLI 或 JetBrains IDE/VS Code 等插件
6. [Gemini](https://geminicli.com/), Pro 会员每月 $19.99, 可用 Gemini CLI 或 JetBrains IDE/VS Code 等插件. 还有 VS Code 定制版 [Antigravity](https://antigravity.google/)
7. AWS 的 [KIRO](https://kiro.dev/), Pro 会员每月 $20, 可用 [KIRO CLI](https://kiro.dev/cli/) 或使用 VS Code 定制版 KIRO
8. Byte Dance 的 [TRAE](https://www.trae.ai/), Pro 会员每月 $10, 可用 VS Code 定制版 TRAE, 也有 JetBrains IDE/VS Code 等插件

对于我个人而言, 会更多的关注 OpenAI 的 Codex, Cursor, Claude Code 和 Copilot, 喜欢它们理由主要是它们提供了相应的 CLI 工具, 
因为本人是 IntelliJ IDEA 的重度使用者, 本机安装一个 Visual Studio Code 纯粹是为养着定期升级用. 临时打开文本也是选择  Vim.
Cursor 的口碑不错, 有点缺憾的是目前没有给 JetBrains 提供官方插件. Windsurf 曾与 OpenAI 闹过绯闻, 后来不了了之, 于是有了 Codex,
用了一小段时间 Claude Code, 感觉产出的质量很高, 也很聪明. Copilot 虽有更便宜的 $10 每月的 Pro 价格, 但要好点的体验还得上一个档次.

上面每家的 AI 辅助编程工具并非都是只用自家的模型, 可以交叉使用别家模型, 比如微软的 Copilot 下也能用 Claude Haiku Sonnet, Gemini, 
有时候更要选对了模型.