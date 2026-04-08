---
title: "LangChain 1.2 入门学习"
url: /langchain-1-2-get-started/
date: 2026-04-07T21:25:48-05:00
featured: false
draft: false
type: post
toc: false
# menu: main
usePageBundles: true
thumbnail: "../images/logos/ai-logo.png"
categories:
  - AI
tags: 
  - LangChain
comment: true
codeMaxLines: 30
lastmod:
---

AI 界日新月异，新名词层出不穷，像 Prompt Engineering, Context Engineering, 到出现又来了 Harness Engineering. 现在最怕两个人出来搞事情，
Dario Amodei 和 Andrej Karpathy, 前者要用 AI 替代一切，后者专造名词。在大语言模型时代，搞机器学习了，专注模型的不用多少人，多数人还能在 AI
上蹭热点的话就剩下 AI Agent 的这个赛道了，其余就是应用了，例如 Vibe Coding，或更贴近具体业务的应用。

像各大 AI 编程工具，如 Codex, Claude Code, OpenCode, GitHub Copilot， Gemini, Cursor, Antigravity, Trae 本质上都是在比拼各家
AI Agent 的能力。最近火的那个 OpenClaw 也是一个学习 AI Agent 的好范例。

而 AI Agent 方面的框架首当其冲就是 LangChain, 它提供了 Python 和 TypeScript 两种语言的支持，网上有人做了个
[langchain4j](https://github.com/langchain4j/langchain4j), 这种跟随项目恐怕也坚持不了多久。其他 AI Agent 框架有 [Pydantic AI]，
微软的 AutoGen, Crew AI, AWS Strands Agents. 各大语言模型 OpenAI, Anthropic, Gemini 都有自己的 SDK, 但要开发一个 Multi Agent
的系统更需要一个第三方的 AI Agent 框架来整合各家模型的能力，提供统一的接口和工具，这个框架就是 LangChain. <!--more-->

当前 LangChain 的版本在 PyPI 上是 1.2.15, 在关注 LangChain 的同时我们也会看到 LangGraph 和 DeepAgents, 那么这三者有何关系呢？按不同
层次来看，它们分别代表着

1. LangChain: Agent 框架, 用于基础构建模型, 万物皆链
2. LangGraph: Agent 运行时，工作流程安排, 状态 + 有向图
3. DeepAgents: Agent Harness，自主推理引擎，自治编排

下面我们只用 LangChain 来体验如何连接本地模型(将用 Ollama) 和远程模型(将用 Gemini)。

LangChain 支持 Python 和 TypeScript, 这里选择 Python。Python 项目依赖和构建工具经历过 venv, Poetry, PDM 之后，基本确立了 uv 的领导
地位。所以自然也就选择用 uv 来创建 Python 项目

```bash
uv init --lib langchain-study
cd langchain-study
uv add langchain
```

增加 `langchain` 后，会添加共 32 个依赖，它们主要有

- langchain == 1.2.15
- langchain-core==1.2.27
- langgraph==1.1.6
- langgraph-sdk==0.3.13
- langchain-sdk==0.3.13
- pydantic==2.12.5

### LangChain 使用本地 Ollama 模型

LangChain 要使用各语言模型的话需要使用相应的集成，见 [LangChain Python integrations](https://docs.langchain.com/oss/python/integrations/providers/overview).
例如，我们这里要使用 Ollama 的话需安装 `langchain-ollam` 依赖

```bash
uv add langchain-ollama
```

它会安装 `langchain-ollama` 和 `ollama` 两个依赖。

首先要准备好 `Ollama`, 本文不介绍它的安装，`Ollama` 启动后会在 `http://127.0.0.1:11434` 上监听，如果要远程访问该 Ollama 服务，需要修改配置并重启。

本人又不得不找来 `4090` 来体验，Ollama 安装在 Ubuntu Linux 上, 修改监听接口的方法如下

```bash
vi /etc/systemd/system/ollama.service
```

然后在 `[Service]` 下加上

```conf
Environment="OLLAMA_HOST=0.0.0.0:11434"
```

或者选定的网络设备，如 `OLLAMA_HOST=192.168.1.100:11434`

再重载并重启服务

```bash
sudo systemctl daemon-reload
sudo systemctl restart ollama
```

这时候 Ollama 就可以从远程进行连接了。

本文打算用 Google 几天前发布的开源模型 `gemma4:26b`，在 Ollama 主机上先下载该模型

```bash
ollama pull gemma4:26b
```

#### 第一个例子 - 使用 ChatOllama

```python
from langchain_ollama import ChatOllama

model = ChatOllama(
    model='gemma4:26b',
    base_url='http://192.168.86.60:11434',
    temperature=0.1,
)

for chunk in model.stream("what's this model, and what can you do?"):
    print(chunk.content, end="", flush=True)
```

`python test1.py` 可以看到在控制台像是一个个单词蹦出来的

```text
python start.py
I am Gemma 4, a large language model developed by Google DeepMind. I am an open weights model designed to process and generate information across different formats.

### What I can do:
* **Text Processing:** I can engage in conversation, answer questions, summarize long documents, write code, translate languages, and assist with creative writing or brainstorming.
* **Image Understanding:** I can analyze and process images that you provide to me, describing their content or answering questions about them.
* **Audio Processing:** If you are interacting with the 2B or 4B versions of the Gemma 4 family, I am also capable of processing audio input.
* **Reasoning and Problem Solving:** I can help with logical reasoning, mathematical problems, and complex instructional tasks.

### What I cannot do:
* **Generate Images:** While I can understand images, I can only generate text as an output.
* **Access the Internet:** I do not have access to Google Search or the live web unless specific tools and endpoints are provided to me in this conversation.
* **Provide Real-time Information:** My knowledge is grounded in data up to January 2025. I do not have information on events that have occurred after that date unless they are provided in our current context.
```

上面的 `chunk` 基本上是对应每一个 Token, 不要想像成与 HTTP 的一个个缓冲大小的块。

#### 第二个例子 -- init_chat_model

```text
from langchain.chat_models import init_chat_model

model = init_chat_model(
    model='ollama:gemma4:26b',
    base_url='http://192.168.86.60:11434',
    temperature=0.1,
)

for chunk in model.stream("what's this model, and what can you do?"):
    print(chunk.content, end="", flush=True)
```

执行效果与前面是一样的，`init_chat_model` 会根据 `model` 中的 `ollama:gemma4:2bb` 前缀 `ollama` 使用相应的 `ChatOllama` 类型。
或者也可以另一种写法来指定 `model_provider`

```python
model = init_chat_model(
    model='gemma4:26b',
    model_provider='ollama',
    base_url='http://192.168.86.60:11434',
    temperature=0.1,
)
```

### 使用远程 Gemini 模型

先安装相应的 Integration 依赖，命令如下

```bash
uv add langchain-google-gemini
```

#### 使用 ChatGoogleGenerativeAI

```python
from langchain_google_genai import ChatGoogleGenerativeAI

model = ChatGoogleGenerativeAI(
    model="gemini-2.5-flash",
    google_api_key="<YOUR_API_KEY>",
)

for chunk in model.stream("what's this model, and what can you do?"):
    print(chunk.content, end="", flush=True)
```

实际应用中使用 `dotenv` 来加载 `GOOGLE API KEY`.

#### 用统一的 `init_chat_model` 方式

```python
from langchain.chat_models import init_chat_model

model = init_chat_model(
    model='gemini-2.5-flash',
    model_provider='google_genai',
    google_api_key='<YOUR_API_KEY>',
)

for chunk in model.stream("what's this model, and what can you do?"):
    print(chunk.content, end="", flush=True)
```

不用 `model_provider` 的话，在 `model` 中指定 `google_genai:gemini-2.5-flash`.

### 理解 LangChain 与 LLM 的交互

我们回到最初的与 LLM 交互的两行代码行来

```python
for chunk in model.stream("what's this model, and what can you do?"):
    print(chunk.content, end="", flush=True)
```

我们调试一下，这里的 chunk 基本上对应于 LLM 的 token, 在 `print` 行上打个条件为 `len(chunk.content) > 0` 的断点，输出到 `language`
时，查看

`chunk.model_dum_json()` 的内容为

```json
{
  "content" : " language",
  "additional_kwargs" : { },
  "response_metadata" : { },
  "type" : "AIMessageChunk",
  "name" : null,
  "id" : "lc_run--019d6a8d-4273-7560-a583-e45faa379dad",
  "tool_calls" : [ ],
  "invalid_tool_calls" : [ ],
  "usage_metadata" : null,
  "tool_call_chunks" : [ ],
  "chunk_position" : null
}
```

从这里看出 `chunk` 的类型是 `AIMessageChunk`, 它有以上那些属性，从中发现可用来调用工具。

除了 `stream()` 流式交互，还可以进行 `invoke` 操作

```python
response = model.invoke("what's this model, and what can you do?")
print(response.content)
```

即等待从 LLM 拿来所有 token 之后，一次性输出，这会让客户端有一段时间没有任何输出。

类似查看 `response.model_dump_json()` 输出内容如下(content 部分截断了)

```json
{
  "content" : "I am Gemma 4, a large language model developed by Google DeepMind. I am an open weights model ...",
  "additional_kwargs" : { },
  "response_metadata" : {
    "model" : "gemma4:26b",
    "created_at" : "2026-04-08T00:56:00.702767023Z",
    "done" : true,
    "done_reason" : "stop",
    "total_duration" : 6067217225,
    "load_duration" : 162074961,
    "prompt_eval_count" : 27,
    "prompt_eval_duration" : 18294703,
    "eval_count" : 883,
    "eval_duration" : 5556447640,
    "logprobs" : null,
    "model_name" : "gemma4:26b",
    "model_provider" : "ollama"
  },
  "type" : "ai",
  "name" : null,
  "id" : "lc_run--019d6a96-b3c6-7683-a5e7-ce428edc57ea-0",
  "tool_calls" : [ ],
  "invalid_tool_calls" : [ ],
  "usage_metadata" : {
    "input_tokens" : 27,
    "output_tokens" : 883,
    "total_tokens" : 910
  }
}
```

这里还能看到 `response_metadata` 的字段信息。`invoke()` 调用得到的是一个 `AIMessage` 对象。

### 更高级的与 LLM 交互

前面都只是进行简单的对话，现在要加入系统提示词, 比如在系统提示词中要求 LLM 只用中文来回复

```python
from langchain.chat_models import init_chat_model
from langchain.messages import HumanMessage, SystemMessage

model = init_chat_model(
    model='gemma4:26b',
    model_provider='ollama',
    base_url='http://192.168.86.60:11434',
    temperature=0.1,
)

system_msg = SystemMessage(content="You are a helpful assistant, and only reply in Chinese")
human_msg = HumanMessage("what's this model, and what can you do?")

for chunk in model.stream([system_msg, human_msg]):
    print(chunk.content, end="", flush=True)
```

执行后，虽然问题是英文，但是回复确实用了中文

```text
我是一个由 Google 训练的大型语言模型。

我可以为你提供多种功能和帮助，具体包括：

1.  **回答问题**：无论是科学、历史、地理、文化还是日常生活中的各种百科知识，你都可以向我提问。
2.  **文字创作**：我可以帮你写邮件、博客文章、故事、诗歌、论文、工作总结，甚至是剧本。
3.  **语言翻译**：我可以在多种语言之间进行流畅的翻译，帮助你理解外语内容或进行跨语言交流。
4.  **编程辅助**：我可以编写代码、解释复杂的编程概念、查找代码中的错误（Debug）以及提供算法建议。
5.  **内容总结**：如果你有一篇很长的文章或文档，我可以帮你提取核心观点，快速生成摘要。
6.  **逻辑推理与数学**：我可以协助你解决数学难题，进行逻辑分析，或者处理复杂的逻辑推理任务。
7.  **创意构思**：如果你需要寻找灵感（例如起名字、策划活动、寻找礼物建议或进行头脑风暴），我也可以参与讨论。
8.  **数据处理**：我可以帮你整理信息、提取结构化数据或将乱序的文本转换成表格格式。

简单来说，你可以把我当作一个知识渊博、多才多艺且随时待命的智能助手。请问今天有什么我可以帮你的吗？
```

以下方式与使用 `SystemMessage` 和 `HumanMessage` 是等效的

```python
messages = [
    ("system", "You are a helpful assistant, and only reply in Chinese"),
    ("human", "what's this model, and what can you do?"),
]
for chunk in model.stream(messages):
    print(chunk.content, end="", flush=True)
```

#### 写一个能交互的 Agent

有了上一节的 `SystemMessage` 和 `HumanMessage` 的基础，并且把 LLM 每次回复添加到 `message` 中，再加入新的问话，这样就实现了一个有短期
记忆的对话。

##### 先实现一个简单的收集两个数相加的 Agent

```python
from typing import Any

from langchain.chat_models import init_chat_model
from langchain.messages import HumanMessage, SystemMessage

model = init_chat_model(
    model='gemma4:26b',
    model_provider='ollama',
    base_url='http://192.168.86.60:11434',
    temperature=0.1,
)

messages: list[Any] = [
    SystemMessage(content="You are a helpful assistant. Ask the user for two numbers, then add them and output flag 'DONE'"),
]

while True:
    response = model.invoke(messages)
    messages.append(response)

    if 'DONE' in response.content:  # 这里是看到标志 `DONE` 结束对话
        print(response.content.removesuffix("DONE"))
        break
    else:
        print(response.content)
        user_input = input("\nYou: ")
        messages.append(HumanMessage(user_input))
```

执行效果

```bash
$ python agent1.py
Please provide two numbers.

You: 23423
Please provide the second number.

You: 344
::: 23767 DONE
$ python start.py
Please provide two numbers.

You: 1235.222
I have received 1235.222. Please provide the second number.

You: 35232.888
36468.11
$
```

加法操作也是由 LLM 完成的。

##### 加入 tools 调用

```python
from typing import Any

from langchain.chat_models import init_chat_model
from langchain.messages import HumanMessage, SystemMessage, ToolMessage
from langchain.tools import tool

model = init_chat_model(
    model='gemma4:26b',
    model_provider='ollama',
    base_url='http://192.168.86.60:11434',
    temperature=0.1,
)

@tool
def add_numbers(a: float, b: float) -> float:
    """Add two numbers together and return the result."""
    return a + b

model_with_tools = model.bind_tools([add_numbers])

messages: list[Any] = [
    SystemMessage(content="You are a helpful assistant. Ask the user for two numbers, then add them."),
]

while True:
    response = model_with_tools.invoke(messages)
    messages.append(response)

    if response.tool_calls:
        for tool_call in response.tool_calls:
            tool_fn = globals()[tool_call["name"]]
            result = tool_fn.invoke(tool_call["args"])
            messages.append(ToolMessage(content=str(result), tool_call_id=tool_call["id"]))

        final = model_with_tools.invoke(messages)
        print(final.content)
        break
    else:
        print(response.content)
        user_input = input("\nYou: ")
        messages.append(HumanMessage(user_input))
```

执行效果

```bash
$ python agent2.py
I'd be happy to help you with that! Please tell me the two numbers you would like me to add.

You: abc
I'm sorry, but "abc" isn't a number. Please provide two numbers that you would like me to add together.

You: 234.435
I have the first number: 234.435. What is the second number you would like me to add to it?

You: 2366
The sum of 234.435 and 2366 is 2600.435.
$
```

`@tool` 方法的注释部分 `"""Add two numbers together and return the result."""`  是必须的，否则会出错，因为 `LLM` 需要 `@tool`
方法的描述来判断何时调用该工具。

如果我们用 `WireShark` 那样的网络抓包工具，可以看到 `model_with_tools.invoke(messages)` 交互时发送给 LLM 的消息如下

```json
{
   "model":"gemma4:26b",
   "stream":true,
   "options":{
      "temperature":0.1
   },
   "messages":[
      {
         "role":"system",
         "content":"You are a helpful assistant. Ask the user for two numbers, then add them."
      }
   ],
   "tools":[
      {
         "type":"function",
         "function":{
            "name":"add_numbers",
            "description":"Add two numbers together and return the result.",
            "parameters":{
               "type":"object",
               "required":[
                  "a",
                  "b"
               ],
               "properties":{
                  "a":{
                     "type":"number"
                  },
                  "b":{
                     "type":"number"
                  }
               }
            }
         }
      }
   ]
}
```

这里是一个捕获的 LangChain Agent 与 LLM 完整的交互过程，点击 [LangChain-LLM-Interact](langchain-llm-interact.txt) 查看内容。

从该本件中又发现了新的关于 `thinking` 推理的内容，比如第一个响应中

```json
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.252247901Z","message":{"role":"assistant","content":"","thinking":"The"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.258806725Z","message":{"role":"assistant","content":"","thinking":" user"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.265426972Z","message":{"role":"assistant","content":"","thinking":" wants"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.271957077Z","message":{"role":"assistant","content":"","thinking":" me"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.278387845Z","message":{"role":"assistant","content":"","thinking":" to"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.284987335Z","message":{"role":"assistant","content":"","thinking":" ask"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.291512131Z","message":{"role":"assistant","content":"","thinking":" for"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.298095059Z","message":{"role":"assistant","content":"","thinking":" two"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.304594988Z","message":{"role":"assistant","content":"","thinking":" numbers"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.311656028Z","message":{"role":"assistant","content":"","thinking":" and"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.318633198Z","message":{"role":"assistant","content":"","thinking":" then"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.325672891Z","message":{"role":"assistant","content":"","thinking":" add"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.332701596Z","message":{"role":"assistant","content":"","thinking":" them"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.339854282Z","message":{"role":"assistant","content":"","thinking":"."},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.353821264Z","message":{"role":"assistant","content":"","thinking":"\nI"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.360968841Z","message":{"role":"assistant","content":"","thinking":" should"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.368507877Z","message":{"role":"assistant","content":"","thinking":" start"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.375167262Z","message":{"role":"assistant","content":"","thinking":" by"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.381718679Z","message":{"role":"assistant","content":"","thinking":" asking"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.388331903Z","message":{"role":"assistant","content":"","thinking":" the"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.394845723Z","message":{"role":"assistant","content":"","thinking":" user"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.401266018Z","message":{"role":"assistant","content":"","thinking":" for"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.40765756Z","message":{"role":"assistant","content":"","thinking":" the"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.4140507Z","message":{"role":"assistant","content":"","thinking":" first"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.42039688Z","message":{"role":"assistant","content":"","thinking":" number"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.426744735Z","message":{"role":"assistant","content":"","thinking":"."},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.439521087Z","message":{"role":"assistant","content":"Please"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.445929201Z","message":{"role":"assistant","content":" provide"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.452233804Z","message":{"role":"assistant","content":" the"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.458677447Z","message":{"role":"assistant","content":" first"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.465064535Z","message":{"role":"assistant","content":" number"},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T01:45:30.471438575Z","message":{"role":"assistant","content":"."},"done":false}
```

在输出 `Please provide the first number.` 之前其实进行了 `thinking`

```text
The user wants me to ask for two numbers and then add them.
I should start by asking the user for the first number.
```

#### 加上交互时的 Thinking 内容显示

最后用 `Claude Code` 来完成的在对话中同时显示 `thinking` 的内容，用了 `AI` 好像又没学到实质性的东西. 完整代码如下

```python
from typing import Any

from langchain.chat_models import init_chat_model
from langchain.messages import AIMessage, HumanMessage, SystemMessage, ToolMessage
from langchain.tools import tool

model = init_chat_model(
    model="gemma4:26b",
    model_provider="ollama",
    base_url="http://192.168.86.60:11434",
    temperature=0.1,
    reasoning=True,
)


@tool
def add_numbers(a: float, b: float) -> float:
    """Add two numbers together and return the result."""
    return a + b


model_with_tools = model.bind_tools([add_numbers])

messages: list[Any] = [
    SystemMessage(content="You are a helpful assistant. Ask the user for two numbers, then add them."),
]


def stream_response(messages: list[Any]) -> AIMessage:
    aggregated = None
    in_thinking = False

    for chunk in model_with_tools.stream(messages):
        thinking = chunk.additional_kwargs.get("reasoning_content", "")
        content = chunk.content

        if thinking:
            if not in_thinking:
                print("\033[90m[Thinking]: ", end="", flush=True)
                in_thinking = True
            print(thinking, end="", flush=True)

        if content:
            if in_thinking:
                print("\033[0m")
                in_thinking = False
            print(content, end="", flush=True)

        aggregated = chunk if aggregated is None else aggregated + chunk

    if in_thinking:
        print("\033[0m")
    print()

    return aggregated


while True:
    response = stream_response(messages)
    messages.append(response)

    if response.tool_calls:
        for tool_call in response.tool_calls:
            tool_fn = globals()[tool_call["name"]]
            result = tool_fn.invoke(tool_call["args"])
            messages.append(ToolMessage(content=str(result), tool_call_id=tool_call["id"]))

        final = stream_response(messages)
        messages.append(final)
        break
    else:
        user_input = input("You: ")
        messages.append(HumanMessage(user_input))
```

并且该代码改成了流式处理，所以在控制台下可以看到 `Thinking` 和实际回复的内容是一个一个字蹦出来的，`Thinking` 内容用灰色显示

{{< bundle-image langchain-get-started-1.png 830 >}}

这是目前为止体验到的 LangChain 简单而强大的创建 AI Agent 的功能。