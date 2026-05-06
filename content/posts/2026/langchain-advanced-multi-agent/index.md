---
title: "LangChain 高级用法之多 Agent协作"
url: /langchain-advanced-multi-agent/
date: 2026-05-05T17:16:48-05:00
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

在往 `Deep Agents` 的路径还要继续啃下一些基础知识，其中之一就是多 Agent 协作(Multi-agent)。复杂的系统需要多个 `Agent`(智能体)同共完成,
不那么复杂的系统可以用动态的 `Prompt`, `Tool`, 或 `Model` 来切换。多 Agent 的好处是每个 Agent 有自己的上下文，工具，专业领域的知识，
可以独立开发，分布式部署，多个 Agent 能并行执行，加速工作与烧钱的速度。

`LangChain` 在构建多 Agent 系统时，有以下几种常见模式：

1. Subagents(子 Agent): 主从 Agent 方式，子 Agent 作为主 Agent 的工具使用，由主 Agent 指挥子 Agent 干活
2. Handoffs(任务交接): 任务可转交给(以 Tool 方式)其他 Agent，其他 Agent 的结果可直接返回给用户, `hand off` 就是传球的意思
3. Skills(技能): 动态加载特定的 Prompt 和知识，用单一 `Agent` 控制按需加载技能，本质上它是单 Agent
4. Router(路由)：由一个路径控制任务如何分配给不同的 Agent，结果汇集成一个组合的响应
5. Custom workflow(自定义工作流)：用更底层的 `LangGraph` 定制工作流程，以上模型均可嵌入到工作流中

下面贴上每种类型的组件和时序图(顺便给自己的 Hugo 加上 `tabs` 标签页功能，以方便对比切换和节约滚动条) <!--more-->

{{< tabs >}}
---tab Subagents
全部子 Agent 注册为主 Agent 的 `Tools`, 与整个系统的沟通都是通过主 Agent 进行的，子 Agent 把响应汇总到主 Agent 后反馈给用户。
易独产开发, 分布式部署，子 Agent 可并行执行，特点是存在一个 `Super Agent`。有时候这种模式只为了隔离会话，比如像 `Sumarization` 和 `Mem0`
用的 `LLM` `Agent`。
<div style="display: flex;">
   <div style="flex: 1;">
{{< bundle-image pattern-subagents.avif 500 >}}
   </div>
   <div style="flex: 1;">
{{< bundle-image oneshot-subagents.avif 500 >}}
   </div>
</div>
---tab Handoffs
Agent 之间互为 `Tool` 的方式网状连接，任务可转交给其他 Agent，其他 Agent 的结果可直接返回给用户。特点是没有 `Super Agent`，Agent 之间平等协作。
<div style="display: flex;">
   <div style="flex: 1;">
{{< bundle-image pattern-handoffs.avif 450 >}}
   </div>
   <div style="flex: 1;">
{{< bundle-image oneshot-handoffs.avif 450 >}}
   </div>
</div>
---tab Skills
系统中只有一个 `Agent`, 通过 `Agent Skills` 方式动态加载特定的提示词与知识库(含在提示词中)，动态加载 `Skills` 意味着经常要额外的 `load_skill`
工具调用。单次与 `LLM` 交互节约 `Token`，但短期记忆期间产生大量的会话 `Token`
<div style="display: flex;">
   <div style="flex: 1;">
{{< bundle-image pattern-skills.avif 450 >}}
   </div>
   <div style="flex: 1;">
{{< bundle-image oneshot-skills.avif 450 >}}
   </div>
</div>
---tab Router
与 `Subagents` 很相似，只是 `Router` 替代了 `Super Agent`, `Router` 只处理路由规则，分发任务，也不维护会话状态，子 `Agent` 可并发执行，
结果汇总后直接返回给用户。
<div style="display: flex;">
   <div style="flex: 1;">
{{< bundle-image pattern-router.avif 450 >}}
   </div>
   <div style="flex: 1;">
{{< bundle-image oneshot-router.avif 450 >}}
   </div>
</div>
{{< /tabs >}}

接下来独个体验以上每一种 `Multi-agent` 的实现方式

### Subagents(子 Agent)

有一个主 `Agent`(常称之为 Supervisor), 其他子 `Agent` 注册为主 `Agent` 的 `Tool`. 主 `Agent` 决定选择哪些个子 `Agent`, 提供什么参数，
以及如何合并结果。子 `Agent` 是无状态的，不用记住历史会话，每次都是一个全新对话。 用户只与主 `Agent` 交互，多个子 `Agent` 可并发执行。

当多个子 `Agent` 负责不同的领域(如日历 agent, email agent, database agent 等)，又如编程方面的 `Java Agent`, `Python Agent`, `Go
Agent` 等。子 `Agent` 可预先注册为主 `Agent` 的 `Tools`, 也可在运行时动态注册。

下面是一个预先注册两个 `Agent` 为工具的例子

```python
from langchain.agents import create_agent
from langchain_core.tools import tool

subagent_py = create_agent(
    model="ollama:gemma4:e4b",
)


@tool("python_language_agent", description="Write python language code")
def call_python_agent(query: str) -> str:
    result = subagent_py.invoke({"messages": [{"role": "user", "content": query}]})
    return result["messages"][-1].content


subagent_go = create_agent(
    model="ollama:llama3.2:1b"
)


@tool("go_language_agent", description="Write go language code")
def call_go_agent(query: str) -> str:
    result = subagent_go.invoke({"messages": [{"role": "user", "content": query}]})

    return result["messages"][-1].content


main_agent = create_agent(
    model="bedrock:us.anthropic.claude-haiku-4-5-20251001-v1:0",
    tools=[call_python_agent, call_go_agent],
)

if __name__ == '__main__':
    result = main_agent.invoke({"messages": {
        "role": "user", "content": """Write the simplest `hello world` code only in both python and go language,
         no any explanation, and choose the appropriate language specific subagent to write the code.
         Output original subagent returns, do not summarize.
         """}})

    print(result["messages"][-1].content)
```

`Supervisor` 给一个最强脑，它和 `python_agent` 和 `go_agent` 所用模型分别为

1. Super Agent: bedrock:us.anthropic.claude-haiku-4-5-20251001-v1:0
2. Python Agent: ollama:gemma4:e4b
3. Go Agent: ollama:llama3.2:1b

执行后某次的输出为

````text
```python
print("Hello, World!")
```

```go
package main

import "fmt"

func main() {
	fmt.Println("Hello, World!")
}
```
````

从后台分别看到向 `gemma4:e4b`(python_agent), `llama3.2:1b`(go_agent) 发送了请求，内容分别为

**python_agent**
```text
POST /api/chat
    Host: localhost:11434  Content-Type: application/json  Accept: application/json  Content-Length: 143
    Body (143 bytes): {"model":"gemma4:e4b","stream":true,"options":{},"messages":[{"role":"user","content":"Write a simple hello world code in python"}],"tools":[]}
```

**go_agent**
```
POST /api/chat
    Host: localhost:11434  Content-Type: application/json  Accept: application/json  Content-Length: 140
    Body (140 bytes): {"model":"llama3.2:1b","stream":true,"options":{},"messages":[{"role":"user","content":"Write a simple hello world code in go"}],"tools":[]}
```

主 `Agent` 选择了正确的子 `Agent` 并发送了对应 `Agent` 的提示词，`Write a simple hello world code in xxx`。

发送给主 `Agent` 的消息带了两个 `Tools`, `tools` 部分如下面那样

```json
{
  "tools": [
    {
      "type": "function",
      "function": {
        "name": "python_language_agent",
        "description": "Write python language code",
        "parameters": {
          "type": "object",
          "required": [
            "query"
          ],
          "properties": {
            "query": {
              "type": "string"
            }
          }
        }
      }
    },
    {
      "type": "function",
      "function": {
        "name": "go_language_agent",
        "description": "Write go language code",
        "parameters": {
          "type": "object",
          "required": [
            "query"
          ],
          "properties": {
            "query": {
              "type": "string"
            }
          }
        }
      }
    }
  ]
}
```

`Subagents` 的方式还是有几个具有挑战性的地方，首先提示词要让 `Super Agent` 容易理解应该怎么分派工作，还有如何收集了 `Agent` 的响应，
不叮嘱好，`Super Agent` 就是像调用普通 `Tool` 那样，拿到结果后会进行总结，然后输出给客户。`Subagents` 这种模式下最好是让子 `Agent` 只干活，
不说话。子 `Agent` 能直接与用户交互的方式就是 `Human-in-the-loop` 那种 `interrupts`, 可直接接收客户的输入确认，而后继续。

### Handoffs(任务交接)

`Handoffs` 的核心实现机制是: 工具更新状态的变量(如自定义的 `current_step` 或 `active_agent`), 系统会读取这个变量并调整行为 -- 动态的调整配置
(system prompt, tools) 或路由到不同的 `Agent`, 该模式可工作在多 `Agents` 下(路由)或单一 `Agent`(动态配置)。`Handoffs` 把整个系统看作是一个
`LangGraph` 构建的状态机(由 GraphState 节点与边构机), 或者更是在定义一个工作流，由条件决定走哪个节点(或应用哪个配置)。

单一 `Agent` 调整配置的方式其实在 `LangChain` 核心组件-Agents 中学习过，就是用中单件函数 `@wrap_model_call` 根据请求中某个状态值改变提示词或工具列表。
在正式进入代码之前，提前看一下 `LangGraph` 的 `middleware` 定义中的所有扩展点，见下图

{{< bundle-image middleware_final.avif 500 >}}

我们的实现方式在 `AIMessage` 请求调用的工具方法中，用 `Command` 主动返回一个 `ToolMessage`, 并同时修改状态信息，调用 `Tool` 是客户端的事，
所以这还在上图 `request` 之前的事，然后在 `wrap_model_call` 中读取到 `Tool` 调用设定的某个状态动态调用系统提示词和工具列表。

完整演示代码

```python
from langchain.agents import AgentState, create_agent
from langchain.agents.middleware import wrap_model_call, ModelRequest, ModelResponse
from langchain.tools import tool, ToolRuntime
from langchain.messages import ToolMessage
from langgraph.checkpoint.memory import InMemorySaver
from langgraph.types import Command
from typing import Callable

class SupportState(AgentState):
    """Track which step is currently active."""
    current_step: str = "triage"
    warranty_status: str | None = None

@tool
def record_warranty_status(status: str, runtime: ToolRuntime[None, SupportState]) -> Command:
    """Record warranty status and transition to next step."""
    return Command(update={
        "messages": [
            ToolMessage(
                content=f"Warranty status recorded: {status}",
                tool_call_id=runtime.tool_call_id
            )
        ],
        "warranty_status": status,
        # Transition to next step
        "current_step": "specialist"
    })

@tool(description="Provide solutions based on warranty status.")
def provide_solution():
    ...

@tool(description="Escalate to specialist for further investigation.")
def escalate():
    ...

configs = {
    "triage": {
        "prompt": "Collect warranty information from the user.",
        "tools": [record_warranty_status]
    },
    "specialist": {
        "prompt": "Provide solutions based on warranty: {warranty_status}",
        "tools": [provide_solution, escalate]
    }
}

@wrap_model_call
def apply_step_config(request: ModelRequest, handler: Callable[[ModelRequest], ModelResponse]) -> ModelResponse:
    step = request.state.get("current_step", "triage")
    config = configs[step]
    request = request.override(system_prompt=config["prompt"].format(**request.state), tools=config["tools"])
    return handler(request)

agent = create_agent(
    model="ollama:gemma4:e4b",
    tools=[record_warranty_status, provide_solution, escalate],
    state_schema=SupportState,
    middleware=[apply_step_config],
    checkpointer=InMemorySaver()  # Persist state across turns  #
)

config = {"configurable": {"thread_id": "1"}}

agent.invoke(
    {"messages": [{"role": "user", "content": "I have a broken iPhone."}]},
    config=config)

result = agent.invoke(
    {"messages": [{"role": "user", "content": "It's still under warranty"}]},
    config=config
)

for message in result["messages"]:
    message.pretty_print()
```

工具 `record_warranty_status` 中修改 `state_schema` 指定的 `SupportState` 中的 `current_step` 的值，然后在 `@wrap_model_call`
装饰的中间件方法中把根据当前的 `current_step` 动态调整系统提示词与工具列表，这样就实现了一个 `Handoffs`, 即工作流。

### Skills(技能)

前不久专门一篇博文 [LangChain 实战 - 使用 On-demand Skills](/langchain-practice-with-on-demand-skills/) 详尽的学习了 `Agent Skills`,
需要了解具体实现方式的请参考该文。`Skills` 的核心就是创建 `Agent` 时指定一个 `load_skill` 工具，并在系统提示词中列出所有 `Skill` 的名称与对应描述，
而后在需要用到某个 `Skill` 时再由 `load_skill` 工具根据名称加载该 `Skill` 对应的一大段提示词，最后按照其中的指令行事。

### Router(路由)

所谓路由就是根据一定的规则把请求分发到特定的 `Agent` 去，这符合广义的 `路由` 一词的定义。`Router` 模式从形式上看还很像 `Subagents` 模式，
但结果返回不像  `Subagents` 那样要汇集到 `Main agent` 再总结才返回，`Router` 模式中各个子 `Agent` 的响应只简单的合并。

看着 `Router` 的模式图好像挺简单

{{< bundle-image pattern-router.avif 550 >}}

其实上图的 `Router` 和 `Synthesize` 这两部分的发挥空间都很大，你可以用简单的 Mapping 或 if/else 来实现 `Router`, 也可用 `Agent`
来实现智能分发，`Synthesize` 节点也一样，可简单合并结果返回，或放一个 `Agent` 把各子 `Agent` 的结果汇合再总结后返回给客户。
而且要实现这个模式还得用到 `LangGraph` 低阶的组件。

下面两个不完整的示例，分别演示了单 `Agent` 和多 `Agent` 并发的 `Router` 模式

#### Single agent

通过 `Command(goto=specific_agent)` 路由到某个特定的 `Agent`

```python
from langgraph.types import Command

def classify_query(query: str) -> str:
  """Use LLM to classify query and determine the appropriate agent."""
  # Classification logic here
...

def route_query(state: State) -> Command:
  """Route to the appropriate agent based on query classification."""
  active_agent = classify_query(state["query"])

      # Route to the selected agent
      return Command(goto=active_agent)
```

#### Multiple agents (并发)

通过 `[Send(agent1, message1), Send(agent2, message2)]` 的方式并发发送给多个 `Agent`

```python
from typing import TypedDict
from langgraph.types import Send

class ClassificationResult(TypedDict):
    query: str
    agent: str

def classify_query(query: str) -> list[ClassificationResult]:
    """Use LLM to classify query and determine which agents to invoke."""
    # Classification logic here
    ...

def route_query(state: State):
    """Route to relevant agents based on query classification."""
    classifications = classify_query(state["query"])

    # Fan out to selected agents in parallel
    return [
        Send(c["agent"], {"query": c["query"]})
        for c in classifications
    ]
```

下面是一个把第一个 `Subagents` 的例子改造为 `Router` 模式

```python
import operator
from typing import TypedDict, Annotated, List, Literal

from langchain.agents import create_agent
from langchain.chat_models import init_chat_model
from langgraph.constants import START, END
from langgraph.graph import StateGraph
from langgraph.types import Send
from pydantic import BaseModel, Field


class AgentInput(TypedDict):
    query: str


class AgentOutput(TypedDict):
    source: str
    message: str


class Classification(TypedDict):
    source: Literal["python", "go"]
    query: str


class RouterState(TypedDict):
    query: str
    classifications: list[Classification]
    results: Annotated[List[AgentOutput], operator.add] #1
    final_answer: str


class ClassificationResult(BaseModel):
    classifications: list[Classification] = Field(
        description="List of agents to invoke with their targeted sub-questions"
    )


python_agent = create_agent(
    model="ollama:gemma4:e4b",
)

go_agent = create_agent(
    model="ollama:llama3.2:1b",
)

router_llm = init_chat_model(
    model="ollama:gemma4:e4b",
)

def classify_query(state: RouterState) -> dict:
    structured_llm = router_llm.with_structured_output(ClassificationResult)

    result = structured_llm.invoke([
        {
            "role": "system",
            "content": """Analyze this query and determine which knowledge bases to consult.
For each relevant source, generate a targeted sub-question optimized for that source.

Available sources:
- python: experts in python, libraries, frameworks, etc.
- go: experts in go, libraries, frameworks, etc.

Return ONLY the sources that are relevant to the query."""
        },
        {"role": "user", "content": state["query"]}
    ])

    return {"classifications": result.classifications} #2


def route_to_agents(state: RouterState) -> list[Send]: #3
    return [
        Send(c["source"], {"query": c["query"]})
        for c in state["classifications"]
    ]


def write_python(state: AgentInput):
  result = python_agent.invoke({
    "messages": [{"role": "user", "content": state["query"]}]
  })
  return {"results": [{             #4
    "source": "python_agent",
    "message": result["messages"][-1].content
  }]}


def write_go(state: AgentInput):
  result = go_agent.invoke({
    "messages": [{"role": "user", "content": state["query"]}]
  })
  return {"results": [{
    "source": "go_agent",
    "message": result["messages"][-1].content
  }]}


def synthesize_results(state: RouterState) -> dict: #5
    return {"final_answer": "\n\n".join(
        [f"**From {r['source'].title()}:**\n{r['message']}" for r in state["results"]]
    )}


workflows = (
    StateGraph(RouterState)
    .add_node("classify", classify_query)
    .add_node("python", write_python)
    .add_node("go", write_go)
    .add_node("synthesize", synthesize_results)
    .add_edge(START, "classify")
    .add_conditional_edges("classify", route_to_agents, ["python", "go"])
    .add_edge("python", "synthesize")
    .add_edge("go", "synthesize")
    .add_edge("synthesize", END)
    .compile()
)

# need python library `grandalf`
# workflows.get_graph().print_ascii()
# mermaid_code = workflows.get_graph().draw_mermaid() #6

if __name__ == '__main__':
    result = workflows.invoke(
        {"query": "Write the simplest `hello world` code only in both python"
                  " and go language without any comments and explanation"})
    print(result["final_answer"])
```

请展开上面的代码, 下面是对代码标注处作的解释

- #1 定义了 `RouterState`, 在任何 `LangGraph` 节点中，只要返回 `{"<field_name>": <value>}` 的格式，就会自动把返回值字段
  赋值到 `RouterState` 中相应字段上，如 `classify_query` 的 `return {"classifications": xxx}`, 在后续用了 `RouterState`
  参数的节点函数中就能用 `state["classifications"]` 访问前面步骤设置的值。
  而 `results: Annotated[List[AgentOutput], operator.add]` 更特别一些，会把 `write_python` 和 `write_go` 返回的 
  `{"results": [{...}]}` 结果进行合并
- #2 `classify_query` 中返回的值是 
   ```json
    {
      "classifications": [
         {"source": "python", "query": "How do I write the most basic 'hello world' program in Python?"},
         {"source": "go", "query": "How do I write the most basic 'hello world' program in Go?"}
      ]
    }
   ```
   `route_llm` 聪明的进行了任务分解，并指派给了适合的 `Agent`, 在后面的节点中可以通过  `state["classifications"]` 访问到这个值
- #3 `route_to_agents` 根据 `state["classifications"]` 中的分类，它返回了
    ```
    [
       Send(node='python', arg={'query': "How do I write the most basic 'hello world' program in Python?"}),
       Send(node='go', arg={'query': "How do I write the most basic 'hello world' program in Go?"})
    ]
    ```
    这就是路由规则
- #4 有了前面的路由规则，在该函数 `write_python` 中收到的 `AgentInput` 的值就是
  `{'query': 'How do I write the most basic 'hello world' program in Python?'}`, 它返回的 `{"results": [{...}]`
  值会合并到最终 `RouterState` 中的 `results` 中。对于 `write_go` 函数也是类似的
- #5 最后所有子 `Agent` 节点的结果汇合到这里，从 `state["results"]` 中可访问到聚合的结果，其中有两个元素，分别是 `write_python` 和
  `write_go` 返回的 `{"source": "python_agent", "message": "xxx"}` 和 `{"source": "go_agent", "message": "xxx"}`, 
  该函数把它们格式化成一个字符串作为 `RouterState` 的 `final_answer` 属性返回
- #6 可以多种方式打印出该 `LangGraph` 流程图，生成的 `mermaid_code` 所对应的图如下
  {{< bundle-image langchain-router-workflow.png 200 >}}
 
执行上面 `Router` 模式的 `Multi-agents` 后的输出为

```` markdown
**From Python_Agent:**
```python
print("hello world")
```

**From Go_Agent:**
```go
package main

import "fmt"

func main() {
	fmt.Println("Hello, world!")
}
```
````

任务成功分发到相对应的子 `Agent`. 通过对 `Router` 模式的学习与实践，第一次真正接触了 `LangGraph` 的 `GraphState`, 不再只 `create_agent(model, tools)`
后底层自动有了一个 `LangGraph` 工作流，而是从零主动搭建了一个 `LangGraph` 工作流。同时除了 `Mermaid` 和 `Apache Airflow` 外，`LangGraph`
还能用来辅助生成一个可视化的工作流程图。而对于 `LangChain` 的 `Custom workflow` 的方式构建 `Multi-agent`，不是已经用 `Router` 练过了吗，
都不用再额外学习了，就是那套

```python
workflow = (
    StateGraph(State)
    .add_node("agent", agent_node)
    .add_edge(START, "agent")
    .add_edge("agent", END)
    .compile()
)
```

样板代码，比起 `create_agent()` 有了更广阔的发挥天地了。