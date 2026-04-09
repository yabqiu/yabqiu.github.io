---
title: "LangChain Quick Start AI Agent 分析"
url: /langchain-quickstart-agent-analysis/
date: 2026-04-08T20:06:48-05:00
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
comment: true
codeMaxLines: 100
lastmod:
---

上一篇 [LangChain 1.2 入门学习](/langchain-1-2-get-started/) 中使用了特定的 ChatOllama/ChatGoogleGenerativeAI, 或统一的
init_chat_model 方式创建一个与 LLM 的交互程序，并以此基础实现了一个简单的 AI Agent, 工具调用是根据响应中消息格式手工调用的，
会话处理是自己拼接全部对话历史。现在知名的 LLM 都支持推理，所以顺代也用灰色字体显示了 'thinking' 的文本内容。

本文直接学习官方 [Quickstart](https://docs.langchain.com/oss/python/langchain/quickstart) 中的一个 `AI Agent`, 该 Agent
实现了以下功能

1. 有了上下文，会记忆会话历史
2. 支持调用多个工具
3. 提供结构化一致性的响应数据格式
4. 能通过上下文处理用户特定的信息
5. 跨交互维护会话状态
<!--more-->

写作本篇测试时用的是本地的 `ollama:gemma4:26b` 的模型

### 完整 AI Agent 代码

```python {linenos=table}
from dataclasses import dataclass

from langchain.agents import create_agent
from langchain.chat_models import init_chat_model
from langchain.tools import tool, ToolRuntime
from langgraph.checkpoint.memory import InMemorySaver
from langchain.agents.structured_output import ToolStrategy
from langchain_core.runnables import RunnableConfig
from pydantic import BaseModel


# Define system prompt
SYSTEM_PROMPT = """You are an expert weather forecaster, who speaks in puns.

You have access to two tools:

- get_weather_for_location: use this to get the weather for a specific location
- get_user_location: use this to get the user's location

If a user asks you for the weather, make sure you know the location. If you can tell from the question
that they meanwherever they are, use the get_user_location tool to find their location."""

# Define context schema
class Context(BaseModel):
    """Custom runtime context schema."""
    user_id: str

# Define tools
@tool
def get_weather_for_location(city: str) -> str:
    """Get weather for a given city."""
    return f"It's always sunny in {city}!"

@tool
def get_user_location(runtime: ToolRuntime[Context]) -> str:
    """Retrieve user information based on user ID."""
    user_id = runtime.context.user_id
    return "Florida" if user_id == "1" else "SF"

# Configure model
model = init_chat_model(
    model='ollama:gemma4:26b',
    base_url='http://192.168.86.60:11434',
    temperature=0.1
)

# Define response format
@dataclass
class ResponseFormat:
    """Response schema for the agent."""
    # A pun response (always required)
    pun_response: str
    # Any interesting information about the weather if available
    weather_conditions: str | None = None

# Set up memory
checkpointer = InMemorySaver()

# Create agent
agent = create_agent(
    model=model,
    system_prompt=SYSTEM_PROMPT,
    tools=[get_user_location, get_weather_for_location],
    context_schema=Context,
    response_format=ToolStrategy(ResponseFormat),
    checkpointer=checkpointer
)

# `thread_id` is a unique identifier for a given conversation.
config: RunnableConfig = {"configurable": {"thread_id": "1"}}

response = agent.invoke(
    {"messages": [{"role": "user", "content": "what is the weather outside?"}]},
    config=config,
    context=Context(user_id="1")
)

print(response['structured_response'])
# ResponseFormat(
#     pun_response="It's looking absolutely sun-sational in Florida! You might want to grab some shades, because the forecast is looking bright!"",
#     weather_conditions="Sunny"
# )


# Note that we can continue the conversation using the same `thread_id`.
response = agent.invoke(
    {"messages": [{"role": "user", "content": "thank you so much!"}]},
    config=config,
    context=Context(user_id="1")
)

print(response['structured_response'])
# ResponseFormat(
#     pun_response="You're very welcome! I'm always happy to help brighten your day!",
#     weather_conditions=None
# )
```

本 Agent 中用 LangChain 的 `create_agent()` 方法创建一个 Agent, 并没有用流式来输出 LLM 过来的响应，而是用 `agent.invoke()`
进行了两次调用，两个问题分别是

1. what is the weather outside?
2. thank you so much!

### 执行 AI Agent

两个 `print(response['structured_response'])` 对应的输出分别为

> ResponseFormat(pun_response="It's looking absolutely sun-sational in Florida! You might want to grab some shades, 
> because the forecast is looking bright!", weather_conditions='Sunny')

> ResponseFormat(pun_response="You're very welcome! I'm always happy to help brighten your day!", weather_conditions=None)

执行过程中连接到一个刚用 `Vibe Coding` 写的一个简单的 HTTP 代理程序，[simple-http-proxy](https://github.com/yabqiu/simple-http-proxy),
它可用来捕获与 Ollama API 的所有请求与响应。在 Python 程序端只要配置环境变量

```bash
export http_proxy=http://127.0.0.1:9090
```

或者把 `http_proxy=http://127.0.0.1:9090` 放在 `.env` 文件中，由 `dotenv.load_dotenv()` 来加载也行。

执行后由代理记录下该 `Agent` 与 `Ollama` 的四个请求响应来回的数据，请点击 [langchain-agent-ollama.txt](langchain-agent-ollama.txt)
查看。后面的分析中会引用其中的内容，注意，响应数据中擦除了所有 'thinking' 的内容。

### 代码分析

我们对照代码与请求响应数据的内容对代码进行分析。

#### 请求消息的组成

定义的 `SYSTEM_PROMPT` 在 `create_agent()` 时用 `system_prompt` 参数指定，它会在请求 `messages` 中作为第一条 `role` 为 `system`
的内容

```json
{
  "messages": [
    {
      "role": "system",
      "content": "You are an expert weather forecaster, who speaks in puns..."
    }
  ]
}
```

与模型交互时，`system_prompt`, `tools`, 和 `agent.invoke()` 时的 `input` 才是传递给模型的内容，它们最终与请求的模型名称，模型参数组成
一个输入. `tools` 中的函数是 `@tools` 装饰并用在 `create_agent()` 时用 `tools` 参数指定的列表。LangChain 读取了 @tools 函数的名称，
文档注释和参数类型。我们看第一次 `agent.invoke()` 它会完成两次与 LLM 的交互, 分别实现 `get_user_location` 与 `get_weather_for_location` 
工具的调用。第一次请求时完整的内容(这时格式化后的内容)

```json
{
  "model": "gemma4:26b",
  "stream": true,
  "options": {
    "temperature": 0.1
  },
  "messages": [
    {
      "role": "system",
      "content": "You are an expert weather forecaster, who speaks in puns.\n\nYou have access to two tools:\n\n- get_weather_for_location: use this to get the weather for a specific location\n- get_user_location: use this to get the user's location\n\nIf a user asks you for the weather, make sure you know the location. If you can tell from the question\nthat they meanwherever they are, use the get_user_location tool to find their location."
    },
    {
      "role": "user",
      "content": "what is the weather outside?"
    }
  ],
  "tools": [
    {
      "type": "function",
      "function": {
        "name": "get_user_location",
        "description": "Retrieve user information based on user ID.",
        "parameters": {
          "type": "object",
          "properties": {}
        }
      }
    },
    {
      "type": "function",
      "function": {
        "name": "get_weather_for_location",
        "description": "Get weather for a given city.",
        "parameters": {
          "type": "object",
          "required": [
            "city"
          ],
          "properties": {
            "city": {
              "type": "string"
            }
          }
        }
      }
    },
    {
      "type": "function",
      "function": {
        "name": "ResponseFormat",
        "description": "Response schema for the agent.",
        "parameters": {
          "type": "object",
          "required": [
            "pun_response"
          ],
          "properties": {
            "pun_response": {
              "type": "string"
            },
            "weather_conditions": {}
          }
        }
      }
    }
  ]
}
```

`system_prompt` 的 `role` 是 `system`, 用户的 `role` 是 `user`, 后面会看到 AI 响应内容的 `role` 是 `assistant`. 提示词中包含了工具
列表，`ResponseFormat` 也是以工具函数的方式存在，`LLM` 会找到它对应的格式参数，从而可用 `response["structured_response"]`
获得预期的格式输出。`Context(user_id: str)` 的信息只是存在于本地，为本地的工具函数所用， 从中获得 `user_id` 来管理上下文。

#### 第一条响应数据

我们再来看请求 Ollama 的 `/api/chat` 后第一条消息的内容, 它是一个 `Line-Delimited JSON`, 即数据由一致多行组成，每一行是一个 JSON, 
又称 NDJSON, 所以响应的 `Content-Type` 是 `application/x-ndjson`.

{{< highlight-wrap json >}}
{"model":"gemma4:26b","created_at":"2026-04-08T22:58:27.002360557Z","message":{"role":"assistant","content":"","tool_calls":[{"id":"call_detex1l0","function":{"index":0,"name":"get_user_location","arguments":{}}}]},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T22:58:27.009025523Z","message":{"role":"assistant","content":""},"done":true,"done_reason":"stop","total_duration":840090124,"load_duration":157000665,"prompt_eval_count":255,"prompt_eval_duration":18281146,"eval_count":99,"eval_duration":624183208}
{{< /highlight-wrap >}}

这是两个 JSON, 第一行通知客户端调用工具 `get_user_location`. AI 回复的 `role` 是 `assistant`. 第一个 JSON 的 `done` 是 `false`,
第二个 JSON 的 `done` 是 `true`, 这说明第一次回复还没有完成，第二次回复才完成了。第一次回复的内容是在请求客户端作一个工具调用。

#### 自动的会话记忆

看第二个请求的内容

```json
{
  "model": "gemma4:26b",
  "stream": true,
  "options": {
    "temperature": 0.1
  },
  "messages": [
    {
      "role": "system",
      "content": "You are an expert weather forecaster, who speaks in puns.\n\nYou have access to two tools:\n\n- get_weather_for_location: use this to get the weather for a specific location\n- get_user_location: use this to get the user's location\n\nIf a user asks you for the weather, make sure you know the location. If you can tell from the question\nthat they meanwherever they are, use the get_user_location tool to find their location."
    },
    {
      "role": "user",
      "content": "what is the weather outside?"
    },
    {
      "role": "assistant",
      "tool_calls": [
        {
          "function": {
            "name": "get_user_location",
            "arguments": {}
          }
        }
      ]
    },
    {
      "role": "tool",
      "content": "Florida"
    }
  ],
  "tools": [ "<与上同，此处省略>" ]
}
```

我们并没有拼接 `message`, 就因为我们在 `create_agent()` 是指定了 `checkpointer=checkpointer`, 即 `InMemorySaver`, 
这种拼接会话的功能变成自动的了。上面的 `messages`， 在最初的

>"role": "system", "content": "&lt;system prompt&gt;"<br/>
>"role": "user", "content": "what is the weather outside?"

的基础上，加上了第一次回复的内容

>"role": "assistant", "tool_calls": \[{"function": { "name": "get_user_location", "arguments": {}}}]

再加上新的工具调用的结果的内容

>"role": "tool", "content": "Florida"

后面的请求也是由 `InMemorySaver` 持续的添加内容。

#### 结构化响应数据

在调用完了工具 `get_user_location` 和 `get_weather_for_location` 后，我们得到了最终的回复，这时候的回复内容是

{{< highlight-wrap json >}}
{"model":"gemma4:26b","created_at":"2026-04-08T22:58:29.288326047Z","message":{"role":"assistant","content":"","tool_calls":[{"id":"call_maio2ggd","function":{"index":0,"name":"ResponseFormat","arguments":{"pun_response":"It's looking absolutely sun-sational in Florida! You might want to grab some shades, because the forecast is looking bright!","weather_conditions":"Sunny"}}}]},"done":false}
{"model":"gemma4:26b","created_at":"2026-04-08T22:58:29.294894197Z","message":{"role":"assistant","content":""},"done":true,"done_reason":"stop","total_duration":1364325806,"load_duration":122615522,"prompt_eval_count":312,"prompt_eval_duration":22412919,"eval_count":184,"eval_duration":1153149215}
{{< /highlight-wrap >}}

这时候要求我们再调用工具 `ResponseFormat`, 并且参数是 LLM 按照 `ResponseFormat` 的参数要求的内容

```json
{
  "pun_response": "It's looking absolutely sun-sational in Florida! You might want to grab some shades, because the forecast is looking bright!",
  "weather_conditions": "Sunny"
}
```

Agent 端实际调用一下 `ResponseFormat` 工具函数就都到我们想要的结构化响应数据了，即 `response['structured_response']` 的内容。

### 转换成 stream() 方式并与用户实时交互

官方 Quickstart 的代码是用 `invoke()` 非流式的交互，并且用户问题也是预设好的。

我们可以把它改成流式的交互方式，并且用户问题也通过 `input()` 来获取，这样就可以与用户实时交互了。启动 `Claude Code`, 只需要改动

```python
config: RunnableConfig = {"configurable": {"thread_id": "1"}}
```

后的代码为如下

```python
def stream_agent(messages: list[dict]) -> None:
    for chunk in agent.stream(
        {"messages": messages},
        config=config,
        context=Context(user_id="1"),
        stream_mode="updates",
    ):
        for node, update in chunk.items():
            print(f"--- [{node}] ---")
            if "structured_response" in update:
                print(update["structured_response"])
            elif "messages" in update:
                for msg in update["messages"]:
                    print(f"  {type(msg).__name__}: {msg.content or msg.tool_calls}")

while True:
    user_input = input("You: ").strip()
    if not user_input:
        continue
    stream_agent([{"role": "user", "content": user_input}])
```

执行时交互效果如下

```bash
python src/langchain_study/agent3.py
You: what is the weather outside?
--- [model] ---
  AIMessage: [{'name': 'get_user_location', 'args': {}, 'id': 'ca4dd0f4-904d-4603-9a75-6dbfe281e1b3', 'type': 'tool_call'}]
--- [tools] ---
  ToolMessage: Florida
--- [model] ---
  AIMessage: [{'name': 'get_weather_for_location', 'args': {'city': 'Florida'}, 'id': 'f479ecff-2444-4841-bc5d-a6269039d18e', 'type': 'tool_call'}]
--- [tools] ---
  ToolMessage: It's always sunny in Florida!
--- [model] ---
ResponseFormat(pun_response="It's looking bright and sunny in Florida! I'm feeling quite *bright* about this forecast!", weather_conditions='Sunny')
You: thank you so much!
Deserializing unregistered type __main__.ResponseFormat from checkpoint. This will be blocked in a future version. Add to allowed_msgpack_modules to silence: [('__main__', 'ResponseFormat')]
--- [model] ---
  AIMessage: You're *weather* welcome! Don't hesitate to reach out if you need another *forecast*!
You:
```

继续丰富的话，可以自定义 `/` 命令，如 `/clear` 清理会话，`/compact` 压缩会话, `/model` 切换模型等。

而且从这里也看到与不同的 role 'tool', 'assistant' 对应的 `ToolMessage`, `AIMessage`. 而另外两个 role `user` 和 `system` 
对应的类没列出，它们分别是 `HumanMessage` 和 `SystemMessage`. 它们的基类都是 `BaseMessage`, 除了前面四个实现类外，还有 `RemoveMessage`,
`ChatMessage`, 和 `FunctionMessage`.