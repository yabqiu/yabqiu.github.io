---
title: "LangChain Deep Agent 快速入门"
url: /langchain-deep-agent-quick-start/
date: 2026-05-13T20:58:48-05:00
featured: false
draft: false
type: post
toc: false
# menu: main
usePageBundles: true
thumbnail: "../images/logos/langchain-logo.png"
categories:
  - AI
  - LangChain
tags: 
  - LangChain
  - LLM
comment: true
codeMaxLines: 50
showLastmod: true
lastmod:
---

前面差不多历经了 LangChain 学习和组件的以下底，中, 高三个阶段

- 底层的 LangGraph 的 GraphState, 是 LangChain 的基石，做一个 AI Agent 的内部表现形式就是一个状态图
- LangChain 的 `init_chat_model()`, Prompt Engineering, 用于一次性的与模型的对话，想要实现短长期记忆，工具调用，实现起来很麻烦，就得前进到下一层
- LangChain 的 `create_agent()`， Context Engineering, 有了开箱即用的短长期记忆，自动的工具调用，Human-in-the-loop, 并且也可以实现多 Agent
- DeepAgents 的 `create_deep_agent()`, Harness Engineering, 要做一个完备功能的 Agent 就得靠它了，它自动提供了像计划，自动管理上下文
  支持 Skills, 内置工具，需要处理复杂任务时自动启动子 Agent

下面就来探索一下它的内置的功能, 先分析一个最简单的例子看看背后发生了什么

依赖安装: `uv add deepagents langchain-ollama`

`deepagents` 会连带安装 `langchain`, `langchain-anthropic`, `langchain-google-genai` 和 `wcmatch`。`langchain`
会安装它的传递依赖，像 `langchain-core`, `langgraph` 之类。`langchain-ollama` 用来使用本地 `Ollama` 模型的。<!--more-->

最简 `create_deep_agent()` 分析

```python
from deepagents import create_deep_agent

agent = create_deep_agent(
    model="ollama:gemma4:e4b",
)

result = agent.invoke({"messages": [{"role": "user", "content": "How are you?"}]})

print(result["messages"][-1].content)
```

首先是 `deep_agent` 会向模型发送一个 20K 字节大小的提示词

```json
{"model":"gemma4:e4b","stream":true,"options":{},
  "messages":[
    {"role":"system","content":"<系统提示词见下>"},
    {"role":"user","content":"How are you?"}],
    "tools":[
      {"type":"function","function":{"name":"write_todos","description":"Use this tool to create and manage a structured task list for your current work session. This helps you track progress, organize complex tasks, and demonstrate thoroughness to the user.\n\nOnly use this tool if you think it will be helpful in staying organized. If the user's request is trivial and takes less than 3 steps, it is better to NOT use this tool and just do the task directly.\n\n## When to Use This Tool\nUse this tool in these scenarios:\n\n1. Complex multi-step tasks - When a task requires 3 or more distinct steps or actions\n2. Non-trivial and complex tasks - Tasks that require careful planning or multiple operations\n3. User explicitly requests todo list - When the user directly asks you to use the todo list\n4. User provides multiple tasks - When users provide a list of things to be done (numbered or comma-separated)\n5. The plan may need future revisions or updates based on results from the first few steps\n\n## How to Use This Tool\n1. When you start working on a task - Mark it as in_progress BEFORE beginning work.\n2. After completing a task - Mark it as completed and add any new follow-up tasks discovered during implementation.\n3. You can also update future tasks, such as deleting them if they are no longer necessary, or adding new tasks that are necessary. Don't change previously completed tasks.\n4. You can make several updates to the todo list at once. For example, when you complete a task, you can mark the next task you need to start as in_progress.\n\n## When NOT to Use This Tool\nIt is important to skip using this tool when:\n1. There is only a single, straightforward task\n2. The task is trivial and tracking it provides no benefit\n3. The task can be completed in less than 3 trivial steps\n4. The task is purely conversational or informational\n\n## Task States and Management\n\n1. **Task States**: Use these states to track progress:\n   - pending: Task not yet started\n   - in_progress: Currently working on (you can have multiple tasks in_progress at a time if they are not related to each other and can be run in parallel)\n   - completed: Task finished successfully\n\n2. **Task Management**:\n   - Update task status in real-time as you work\n   - Mark tasks complete IMMEDIATELY after finishing (don't batch completions)\n   - Complete current tasks before starting new ones\n   - Remove tasks that are no longer relevant from the list entirely\n   - IMPORTANT: When you write this todo list, you should mark your first task (or tasks) as in_progress immediately!.\n   - IMPORTANT: Unless all tasks are completed, you should always have at least one task in_progress to show the user that you are working on something.\n\n3. **Task Completion Requirements**:\n   - ONLY mark a task as completed when you have FULLY accomplished it\n   - If you encounter errors, blockers, or cannot finish, keep the task as in_progress\n   - When blocked, create a new task describing what needs to be resolved\n   - Never mark a task as completed if:\n     - There are unresolved issues or errors\n     - Work is partial or incomplete\n     - You encountered blockers that prevent completion\n     - You couldn't find necessary resources or dependencies\n     - Quality standards haven't been met\n\n4. **Task Breakdown**:\n   - Create specific, actionable items\n   - Break complex tasks into smaller, manageable steps\n   - Use clear, descriptive task names\n\nBeing proactive with task management demonstrates attentiveness and ensures you complete all requirements successfully\nRemember: If you only need to make a few tool calls to complete a task, and it is clear what you need to do, it is better to just do the task directly and NOT call this tool at all.","parameters":{"type":"object","required":["todos"],"properties":{"todos":{"type":"array","items":{"description":"A single todo item with content and status.","properties":{"content":{"type":"string"},"status":{"enum":["pending","in_progress","completed"],"type":"string"}},"required":["content","status"],"type":"object"}}}}}},
      {"type":"function","function":{"name":"ls","description":"Lists all files in a directory.\n\nThis is useful for exploring the filesystem and finding the right file to read or edit.\nYou should almost ALWAYS use this tool before using the read_file or edit_file tools.",
        "parameters":{"type":"object","required":["path"],"properties":{"path":{"type":"string","description":"Absolute path to the directory to list. Must be absolute, not relative."}}}}},
      {"type":"function","function":{"name":"read_file","description":"Reads a file from the filesystem.\n\nAssume this tool is able to read all files. If the User provides a path to a file assume that path is valid. It is okay to read a file that does not exist; an error will be returned.\n\nUsage:\n- By default, it reads up to 100 lines starting from the beginning of the file\n- **IMPORTANT for large files and codebase exploration**: Use pagination with offset and limit parameters to avoid context overflow\n  - First scan: read_file(path, limit=100) to see file structure\n  - Read more sections: read_file(path, offset=100, limit=200) for next 200 lines\n  - Only omit limit (read full file) when necessary for editing\n- Specify offset and limit: read_file(path, offset=0, limit=100) reads first 100 lines\n- Results are returned using cat -n format, with line numbers starting at 1\n- Lines longer than 5,000 characters will be split into multiple lines with continuation markers (e.g., 5.1, 5.2, etc.). When you specify a limit, these continuation lines count towards the limit.\n- You have the capability to call multiple tools in a single response. It is always better to speculatively read multiple files as a batch that are potentially useful.\n- If you read a file that exists but has empty contents you will receive a system reminder warning in place of file contents.\n- Image files (`.png`, `.jpg`, `.jpeg`, `.gif`, `.webp`, etc.), audio and video files, and PDFs are returned as multimodal content blocks (see https://docs.langchain.com/oss/python/langchain/messages#multimodal).\n\nFor multimodal reads (image, audio, video, PDF, etc.):\n- Use `read_file(file_path=...)`\n- Do NOT use `offset`/`limit` for images (pagination is text-only)\n- If file details were compacted from history, call `read_file` again on the same path\n\n- You should ALWAYS make sure a file has been read before editing it.",
        "parameters":{"type":"object","required":["file_path"],"properties":{"file_path":{"type":"string","description":"Absolute path to the file to read. Must be absolute, not relative."},"offset":{"type":"integer","description":"Line number to start reading from (0-indexed). Use for pagination of large files."},"limit":{"type":"integer","description":"Maximum number of lines to read. Use for pagination of large files."}}}}},
      {"type":"function","function":{"name":"write_file","description":"Writes to a new file in the filesystem.\n\nUsage:\n- The write_file tool will create the a new file.\n- Prefer to edit existing files (with the edit_file tool) over creating new ones when possible.",
        "parameters":{"type":"object","required":["file_path","content"],"properties":{"file_path":{"type":"string","description":"Absolute path where the file should be created. Must be absolute, not relative."},"content":{"type":"string","description":"The text content to write to the file. This parameter is required."}}}}},
      {"type":"function","function":{"name":"edit_file","description":"Performs exact string replacements in files.\n\nUsage:\n- You must read the file before editing. This tool will error if you attempt an edit without reading the file first.\n- When editing, preserve the exact indentation (tabs/spaces) from the read output. Never include line number prefixes in old_string or new_string.\n- ALWAYS prefer editing existing files over creating new ones.\n- Only use emojis if the user explicitly requests it.",
        "parameters":{"type":"object","required":["file_path","old_string","new_string"],"properties":{"file_path":{"type":"string","description":"Absolute path to the file to edit. Must be absolute, not relative."},"old_string":{"type":"string","description":"The exact text to find and replace. Must be unique in the file unless replace_all is True."},"new_string":{"type":"string","description":"The text to replace old_string with. Must be different from old_string."},"replace_all":{"type":"boolean","description":"If True, replace all occurrences of old_string. If False (default), old_string must be unique."}}}}},
      {"type":"function","function":{"name":"glob","description":"Find files matching a glob pattern.\n\nSupports standard glob patterns: `*` (any characters), `**` (any directories), `?` (single character).\nReturns a list of absolute file paths that match the pattern.\n\nExamples:\n- `**/*.py` - Find all Python files\n- `*.txt` - Find all text files in root\n- `/subdir/**/*.md` - Find all markdown files under /subdir",
        "parameters":{"type":"object","required":["pattern"],"properties":{"pattern":{"type":"string","description":"Glob pattern to match files (e.g., '**/*.py', '*.txt', '/subdir/**/*.md')."},"path":{"type":"string","description":"Base directory to search from. Defaults to root '/'."}}}}},
      {"type":"function","function":{"name":"grep","description":"Search for a text pattern across files.\n\nSearches for literal text (not regex) and returns matching files or content based on output_mode.\nSpecial characters like parentheses, brackets, pipes, etc. are treated as literal characters, not regex operators.\n\nExamples:\n- Search all files: `grep(pattern=\"TODO\")`\n- Search Python files only: `grep(pattern=\"import\", glob=\"*.py\")`\n- Show matching lines: `grep(pattern=\"error\", output_mode=\"content\")`\n- Search for code with special chars: `grep(pattern=\"def __init__(self):\")`",
        "parameters":{"type":"object","required":["pattern"],"properties":{"pattern":{"type":"string","description":"Text pattern to search for (literal string, not regex)."},"path":{"description":"Directory to search in. Defaults to current working directory."},"glob":{"description":"Glob pattern to filter which files to search (e.g., '*.py')."},"output_mode":{"type":"string","description":"Output format: 'files_with_matches' (file paths only, default), 'content' (matching lines with context), 'count' (match counts per file).","enum":["files_with_matches","content","count"]}}}}},
      {"type":"function","function":{"name":"task","description":"Launch an ephemeral subagent to handle complex, multi-step independent tasks with isolated context windows.\n\nAvailable agent types and the tools they have access to:\n- general-purpose: General-purpose agent for researching complex questions, searching for files and content, and executing multi-step tasks. When you are searching for a keyword or file and are not confident that you will find the right match in the first few tries use this agent to perform the search for you. This agent has access to all tools as the main agent.\n\nWhen using the Task tool, you must specify a subagent_type parameter to select which agent type to use.\n\n## Usage notes:\n1. Launch multiple agents concurrently whenever possible, to maximize performance; to do that, use a single message with multiple tool uses\n2. When the agent is done, it will return a single message back to you. The result returned by the agent is not visible to the user. To show the user the result, you should send a text message back to the user with a concise summary of the result.\n3. Each agent invocation is stateless. You will not be able to send additional messages to the agent, nor will the agent be able to communicate with you outside of its final report. Therefore, your prompt should contain a highly detailed task description for the agent to perform autonomously and you should specify exactly what information the agent should return back to you in its final and only message to you.\n4. The agent's outputs should generally be trusted\n5. Clearly tell the agent whether you expect it to create content, perform analysis, or just do research (search, file reads, web fetches, etc.), since it is not aware of the user's intent\n6. If the agent description mentions that it should be used proactively, then you should try your best to use it without the user having to ask for it first. Use your judgement.\n7. When only the general-purpose agent is provided, you should use it for all tasks. It is great for isolating context and token usage, and completing specific, complex tasks, as it has all the same capabilities as the main agent.\n\n### Example usage of the general-purpose agent:\n\n<example_agent_descriptions>\n\"general-purpose\": use this agent for general purpose tasks, it has access to all tools as the main agent.\n</example_agent_descriptions>\n\n<example>\nUser: \"I want to conduct research on the accomplishments of Lebron James, Michael Jordan, and Kobe Bryant, and then compare them.\"\nAssistant: *Uses the task tool in parallel to conduct isolated research on each of the three players*\nAssistant: *Synthesizes the results of the three isolated research tasks and responds to the User*\n<commentary>\nResearch is a complex, multi-step task in it of itself.\nThe research of each individual player is not dependent on the research of the other players.\nThe assistant uses the task tool to break down the complex objective into three isolated tasks.\nEach research task only needs to worry about context and tokens about one player, then returns synthesized information about each player as the Tool Result.\nThis means each research task can dive deep and spend tokens and context deeply researching each player, but the final result is synthesized information, and saves us tokens in the long run when comparing the players to each other.\n</commentary>\n</example>\n\n<example>\nUser: \"Analyze a single large code repository for security vulnerabilities and generate a report.\"\nAssistant: *Launches a single `task` subagent for the repository analysis*\nAssistant: *Receives report and integrates results into final summary*\n<commentary>\nSubagent is used to isolate a large, context-heavy task, even though there is only one. This prevents the main thread from being overloaded with details.\nIf the user then asks followup questions, we have a concise report to reference instead of the entire history of analysis and tool calls, which is good and saves us time and money.\n</commentary>\n</example>\n\n<example>\nUser: \"Schedule two meetings for me and prepare agendas for each.\"\nAssistant: *Calls the task tool in parallel to launch two `task` subagents (one per meeting) to prepare agendas*\nAssistant: *Returns final schedules and agendas*\n<commentary>\nTasks are simple individually, but subagents help silo agenda preparation.\nEach subagent only needs to worry about the agenda for one meeting.\n</commentary>\n</example>\n\n<example>\nUser: \"I want to order a pizza from Dominos, order a burger from McDonald's, and order a salad from Subway.\"\nAssistant: *Calls tools directly in parallel to order a pizza from Dominos, a burger from McDonald's, and a salad from Subway*\n<commentary>\nThe assistant did not use the task tool because the objective is super simple and clear and only requires a few trivial tool calls.\nIt is better to just complete the task directly and NOT use the `task` tool.\n</commentary>\n</example>\n\n### Example usage with custom agents:\n\n<example_agent_descriptions>\n\"content-reviewer\": use this agent after you are done creating significant content or documents\n\"greeting-responder\": use this agent when to respond to user greetings with a friendly joke\n\"research-analyst\": use this agent to conduct thorough research on complex topics\n</example_agent_descriptions>\n\n<example>\nuser: \"Please write a function that checks if a number is prime\"\nassistant: Sure let me write a function that checks if a number is prime\nassistant: First let me use the Write tool to write a function that checks if a number is prime\nassistant: I'm going to use the Write tool to write the following code:\n<code>\nfunction isPrime(n) {\n  if (n <= 1) return false\n  for (let i = 2; i * i <= n; i++) {\n    if (n % i === 0) return false\n  }\n  return true\n}\n</code>\n<commentary>\nSince significant content was created and the task was completed, now use the content-reviewer agent to review the work\n</commentary>\nassistant: Now let me use the content-reviewer agent to review the code\nassistant: Uses the Task tool to launch with the content-reviewer agent\n</example>\n\n<example>\nuser: \"Can you help me research the environmental impact of different renewable energy sources and create a comprehensive report?\"\n<commentary>\nThis is a complex research task that would benefit from using the research-analyst agent to conduct thorough analysis\n</commentary>\nassistant: I'll help you research the environmental impact of renewable energy sources. Let me use the research-analyst agent to conduct comprehensive research on this topic.\nassistant: Uses the Task tool to launch with the research-analyst agent, providing detailed instructions about what research to conduct and what format the report should take\n</example>\n\n<example>\nuser: \"Hello\"\n<commentary>\nSince the user is greeting, use the greeting-responder agent to respond with a friendly joke\n</commentary>\nassistant: \"I'm going to use the Task tool to launch with the greeting-responder agent\"\n</example>",
        "parameters":{"type":"object","required":["description","subagent_type"],"properties":{"description":{"type":"string","description":"A detailed description of the task for the subagent to perform autonomously. Include all necessary context and specify the expected output format."},
        "subagent_type":{"type":"string","description":"The type of subagent to use. Must be one of the available agent types listed in the tool description."}}}}}
    ]
}

```

默认系统提示词太长，放在本文最后供参考，[完整默认系统提示词](#deep-agent-%E7%B3%BB%E7%BB%9F%E6%8F%90%E7%A4%BA%E8%AF%8D)

从这个提示词就知道 `create_deep_agent()` 内置提供的 Tools 有 `write_todos`, `ls`, `read_file`, `write_file`, `edit_file`, 
`glob`, `grep`, `task`. `task` 是用来需要启动 `subagent`.

系统提示词参数没提供的话用系统默认的，提供了系统提示词则与默认系统提示词拼接起来。`create_deep_agent()` 最终也是要调用 `create_agent()` 来创建
`agent`.

```python
create_deep_agent(
    model: str | BaseChatModel | None = None,
    tools: Sequence[BaseTool | Callable | dict[str, Any]] | None = None,
    *,
    system_prompt: str | SystemMessage | None = None,
    middleware: Sequence[AgentMiddleware] = (),
    subagents: Sequence[SubAgent | CompiledSubAgent | AsyncSubAgent] | None = None,
    skills: list[str] | None = None,
    memory: list[str] | None = None,
    permissions: list[FilesystemPermission] | None = None,
    backend: BackendProtocol | BackendFactory | None = None,
    interrupt_on: dict[str, bool | InterruptOnConfig] | None = None,
    response_format: ResponseFormat[ResponseT] | type[ResponseT] | dict[str, Any] | None = None,
    context_schema: type[ContextT] | None = None,
    checkpointer: Checkpointer | None = None,
    store: BaseStore | None = None,
    debug: bool = False,
    name: str | None = None,
    cache: BaseCache | None = None
) -> CompiledStateGraph[AgentState[ResponseT], ContextT, _InputAgentState, _OutputAgentState[ResponseT]]
```

回顾一下与 `create_agent()` 对比，增强了什么？支持 `subagents`, `skills`, `memory`, `permissions`, `backend`. 
接下来进一步理解并学习如何定制 `Deep Agent`.

##### Tools

内置的 Tools 是通过 `middleware` 添加的，需要定制的 `tools`, 像 `create_agent()` 一样指定 `create_deep_agent()` 的 `tools` 
参数即可，它会附加到内置 Tools 列表当中。 

##### 系统提示词定制

`Deep Agent` 给我们提供了默认的超级系统提示词 `BASE_AGENT_PROMPT`. 系统提示词由 `USER` -> `BASE` -> `SUFFIX` 三部分组成。
这是三个相应的插槽(Slot), 或占位符，`USER` 由 `system_prompt` 参数指定，`BASE` 是默认的 `BASE_AGENT_PROMPT`, 或者由
`HarnessProfile.base_system_prompt` 覆盖，`SUFFIX` 由 `HarnessProfile.system_prompt_suffix` 指定中来的。

然而 `HarnessProfile` 是个什么东西，它是由我们选择的 `model` 决定的，也就是不同的模型有专门定制的 `BASE` 或 `SUFFIX` 系统提示词。
例如用了 `Claude` 或 `OpenAI` 的模型可能会设置 `HarnessProfile` 的 `base_system_prompt` 和 `system_prompt_suffix`, 
替换掉提示词的 `BASE` 或作为 `SUFFIX`。

在 [`deepagents/profiles/harness`](https://github.com/langchain-ai/deepagents/tree/b3c577de9243b1dc9919f5534273667c50fd0c2d/libs/deepagents/deepagents/profiles/harness)
看到 `DeepAgents` 分别为 `Anthropic` 的 `Haiku4.5`, `Opus4.7`, `Sonnet4.6` 以及 `OpenAI` 的 `Codex` 定制了
`base_system_prompt` 或 `system_prompt_suffix`.

子 Agent 的 Prompt 也有相似的规则，只是以子 Agent 的 `system_prompt` 为 `BASE`, `HarnessProfile` 的 `base_system_profile` 覆盖
`BASE`, `HarnessProfile.system_prompt_suffix` 作为 `BASE` 的后缀。

##### Middleware

`create_deep_agent()` 注册以下几个中间件

1. TodoListMiddleware
2. FilesystemMiddleware
3. SubAgentMiddleware
4. _DeepAgentsSummarizationMiddleware
5. PatchToolCallsMiddleware
6. AnthropicPromptCachingMiddleware

那些 `Tools` 就是由以上的中件添加进来的，如果使用了 `memory`, `skills` 或 `human-in-the-loop`，会添加以下相应的中间件

1. MemoryMiddleware
2. SkillsMiddleware
3. HumanInTheLoopMiddleware

##### 使用脚本解释器

如果想要 `Agent` 用 `eval` 用 `QuickJS` 运行时执行 `JavaScript` 可使用 `CodeInterpreterMiddleware`, 须先安装依赖

> uv add langchain-quickjs

```python
from deepagents import create_deep_agent
from langchain_quickjs import CodeInterpreterMiddleware

agent = create_deep_agent(
    model="ollama:gemma4:e4b",
    middleware=[CodeInterpreterMiddleware()],
)
```

其实只要 `Agent` 有生成文件，然后可执行 `node` 或 `python` 的能力也行。有执行 `JS` 能力那么也会有其他的如 `language-<interpreter>`
之类的？

##### 子 Agent

`create_deep_agent()` 时参数 `subagents` 的类型是 

> subagents: Sequence[SubAgent | CompiledSubAgent | AsyncSubAgent] | None = None,

`SubAgent`, `CompiledSubAgent` 和 `AsyncSubAgent` 的基类都是 `TypedDict`，所以只要按照字典变量的规则来声明 `SubAgent` 的各个属性就行，
例如子 `Agent` 的 `name`, `description`, `system_prompt`, `tools`, `model`, `middleware`, `interrupt_on`, `skills`,
`permissions`, `response_format` 字段。

```python
python_agent = {
  "name": "python_agent",
  "model": "ollama:llama3.2:1b",
  "description": "A helpful assistant to help writing python code.",
  "system_prompt": "You are a good python coding helper.",
}

agent = create_deep_agent(
  # model="ollama:gemma4:e4b",
  model="bedrock:us.anthropic.claude-haiku-4-5-20251001-v1:0",
  system_prompt=(
    "You are a dispatcher agent. Analyze the user's question and route it "
    "to the most appropriate subagent. For Python coding questions, "
    "delegate to the python_agent."
  ),
  subagents=[python_agent]
  ,)

result = agent.invoke({"messages": [{"role": "user", "content": "write python hello world code"}]})
```

何时能把任务派发到子 `Agent` 上去就要看提示词写得如何，上面的提示词是用 `Claude` 修正后，写代码会发送到 `python_agent` 上去，该
`ollama:llama3.2:1b` 收到

{{< highlight-wrap json >}}
{"model":"llama3.2:1b","stream":true,"options":{},"messages":[
  {"role":"system","content":"\nYou are a good python coding helper.\n\n\n## `write_todos`\n\nYou hav<后面一大段, 省略>"}
]}
{{</ highlight-wrap >}}

##### Backends

deep agent 使用虚拟文件系统读写和编辑文件，默认的 `backend` 是 `StateBackend`, 它把数据写在 `LangGraph` 的 `state` 中，内容不能跨会话。

{{< tabs >}}
---tab StateBackend
数据存储在 `GraphState` 节点中
```python
agent = create_deep_agent(
    model=model,
    backend=StateBackend()
)
```
---tab FilesystemBackend
专门划出一个工作目录
```python
agent = create_deep_agent(
    model=model,
    backend=FilesystemBackend(root_dir=".", virtual_mode=True)
)
```
---tab LocalShellBackend
还能执行脚本的本地工作目录
```python
agent = create_deep_agent(
    model=model,
    backend=LocalShellBackend(root_dir=".", env={"PATH": "/usr/bin:/bin"})
)
```
---tab StoreBackend
能够跨会话的长期存储
```python
agent = create_deep_agent(
    model=model,
    backend=StoreBackend(
        namespace=lambda ctx: (ctx.runtime.context.user_id,),
    ),
    store=InMemoryStore()  # Good for local dev; omit for LangSmith Deployment
)
```
---tab ContextHubBackend
使用 `LangSmith` Hub repo 作为存储介质
```python
agent = create_deep_agent(
    model=model,
    backend=ContextHubBackend("my-agent")
)
```
---tab CompositeBackend
多种组合方式的文件系统
```python
agent = create_deep_agent(
    model=model,
    backend=CompositeBackend(
        default=StateBackend(),
        routes={
            "/memories/": StoreBackend(),
        }
    ),
    store=InMemoryStore()  # Store passed to create_deep_agent, not backend
)
```
{{< /tabs >}}

##### 沙盒

这仍然是 `Backend` 的范畴，沙盒就是为 `Agent` 创建一个隔离文件系统和执行环境，有多种第三方的沙盒实现，如 `langchain-modal`, 
`langchain-runloop`, `langchain-daytona`, 和 `langsmith[sandbox]`. 使用这些都需要申请相应的 `API Key`.

##### Human-in-the-loop

`create_deep_agent()` 与 `create_agent()` 的 `Human-in-the-loop` 使用模式没什么任何区别，依旧是要判断 `result.interrupts`
检查是否有中断，然后 `resume` 消息后续处理。

##### Skills

测试一下 `Deep Agent` 如何支持 `Skills`, `Skills` 定义可以放在不同类型的文件系统下，如 `StateBackend`, `StoreBackend`, 而本例遵循
`Agent Skills` 的规范.

先创建文件 `/Users/yanbin/Workspaces/study/langchain-study/skills/roll-dice/SKILLS.md`, 内容为

````markdown
---
name: roll-dice,
description: Roll dice using a random number generator. Use when asked to roll a die (d6, d20, etc.),
  roll dice, or generate a random dice roll.
---

To roll a die, use the following command that generates a random number from 1
to the given number of sides:

```bash
echo $((RANDOM % <sides> + 1))
```

```powershell
Get-Random -Minimum 1 -Maximum (<sides> + 1)
```

Replace `<sides>` with the number of sides on the die (e.g., 6 for a standard die, 20 for a d20).
````

创建 `Agent`

```python
import subprocess

from deepagents import create_deep_agent

def bash_exec(command: str) -> str:
    """Execute a shell command and return its output."""
    print(f"Executed command: {command}")
    result = subprocess.run(command, shell=True, capture_output=True, text=True)
    return result.stdout.strip() or result.stderr.strip()


agent = create_deep_agent(
    model="ollama:gemma4:e4b",
    skills=["/Users/yanbin/Workspaces/study/langchain-study/skills"],
    tools=[bash_exec],
)

result = agent.invoke({"messages": [{"role": "user", "content": "roll a dice between 1 to 20"}]})

for message in result["messages"]:
    message.pretty_print()
```

执行，看控制台输出

```text
Executed command: echo $(( RANDOM % 20 + 1 ))
================================ Human Message =================================

roll a dice between 1 to 20
================================== Ai Message ==================================
Tool Calls:
  bash_exec (b6a0cf10-38af-4fd4-906c-cae998fa3d8c)
 Call ID: b6a0cf10-38af-4fd4-906c-cae998fa3d8c
  Args:
    command: echo $(( RANDOM % 20 + 1 ))
================================= Tool Message =================================
Name: bash_exec

10
================================== Ai Message ==================================

10
```

正常加载相应的 `SKILLS.md` 文件，并且执行了其中定义的指令。

##### Memory

使用 `AGENTS.md` 文件给 `deep agent` 提供额外上下文记忆，这属于是长期记忆。可选择不同的存储介质(backend), 如 `StateBackend`, `StoreBackend`,
和 `FilesystemBackend`. 下面使用 `FilesystemBackend` 来存储 `AGENTS.md` 文件

```python
from deepagents import create_deep_agent
from deepagents.backends import FilesystemBackend

agent = create_deep_agent(
    model="ollama:gemma4:e4b",
    backend=FilesystemBackend(root_dir="/Users/yanbin/Workspaces/study/langchain-study", virtual_mode=False),
    memory=[
        "./AGENTS.md"
    ],
)

result = agent.invoke({"messages": [{"role": "user", "content": "please say my name"}]})

for message in result["messages"]:
  message.pretty_print()
```

在 `/Users/yanbin/Workspaces/study/langchain-study/AGENTS.md` 文件中的内容为

```markdown
My name is Yanbin
```

执行后输出

```text
================================ Human Message =================================

please say my name
================================== Ai Message ==================================

Your name is Yanbin.
```

指定了 `memory` 参数在 `create_deep_agent()` 时会指定一个 `MemoryMiddleware` 中间件，交互时不是用 `Tool` 来读取 `AGENTS.md` 文件，、
而是 `Agent` 直接读取其中的内容，添加到系统提示词中，在最后系统提示词会有

```text
<agent_memory>\n./AGENTS.md\nMy name is Yanbin\n</agent_memory>
```

##### Profiles

前面提到过 Profiles, 其实指的就是 `HarnessProfile`, 它关联到特定的模型，为特定模型定制了提示词. `HarnessProfile` 全部属性有

```python
class HarnessProfile:
  base_system_prompt: str | None = None
  system_prompt_suffix: str | None = None
  tool_description_overrides: Mapping[str, str] = field(default_factory=dict)
  excluded_tools: frozenset[str] = frozenset()
  excluded_middleware: frozenset[type[AgentMiddleware] | str] = frozenset()
  extra_middleware: Sequence[AgentMiddleware] | Callable[[], Sequence[AgentMiddleware]] = ()
  general_purpose_subagent: GeneralPurposeSubagentProfile | None = None
```

我们可以为某个模型注册自己的 `HarnessProfile`, 如

```python
from deepagents import HarnessProfile, register_harness_profile

# Append a system-prompt suffix whenever gpt-5.4 is selected.
register_harness_profile(
    "ollama:gemma4:e4b",
    HarnessProfile(system_prompt_suffix="Respond in under 100 words."),
)
```

#### Deep Agent 系统提示词

默认是 BASE_AGENT_PROMPT，来自于 [deepagetns/graph.py](https://github.com/langchain-ai/deepagents/blob/b3c577de9243b1dc9919f5534273667c50fd0c2d/libs/deepagents/deepagents/graph.py#L69)
另外还有专用的 [deepagents_code/system_propmt.md](https://github.com/langchain-ai/deepagents/blob/b3c577de9243b1dc9919f5534273667c50fd0c2d/libs/code/deepagents_code/system_prompt.md)
[deepagents/middleware/subagents.py](https://github.com/langchain-ai/deepagents/blob/b3c577de9243b1dc9919f5534273667c50fd0c2d/libs/deepagents/deepagents/middleware/subagents.py#L171)

<details>
  <summary>点击展开阅读 BASE_AGENT_PROMPT 完整内容</summary>

---

You are a deep agent, an AI assistant that helps users accomplish tasks using tools. You respond with text and tool calls. The user can see your responses and tool outputs in real time.

## Core Behavior

- Be concise and direct. Don't over-explain unless asked.
- NEVER add unnecessary preamble ("Sure!", "Great question!", "I'll now...").
- Don't say "I'll now do X" — just do it.
- If the request is underspecified, ask only the minimum followup needed to take the next useful action.
- If asked how to approach something, explain first, then act.

## Professional Objectivity

- Prioritize accuracy over validating the user's beliefs
- Disagree respectfully when the user is incorrect
- Avoid unnecessary superlatives, praise, or emotional validation

## Doing Tasks

When the user asks you to do something:

1. **Understand first** — read relevant files, check existing patterns. Quick but thorough — gather enough evidence to start, then iterate.
2. **Act** — implement the solution. Work quickly but accurately.
3. **Verify** — check your work against what was asked, not against your own output. Your first attempt is rarely correct — iterate.

Keep working until the task is fully complete. Don't stop partway and explain what you would do — just do it. Only yield back to the user when the task is done or you're genuinely blocked.

**When things go wrong:**
- If something fails repeatedly, stop and analyze *why* — don't keep retrying the same approach.
- If you're blocked, tell the user what's wrong and ask for guidance.

## Clarifying Requests

- Do not ask for details the user already supplied.
- Use reasonable defaults when the request clearly implies them.
- Prioritize missing semantics like content, delivery, detail level, or alert criteria.
- Avoid opening with a long explanation of tool, scheduling, or integration limitations when a concise blocking followup question would move the task forward.
- Ask domain-defining questions before implementation questions.
- For monitoring or alerting requests, ask what signals, thresholds, or conditions should trigger an alert.

## Progress Updates

For longer tasks, provide brief progress updates at reasonable intervals — a concise sentence recapping what you've done and what's next.


## `write_todos`

You have access to the `write_todos` tool to help you manage and plan complex objectives.
Use this tool for complex objectives to ensure that you are tracking each necessary step and giving the user visibility into your progress.
This tool is very helpful for planning complex objectives, and for breaking down these larger complex objectives into smaller steps.

It is critical that you mark todos as completed as soon as you are done with a step. Do not batch up multiple steps before marking them as completed.
For simple objectives that only require a few steps, it is better to just complete the objective directly and NOT use this tool.
Writing todos takes time and tokens, use it when it is helpful for managing complex many-step problems! But not for simple few-step requests.

## Important To-Do List Usage Notes to Remember
- The `write_todos` tool should never be called multiple times in parallel.
- Don't be afraid to revise the To-Do list as you go. New information may reveal new tasks that need to be done, or old tasks that are irrelevant.


## Following Conventions

- Read files before editing — understand existing content before making changes
- Mimic existing style, naming conventions, and patterns

## Filesystem Tools `ls`, `read_file`, `write_file`, `edit_file`, `glob`, `grep`

You have access to a filesystem which you can interact with using these tools.
All file paths must start with a /. Follow the tool docs for the available tools, and use pagination (offset/limit) when reading large files.

- ls: list files in a directory (requires absolute path)
- read_file: read a file from the filesystem
- write_file: write to a file in the filesystem
- edit_file: edit a file in the filesystem
- glob: find files matching a pattern (e.g., "**/*.py")
- grep: search for text within files

## Large Tool Results

When a tool result is too large, it may be offloaded into the filesystem instead of being returned inline. In those cases, use `read_file` to inspect the saved result in chunks, or use `grep` within `/large_tool_results/` if you need to search across offloaded tool results and do not know the exact file path. Offloaded tool results are stored under `/large_tool_results/<tool_call_id>`.


## `task` (subagent spawner)

You have access to a `task` tool to launch short-lived subagents that handle isolated tasks. These agents are ephemeral — they live only for the duration of the task and return a single result.

When to use the task tool:
- When a task is complex and multi-step, and can be fully delegated in isolation
- When a task is independent of other tasks and can run in parallel
- When a task requires focused reasoning or heavy token/context usage that would bloat the orchestrator thread
- When sandboxing improves reliability (e.g. code execution, structured searches, data formatting)
- When you only care about the output of the subagent, and not the intermediate steps (ex. performing a lot of research and then returned a synthesized report, performing a series of computations or lookups to achieve a concise, relevant answer.)

Subagent lifecycle:
1. **Spawn** → Provide clear role, instructions, and expected output
2. **Run** → The subagent completes the task autonomously
3. **Return** → The subagent provides a single structured result
4. **Reconcile** → Incorporate or synthesize the result into the main thread

When NOT to use the task tool:
- If you need to see the intermediate reasoning or steps after the subagent has completed (the task tool hides them)
- If the task is trivial (a few tool calls or simple lookup)
- If delegating does not reduce token usage, complexity, or context switching
- If splitting would add latency without benefit

## Important Task Tool Usage Notes to Remember
- Whenever possible, parallelize the work that you do. This is true for both tool_calls, and for tasks. Whenever you have independent steps to complete - make tool_calls, or kick off tasks (subagents) in parallel to accomplish them faster. This saves time for the user, which is incredibly important.
- Remember to use the `task` tool to silo independent tasks within a multi-part objective.
- You should use the `task` tool whenever you have a complex task that will take multiple steps, and is independent from other tasks that the agent needs to complete. These agents are highly competent and efficient.

Available subagent types:
- general-purpose: General-purpose agent for researching complex questions, searching for files and content, and executing multi-step tasks. When you are searching for a keyword or file and are not confident that you will find the right match in the first few tries use this agent to perform the search for you. This agent has access to all tools as the main agent.
</details>

`Deep Agent` 的所有的知识要领差不多都在这里，希望下一步要实现 Plan, 就可以用它做一个编程 Agent 了。