---
title: "LangChain 高级用法之多 Agent协作"
url: /langchain-advanced-multi-agent/
date: 2026-05-05T17:16:48-05:00
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

在往 `Deep Agents` 的路径还要继续啃下一些基础知识，其中之一就是多 Agent 协作(Multi-agent)。复杂的系统需要多个 `Agent`(智能体)同共完成复杂的流程,
不那么复杂的系统可以用动态的 `Prompt`, `Tool`, 或 `Model` 来切换。多 Agent 的好处是每个 Agent 有自己的上下文，工具，专业领域的知识，
可以独立开发，分布式部署，多个 Agent 能并行执行，加速工作与烧钱的速度。

`LangChain` 在构建多 Agent 系统时，有以下几种常见模式：

1. Subagents(子 Agent): 主从 Agent 方式，子 Agent 作为主 Agent 的工具使用，由主 Agent 指挥子 Agent 干活
2. Handoffs(任务交接): 任务可转交给(以 Tool 方式)其他 Agent，其他 Agent 的结果可直接返回给用户, `hand off` 就是传球的意思
3. Skills(技能): 动态加载特定的 Prompt 和知识，用单一 `Agent` 控制按需加载技能，本质上它是单 Agent
4. Router(路由)：由一个路径控制任务如何分配给不同的 Agent，结果汇集成一个组合的响应
5. Custom workflow(自定义工作流)：用更底层的 `LangGraph` 定制工作流程，以上模型可嵌入到工作流中

下面贴上每种类型的组件和时序图(顺便给自己的 Hugo 加上 `tabs` 标签页功能，以方便对比切换和节约滚动条) <!--more-->

{{< tabs >}}
---tab Subagents
全部子 Agent 注册为主 Agent 的 `Tools`, 与整个系统的沟通都是通过主 Agent 进行的，子 Agent 把响应汇总到主 Agent 后反馈给用户。
易独产开发分布部署，子 Agent 可并行执行，特点是存在一个 `Super Agent`。有时候这种模式只为了隔离会话，比如像 `Sumarization` 和 `Mem0`
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

### Subagents

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
    result = subagent_py.invoke({"messages": {"role": "user", "content": query}})
    return result["messages"][-1].content


subagent_go = create_agent(
    model="ollama:llama3.2:1b"
)


@tool("go_language_agent", description="Write go language code")
def call_go_agent(query: str) -> str:
    result = subagent_go.invoke({"messages": {"role": "user", "content": query}})

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

从后台分别看到向 `gemma4:e4b`(python_agent), `llama3.2:1b`(go_agent) 发送了请求，请求内容为

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
不叮嘱好，`Super Agent` 就是像调用普通 `Tool` 那样，拿到结果后会进行总结，然后输出给客户。`SubAgents` 这种模式下最好是让子 `Agent` 只干活，
不说话。子 `Agent` 能直接与用户交互的方式就是 `Human-in-the-loop` 那种 `interrupts`, 可直接接收客户的输入确认，而后继续。

### Handoffs

