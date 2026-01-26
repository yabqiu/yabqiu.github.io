---
title: 准备迎接 Vibe Coding - 相关工具与资源
url: /preparation-for-vibe-coding/
date: 2025-12-31T23:21:00-05:00
featured: false
type: post
draft: false
toc: false
# menu: main
usePageBundles: true
thumbnail: "/images/logos/vibe-coding-logo.png"
categories:
  - ai
tags:
  - vibe coding
  - cursor
  - claude code
  - copilot
  - gemini
comment: true
codeMaxLines: 80
showLastmod: true
---

2022 年 11 月 ChatGPT 横空出世, 史称 ChatGPT 时刻, 从那一刻起, 不管你接不接受, 事情正在迅速起变化. 如果写代码从记事本, 一边查文档开始, 
到 IDE 的智能提示, 再到 Google 搜索代码, 从 StackOverflow 拷贝代码, 甚至是用 ChatGPT 对话抄写代码这些阶段, 软件工程方面并没有发生太大的变化.

在去年面对 AI 还犹豫做什么的时候, 今年毫无疑问就是 AI Agent. 就目前 AI 最大的成就莫过于解决掉了很多程序员的工作问题. 程序员们在面临 AI 
应该作出什么变化的话, 那 Vibe Coding 就不得不认真去看待. Vibe Coding 给我们带来某种快感的同时, 也伴随着焦虑. 从 TDD, BDD, DDD, 到现在的
SDD, 大脑就外包给了 LLM.

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

上面每家的 AI 辅助编程工具并非都是只用自家的模型, 都是交叉使用别家模型, 比如微软的 Copilot 下也能用 Claude Haiku Sonnet, Gemini, 
有时候选对了模型才更重要. 基于对 Claude Code 的正面反馈, 所以在很多时候都会选择 Claude 的大语言模型.

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
输入 `!` 后面可输入执行系统命令. `codex` 默认运行在一个沙箱之中, 更安全.

下面是几个主要的 Codex 命令

1. /model 选择大语言模型, 目前我的 Plus 用户只看到只个可选模型, gpt-5.2-codex, gpt-5.1-codex-max, gtp-5.1-codex-min, 和 gpt-t.2
2. /init 如果没有的话会生成 Codex 的  `[AGENTS.md](https://developers.openai.com/codex/guides/agents-md)` 文件, 这是 Codex 的指导文件, 子目录中也可以自己的 `AGENTS.md` 文件
3. /approvals 进行 `Full Access`  授权后就不会问次执行命令前都询问确认
4. /new  新开启一个新的会话, 节约上下文, Token
5. /compact 压缩上下文
6. /mcp 查看 MCP servers
7. /resume 列出并进入到某个已保存的会话
8. /skills 使用 skill 来指导 Codex 如何执行特定的任务

粘贴图片, 尽管你看到是 CLI 交互界面, Codex CLI 还是很方便的粘贴图片到输入行, 有两种方式, 截图或复制图片到剪贴板, 然后在 Codex CLI 
中按下 `ctrl + v`, 或者直接从文件浏览器上拖拽图片或普通文件来输入框处

{{< bundle-image codex-files.png 725 >}}

这张图片的三个文件分别是用 `ctrl+v` 粘贴的, 拖拽的图片, 以及拖拽的普通文件.

添加 MCP server, 直接编辑 `~/.codex/config.toml`, 比如添加一个 `demo` MCP Server, 添加类似下面的行

```toml
[mcp_servers.demo]
args = ["-u", "/Users/yanbin/Workspaces/codex-mcp-server/server.py"]
command = "/Users/yanbin/Workspaces/codex-mcp-server/.venv/bin/python"
```

内容到 `~/.codex/config.toml` 文件中.

想要 Codex 生产什么的软件, 就看你和 Codex 的自然语言沟通能力了. Codex 还有一个好处就像是在 ChatGPT 中一样, 会主动询问要不要帮你做下面的
1, 2, 3 选项.

多行输入的支持, 在 macOS 下可用 `ctrl + enter`, `shift + enter`, `option + enter`, `ctrl + j`

### Cursor

可以使用 Cursor 定制版的 VS Code, 或者使用 Cursor CLI(Cursor Agent). 安装用 Cursor 启动后会询问要不要安装 `cursor` 命令, 相当于
VS Code 的 `code`, 以后就用 `cursor` 命令来启动定制版的 VS Code.

从 Cursor 中可看到有不少模型可以选择, 如 `Composer`, `Claude Opus`, `Claude Sonnet`, `GPT-5.1 Codex Max`, `GPT-5.2`,
`Gemini 3 Flash`, `GPT-5.1 Codex Mini`, `Grok Code`, `Gemini 2.5 Flash`, 这要看你花多少钱用什么样的功能.

或者安装 Cursor CLI, 它叫做  Cursor Agent, 安装命令如下

```shell
curl https://cursor.com/install -fsS | bash
```

安装后还要把 `cursor-agent` 加到 PATH 环境变量中, 用 `zsh` 的话, 执行

```shell
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

第一次启动 `cursor-agent`, 还有个控制台下的动画, 配置好 Cursor 用户后就进到与 `Codex` 一样的输入界面.

{{< bundle-image cursor-start-1.png 632 inline >}}
{{< bundle-image cursor-start-2.png 354 alignright >}}

操作也是一样的, `/` 命令, `@` 选择文件, `!` 执行 shell 命令. 看下贴图片如何操作.

可以拖拽图片或普通文件到这个输入框, 拖拽后显示为

{{< bundle-image cursor-files.png 454 >}}

但是在 macOS 下的 cursor-agent, 目前还没找到办法如何把剪贴板中的图片贴到输入框中, 这一点就不如 Codex.

`cursor-agent` 也有类似 `Codex` 的命令('/')

1. /model 选择大语言模型
2. /clear 和 `Codex` 的 /new 相同, 用来启动一个新的会话
3. /compress, 与 `Codex` 的 /compact 一样, 压缩上下文
4. /shell 是 `!` 的命令形式
5. /open 或 /cursor: 用来从 `cursor-agent` 命令行下打开 `cursor` 编辑器
6. /resume, /list 和 /resume-chat 前两个是从 chat 列表中选择恢复, 后一下是恢复某个目录的上一次对话
7. /mcp 查看 MCP Servers

给 Cursor 添加 MCP Server 的方法很简单, 在 Cursor 的官网 [Model Context Protocol(MCP)](https://cursor.com/docs/context/mcp)
中目前有 60 多个可添加的 MCP Servers

{{< bundle-image cursor-mcp-servers.png 714 >}}

点击 `Add to Cursor` 就会打开 `Cursor` 编辑器, 然后依照提示完成 MCP Server 的添加, 完后重新打开 `cursor-agent` CLI, 会提示
`MCP Server Approval Required`,  选择 `Approve all servers`, 而后用 `/mcp list` 命令就能看到该

```shell
  MCP Servers:
    Context7 - ready
```

在对话中就可以使用该 MCP Server 了, 比如问 `show documents about next.js` 就会调用 `Conext7`.

`Cursor` 的 MCP Server 配置文件在 `~/.cursor/mcp.json` 中, 通过 UI 添加的  `Context7` 也在其中. 我们当然可以手动添加, 比如添加之前的那个
Demo MCP Server, 最终的 `~/.cursor/mcp.json` 是

```json { hl_lines="7-10" }
{
  "mcpServers": {
    "Context7": {
      "url": "https://mcp.context7.com/mcp",
      "headers": {}
    },
    "Demo": {
      "command": "/Users/yanbin/Workspaces/codex-mcp-server/.venv/bin/python",
      "args": ["-u", "/Users/yanbin/Workspaces/codex-mcp-server/server.py"]
    }
  }
}
```

`Curror` 的开发规则可以使用 `Codex` 的 `AGENTS.md` 文件或 `Claude Code` 的 `CLAUDE.md` 作为指引, 它原生采用的是定义在工程目录下的

```shell
.cursor/rules/
    my-rule1/
        RULE.md     # 主规则文件
        scripts/    # 可选择的脚本目录
    my-rule2/
        ...
```

用 Cursor 编辑器进行前端开发, 可以很方便与网页进行交互, 比如说从网页上选择一个按钮, `cursor` 的输入框出现 inspect 的

<blockquote>
<span style="opacity: 0.5">&lt;button&gt;</span> change background color to black
</blockquote>

多行输入的支持, 在 macOS 下有 `shift + enter` 或 `ctrl + j`

其他更多有关 `Cursor` 的特性就要深度使用了.

### Claude Code

Anthropic 的 Claude Code 在 AI 编程界表现很出色, 提供 VS Code 和 JetBrains IDE 的插件是基本的, 还有我喜欢的 Claude Code CLI, 
与 Cursor, Windsurf 等不同, 它对定制版的 VS Code 不那么热衷, 此外它还与 Slack 进行了集成.

我比较关心它的 CLI, 安装命令是

```shell
curl -fsSL https://claude.ai/install.sh | bash
```

它的 CLI 启动命令是 `claude`, 在某个工程目中启动后进到命令行下的交互界面

{{< bundle-image claude-code-start.png 861 >}}

输入 `?` 号看到如下帮助信息

```text
  ! for bash mode       double tap esc to clear input      ctrl + _ to undo
  / for commands        shift + tab to auto-accept edits   ctrl + z to suspend
  @ for file paths      ctrl + o for verbose output        ctrl + v to paste images
  & for background      ctrl + t to show todos             opt + p to switch model
                        shift + ⏎ for newline              ctrl + s to stash prompt
```

基本命令前缀和 `Codex`, `Cursor` 是一样的, `/`, `@`, `!`.

粘贴图片和普通文件的操作与 `Codex` 是一样的, `ctrl+v` 粘贴剪贴板中的图片, 拖拽可从文件浏览器中选择一个或多个图片或普通文件.

Claude Code 通过 `shift+tab` 在不同模式间来回切换，像正常模式，`accept edits`, `plan mode`。

`/` 命令选项比 `Codex` 和 `Cursor` 都丰富的多, 这里是[所有 Claude Code CLI 命令](https://code.claude.com/docs/en/slash-commands).
以下是主要的一些命令

1. /clear 或 new, 清除会话历史释放上下文, 启动一个新的会话
2. /compat  压缩会话
3. /model 选择大语言模型
4. /init 如果没有 `CLAUDE.md` 文件会扫描当前项目文件, 还会读取 `AGENTS.md` 文件来生成 `CLUADE.md` 文件
5. /ide 会连接到安装了 `Claude Code` 插件的 IDE, 如 JetBrains IDE 或 VS Code 等, 然后与 IDE 进交交互
6. /chrome 打开 Chrome 浏览器, 提示 Chrome 安装 `Claude` 插件, 如果没有安装的话. 该插件能于 Chrome 中的内容进行交互
7. /theme 可切换不同的主题, 如不同的 Dark mode, Light mode
8. /config  `Claude Code CLI` 的配置
9. /export 导出当前会话内容到文件或剪贴板
10. /hooks 管理一些事件钩子, 如调用工具前与后, 调用工具失败后, 会话结束后等
11. /permissions 管理是否允许或拒绝工具的的权限规则
12. /plan 查看当前会话计划
13. /memory 编辑会话文件, 如是用工程的 `./CLAUDE.md` 还是用户的 `~/.claude/CLAUDE.md` 文件
14. /plugin 管理 `Claude Code` 插件, 在其中可以列出许多的 MCP Server, 如选择了 `context7` 就可以
15. /mcp 管理 MCP Servers, 比如通过上面 `/plugin` 安装的 MCP Server, 用 `/mcp` 也能显示出来.
16. /resume 恢复一个历史会话
17. /rewind 恢复代码和会话到前一个检查点
18. /review, Review 一个 Pull Request
19. /skills 列出所有的 skills
20. /status 显示 Claude Code 信息, 包括版本, 模型, 帐号, API 连接和工具的状态
21. /stats  显示 Clause Code 的活动和使用统计
22. /tasks  列举和管理后台任务
23. /todos 列出当前待办项
24. /usage 显示当前 Plan 的使用配额

`Claude Code` MCP Server 的配置文件有两处, 项目目录中的 `./.mcp.json` 和用户目录的 `~/.claude.json` 文件, 例如编辑项目目录下的 `.mcp.json`,
加上如下内容

```json
{
  "mcpServers": {
    "Demo": {
      "command": "/Users/yanbin/Workspaces/codex-mcp-server/.venv/bin/python",
      "args": ["-u", "/Users/yanbin/Workspaces/codex-mcp-server/server.py"]
    }
  }
}
```

就能使用该 MCP Server 了, 命令 `/mcp` 能列出该 `Demo`

```text
> /mcp

──────────────────────────────────────────────────────────────────────────────────────────────────────────────
 Manage MCP servers
 2 servers

 ❯ 1. Demo                      ✔ connected · Enter to view details
   2. plugin:context7:context7  ✔ connected · Enter to view details
```

`plugin:context7:context7` 是用 `/plugin` 命令从列表中选择安装的. Claude 的插件不仅仅是 MCP Server, 许多可定制的东西都可以打包成插件，
例如 Agent, Prompt, 或 Skills.

如果我们查看用户目录下的 `~/.claude/` 目录, 其中有很多相关的配置文件.

```text
ls ~/.claude/
chrome           file-history     plugins          settings.json    statsig
debug            history.jsonl    projects         shell-snapshots  telemetry
downloads        ide              session-env      stats-cache.json todos
```

多行输入的支持, 在 macOS 下的多种方式 `\ + Enter`, `Shift + Enter`, `Ctrl + J`.

### Copilot

`Copilot` 是微软出品的 AI 编程工具, 提供了 VS Code 和 JetBrains IDE 插件 和 `Copilot CLI`. `Copilot CLI` 在 macOS 下的安装

```shell
brew install copilot-cli
```

用 `copilot` 命令启动

{{< bundle-image copilot-cli-start.png 566 >}}

图片与文件和粘贴操作与 `Codex`, `Claude Code` 是一样的. 基本操作前缀也是一样的, `@` 引用文件, `!` 执行 shell 命令, `/` 命令.

`Copilot` 的 `/` 命令数量与 `Claude Code` 相当, `/help` 显示的命令用法也很详细, 不需要像前面那样列出主要的命令, 直接显示 `/help` 信息即可

```shell
 Welcome to GitHub Copilot CLI
 Version 0.0.373 · Commit 1f9ed04

 Copilot can write, test and debug code right from your terminal. Describe a task to get started or ↩︎
 enter ? for help. Copilot uses AI, check for mistakes.

 ● Global shortcuts:
     @ - mention files, include contents in the current context
     Esc - cancel the current operation
     ! - Execute the command in your local shell without sending to Copilot
     Ctrl+c - cancel operation if thinking, clear input if present, or exit
     Ctrl+d - shutdown
     Ctrl+l - clear the screen

   Expand timeline content shortcuts:
     Ctrl+o - expand all timeline/collapse timeline
     Ctrl+r - expand recent timeline/collapse timeline

   Motion shortcuts:
     Ctrl+a - move to the beginning of the line
     Ctrl+e - move to the end of the line
     Ctrl+h - delete previous character
     Ctrl+w - delete previous word
     Ctrl+u - delete from cursor to beginning of line
     Ctrl+k - delete from cursor to end of line
     Meta+←/→ - move cursor by word

   Use ↑↓ keys to navigate command history

   Respects instructions sourced from various locations:
     `.github/instructions/**/*.instructions.md` (in git root and cwd)
     `.github/copilot-instructions.md`
     `AGENTS.md` (in git root and cwd)
     `CLAUDE.md`
     `GEMINI.md`
     `$HOME/.copilot/copilot-instructions.md`
     Additional directories via `COPILOT_CUSTOM_INSTRUCTIONS_DIRS`

   To learn about what I can do:
     Ask me "What can you do?"
     Or visit: https://docs.github.com/en/copilot/how-tos/use-copilot-agents/use-copilot-cli

   Available commands:
     /add-dir <directory> - Add a directory to the allowed list for file access
     /agent - Browse and select from available agents (if any)
     /clear - Clear the conversation history
     /context - Show context window token usage and visualization
     /cwd [directory] - Change working directory or show current directory
     /delegate <prompt> - Delegate changes to remote repository with AI-generated PR
     /exit, /quit - Exit the CLI
     /share [file|gist] [path] - Share session to markdown file or GitHub gist
     /feedback - Provide feedback about the CLI
     /help - Show help for interactive commands
     /list-dirs - Display all allowed directories for file access
     /login - Log in to Copilot
     /logout - Log out of Copilot
     /mcp [show|add|edit|delete|disable|enable] [server-name] - Manage MCP server configuration
     /model [model] - Select AI model to use
     /reset-allowed-tools - Reset the list of allowed tools
     /session - Show information about the current CLI session
     /skills [list|info|add|remove|reload] [args...] - Manage skills for enhanced capabilities
     /terminal-setup - Configure terminal for multiline input support (Shift+Enter and Ctrl+Enter)
     /theme [show|set|list] [auto|dark|light] - View or configure terminal theme
     /usage - Display session usage metrics and statistics
     /user [show|list|switch] - Manage GitHub user list
```

/model 显示目前 $10 每月可选的模型有 `Claude Haiku 4.5`, `GPT-5 mini`, `GPT-4.1`, 加钱后可选的模型有  `Claude Sonet 4.5`, 
`Claude Opus 4.5`, `Claude Sonnet 4`, `GPT-5.1-Codex-Max`, `GPT-5.1-Codex`, `GPT-5.2`, `GPT-5.1`, `GPT-5`,
`GPT-5.1-Codex-Mini` 和 `Gemini 3 Pro (Preview)`.

也介绍一下它的 MCP 管理与使用.

输入 `/mcp add` 后在命令行下有输入向导

{{< bundle-image copilot-mcp-add.png 860 >}}

MCP Server 的信息会添加到用户目录的 `~/.copilot/mcp-config.json` 中.

多行输入的支持, 在 macOS 下有 `ctrl + enter`, `shift + enter`, `option + enter`,  `\ + enter`.

### 对 `Codex`, `Cursor`, `Claude Code` 的 `Copilot` 的整体感受

模型是不断的较劲中发展, AI 编程工具的交互也在不停的进步, 很难说哪个工具的优劣. 因为它们基本的功能都差不多, 目前除 `Cursor` 外都支持 `/skills`.
更关键是看钱, 每月钱交得多了才能决定你能用上哪个模型, 而且每个 AI 编程工具使用到的模型列表都差不多, 无外乎是 `Claude`, `GPT`, `Gemini`,
`Cursor` 有自家的 `Composer` 并马斯克的 `Grok`, 列表也会在不断的更新中. 不同数量的钱决定的不同质量的模型及变种, 不太可能某一家 $10 
比别家 $20 钱的好, $20 更不可别其他家的 $200 要好, 因为这个类型的用户不会是傻瓜.

目前单从 `/` 命令上来看, 个人感觉是 `Claude Code` 和 `Copilot` 的功能要强, `Copilot` 的基本会员 $10 还是显得有点弱, 只能用模型
`Claude Kaiku 4.5`, 用不上 `Claude Sonet 4.5`, 想要过得稍舒服一些, 有些钱还是没法省, `$20` 起步价还是基本的.

上面没有介绍 Gemini CLI, 快速看一下, 安装命令为 `npm install -g @google/gemini-cli`, 安装后启动命令为 `gemini`, 它所用的主要文件是
`GEMINI.md`, 输入框的操作都是一样的, `/help` 显示的帮助信息很详细. `Ctrl+Y` 切换 `YOLO(激进)` 模式. `/model` 免费版可见模型只有
`gemini-2.5-pro`, `gemini-2.5-flash`, `gemini-2.5-flash-lite`.

所有以上的 AI 编程工具 CLI 操作都差不多, `/`(命令), `@`(选择文件), `!`(执行系统命令) 的含义都是一样的. 除 `Codex` 外都有 `Vim` 键绑定.
粘贴图片或文件的方式也是一样的, `ctrl+v` 从剪贴板粘贴吧(cursor 在 macOS 的 ctrl+v 不工作), 拖拽可从文件浏览器中拖入图片或任格式的文件,
多行输入可以试下 `\ + enter`, `ctrl + eneter`, `ctrl + j`, `shift + enter`, `option + enter` 中一种, 从剪贴板贴入的大文本保持换行格式.

### Vibe Coding 的其他工具

除了 AI 编程工具 `Codex`, `Claude Code`, `Copilot`, `Cursor`, `Gemini` 等, 还有一些其他的技术或工具值得了解.

#### Spec Kit

除了单纯的使用对话框, 和 `AGENTS.md`, `CLAUDE.md`, `GEMINI.md`, Claude 的 `ruls` 以及 Copilot 的 `instructions.md` 文件, 
微软件还进一步推出的 `[Spec Kit](https://github.com/github/spec-kit))`, 试图对 Vibe Coding 建立起规范, 它支持当前主流的 AI
编程工具.

安装命令是

```shell
uv tool install specify-cli --from git+https://github.com/github/spec-kit.git
```

然后系统里多了个 `specify` 命令, 使用方法

```shell
specify init spec-kit-demo --ai claude --script sh   # 创建项目, 并使用 claude 和 shell command

# 或这样对当前目录初始化为 spec-kit 项目
specify init --here --ai claude --script sh
# 或
specify init . --ai claude --script sh
```

如果 `specify init <PROJECT_NAME>` 时没有指定 `--ai` 和 `--script` 参数, 将会有列表提示选择哪个 AI 编程工具, 或 `sh` 还是 `powershell`.
项目创建(初始化)后将在看到 `.claude` 目录, 内容为

```text
spec-kit-demo
├── .claude
│   └── commands
│       ├── speckit.analyze.md
│       ├── speckit.checklist.md
│       ├── speckit.clarify.md
│       ├── speckit.constitution.md
│       ├── speckit.implement.md
│       ├── speckit.plan.md
│       ├── speckit.specify.md
│       ├── speckit.tasks.md
│       └── speckit.taskstoissues.md
└── .specify
    ├── memory
    │   └── constitution.md
    ├── scripts
    │   └── bash
    │       ├── check-prerequisites.sh
    │       ├── common.sh
    │       ├── create-new-feature.sh
    │       ├── setup-plan.sh
    │       └── update-agent-context.sh
    └── templates
        ├── agent-file-template.md
        ├── checklist-template.md
        ├── plan-template.md
        ├── spec-template.md
        └── tasks-template.md
```

选用不同的 AI 编程工具, 隐藏目录名也不一样, 例如 `--ai cursor-agent`, 目录名为 `.cursor`; `--ai copilot`, 目录名则为 `.github`. 
其下的内容也有所不同, 不同 `--ai`, `--script` 选择对应 [spec-kit templates](https://github.com/github/spec-kit/releases)
的不同模板.

[Specification-Driven Development(SDD)](https://github.com/github/spec-kit/blob/main/spec-driven.md)

以 Claude 为例, 在该项目中启动 `claude` CLI, 看到多了一些 `spec-kit` 的自定义 `/` 命令

{{< bundle-image spec-kit-claude.png 850 >}}

首先要用 `/speckit.constitution`, `/speckit.specify`, `speckit.plan` 来完善项目的规范文档. 以后每次有新需求(spec) 就做以下迭代操作

1. `/speckit.specify` 创建一个 spec, 每次会自动创建一个分支
2. `/speckit.plan` 制定技术实现计划
3. `/speckit.tasks`  创建一系列的分解任务
4. `/speckit.implment` 执行本次 spec 中的任务

### 关于 Vibe Coding 的脚手架项目

再怎么零编程经验还是对一些程序概念有所了解吧, 什么是 iOS 项目, 什么是 Android 项目, 是 Web 项目还是 GUI, TUI, 还是 CLI. 如果是 Web
项目是用 Vue.js, React.js, 或者 GUI 的 Tauri, 或 Electron 项目等.

我们可能需要事先用

1. npm create vue@latest
2. npx create-react-app my-react-app
3. npx create-next-app@latest

创建好脚手架后, 然后让 Vibe Coding 介入.

看了别人一些演示项目, 似乎 [Next.js](https://nextjs.org/) 才是 AI 友好的技术栈. Next.js 把前端的 React.js 技术应用到了后端, 实现了
SSR(服务端渲染)/SSG(静态生成)/ISR(增量静态再生成). Next.js 由 `Vercel` 公司运营, Next.js 故意模糊了前后端的边界, 也因此创造一个致命的漏洞
[CVE-2025-66478](https://nextjs.org/blog/CVE-2025-66478).

使用 Next.js 技术栈简单, 只需要 JS 或 TypeScript 负责前后端(甚至不同去把前后端分清), 有点像当初的 `GWT`(Google Web Toolkit). 在 Next.js 中
[Templates](https://vercel.com/templates/next.js?utm_source=next-site&utm_medium=navbar&utm_campaign=next_site_nav_templates)
提供了很多项目模板. 我们只需要找一款适合自己的模板, 再交由 Vibe Coding 去扩展.

`Vercel` 除了 `Next.js` 外, 还有 [v0.app](https://v0.app), 直接在它的 Web 界面用 AI 创建 Web 项目并由它部署到云主机上去. 类似的网站有

1. [Lovable](https://lovable.dev/)
2. [Framer](https://www.framer.com/)

Web UI 方面, Vue 有 [Vuetify](https://vuetifyjs.com/), [Element UI](https://element.eleme.io/), Vue3 用 `Element Plus`. 
又有了一个新的适于 React.js(尤其是 Next.js) + Tailwind CSS 的 [shadcn/ui](https://ui.shadcn.com/).

数据库方面有免费的基于 `PostgreSQL` 的 [Supabase](https://supabase.com/).

在与 AI 描述界面需求时, 光文字描述有点难度, AI 还不易理解, 如果能画个线框图, 或是 Wireframe(Mockup) 图就更好, 或者用
[Figma](https://www.figma.com/) 来简单的设计 UI. 相关的工具还有 Excalidaw, Gliffy, balsamiq, sketch, 免费的只看到一个
[wireframe.cc](https://wireframe.cc/). 网上逛的时间多了, 发现当今网站最流行的模板莫过于

{{< bundle-image popular-website-template.png 860 >}}

对了, 最好关注一些公共的 MCP Server 资源, 如 [Context7](https://context7.com/) -- 有最新的软件相关文档. `Claude Code` CLI 中输入
`/plugin` 显示出来的列表就是一些 MCP Server. 

##### MCP Servers:
- Official MCP Registry (https://registry.modelcontextprotocol.io/)
- Awesome MCP Servers(https://mcpservers.org/)
- MCP Servers(https://mcp.so/)
- MCP Market(https://mcpmarket.com/)
- Smithery - Model Context Protocol Registry(https://smithery.ai/)
- MCP Marketplace(https://mcp.higress.ai/)
- MCP | Composio(https://mcp.composio.dev/)
- Glama - MCP Directory (https://glama.ai/mcp)
- MCP Server Finder (https://www.mcpserverfinder.com/)
- Pulse MCP Server Directory (https://www.pulsemcp.com/servers)