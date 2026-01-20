---
title: "如何定义 Copilot 斜线命令"
url: /how-to-customize-copilot-slash-command/
date: 2026-01-20T13:15:55-06:00
featured: false
draft: false
type: post
toc: false
# menu: main
usePageBundles: true
thumbnail: "/images/logos/ai-logo.png"
categories:
  - ai
tags: 
  - copilot
comment: true
codeMaxLines: 50
# additional
lastmod: 
---
各种 AI 编程工具，如 Codex, Claude Code, Gemini 等提供了一些类似的斜线命令，每个斜线命令大约也是对应着一段特定的提示词。由于工作中更方便的使用
Copilot, 所以本文来探讨如何定义自己的 Copilot 斜线命令。比如想要定义一个命令 `/c2py-dataclass` 用于实现把 C/C++ 的类或结构转换成 Python 
的 @dataclass 类，并遵循 Python 的命名规则和设置默认字段值, 也就是采用如下提示词

> Covert following C/C++ class/struct to Python dataclass, following Python naming convention, and set default field values.<br/>
> <C/C++ source code goes here>

{{< bundle-image "copilot-custom-command2.png" 331 >}}

有了自定义的 `/c2py-dataclass` 命令的话，就不需要每次重复上面的描述，而只用输入 `/c2py-dataclass` 然后指定某个 C++ 代码文件或粘贴 C/C++ 
代码就能实现转换需求。

实现方式可以借鉴几天前写的一篇 [准备迎接 Vibe Coding - 相关工具与资源](/preparation-for-vibe-coding/) 中关于 
[Spec Kit](/preparation-for-vibe-coding/#spec-kit) 一节。

### 实现方法

开门见山吧，想要添加一个自定义的命令，如 `/c2py-dataclass`, 仅需在项目目录中添加 `.github/prompts/c2py-dataclass.prompt.md` 文件,
立马就会在 Copilot 中出现一个 `/c2py-dataclass` 命令，在 `.github/prompts/c2py-dataclass.prompt.md` 添加所需的提示词即可。

要像 `Spec Kit` 那样的话可以使用两个文件 `.github/agents/c2py-dataclass.agent.md` 和 `.github/prompts/c2py-dataclass.prompt.md` 
来配合。 <!--more-->

下面的内容可以不用看了，主要是没看 Copilot 官方文档的情况下，灵感来自于 `Spec Kit` 的自定义命令的方式。

### 首先从 Colipot 内置命令开始

Copilot CLI 与 VS Code 和 JetBrains 的插件 GitHub Copilot Your AI pair programmer 显示的斜线命令还有所不同. 

Copilot CLI 的命令列表如下(当前版本 0.0.353)

```bash
   Available commands:
     /add-dir <directory> - Add a directory to the allowed list for file access
     /agent - Browse and select from available agents (if any)
     /clear - Clear the conversation history
     /cwd [directory] - Change working directory or show current directory
     /delegate <prompt> - Delegate changes to remote repository with AI-generated PR
     /exit, /quit - Exit the CLI
     /feedback - Provide feedback about the CLI
     /help - Show help for interactive commands
     /list-dirs - Display all allowed directories for file access
     /login - Log in to Copilot
     /logout - Log out of Copilot
     /mcp [show|add|edit|delete|disable|enable] [server-name] - Manage MCP server configuration
     /model [model] - Select AI model to use
     /reset-allowed-tools - Reset the list of allowed tools
     /session - Show information about the current CLI session
     /terminal-setup - Configure terminal for multiline input support (Shift+Enter and Ctrl+Enter)
     /theme [show|set|list] [auto|dark|light] - View or configure terminal theme
     /usage - Display session usage metrics and statistics
     /user [show|list|switch] - Manage GitHub user list
```

以下是 Copilot 插件分别在 IntelliJ IDEA 和 VS Code 中的命令列表

<table>
<tr>
<td>
在 IntelliJ IDEA 的 <code>GitHub Copilot Your AI Pair Programmer</code>(1.5.63-243) 中的命令有
{{< bundle-image "jetbrains-copilot-commands.png" 456 >}}
</td>
<td>
VS Code 的 <code>Github Copilot Your AI pair programmer</code>(1.388.0) 中的命令有
{{< bundle-image "vscode-copilot-commands.png" 433 >}}
</td>
</tr>
</table>

不管是 Copilot CLI 还是 IDE 插件，功能差不多，因为在 IDE 插件中某些功能是通过 UI 进行的，比如切换 Model, 管理会话, MCP 等。

### 下面来看 `Spec Kit` 是如何添加新命令

稍微回顾一下 `Spec Kit`

安装命令

```bash
uv tool install specify-cli --from git+https://github.com/github/spec-kit.git
```

给当前项目初始化用 `Spec Kit`, 选用 AI 工具 `Copilot`, 使用 shell 脚本

```bash
specify init . --ai copilot --script sh
```

以 IntelliJ IDEA 为例，以上命令在当前项目中生成了 `.github` 和 `.specify` 两个目录，而新产生的斜线命令对应于 `.github` 中的 `agents`
或 `prompts`。具体是用 `agents` 还是 `prompts` 产生的斜线命令，后面会有答案。

{{< bundle-image "jetbrains-copilot-spec-kit.png" 850 >}}

`Spec Kit` 给我们添加了一系列的 `/speckit.xxx`  命令，但是目前在 `Copilot CLI` 中无法显示那些 `/speckit.xxx` 命令。

现在查看一下 `.github` 目录中的几个 `md` 文件

.github/agents/speckit.analyze.agent.md

{{< highlight markdown >}}
---
description: Perform a non-destructive cross-artifact consistency and quality analysis across spec.md, plan.md, and tasks.md after task generation.
---

## User Input
```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty).

## Goal

Identify inconsistencies, duplications, ambiguities, and underspecified items across the three core artifacts (`spec.md`, `plan.md`, `tasks.md`) before implementation. This command MUST run only after `/speckit.tasks` has successfully produced a complete `tasks.md`.

## Operating Constraints
......
{{</highlight >}}

.github/agents/speckit.plan.agent.md

{{< highlight markdown >}}
---
description: Execute the implementation planning workflow using the plan template to generate design artifacts.
handoffs:
- label: Create Tasks
  agent: speckit.tasks
  prompt: Break the plan into tasks
  send: true
- label: Create Checklist
  agent: speckit.checklist
  prompt: Create a checklist for the following domain...
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty).

## Outline
{{</ highlight >}}

xxx.agent.md 文件头也是 `front matter` 内容，主要是一个描述, 会关联同名的 `prompts/` 中文件，或者可选的 `handoffs`，用于定义后续的 `agent` 执行。

查看对应的 `prompts/speckit.xxx.prompt.md` 文件

*.github/prompts/speckit.analyze.prompt.md*

{{< highlight yaml >}}
---
agent: speckit.analyze
---
{{</highlight >}}

*.github/prompts/speckit.plan.prompt.md*

{{< highlight yaml >}}
---
agent: speckit.plan
---
{{</highlight >}}

大致知晓了 Copilot 斜线命令与 `.github` 目录中文件的对应，可以新手做一些尝试了

### 开始尝试添加自定义命令

首先删除掉 `.github/agents` 和 `.github/prompts` 下所有的文件，这时候在 `Copilot` 中的 `/spceckit.xxx` 命令就全部消失了。

现在只添加一个空文件 `.github/prompts/c2py-dataclass.prompt.md`, 立马就能看到一个新斜线命令 `/c2py-dataclass` 出现了。所以很清楚，
`Copliot` 斜线命令自定义命令是与 `.github/prompts` 中的 `xxx.prompt.md`  文件相对应的。而且文件名必须规范，一定要符合格式  `xxx.prompt.md`。

{{< bundle-image "copilot-custom-command1.png" 850 >}}

你现在要执行它也行，只是没有足够的上下文，它大约会提示

```text
The referenced file c2py-dataclass.prompt.md is empty, so there are no specific instructions to follow.
If you'd like me to help you with something related to Python dataclasses or C-to-Python conversion, please provide:
    1.The instructions you'd like me to follow, or
    2. A specific task or question about your code
I'm ready to assist once you provide more details.
```

如果命令名本身能表达清楚你的意图 Copilot 甚至就能帮你做你想要的事。但我们希望给它加上更详细的提示词，只要在 
`.github/prompts/c2py-dataclass.prompt.md` 中添加如下内容即可

{{< highlight markdown >}}
### Goal
Covert following C/C++ class/struct to Python dataclass, following Python naming convention, and set default field values.
{{</highlight >}}

接着在 Colpilot 中输入

```cpp
/c2py-dataclass

#include <string>

class Person
{
public:
	Person():m_firstName("Scott"), age(18)
	{}
private:
    std::string m_firstName;
    unsigned int age;
}
```

结果很快就出来了

```python
from dataclasses import dataclass

@dataclass
class Person:
  first_name: str = "Scott"
  age: int = 18
```

{{< bundle-image copilot-custom-c2py-dataclass.png 560 >}}

AI 远比我们聪明多了，根本就不需要参照 `Spec Kit` 那样在 `.github/agents` 中添加一个 xxx.agent.md 文件，然后在 `.github/prompts` 
中添加一个对应的 xxx.prompt.md 文件。多数情况下只需要一个 `.github/prompts/your-command.prompt.md` 文件就能搞定。

再作一个尝试，只在 `.github/agents/` 中添加一个 `c2py-dataclass.agent.md` 文件又会如何呢？答案是不会有相应的 `/c2py-dataclass` 命令出现。

### 参照 `Spec Kit` 的风格添加命令

我们可以实现的更正式一点，也用 `agent` 和 `prompt` 两个文件

*.github/agents/c2py-dataclass.agent.md*

{{< highlight markdown >}}
```
description: convert C/C++ class/struct to Python dataclass
```

## User Input

```text
$ARGUMENTS
```

### Goal
Covert input C/C++ class/struct to Python dataclass,
following Python naming convention,
and set default field values.
{{</highlight >}}

*.github/prompts/c2py-dataclass.prompt.md*

{{< highlight markdown >}}
---
agent: c2py-dataclass
---
{{</highlight >}}

输入的内容可以是粘贴的 c++ 代码, 或者是选择的源码文件。下面是选择一个 c++ 文件 `person.cpp` 来执行 `/c2py-dataclass` 命令的效果

{{< bundle-image "spec-kit-similar-custom-command.png" 950 >}}

### 小结一下

添加一个自定义的 `Copilot` 命令只需在项目目录中添加 `.github/prompts/my-command.prompt.md` 文件, 在其中添加你的提示词，
然后就可以快乐的使用 `/my-command` 命令了, 它会自动接收命令后的内容或选择的文件作为输入。

复杂的命令需求可参照 `Spec Kit` 的风格, 使用 `.github/agents/my-command.agent.md` 和 
`.github/prompts/my-command.prompt.md` 两个文件配合使用, 好像基本没这个必要，因为这时候 `my-command.prompt.md` 文件没什么内容。

### 官方相关的文档

官方关于自定义斜线命令的主题是 [Using prompt files](https://docs.github.com/en/enterprise-cloud@latest/copilot/how-tos/configure-custom-instructions/add-repository-instructions#using-prompt-files-2).

更详细内容参考 [Use prompt files in VS Code](https://code.visualstudio.com/docs/copilot/customization/prompt-files). 
在 `.github/prompts/my-command.prompt.md` 文件的 front matter 支持 `description`, `name`, `argument-hint`, `agent`, `model`,
`tools` 属性，所以在其中还能指定使用工具。