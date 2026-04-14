---
title: "LangChain 核心组件之 Agent"
url: /langchain-core-component-agents/
date: 2026-04-14T15:14:48-05:00
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
codeMaxLines: 50
lastmod:
---

经过了一番 `LangChain` 的学习之后，我们开始跟随 `LangChain` 的官方文档系统性的学习。首先是它的核心组件，包括 `Agents`, `Models`, 
`Messages`, `Tools`, `Short-term memory`, `Streaming`, 和 `Structured output`. 现在从 [Agents](https://docs.langchain.com/oss/python/langchain/agents)
开始。

### 创建 Agent

`create_agent` 是在正式产品中用于创建 Agent 的函数， 它返回的是一个 `langgraph.graph.state.CompiledStateGraph` 对象，而非一个 `XxxAgent`
样的东西。所以它创建的是一个状态图，图吗，它包括顶点和边，Agent 就是这个图中移动，比如在 `Model` 和 `Tools` 之间往复, 或加入 `Middleware`,
或者中间可以加入人的互动(Human-in-the-loop).

`LangGraph` 把与模型，工具，以及人的交互做成一个图还是很表意的，取了文档中的这个图来表示 Agent 的状态图。<!--more-->

{{< bundle-image langgraph-state-graph.png 338 >}}

比如用代码

```python
agent = create_agent(
    "ollama:gemma4:e4b",
    tools=[get_weather],
)
```

查看 `agent.nodes` 就能看到它的三个节点分别是 `__start__`, `model`, 和 `tools`. `agent.get_graph().edges` 可看到有四个边，分别为

1. \_\_start\_\_ --> model, conditional=False
2. model --> end, conditional=True
3. model --> tools, conditional=True
4. tools --> model, conditional=True

这实际上已经把前面的那个状态图描述出来了。

在继续往下进一步了解 Model 之前有必要知道 `create_agent()` 方法的原型

```python
def create_agent(
    model: str | BaseChatModel,
    tools: Sequence[BaseTool | Callable[..., Any] | dict[str, Any]] | None = None,
    *,
    system_prompt: str | SystemMessage | None = None,
    middleware: Sequence[AgentMiddleware[StateT_co, ContextT]] = (),
    response_format: ResponseFormat[ResponseT] | type[ResponseT] | dict[str, Any] | None = None,
    state_schema: type[AgentState[ResponseT]] | None = None,
    context_schema: type[ContextT] | None = None,
    checkpointer: Checkpointer | None = None,
    store: BaseStore | None = None,
    interrupt_before: list[str] | None = None,
    interrupt_after: list[str] | None = None,
    debug: bool = False,
    name: str | None = None,
    cache: BaseCache[Any] | None = None,
) -> CompiledStateGraph[
    AgentState[ResponseT], ContextT, _InputAgentState, _OutputAgentState[ResponseT]
]
```

`*` 处主要是使用[模型的参数](https://docs.langchain.com/oss/python/langchain/models#parameters)，例如, `api_key`, `temperature`,
`max_tokens`, `timeout`, `max_reties`

每个 `Agent` 可以有一个名称，用 `name` 参数指定，在多 `Agent` 的场景下，可以方便的识别和管理。

### 动态模型

Model 是 Agent 的核心，没了 Model 什么也不是，它分静态和动态的 Model, 动态模型即动态的选择适合于干某类事的特定的模型。静态模型就是创建 `Agent`
时直接指定的模型. 例如本地有两个 Ollama 下载的模型

```bash
ollama list
NAME           ID              SIZE      MODIFIED
llama3.2:1b    baf6a787fdff    1.3 GB    31 seconds ago
gemma4:e4b     c6eb396dbd59    9.6 GB    3 days ago
```

现在想要简单的如 `hello`, `how are you` 这样的问题用小模型 `llama3.2:1b` 来回答，而复杂的问题用 `gemma4:e4b` 回答, 这种降智处理可以快速
回答外，也可以为模型提供商节约成本。

以下例子根据官方代码进行改编的

```python
from langchain.chat_models import init_chat_model
from langchain.agents import create_agent
from langchain.agents.middleware import wrap_model_call, ModelRequest, ModelResponse
from langchain_core.messages import AIMessage

basic_model = init_chat_model(model="ollama:llama3.2:1b")
advanced_model = init_chat_model(model="ollama:gemma4:e4b")

@wrap_model_call
def dynamic_model_selection(request: ModelRequest, handler) -> ModelResponse:
    """Choose model based on conversation complexity."""
    # message_count = len(request.state["messages"])
    message_length = len(request.messages[-1].content)
    model = advanced_model if message_length > 10 else basic_model
    return handler(request.override(model=model))

agent = create_agent(
    model=basic_model,  # Default model
    middleware=[dynamic_model_selection]
)

def chat(question: str) -> None:
    result = agent.invoke({"messages": {"role": "user", "content": question}})
    for message in result["messages"]:
        if isinstance(message, AIMessage):
            print(f"model: {message.response_metadata['model']}\nanswer: {message.content}")


chat("hello")
print("--------------")
chat("write python code to shutdown an ec2 instance")
```

`LangChain` 靠 `middleware` 来选择适当模型的，以上代码执行后输出为

> model: llama3.2:1b<br/>
> answer: Hello. Is there something I can help you with or would you like to chat?<br/>
> --------------<br/>
> model: gemma4:e4b<br/>
> answer: To write Python code that shuts down an EC2 instance, you need to use the AWS SDK for Python, which is called **`boto3`**.<br/>
><br/>
> This code assumes you have already:<br/>
> ......

注：如果在 `init_chat_model()` 创建模型时绑定了工具的话，则动态模型不能与结构化输出同时使用，此时应该在 `create_agent()` 时绑定工具。

大语言模型发展到现在 `Tools` 是必不可少的，没有工具的话就像早期的 `ChatGPT` 只能基于训练时的知识来回答问题，对于问询天气，订票之类的根本无能为力，
也就成不了现在的所谓的智能体。`LangChain` 支持工具方面，多个工具逐个触发，但可并发调用; 像动态模型一样，工具也可以动态的, 这就可以根据应用场景
动态的发送工具列表给模型，不用每次把所有的工具送给大模型，节约 Token; 工具调用有重试和错误处理机制; 状态可跨越不同的工具调用。

静态工具的使用很简单，不再讲述，接下看 `LangChain` 如何支持动态工具调用，我们可以根据用户权限，特性标记或会话阶段来选择不同的工具集。动态选择
工具有两种种方式

1. 创建 Agent 时预注册所有的工具，在每次交互时用 `middleware` 过滤出当前会话需要的工具
2. 不预注册动态的工具，每次与 LLM 交互时也是用 `middelware` 直接选择需要的工具

下面体验第一种方式，根据用户角色来过滤工具, 演示代码

```python
from langchain.agents import create_agent
from langchain.agents.middleware import wrap_model_call, ModelRequest, ModelResponse
from typing import Callable, Literal

from langchain_core.tools import tool
from pydantic import BaseModel


@tool
def read(filepath: str) -> str:
    """Read file data by filepath."""
    return f"Read {filepath}"


@tool
def update(filepath: str, new_content: str) -> str:
    """Update file data by filepath and new content."""
    return f"Updated {filepath}"


@tool
def write(filepath: str, content: str) -> str:
    """Write file data with filepath and content."""
    return f"Writing {filepath} with content: {content}"


class Context(BaseModel):
    user_role: Literal["admin", "editor", "viewer"]


@wrap_model_call
def context_based_tools(
        request: ModelRequest,
        handler: Callable[[ModelRequest], ModelResponse]
) -> ModelResponse:
    user_role = request.runtime.context.user_role

    if user_role == "editor":
        tools = [t for t in request.tools if t.name == "update"]
        request = request.override(tools=tools)
    elif user_role == "viewer":
        tools = [t for t in request.tools if t.name == "read"]
        request = request.override(tools=tools)

    return handler(request)


agent = create_agent(
    model="ollama:gemma4:e4b",
    tools=[read, update, write],
    middleware=[context_based_tools],
    context_schema=Context
)

question = "create a file and put content xyz"

agent.invoke({"messages": [{"role": "user", "content": question}]},
             context=Context(user_role="admin"))

agent.invoke({"messages": [{"role": "user", "content": question}]},
             context=Context(user_role="editor"))

agent.invoke({"messages": [{"role": "user", "content": question}]},
             context=Context(user_role="viewer"))
```

`user_role` 为 `admin`, `editor` 和 `viewer` 时，这三个请求分别向模型发送的提示词是(从中看到相应的 tools)

#### user_role = "admin"

```json
{
  "tools": [ {
      "type": "function",
      "function": { "name": "read", "description": "Read file data by filepath.",
        "parameters": { "type": "object", "required": [ "filepath" ],
          "properties": { "filepath": { "type": "string" } }
        } } },
    {
      "type": "function",
      "function": { "name": "update", "description": "Update file data by filepath and new content.",
        "parameters": { "type": "object", "required": [ "filepath", "new_content" ],
          "properties": { "filepath": { "type": "string" }, "new_content": { "type": "string" } }
        } } },
    {
      "type": "function",
      "function": { "name": "write", "description": "Write file data with filepath and content.",
        "parameters": { "type": "object", "required": [ "filepath", "content" ],
          "properties": { "filepath": { "type": "string" }, "content": { "type": "string" } }
        } } }
  ]
}
```

#### user_role = "editor"

```json
{
  "tools": [ {
      "type": "function",
      "function": { "name": "update", "description": "Update file data by filepath and new content.",
        "parameters": { "type": "object", "required": [ "filepath", "new_content" ],
          "properties": { "filepath": { "type": "string" }, "new_content": { "type": "string" } }
        } } }
  ]
}
```

### user_role = "viewer"
```json
{
  "tools": [ {
      "type": "function",
      "function": { "name": "read", "description": "Read file data by filepath.",
        "parameters": { "type": "object", "required": [ "filepath" ],
          "properties": { "filepath": { "type": "string" } }
        } } }
  ]
}
```

`LangChain` 的 Tools 调用使用的是 ReAct 循环模型，即 `Reasoning/Acting` 循环。

#### 工具调用的异常处理

仍然是用 `middleware`, 这次配置 `@wrap_tool_call` 装饰器，下面是演示代码

```python
from langchain.agents import create_agent
from langchain.agents.middleware import wrap_tool_call
from langchain_core.messages import ToolMessage

from langchain_core.tools import tool


@tool
def read(name: str) -> str:
    """Read file data by filepath."""
    raise FileNotFoundError(f"File {name} not found")


@wrap_tool_call
def handle_tool_errors(request, handler):
    """Handle tool execution errors with custom messages."""
    try:
        return handler(request)
    except Exception as e:
        return ToolMessage(
            content=f"Tool error: Please check your input and try again. ({str(e)})",
            tool_call_id=request.tool_call["id"]
        )


agent = create_agent(
    model="ollama:gemma4:e4b",
    tools=[read],
    middleware=[handle_tool_errors]
)

result = agent.invoke(
    {"messages": [{"role": "user", "content": "read file /abc.log and explain the content"}]},
)

for message in result["messages"]:
    print(f"{type(message)}: {message.tool_calls if isinstance(message, AIMessage) else ""}: {message.content}")
```

调用方法 `read(name)` 时报出异常， 看下这时 `Agent` 会如何处理, 打印出上面的所有消息

{{< highlight-wrap text >}}
<class 'langchain_core.messages.human.HumanMessage'>: : read file /abc.log and explain the content
<class 'langchain_core.messages.ai.AIMessage'>: [{'name': 'read', 'args': {'name': '/abc.log'}, 'id': 'e733af10-fd95-48fa-b12d-b8224fb4291e', 'type': 'tool_call'}]: 
<class 'langchain_core.messages.tool.ToolMessage'>: : Tool error: Please check your input and try again. (File /abc.log not found)
<class 'langchain_core.messages.ai.AIMessage'>: []: I was unable to read the file `/abc.log` because the system returned an error stating that the file was not found.
{{< /highlight-wrap >}}

中间的 `ToolMessage` 报告了 `Tool error`.

### 系统提示词(System Prompt)

系统提示词是考验使用 AI, 实现代理的功力的地方，像 [`OpenClaw` 的系统提示词参考](http://localhost:1313/why-openclaw-burn-your-token/#tooling) 就非常庞大。
这里不是学习如何写系统提示词，当前我们也可以问 AI 帮我们生成所需的系统提示词，一切皆可 AI。而要讲的是系统提示词也可以动态，创建 agent 时不指系统提示词，
也像 Model, Tools 那样可以根据条件动态选择。

```python
class Context(TypedDict):
    user_role: str
    
@dynamic_prompt
def user_role_prompt(request: ModelRequest) -> str:
    """Generate system prompt based on user role."""
    user_role = request.runtime.context.get("user_role", "user")
    base_prompt = "You are a helpful assistant."

    if user_role == "expert":
        return f"{base_prompt} Provide detailed technical responses."
    elif user_role == "beginner":
        return f"{base_prompt} Explain concepts simply and avoid jargon."

    return base_prompt

agent = create_agent(
    model="...",
    middleware=[user_role_prompt],
    context_schema=Context
)
```

### 结构化输出

在 `create_agent()` 时用 `response_format=TollStrategy(CustomModel)` 指定一个自定义的模型类，用于对 LLM 的输出进行格式化，并作为
消息返回给 `Agent` 的使用者，它本质上是一个 `Tool`.

我们借鉴官方的例子

```python
from pydantic import BaseModel
from langchain.agents import create_agent
from langchain.agents.structured_output import ToolStrategy


class ContactInfo(BaseModel):
    name: str
    email: str
    phone: str

    def __str__(self):
        return f"ContactInfo(name={self.name}, email={self.email}, phone={self.phone})"

agent = create_agent(
    model="ollama:gemma4:e4b",
    response_format=ToolStrategy(ContactInfo)
)

result = agent.invoke({
    "messages": [{"role": "user", "content": "Extract contact info from: John Doe, john@example.com, (555) 123-4567"}]
})

print(result["structured_response"])

for message in result["messages"]:
    print(f"{type(message)}: {message.tool_calls if isinstance(message, AIMessage) else ""}: {message.content}")
```

使用了 `response_format=ToolStrategy(ContactInfo)` 之后，返回的 `result` 就有两个字段 `messages` 和 `structured_response`.
result["structured_response"] 的内容就是一个 `ContactInfo` 实例, 输出结果为

{{< highlight-wrap text >}}
ContactInfo(name=John Doe, email=john@example.com, phone=(555) 123-4567)
<class 'langchain_core.messages.human.HumanMessage'>: : Extract contact info from: John Doe, john@example.com, (555) 123-4567
<class 'langchain_core.messages.ai.AIMessage'>: [{'name': 'ContactInfo', 'args': {'email': 'john@example.com', 'name': 'John Doe', 'phone': '(555) 123-4567'}, 'id': 'e0201c29-21f8-45ac-9b2f-537616e56b2e', 'type': 'tool_call'}]:
<class 'langchain_core.messages.tool.ToolMessage'>: : Returning structured response: ContactInfo(name=John Doe, email=john@example.com, phone=(555) 123-4567)
{{< /highlight-wrap >}}

为什么说格式化输出本质上是一个 `Tool`，我们看它实际上向 LLM 发送了下面的工具提示词

```json
{
  "model": "gemma4:e4b", "stream": true, "options": {}, "messages": [{ "role": "user",
      "content": "Extract contact info from: John Doe, john@example.com, (555) 123-4567"
    }],
  "tools": [ {
      "type": "function",
      "function": { "name": "ContactInfo", "description": "",
        "parameters": {
          "type": "object",
          "required": [ "name", "email", "phone" ],
          "properties": {
            "name": { "type": "string" },
            "email": { "type": "string" },
            "phone": { "type": "string" }
          } } } } ] }
```

注：对不重要的信息进行了折叠

如果是一个无法正确格式化的消息会出现什么状况呢？ 把问题换成

> Extract contact info from: hello

这会让 LLM 一直 thinking, 并回答

> I could not find any contact information (name, email, or phone number) in the text \"hello\". Please provide the text you would like me to extract from.

陷入了一个几乎是与 LLM 交互的死循环， 从方法 `agent_inoke()` 中退不出来，因为我们在 `create_agent()` 时默认的 `recursion_limit` 是 9999,

> {'configurable': {}, 'metadata': {'ls_integration': 'langchain_create_agent'}, 'recursion_limit': 9999}

可以在创建 `agent` 是修改 `recursion_limit` 值，或在 `agent.invoke()` 时设定该值，`invoke()` 调用改成

```python
result = agent.invoke({
    "messages": [{"role": "user", "content": "Extract contact info from: hello"}],
},
    config={"recursion_limit": 50}
)
```

这样的话达到与 LLM 的交互次数上限 50 后就会报告异常退出 `invoke()` 调用

{{< highlight-wrap text >}}
raise GraphRecursionError(msg)
langgraph.errors.GraphRecursionError: Recursion limit of 50 reached without hitting a stop condition. You can increase the limit by setting the `recursion_limit` config key.
For troubleshooting, visit: https://docs.langchain.com/oss/python/langgraph/errors/GRAPH_RECURSION_LIMIT
{{< /highlight-wrap >}}

实际应用中需根据工具数量来决定 `recursion_limit` 的值，工具越多，循环的可能性越大，值也要相应的增大。

`ProviderStrategy` 使用模型自己的结构化输出方式，模型没有的话会使用 `ToolStrategy`

```python
agent = create_agent(
    model="ollama:gemma4:e4b",
    response_format=ProviderStrategy(ContactInfo)
)
```

与 `ToolStrategy` 的区别是不需要依赖于工具调用，使用 `ProviderStrategy` 后发送给大模型的的提示词是

```json
{
  "model": "gemma4:e4b", "stream": true, "options": {},
  "format": {
    "properties": {
      "name": { "title": "Name", "type": "string" },
      "email": { "title": "Email", "type": "string" },
      "phone": { "title": "Phone", "type": "string" }
    },
    "required": [ "name", "email", "phone"
    ],
    "title": "ContactInfo",
    "type": "object"
  },
  "messages": [ { "role": "user", "content": "Extract contact info from: John Doe, john@example.com, (555) 123-4567" } ],
  "tools": []
}
```

不再需要 `tools`, 而是发送了 `format`，测试了 `ollama:gemma4:e4b` 模型，可以支持，这样就省去了一次 `ToolMessage` 的调用，相当于省了
一个消息来回，所以整个通信过程就一个 `MumanMessage` 和一个 `AIMessage` 就完事了，`AIMessage` 中直接输出格式化后的消息

```text
ContactInfo(name=John Doe, email=john@example.com, phone=(555) 123-4567)
<class 'langchain_core.messages.human.HumanMessage'>: : Extract contact info from: John Doe, john@example.com, (555) 123-4567
<class 'langchain_core.messages.ai.AIMessage'>: []: {
  "name": "John Doe",
  "email": "john@example.com",
  "phone": "(555) 123-4567"
}
```

### Agent 状态

`LangChain` 的技术文档 [Agents/Memory](https://docs.langchain.com/oss/python/langchain/agents#memory) 好像不是关于记忆的内容，
而是有关 `AgentState` 的，比如默认的 `AgentState` 有三个字段 `messages`, `jump_to`, 和 `structured_response`. 我们调用 `agent.invoke()`
时通常用 `{"messages": [...]}` 发送消息，通过定制 `AgentState` 可以附更多的信息, 但是有什么用途呢？

`LangChain` 提供两种方式来使用定制的 `AgentState`

1. 通过 `middleware` 来使用 `AgentState`(这种方式稍复杂些，但是首选的，因为状态能与相应的 Tools 关联)
2. 通过 `create_agent` 的 `state_schema` 参数使用 `AgentState`

```python
class CustomState(AgentState):
    user_preferences: dict

class CustomMiddleware(AgentMiddleware):
    state_schema = CustomState

    def before_model(self, state: CustomState, runtime: Runtime) -> dict[str, Any] | None:
        print("message ", state["messages"][-1].content)  # message hello
        print("task_type" , state["user_preferences"].get("task_type")) # task_type simple

agent = create_agent(
    model="ollama:gemma4:e4b",
    middleware=[CustomMiddleware()]
)

result = agent.invoke({
    "messages": [{"role": "user", "content": "hello"}],
    "user_preferences": {"task_type": "simple"},
})
```

`user_preferences` 也只是让 `middleware` 看到，并不会发送到大语言模型去。

### Streaming 流式交互

我们除了调用 `agent.invoke()` 等与模型交互全部完成后得到最后的结果，也可以在与模型交互过程中 通过 `agent.stream()` 来获取流式交互的结果.
也就是要调用 `agent.stream()` 方法，它与 `model.stream()` 还不尽相同.

```python
for chunk, metadata in agent.stream(
        {"messages": [{"role": "user", "content": "python code to read s3 object"}]},
        stream_mode="messages"):
    print(chunk.content, end="", flush=True)
```

这与 `model.stream()` 是一样的效果，一个一个 token 输出。`stream_mode` 的取值有 `values`, `updates`, `checkpoints`, `tasks`, 
`debug`, `messages`, `custom`, 默认值为像是 `updates`. 不同 `stream_mode` 下 `agent.stream()` 返回值的格式不一样，需因时读取数据。

### Middleware 中单件

`LangChain` 的 `middleware` 可以参考 Node.js 框架 `Express` 的 `middleware`, 在 `LangChain` 中 `middleware` 可执行许多的定制功能，如

1. 根据 `AgentState` 在调用模型前定制消息，向上下文中注入信息
2. 修改或校验模型的响应信息，比如内容敏感词过滤。LangChain 还要靠 Agent 来做过滤，中国产的模型在模型内部就能过滤
3. 处理工具执行异常
4. 基于 State 或上下文实现动态的选择模型
5. 添加定制的日志，进行监控与分析