---
title: "LangChain 高级用法之 MCP"
url: /langchain-advanced-usage-mcp/
date: 2026-04-22T14:39:48-05:00
featured: false
draft: true
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

`LangChain` 1.0 于 2025 年 10 月 22 日发布，这是一个里程碑式的版本，听说在 0.x 要创建一个 `agent` 很麻烦， 那时候内部是真正的 `链`，
1.0 后虽然还叫 `LangChain`, 实际上内部实现是图(`LangGraph`), 用 `create_agent()` 创建 `agent`. 从数据结构来看，`图` 比 `链表`
更能直观的表达 `Agent` 与模型及工具的交互场景。很庆幸在 `LangChain` 1.0 之后才开始学习这个框架，不用体验 `LangChain` 0.x 的痛苦。

大概对 `LangChain` 的 tools 有些许了解之后，现在跳到 `Model Context Protocol(MCP)` 协议这一章，本人对 `MCP` 的初步理解是相对于工具，
`MCP` 是一个远程(跨进程)的工具。为了方便的使用互联网上的各种资源，`MCP` 在实现一个完备的 `Agent` 也是一个非常重要的工具。

[Model Context Protocol (MCP)](https://modelcontextprotocol.io/introduction) 是 `Anthropic` 推出并开放的协议，用于构建
`Agent` 与外部资源的交互,下面会与工具对照着学习它. 以前也写过一篇关于 `MCP` 的文章，今天从不同的角度再次强化对 `MCP` 的理解。

在 `LangChain` 中要使用 `MCP` 需安装 `langchain-mcp-adapters` 依赖，然后使用它的 `MultiServerMCPClient`, 它是无状态的。要创建自己的
`MCP` 服务，使用 [`FastMCP`](https://gofastmcp.com/getting-started/welcome) 库。<!--more-->

```bash
uv add langchain-mcp-adapters
uv add fastmcp   # develop MCP server
```

### 体验 `LangChain` 和 `MCP`

先来体验一下 `MCP` 的能力，创建三个 Python 代码文件，分别是

1. math_server.py:  通过 `stdio`(标准输入输出) 使用的 `MCP` 服务
2. weather_server.py: 通过 `http` 使用的 `MCP` 服务
3. mcp_client_agent.py: 使用以上两个 `MCP` 服务的 `Agent`, 它也是一个 `MCP` 客户端

#### math_server.py

```python
import sys

from fastmcp import FastMCP

mcp = FastMCP("Math")

@mcp.tool()
def add(a: int, b: int) -> int:
    """Add two numbers"""
    print(f"math:add called with {a=} and {b=}", file=sys.stderr)
    return a + b

@mcp.tool()
def multiply(a: int, b: int) -> int:
    """Multiply two numbers"""
    print(f"math:multiply called with {a=} and {b=}", file=sys.stderr)
    return a * b

if __name__ == "__main__":
    mcp.run(transport="stdio", show_banner=False)
```

#### weather_server.py

```python
from fastmcp import FastMCP

mcp = FastMCP("Weather")

@mcp.tool()
async def get_weather(location: str) -> str:
    """Get weather for location."""
    print(f"Weather:get_weather called with {location=}")
    return "It's always sunny in New York"

if __name__ == "__main__":
    mcp.run(transport="streamable-http", show_banner=False)
```

#### mcp_client_agent.py

```python
import asyncio
from langchain_mcp_adapters.client import MultiServerMCPClient
from langchain.agents import create_agent
from pathlib import Path


async def main():
    math_server_path = str((Path(__file__).resolve().parent / "math_server.py"))

    client = MultiServerMCPClient(
        {
            "math": {
                "transport": "stdio",  # Local subprocess communication
                "command": "python",
                # Absolute path to your math_server.py file
                "args": [math_server_path],
            },
            "weather": {
                "transport": "http",  # HTTP-based remote server
                # Ensure you start your weather server on port 8000
                "url": "http://localhost:8000/mcp",
            }
        }
    )

    tools = await client.get_tools()
    agent = create_agent(
        "ollama:gemma4:e4b",
        tools=tools,
    )
    math_response = await agent.ainvoke(
        {"messages": [{"role": "user", "content": "what's (3 + 5) x 12?"}]}
    )

    for message in math_response["messages"]:
        message.pretty_print()

    weather_response = await agent.ainvoke(
        {"messages": [{"role": "user", "content": "what is the weather in nyc?"}]}
    )

    for message in weather_response["messages"]:
        message.pretty_print()

if __name__ == "__main__":
    asyncio.run(main())
```


先要启动 `http` 模式的 `weather_server.py`

```bash
uv run python src/langchain_study/mcp/weather_server.py
[04/22/26 16:02:46] INFO     Starting MCP server 'Weather' with transport 'streamable-http' on http://127.0.0.1:8000/mcp                                                                                                            transport.py:301
INFO:     Started server process [99625]
INFO:     Waiting for application startup.
INFO:     Application startup complete.
INFO:     Uvicorn running on http://127.0.0.1:8000 (Press CTRL+C to quit)
```

然后执行 `mcp_client_agent.py`

```bash
uv run python src/langchain_study/mcp/mcp_client_agent.py 
[04/22/26 16:06:20] INFO     Starting MCP server 'Math' with transport 'stdio'                                                                                                                                                      transport.py:209
[04/22/26 16:06:39] INFO     Starting MCP server 'Math' with transport 'stdio'                                                                                                                                                      transport.py:209
[04/22/26 16:06:39] INFO     Starting MCP server 'Math' with transport 'stdio'                                                                                                                                                      transport.py:209
math:multiply called with a=8 and b=12
math:add called with a=3 and b=5
================================ Human Message =================================

what's (3 + 5) x 12?
================================== Ai Message ==================================
Tool Calls:
  add (497dc421-b912-4664-82a4-e7ca8cfbc5e2)
 Call ID: 497dc421-b912-4664-82a4-e7ca8cfbc5e2
  Args:
    a: 3
    b: 5
================================= Tool Message =================================
Name: add

[{'type': 'text', 'text': '8', 'id': 'lc_d3ef66f9-d7ba-4aeb-86fb-2548bd43d9bf'}]
================================== Ai Message ==================================
Tool Calls:
  multiply (f242adcd-d1a1-4687-be75-a0a307ca32fe)
 Call ID: f242adcd-d1a1-4687-be75-a0a307ca32fe
  Args:
    a: 8
    b: 12
================================= Tool Message =================================
Name: multiply

[{'type': 'text', 'text': '96', 'id': 'lc_dc22f392-4fb2-45a4-a57a-e00a5de2d960'}]
================================== Ai Message ==================================

96
================================ Human Message =================================

what is the weather in nyc?
================================== Ai Message ==================================
Tool Calls:
  get_weather (cc8dd3bd-cb54-4856-808a-1f3979fda25c)
 Call ID: cc8dd3bd-cb54-4856-808a-1f3979fda25c
  Args:
    location: nyc
================================= Tool Message =================================
Name: get_weather

[{'type': 'text', 'text': "It's always sunny in New York", 'id': 'lc_aa46f91e-37a9-48bf-8959-558d233a44fe'}]
================================== Ai Message ==================================
Tool Calls:
  get_weather (6d8a8492-22eb-4a72-b8f2-3f82194276e6)
 Call ID: 6d8a8492-22eb-4a72-b8f2-3f82194276e6
  Args:
    location: nyc
================================= Tool Message =================================
Name: get_weather

[{'type': 'text', 'text': "It's always sunny in New York", 'id': 'lc_07da67af-1ce3-474e-9199-8b7be3a6769e'}]
================================== Ai Message ==================================

```

从这个输出对话，可见它与使用普通工具是完全一样的交互过程, 也是下面几个过程

- HumanMessage: 告诉模型有哪个工具可调用
- AIMessage: 告诉客户端应如何调用工具，工具名及参数列表
- ToolMessage: 客户端调用工具，把结果用 ToolMessage 回送给模型
- AIMessage: 模型根据工具调用的结果继续对话

只不过用 `MCP` 时，工具调用是跨进程，由启动的子进程通过 `stdio` 交互，或与远程的 `HTTP` 交互，背后的交互协议是 `JSON-RPC` 2.0.

在 `math_server` 中看到 `print()` 到标准错误输出的内容，以及 `http` 时看到 `weather_server` 中的控制台输出。

> math:multiply called with a=8 and b=12<br/>
> math:add called with a=3 and b=5<br/>
> Weather:get_weather called with location='nyc'

### 剖析上面的 `MCP`

`math_server` 和 `weather_server` 中的实现与启动差不多，`mcp.run()` 时，`transport` 参数的选择有 `http`, `stdio`, `streamable-http`,
`sse(Sever Send Event)`. 对于 `factmcp`, `http` 和 `streamable-http` 是一样的的。 本例用了 `stdio` 和 `streamable-http` 两种方式。
特别要注意的是，对于 `stdio` 交互方式是通过标准的输入与输出，如果在 `stdio` 的 `@mcp.tool()` 装饰的工具函数中用了往标准输出打印的信息，
如 `print("hello")` 将把把该输出作为工具调用的返回而造成异常。

`stdio` 模型的 `MCP` 要通过 `command` 和 `args` 来告诉 `MultiServerMCPClient` 如何启动该 `MCP` 子进程，启动在 `MCP`
客户端与服务端之间通过标准输入与输出交互，所以不支持并发。试想多个线程或进程住同一进程的标准输入发送内容，将会造成混乱。

#### MulltiServerMCPClient 是异步

由于 `MultiServerMCPClient` 是异步，因此，启动源头就要用 `asyncio.run(main())` 来执行，外层是 `async`, 可以调用 `agent` 的相应带 `a`
前缀的异步方法 `await agent.ainvoke()`.

#### 收集工具函数

什么说本质上 `MCP` 也是工具调用，在 `agent = create_agent()` 行打个断点查看 `tools = await client.get_tools()` 的内容为

{{< highlight-wrap "text" >}}
[
    StructuredTool(name='add', description='Add two numbers', args_schema={'additionalProperties': False, 'properties': {'a': {'type': 'integer'}, 'b': {'type': 'integer'}}, 'required': ['a', 'b'], 'type': 'object'}, metadata={'_meta': {'fastmcp': {'tags': []}}}, response_format='content_and_artifact', coroutine=<function convert_mcp_tool_to_langchain_tool.<locals>.call_tool at 0x10c35b060>),
    StructuredTool(name='multiply', description='Multiply two numbers', args_schema={'additionalProperties': False, 'properties': {'a': {'type': 'integer'}, 'b': {'type': 'integer'}}, 'required': ['a', 'b'], 'type': 'object'}, metadata={'_meta': {'fastmcp': {'tags': []}}}, response_format='content_and_artifact', coroutine=<function convert_mcp_tool_to_langchain_tool.<locals>.call_tool at 0x10c35a520>),
    StructuredTool(name='get_weather', description='Get weather for location.', args_schema={'additionalProperties': False, 'properties': {'location': {'type': 'string'}}, 'required': ['location'], 'type': 'object'}, metadata={'_meta': {'fastmcp': {'tags': []}}}, response_format='content_and_artifact', coroutine=<function convert_mcp_tool_to_langchain_tool.<locals>.call_tool at 0x10c2aa3e0>)
]
{{< /highlight-wrap >}}

`tools` 和用如下常规用 `from langchain.tools import tool` 装饰的函数类型是一样的，都是 `StructuredTool`，

```python
from langchain.tools import tool

@tool
def add(a: int, b: int) -> int:
    """Add two numbers"""
    return a + b
```

也就是说 `LangChain`把 `MCP` tools 转换为 `LangChain` 的 tools.

每个工具有相应的 `coroutine`, 即实际对应的执行函数. 使用 `MCP` 时的工具函数的描述是由 `MultiServerMCPClient` 通过 `stdio` 或 `http` 获取的。

比如下面的方式可以从 `streamable-http` 的 `MCP` 上获取工具函数的描述：

{{< highlight-wrap "bash" >}}
# 先获得 mcp-session-id
curl -i 'http://localhost:8000/mcp' \
-H 'Accept: application/json, text/event-stream' \
-H 'Content-Type: application/json' \
--data '{"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"mcp","version":"0.1.0"}},"jsonrpc":"2.0","id":0}'
HTTP/1.1 200 OK
date: Wed, 22 Apr 2026 21:38:52 GMT
server: uvicorn
cache-control: no-cache, no-transform
connection: keep-alive
content-type: text/event-stream
mcp-session-id: 9b5fd6e4c7804ca69bf3f970da21cbf7
x-accel-buffering: no
Transfer-Encoding: chunked

event: message
data: {"jsonrpc":"2.0","id":0,"result":{"protocolVersion":"2025-11-25","capabilities":{"experimental":{},"logging":{},"prompts":{"listChanged":true},"resources":{"subscribe":false,"listChanged":true},"tools":{"listChanged":true},"extensions":{"io.modelcontextprotocol/ui":{}}},"serverInfo":{"name":"Weather","version":"3.2.4"}}}

# 再由 mcp-session-id 得到工具列表
curl 'http://localhost:8000/mcp' \
-H 'Accept: application/json, text/event-stream' \
-H 'Content-Type: application/json' \
-H 'mcp-session-id: 9b5fd6e4c7804ca69bf3f970da21cbf7' \
--data '{"method":"tools/list","jsonrpc":"2.0","id":1}'
event: message
data: {"jsonrpc":"2.0","id":1,"result":{"tools":[{"name":"get_weather","description":"Get weather for location.","inputSchema":{"additionalProperties":false,"properties":{"location":{"type":"string"}},"required":["location"],"type":"object"},"outputSchema":{"properties":{"result":{"type":"string"}},"required":["result"],"type":"object","x-fastmcp-wrap-result":true},"_meta":{"fastmcp":{"tags":[]}}}]}}
{{< /highlight-wrap >}}

`stdio` 的 `MCP` Server 也类似，只是交互是通过标准输入输出进行的，使用前面 `post body` 中的内容作为标准输入，见下

{{< highlight-wrap bash "hl_lines=3 5">}}
uv run python src/langchain_study/mcp/math_server.py
[04/22/26 16:45:00] INFO     Starting MCP server 'Math' with transport 'stdio'                                                                                                                                                                              transport.py:209
{"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"mcp","version":"0.1.0"}},"jsonrpc":"2.0","id":0}
{"jsonrpc":"2.0","id":0,"result":{"protocolVersion":"2025-11-25","capabilities":{"experimental":{},"logging":{},"prompts":{"listChanged":false},"resources":{"subscribe":false,"listChanged":false},"tools":{"listChanged":true},"extensions":{"io.modelcontextprotocol/ui":{}}},"serverInfo":{"name":"Math","version":"3.2.4"}}}
{"method":"tools/list","jsonrpc":"2.0","id":1}
{"jsonrpc":"2.0","id":1,"result":{"tools":[{"name":"add","description":"Add two numbers","inputSchema":{"additionalProperties":false,"properties":{"a":{"type":"integer"},"b":{"type":"integer"}},"required":["a","b"],"type":"object"},"outputSchema":{"properties":{"result":{"type":"integer"}},"required":["result"],"type":"object","x-fastmcp-wrap-result":true},"_meta":{"fastmcp":{"tags":[]}}},{"name":"multiply","description":"Multiply two numbers","inputSchema":{"additionalProperties":false,"properties":{"a":{"type":"integer"},"b":{"type":"integer"}},"required":["a","b"],"type":"object"},"outputSchema":{"properties":{"result":{"type":"integer"}},"required":["result"],"type":"object","x-fastmcp-wrap-result":true},"_meta":{"fastmcp":{"tags":[]}}}]}}
{{< /highlight-wrap >}}

高亮是标准输入的内容，然后看到相应输出就是工具函数列表，带详细描述。

#### 调用工具

模型会告诉客户端调用哪个工具，以及相应的参数，`MCP` 客户能把它们组成 `JSON-RPC` 请求的消息，如果对于 `http` 的 `MCP` 服务，进行如下的请求

{{< highlight-wrap bash >}}
curl 'http://localhost:8000/mcp' \
-H 'Accept: application/json, text/event-stream' \
-H 'Content-Type: application/json' \
-H 'mcp-session-id: 9b5fd6e4c7804ca69bf3f970da21cbf7' \
--data '{"method":"tools/call","params":{"name":"get_weather","arguments":{"location":"nyc"}},"jsonrpc":"2.0","id":1}'
event: message
data: {"jsonrpc":"2.0","id":1,"result":{"_meta":{"fastmcp":{"wrap_result":true}},"content":[{"type":"text","text":"It's always sunny in New York"}],"structuredContent":{"result":"It's always sunny in New York"},"isError":false}}
{{< /highlight-wrap >}}

对于 `http` 的 `MCP` 调用就完成了。对于 `stdio` 的 `MCP` 服务，也可以猜出来如何调用了

{{< highlight-wrap bash "hl_lines=3 5">}}
uv run python src/langchain_study/mcp/math_server.py
[04/22/26 17:09:39] INFO     Starting MCP server 'Math' with transport 'stdio'                                                                                                                                                                              transport.py:209
{"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"mcp","version":"0.1.0"}},"jsonrpc":"2.0","id":0}
{"jsonrpc":"2.0","id":0,"result":{"protocolVersion":"2025-11-25","capabilities":{"experimental":{},"logging":{},"prompts":{"listChanged":false},"resources":{"subscribe":false,"listChanged":false},"tools":{"listChanged":true},"extensions":{"io.modelcontextprotocol/ui":{}}},"serverInfo":{"name":"Math","version":"3.2.4"}}}
{"method":"tools/call","params":{"name":"add","arguments":{"a":3,"b":5}},"jsonrpc":"2.0","id":1}
math:add called with a=3 and b=5
{"jsonrpc":"2.0","id":1,"result":{"_meta":{"fastmcp":{"wrap_result":true}},"content":[{"type":"text","text":"8"}],"structuredContent":{"result":8},"isError":false}}
{{< /highlight-wrap >}}

同样，高亮为标准输入的内容，紧随其后的是输出的结果。

### HTTP MCP 传 HTTP Header

要向 `http`(也是 `streamable-http`) 或 `sse` 的 `MCP` 服务端发送请求头的方法是

```python
client = MultiServerMCPClient(
    {
        "weather": {
            "transport": "http",
            "url": "http://localhost:8000/mcp",
            "headers": {
                "Authorization": "Bearer YOUR_TOKEN",
                "X-Custom-Header": "custom-value"
            },
        }
    }
)
```

对于 `Authentication` 应该也能通过请求头的方式发送，但这里有个特殊的参数 `auth`

```python
client = MultiServerMCPClient(
    {
        "weather": {
            "transport": "http",
            "url": "http://localhost:8000/mcp",
            "auth": auth,
        }
    }
)
```

`auth` 是一个 `httpx.Auth` 实现。

`http` 的 `MCP` 通过回调接口应该可以实现 `OAuth` 的认证流程，此处不继续深入。参考两个链接：

- [Example custom auth implementation](https://github.com/modelcontextprotocol/python-sdk/blob/main/examples/clients/simple-auth-client/mcp_simple_auth_client/main.py)
- [Built-in OAuth flow](https://github.com/modelcontextprotocol/python-sdk/blob/main/src/mcp/client/auth/oauth2.py#L216)

### `MultiServerMCPClient` 默认无状态

`http` 的 `MCP` 每次调用函数时都会初始化得到一个 `mcp-session-id`，据此调用相应的方法，`stdio` 也是无状态的，这符合正常使用 `MCP` 的需求，
记忆应该维护在 `Agent` 端. 用 `ClientSeesion` 可以让 `MCP` session 变成有状态的，但还是想不出作为一个工具为何要维护状态。

### `MCP` 工具调用的内容

####  结构化内容

从 `client.get_tools()` 返回的 `StructuredTool`, 它有一个属性 `response_format='content_and_artifact'`, 这意味着 `MCP`
的工具除了返回适于机器读的内容，还伴随着适于人阅读的内容。

我们窥探一下第一个算 `3 + 5` 的 `ToolMessage` 的内容片断

{{< bundle-image langchain-mcp-artifact.png 850 >}}

在 `LangChain` 中消息的 `artifact` 中的内容是不会加到会话历史中去的，是给人阅读或记日志用的。图中 `content`, `content_blocks` 中的内容才会加到会话历史中去。
如果要把 `structured_content` 加到与 `LLM` 交互的会话历史中去，则要用 [interceptor](https://docs.langchain.com/oss/python/langchain/mcp#tool-interceptors).
暂时没想到这种需求，跳过相关的演示代码。

#### 多模态内容

除了文本外，`MCP` 工具也可以返回图片，音视频之类的，还是对比上一个图，此时内容要放在 `content_blocks` 中, 这时的 `type` 就要是 `image`
之类的。这与 `MCP` 关系不大，就是一个常规的 `Tool` 也是同样的。

先不管 `MCP`, 看如果一个 `LangChain` 的 `tool` 返回一个图片内容时，工具函数该返回怎么样的格式

```python
import base64

import httpx
from langchain.agents import create_agent
from langchain_core.messages import HumanMessage
from langchain_core.tools import tool


@tool
def fetch_image(image_url: str) -> list[dict]:
    """Fetch image from the web"""
    response = httpx.get(image_url)
    content_type = response.headers.get("content-type", "image/png")
    b64_data = base64.b64encode(response.content).decode("utf-8")
    return [{"type": "image_url", "image_url": {"url": f"data:{content_type};base64,{b64_data}"}}]


agent = create_agent(
    # "ollama:gemma4:e4b",
    "bedrock:us.anthropic.claude-haiku-4-5-20251001-v1:0",
    tools=[fetch_image],
)

result = agent.invoke({"messages": [HumanMessage(content=(
    "describe the the image at https://yanbin.blog/my-first-langchain-ai-agent/my-first-ai-agent-cats.png"
    ))]})

print(result["messages"][-1].content)
```

这段代码还必须切换到 `claude` 的模型才能正确识别图片的内容，最后显示对图片的描述信息

```markdown
This image shows a Telegram chat interface with a bot called "Seek Cat." Here's what it displays:

**Top portion:**
- A Telegram conversation header showing "Seek Cat bot" with a blue circular avatar containing an "S"
- A high-quality close-up photo of a tabby cat's face with striking yellow/green eyes and prominent whiskers, timestamped 6:29 PM

**Bottom portion:**
- A message bubble announcing "🐱 New Cat Found!" with details about a cat named **Ribeye**:
  - **Sex:** Male/Neutered
  - **Breed:** Domestic Shorthair/Mix
  - **Age:** 1 year 9 months
  - Links to "View Photo" and "View Full Profile"
  - Below that is another cat photo showing a tabby and white cat lying in a cat hammock/bed, looking at the camera, also timestamped 6:29 PM

**Background:** The chat window has a light green background with cute cat-themed emoji patterns (cats, fish, watermelons, etc.)

This appears to be a demo or example screenshot from a blog post about creating an AI agent using LangChain, showing how the bot can present information about cats in a Telegram interface.
```

调试看看 `fetch_image()` 调用后 `ToolMessage` 的内容是什么

```json
{
  "artifact": null,
  "content": [{"type": "image_url","image_url":  {"url":"data:image/png;base64,iVBORw0KGgoAAAAN......"}}],
  "content_blocks": [{"mime_type": "image/png", "type": "image", "base64": "iVBORw0KGgoAAAAN......", 
    "id": "lc_32058fe8-d4b8-4aad-bdfa-e0b88473becb"}],
  "name": "fetch_image",
  "text": "",
  "tool_call_id": "toolu_bdrk_01VFAFDYiu8jjnEb4Vf6QmkW",
  "type": "tool"
}
```

`LangChain` 的 `@tool` 可指定属性 `@tool(response_format="content_and_artifact")`, 在方法中返回一个 `tuple`, 前为 content, 后
为 artifact.

如果一个 `MCP` 工具方法获取图片数据的话是否也要返回相同的数据格式呢？ 尝试写成如下的 `MCP` tool

```python
@mcp.tool()
def fetch_image(image_url: str) -> ImageContent:
    """Fetch image from the web"""
    print(f"Image:fetch_image called with {image_url=}", file=sys.stderr)
    response = httpx.get(image_url)
    content_type = response.headers.get("content-type", "image/png")
    b64_data = base64.b64encode(response.content).decode("utf-8")

    return ImageContent(
            type="image",
            data=b64_data,
            mimeType=content_type
        )
```

可是失败了，生成的 `ToolMessage` 中 `content_blocks` 的内容和上面一样，符合预期，可是 `content` 也和 `content_blocks` 一样。

### MCP 的资源与提示词

`MCP` 的 [`Resources`](https://modelcontextprotocol.io/specification/2025-11-25/server/resources)
与 [`Prompts`](https://modelcontextprotocol.io/specification/2025-11-25/server/prompts) 很类似，都是 `MCP` 暴露的资源(比如静态文件)
只是它们的用途不同，它们在改变时可激发事件，客户端可订阅它们的变化事件。 而且获取方式也相近， 通过 `MultiServerMCPClient` 的方法

- Resources: get_resources(server_name, uris), 通过 `uris` 读取资源的内容
- Prompts: get_prompts(server_name, prompt_name, arguments), 通过 `prompt_name` 读取提示词的内容, 如果是一个模板用 `arguments`
  字典填充

我们可遵循 `MCP` 的规范用 `JSON-RPC` 协议列出 Resources, Resource 模板, Prompts. 创建 `@mcp.resource()` 和 `@mcp.prompt()`
很像是在定义 `RESTFul` API 一样，

Resource 有参数时用占位符，如

```python
@mcp.resource("resource://users/{user_id}")
def get_user(user_id: str) -> str:
    ...
```

Prompt 有参数时，声明为函数参数即可

```python
@mcp.prompt()
def translate(text: str, target_lang: str = "English") -> str:
    ...
```

