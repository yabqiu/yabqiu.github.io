---
title: "LangChain 核心组件之短期记忆"
url: /langchain-core-component-short-term-memory/
date: 2026-04-21T19:11:48-05:00
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

学习了 `Agent`, `Models` 之后直接跳到 `Short Term Memory` 节，短期记忆能让我们更好的理解与模型的交互, 为什么与机器人越聊到最后它可能就
偏离主题，智商降低了。前面所用的手动拼接整个会话历史和使用 `InMemorySaver` 就是模型的短期记忆，如果会话历史过长，超过上下文件大小，
导致上下文丢失或错误，这就需要对会话进行压缩，压缩做了些什么事情呢，这就是本文要学习的内容。

之前写过一篇 [LangChain - 关于会话记忆](/langchain-about-session-memory/), 也是关于短期记忆的，这里重新阅读官方的文档，也是加以巩固。

模型是没有记忆的，所有你和模型说过的话你都必须记住，记忆就是让 Agent 记住你与模型的会话历史，短期记忆特指单一会话的历史。短期记忆与长期记忆这两
个概念总会让人有所迷惑的，简单的可以这么理解，比如使用 ChatGPT 时

1. 在同一个 `Chat` 中的对话就是短期记忆，即便你一年后再回到那个没有删的 `Chat` 中接着聊，也是短期记忆
2. 而长期记忆是跨会话的，比如在某个 `Chat` 中说过喜欢猪头饭，再点 `New chat` 打开一个新的 `Chat`，问今天吃什么，模型直接建议吃猪头饭，这就是长期记忆

还是那句话，大语言模型是没有记忆的, 和大语言模型对话就像和一个失忆症的人对话, 每次问话都必需把之前的对话复述一遍, 过程相当于:<!--more-->

> "我曾经问了这个，你回复过那个，问过这个，你回复过那个......，我现在问一个新问题，请基于我们的会话历史作出回答"

有时候复述的太多, 对方处理不过来，太慢或抓不到题, 就要对会话历史进行总结, 这就是超过会话上下文大小要进行的会话压缩, 但压缩后同样会造成信息的不准确。

对于 `LangChain` 的 `model` 或 `agent` 来说，短期记忆也是下面的样子

```python
{"messages": [
    {"role": "system", "content": "..."},
    {"role": "user", "content": "..."},
    {"role": "assistant", "content": "..."},
    {"role": "tool", "content": "..."},
    {"role": "assistant", "content": "..."},
    {"role": "user", "content": "..."},
]}
```

如果用 `model.invoke()` 必须全手动拼接历史，而用 `agent.invoke()` 返回的 `result` 中会自动包含历史对话，新的问题只要添加到最后面就行。
而使用 `LangChain` 的 `checkpointer=InMemorySaver()` 就更自动化，只管 `agent.invoke()` 时传新的问话，历史会话自动添加到当前对话的前头。

看下面的例子

```python
from langchain.agents import create_agent
from langgraph.checkpoint.memory import InMemorySaver

agent = create_agent(
    "ollama:gemma4:e4b",
    checkpointer=InMemorySaver(),
)

config = {"configurable": {"thread_id": "1"}}

agent.invoke(
    {"messages": [{"role": "user", "content": "Hi! My name is Yanbin."}]},
    config = config,
)

result = agent.invoke(
    {"messages": [{"role": "user", "content": "please state my name"}]},
    config = config,
)

print(result["messages"][-1].content)
print(result)
```

第二次 `agent.invoke()` 问 `please state my name`，模型因为有短期记忆，所以能回答出 `Yanbin`。观察第二次 `agent.invoke()` 发送给模型
的请求数据

{{< highlight-wrap json >}}
{"model":"gemma4:e4b","stream":true,"options":{},"messages":[{"role":"user","content":"Hi! My name is Yanbin."},{"role":"assistant","content":"Hello Yanbin! It's nice to meet you. How can I help you today? 😊"},{"role":"user","content":"please state my name"}],"tools":[]}
{{< /highlight-wrap >}}

就能看到 `agent` 自动把前面的对话加到当前对话的前头，而不需要手动拼接历史, 再看第二次 `agent.invoke()` 的返回结果，其中的内容(消息部分)是

```text
{'messages': [
    HumanMessage(content='Hi! My name is Yanbin.'),
    AIMessage(content="Hello Yanbin! It's nice to meet you. How can I help you today? 😊"),
    HumanMessage(content='please state my name'),
    AIMessage(content='Your name is Yanbin.')
```

这就是 `checkpointer=InMemorySaver()` 所起的作用，每次调用 `agent.invoke()` 时指定 `config=config` 来关联 `"thread_id": 1` 的会话。
如果去掉 `checkpointer=InMemorySaver()` 和 `config=config` 参数，那么第二次 `agent.invoke()` 会得到一个全新的对话，模型就可能回答

> I do not know your name, as you have not told me.

在实际用用中，用 `InMemorySaver` 的话 `Agent` 一重启便丢失了会话历史，虽说是短期记忆，但也没有你想像的那么短，用户还是需要在将来某个时候重拾
原来的 `thread_id` 又能继续之前的对话。所以短期记忆的实现除了 `InMemorySaver` 外，还有其他能特久化会话的实现，如 `PostgresSaver`, `RedisSaver`
等其他存储介质的短期记忆实现。

下面测试一下 `sqlite3` 数据库，用文件存储, 需先 `langgraph-checkpoint-redis` Python 库

```python
from langchain.agents import create_agent
from langgraph.checkpoint.sqlite import SqliteSaver

with SqliteSaver.from_conn_string("langchain.db") as checkpointer:
    agent = create_agent(
        "ollama:gemma4:e4b",
        checkpointer=checkpointer,
    )

    config = {"configurable": {"thread_id": "1"}}

    agent.invoke(
        {"messages": [{"role": "user", "content": "Hi! My name is Yanbin."}]},
        config = config,
    )

    result = agent.invoke(
        {"messages": [{"role": "user", "content": "please state my name"}]},
        config = config,
    )

    print(result["messages"][-1].content)
```

执行完后产生了一个 `langchain.db` 文件，用 `sqlite3` 命令打开查看

```shell
sqlite3 langchain.db
SQLite version 3.51.0 2025-06-12 13:14:41
Enter ".help" for usage hints.
sqlite> .tables
checkpoints  writes     
sqlite> .schema checkpoints
CREATE TABLE checkpoints (
                thread_id TEXT NOT NULL,
                checkpoint_ns TEXT NOT NULL DEFAULT '',
                checkpoint_id TEXT NOT NULL,
                parent_checkpoint_id TEXT,
                type TEXT,
                checkpoint BLOB,
                metadata BLOB,
                PRIMARY KEY (thread_id, checkpoint_ns, checkpoint_id)
            );
sqlite> .schema writes
CREATE TABLE writes (
                thread_id TEXT NOT NULL,
                checkpoint_ns TEXT NOT NULL DEFAULT '',
                checkpoint_id TEXT NOT NULL,
                task_id TEXT NOT NULL,
                idx INTEGER NOT NULL,
                channel TEXT NOT NULL,
                type TEXT,
                value BLOB,
                PRIMARY KEY (thread_id, checkpoint_ns, checkpoint_id, task_id, idx)
            );
sqlite> select count(*) from checkpoints;
12
sqlite> select count(*) from writes;
12
sqlite> .headers on
sqlite> select * from writes;
thread_id|checkpoint_ns|checkpoint_id|task_id|idx|channel|type|value
1||1f13cf1a-d76c-622a-bfff-20eadd4e28c9|cd36dd28-31aa-2000-dcd3-c861335f1409|0|messages|msgpack|���role�user�content�Hi! My name is Yanbin.
1||1f13cf1a-d76c-622a-bfff-20eadd4e28c9|cd36dd28-31aa-2000-dcd3-c861335f1409|1|branch:to:model|null|
1||1f13cf1a-d76d-683c-8000-557b2de4647c|682ff5b2-9f91-10ee-3700-6cc8382f4bac|0|messages|msgpack|��^B�^E��langchain_core.messages.ai�AIMessage��content�yHi Yanbin! It's great to meet you. 😊

How can I help you today? Is there anything you'd like to chat about or work on?�additional_kwargs��response_metadata��model�gemma4:e4b�created_at�2026-04-20T19:47:02.115597Z�doneëdone_reason�stop�total_duration�
1||1f13cf1b-37b9-62ae-8002-c11e828de33b|403055ae-cc85-a4b8-77c3-3b4a3619a6cd|0|messages|msgpack|���role�user�content�please state my name
1||1f13cf1b-37b9-62ae-8002-c11e828de33b|403055ae-cc85-a4b8-77c3-3b4a3619a6cd|1|branch:to:model|null|
1||1f13cf1b-37ba-6186-8003-a5876bf645a4|ad82dfc5-9509-10e5-33c0-26037274f4e0|0|messages|msgpack|��^B^S^E��langchain_core.messages.ai�AIMessage��content�Your name is Yanbin.�additional_kwargs��response_metadata��model�gemma4:e4b�created_at�2026-04-20T19:47:05.104501Z�doneëdone_reason�stop�total_durationαɂ��load_duration�^H^Z2��prompt_eval_countI�prompt_eval_duration�^H���eval_count_�eval_durationΟ^G
```

里面都是序列化的数据，我们也可以用 `checkpointer.get(config)`, `checkpoint.list(config)` 读取其中的内容。

下面再来作个测试，`agent.invoke()` 时直接问 `please state my name`

```python
from langchain.agents import create_agent
from langgraph.checkpoint.sqlite import SqliteSaver

with SqliteSaver.from_conn_string("langchain.db") as checkpointer:
    agent = create_agent(
        "ollama:gemma4:e4b",
        checkpointer=checkpointer,
    )

    config = {"configurable": {"thread_id": "1"}}

    result = agent.invoke(
        {"messages": [{"role": "user", "content": "please state my name"}]},
        config = config,
    )

    print(result["messages"][-1].content)
```

得到结果

> Your name is Yanbin.

这就是持久化的 `SqliteSaver` 完成的工作，它会根据 `thread_id: 1` 把历史会话从 Sqlite 数据库中掏出来，完整的传递给模型。最后的问话相当于

```json
{ "messages": [
  {"role":"user","content":"Hi! My name is Yanbin."},
  {"role":"assistant","content":"Hi Yanbin! It's very nice to meet you. 😊\n\nHow can I help you today? Feel free to ask me anything!"},
  {"role":"user","content":"please state my name"},
  {"role":"assistant","content":"Your name is Yanbin. 😊"},
  {"role":"user","content":"please state my name"},
  {"role":"assistant","content":"Your name is Yanbin."},
  {"role":"user","content":"please state my name"},
  {"role":"assistant","content":"Your name is Yanbin."}
]}
```

再继续问相同的问题，后面又会在历史中追加

```json
  {"role":"user","content":"please state my name"},
  {"role":"assistant","content":"Your name is Yanbin."}
```

这时候我们应该意识到这种重复的会话是多余的，`LangChain` 的 `Agent` 可以指定的从历史中移除消息，或者给定条件对会话进行总结(压缩)，后面会学到。

### 定制 Agent 记忆

`LangChain` 是用 `AgentState` 管理的短期记忆，可继承 `AgentState` 往记忆中存中额外的信息，借鉴用官方的例子，但是存储在 Sqlite 数据库中

```python
from langchain.agents import create_agent, AgentState
from langgraph.checkpoint.sqlite import SqliteSaver

class CustomAgentState(AgentState):
    user_id: str
    preferences: dict

with SqliteSaver.from_conn_string("langchain.db") as checkpointer:
    agent = create_agent(
        "ollama:gemma4:e4b",
        state_schema=CustomAgentState,
        checkpointer=checkpointer,
    )

    # Custom state can be passed in invoke
    result = agent.invoke(
        {
            "messages": [{"role": "user", "content": "Hello"}],
            "user_id": "user_123",
            "preferences": {"theme": "dark"}
        },
        {"configurable": {"thread_id": "1"}})
```

这样在记忆中就会保存额外的 `user_id` 和 `preferences` 字段信息。执行后查看 `langchain.db` 中的 `writes` 表

```shell
sqlite> select * from writes;
1||1f13cf48-a5a7-6b9e-bfff-2e061d76876a|736b1865-6e7b-bb4a-07d0-013d0fb99547|1|user_id|msgpack|�user_123
1||1f13cf48-a5a7-6b9e-bfff-2e061d76876a|736b1865-6e7b-bb4a-07d0-013d0fb99547|2|preferences|msgpack|��theme�dark
```

看到上面额外的两条记录。

### 会话的裁剪与压缩

当使用了短期记忆后，会话历史或数据太长，为避免超过 `LLM` 的上下文窗口，可采用以下方式处理

1. 裁剪消息，移除前面或后面若干条消息
2. 删除消息，直接从 `LangGraph` 的 `state` 中清除
3. 总结会话，将历史消息压缩成摘要信息
4. 定制策略：例如消息过滤

模型总有一个上下文窗口大小，或大或小，例如 `ollama:gemma4:e4b` 的上下文窗口是 `128k`, 即 128k Token, Token 不完全等同于单词(英文)
和汉字的数目，存在相关性。

我们来试下超过上下文容器大小会出现什么情况, 如果把 "Hello " 重复  1024*1024 遍

```python
agent = create_agent("ollama:gemma4:e4b")
result = agent.invoke({"messages": [{"role": "user", "content": "Hello " * 1024 * 1024}]})
```

`ollama:gemma4:e4b` 没有任何抱怨，最后的结果说 `input_tokens` 是 32768, 怎么不是 1024*1024=1M 呢，是因为重复的？

> usage_metadata={'input_tokens': 32768, 'output_tokens': 123, 'total_tokens': 32891}

把 红楼梦.txt 放进去，

```python
with open("红楼梦.txt") as f:
    content = f.read()

agent = create_agent("ollama:gemma4:e4b")
result = agent.invoke({"messages": [{"role": "user", "content": content}]})
```

也没问题，也是 `usage_metadata={'input_tokens': 32768, 'output_tokens': 614, 'total_tokens': 33382})`, 看来是 `agent` 直接把
输入截断的, 所以实际上它的上下文窗口大小是 32k, 通过环境变量 `OLLAMA_CONTEXT_LENGTH=32768` 可修改 `ollama serve` 的上下文大小。

再次测试

```python
from langchain.agents import create_agent

agent = create_agent("ollama:gemma4:e4b")
result = agent.invoke({"messages": [{
    "role": "user",
    "content": "My name is Yanbin, please state my name" + ("Hello " * 32768 * 2)}]
})

print(len(result["messages"]))
for message in result["messages"]:
    print(len(message.content))

print(result["messages"][-1].content)
```

输出

```text
2
393255
52
Can you please specify what you would like me to do?
```

说明 `Ollama` 直接把前面的内容截掉了，393255 / 6 =  65542.5, 这个长度包含全部字符。

再试下

```python
result = agent.invoke({"messages": [{
    "role": "user",
    "content": "My name is Yanbin, please state my name" + ("Hello " * 3276 * 2)}]
})
```

现在没问题了

```text
2
39351
20
Your name is Yanbin.
```

`Ollama` 这种超过上下文窗口不报错却回答不了问题的坑只能告诉 `Agent` 事前检查输入大小。 不同的模型在处理超过上下文窗口时的行为有所差异.

#### 删除消息

如果是用的 `model.invoke()` 或 `result=agent.invoke()`, 自己拼接历史会话的话，可选择性(比如从 UI 选择)的从 `result["messages"]`
中删除某条消息，你模型指示调用工具的 `AIMessage` 也可以删除掉。下面看看用中间件的方式如何操作，首先来理解官方例子的用意

```python
from langchain.messages import RemoveMessage
from langgraph.graph.message import REMOVE_ALL_MESSAGES
from langgraph.checkpoint.memory import InMemorySaver
from langchain.agents import create_agent, AgentState
from langchain.agents.middleware import before_model
from langgraph.runtime import Runtime
from langchain_core.runnables import RunnableConfig
from typing import Any


@before_model
def trim_messages(state: AgentState, runtime: Runtime) -> dict[str, Any] | None:
    """Keep only the last few messages to fit context window."""
    messages = state["messages"]

    if len(messages) <= 3:
        return None  # No changes needed

    first_msg = messages[0] # 通常第一条消息最重要，特别是 sytem_prompt 时
    
    # 消息数为偶数时保留最后 3 条消息，为奇数时保留最后 4 条消息
    recent_messages = messages[-3:] if len(messages) % 2 == 0 else messages[-4:]
    new_messages = [first_msg] + recent_messages

    return {
        "messages": [
            RemoveMessage(id=REMOVE_ALL_MESSAGES),
            *new_messages
        ]
    }

agent = create_agent(
    "ollama:gemma4:e4b",
    checkpointer=InMemorySaver(),
    middleware=[trim_messages],
)

config: RunnableConfig = {"configurable": {"thread_id": "1"}}

agent.invoke({"messages": "hi, my name is bob"}, config)                 #1
agent.invoke({"messages": "write a short poem about cats"}, config)      #2
agent.invoke({"messages": "now do the same but for dogs"}, config)       #3
final_response = agent.invoke({"messages": "what's my name?"}, config)   #4

final_response["messages"][-1].pretty_print()
```

下面对每次 `agent.invoke()`, 函数 `trim_messages()` `messages` 的内容以及从中的返回值与发往模型的消息列表

#1, `trim_messages()` 中 `messages` 的内容为

```text
[HumanMessage(content='hi, my name is bob', additional_kwargs={}, response_metadata={}, id='d573df9d-3a4f-4c63-917c-540383fab41b')]
```
*以下列出消息时只显示消息类型与 `content`, 其它字段省略*

从 `trim_message()` 中返回 `None`, 表示不对消息进行裁剪, 所以发往模型的请求是

```json
{"model":"gemma4:e4b","stream":true,"options":{},"messages":[{"role":"user","content":"hi, my name is bob"}],"tools":[]}
```
*后面展示发往模型的消息时也只显示 `messages` 中的内容，仅包括 `role` 和 `content` 字段*

#2, `trim_messages()` 中 `messages` 的内容为

```text
HumanMessage(content='hi, my name is bob'),
AIMessage(content="Hi Bob! It's nice to meet you. 😊 How can I help you today?"),
HumanMessage(content='write a short poem about cats')
```

因为 `len(messages)` 等于 3，所以从 `trim_message()` 中仍然返回 `None`, 表示不对消息进行裁剪, 发往模型的消息是

```json
{"role":"user","content":"hi, my name is bob"},
{"role":"assistant","content":"Hi Bob! It's nice to meet you. 😊 How can I help you today?"},
{"role":"user","content":"write a short poem about cats"}
```
#3, `trim_messages()` 中 `messages` 的内容为

```text
HumanMessage(content='hi, my name is bob'),
AIMessage(content="Hi Bob! It's nice to meet you. 😊 How can I help you today?"),
HumanMessage(content='write a short poem about cats'),
AIMessage(content="Here are a few options, depending on the mood you'd like—sweet, playful, or slightly mysterious!\n\n***\n\n### 🐾 Option 1: The Cozy Nap (Sweet and Gentle)\n\nA patch of sun, a gentle curl,\nA velvet dream in a sleepy world.\nWith purrs that hum a steady tune,\nThey guard the day beneath the moon\nOf a pillow soft and warm,\nSafe from every passing storm.\nOh, little cat, with fur so deep,\nSweetest little secrets you keep.\n\n***\n\n### 🌪️ Option 2: The Hunter (Playful and Energetic)\n\nA whisker twitch, a silent grace,\nA hunter poised in time and space.\nThey stalk the dust, they chase a string,\nA sudden joyful, leaping spring.\nWith velvet paw and emerald eye,\nA jungle king beneath the sky.\nThen curled up deep, they drift to sleep,\nWhile silent jungle secrets keep.\n\n***\n\n### ✨ Option 3: The Majestic Spirit (Poetic and Mysterious)\n\nA shadow sleek, a midnight sheen,\nThe queen of all the domestic scene.\nWith ancient grace, and knowing stare,\nA wild heart wrapped in velvet hair.\nThey walk as ghosts, aloof and fine,\nA mystery and a perfect sign.\nA silent watch, a gentle purr,\nThe soul of comfort, night and blur."),
HumanMessage(content='now do the same but for dogs')
```

现在 `len(messages)` 是 5，要进入 `trim_message()` 方法后的的逻辑，从 `trim_messages()` 中返回的值为

```text
{'messages': [
    RemoveMessage(content='', additional_kwargs={}, response_metadata={}, id='__remove_all__'),
    HumanMessage(content='hi, my name is bob', additional_kwargs={}, response_metadata={}, id='31b27786-bdbb-48c9-9245-cc44b01d51e3'),
    AIMessage(content="Hi Bob! It's nice to meet you. 😊 How can I help you today?", additional_kwargs={}, response_metadata={'model': 'gemma4:e4b', 'created_at': '2026-04-21T03:12:52.927078Z', 'done': True, 'done_reason': 'stop', 'total_duration': 6811977125, 'load_duration': 6108695834, 'prompt_eval_count': 22, 'prompt_eval_duration': 104717709, 'eval_count': 20, 'eval_duration': 512618206, 'logprobs': None, 'model_name': 'gemma4:e4b', 'model_provider': 'ollama'}, id='lc_run--019dae06-ac16-7cb2-bd08-9c397c542ca1-0', tool_calls=[], invalid_tool_calls=[], usage_metadata={'input_tokens': 22, 'output_tokens': 20, 'total_tokens': 42}), 
    HumanMessage(content='write a short poem about cats', additional_kwargs={}, response_metadata={}, id='e95f2c2d-082b-45a8-b2a6-f2c050b4034e'), 
    AIMessage(content="Here are a few options, depending on the mood you'd like—sweet, playful, or slightly mysterious!\n\n***\n\n### 🐾 Option 1: The Cozy Nap (Sweet and Gentle)\n\nA patch of sun, a gentle curl,\nA velvet dream in a sleepy world.\nWith purrs that hum a steady tune,\nThey guard the day beneath the moon\nOf a pillow soft and warm,\nSafe from every passing storm.\nOh, little cat, with fur so deep,\nSweetest little secrets you keep.\n\n***\n\n### 🌪️ Option 2: The Hunter (Playful and Energetic)\n\nA whisker twitch, a silent grace,\nA hunter poised in time and space.\nThey stalk the dust, they chase a string,\nA sudden joyful, leaping spring.\nWith velvet paw and emerald eye,\nA jungle king beneath the sky.\nThen curled up deep, they drift to sleep,\nWhile silent jungle secrets keep.\n\n***\n\n### ✨ Option 3: The Majestic Spirit (Poetic and Mysterious)\n\nA shadow sleek, a midnight sheen,\nThe queen of all the domestic scene.\nWith ancient grace, and knowing stare,\nA wild heart wrapped in velvet hair.\nThey walk as ghosts, aloof and fine,\nA mystery and a perfect sign.\nA silent watch, a gentle purr,\nThe soul of comfort, night and blur.", additional_kwargs={}, response_metadata={'model': 'gemma4:e4b', 'created_at': '2026-04-21T03:18:15.506986Z', 'done': True, 'done_reason': 'stop', 'total_duration': 23219357750, 'load_duration': 161442208, 'prompt_eval_count': 57, 'prompt_eval_duration': 1565732875, 'eval_count': 727, 'eval_duration': 21226697504, 'logprobs': None, 'model_name': 'gemma4:e4b', 'model_provider': 'ollama'}, id='lc_run--019dae0b-5813-7ec2-87be-5ca9ac46001f-0', tool_calls=[], invalid_tool_calls=[], usage_metadata={'input_tokens': 57, 'output_tokens': 727, 'total_tokens': 784}), 
    HumanMessage(content='now do the same but for dogs', additional_kwargs={}, response_metadata={}, id='3fca7886-d421-44b1-b833-76030f742ea4')]}
```

按照 `recent_messages` 的逻辑，偶数取最后三条，奇数取最后四条，当前 `len(messages)` 为 5，取最后四条，再加上第一条，结果是所有的 5 条记录都保留了，
反而最前面多了一条 `RemoveMessage(id='__remove_all__')`， 不知有何用意，看它发往模型的消息

```json
{"role":"user","content":"hi, my name is bob"},
{"role":"assistant","content":"Hi Bob! It's nice to meet you. 😊 How can I help you today?"},
{"role":"user","content":"write a short poem about cats"},
{"role":"assistant","content":"Here are a few options, depending on the mood you'd like—sweet, playful, or slightly mysterious!\n\n***\n\n### 🐾 Option 1: The Cozy Nap (Sweet and Gentle)\n\nA patch of sun, a gentle curl,\nA velvet dream in a sleepy world.\nWith purrs that hum a steady tune,\nThey guard the day beneath the moon\nOf a pillow soft and warm,\nSafe from every passing storm.\nOh, little cat, with fur so deep,\nSweetest little secrets you keep.\n\n***\n\n### 🌪️ Option 2: The Hunter (Playful and Energetic)\n\nA whisker twitch, a silent grace,\nA hunter poised in time and space.\nThey stalk the dust, they chase a string,\nA sudden joyful, leaping spring.\nWith velvet paw and emerald eye,\nA jungle king beneath the sky.\nThen curled up deep, they drift to sleep,\nWhile silent jungle secrets keep.\n\n***\n\n### ✨ Option 3: The Majestic Spirit (Poetic and Mysterious)\n\nA shadow sleek, a midnight sheen,\nThe queen of all the domestic scene.\nWith ancient grace, and knowing stare,\nA wild heart wrapped in velvet hair.\nThey walk as ghosts, aloof and fine,\nA mystery and a perfect sign.\nA silent watch, a gentle purr,\nThe soul of comfort, night and blur."},
{"role":"user","content":"now do the same but for dogs"}
```
如果没有 `trim_messages()` 的话，应该发往模型的消息都还在，也就是说
`RemoveMessage(content='', additional_kwargs={}, response_metadata={}, id='__remove_all__'),` 没起来任何作用。

#4, `trim_messages()` 中 `messages` 的内容为

```text
HumanMessage(content='hi, my name is bob'),
AIMessage(content="Hi Bob! It's nice to meet you. 😊 How can I help you today?"),
HumanMessage(content='write a short poem about cats'),
AIMessage(content="Here are a few options, depending on the mood you'd like—sweet, playful, or slightly mysterious!\n\n***\n\n### 🐾 Option 1: The Cozy Nap (Sweet and Gentle)\n\nA patch of sun, a gentle curl,\nA velvet dream in a sleepy world.\nWith purrs that hum a steady tune,\nThey guard the day beneath the moon\nOf a pillow soft and warm,\nSafe from every passing storm.\nOh, little cat, with fur so deep,\nSweetest little secrets you keep.\n\n***\n\n### 🌪️ Option 2: The Hunter (Playful and Energetic)\n\nA whisker twitch, a silent grace,\nA hunter poised in time and space.\nThey stalk the dust, they chase a string,\nA sudden joyful, leaping spring.\nWith velvet paw and emerald eye,\nA jungle king beneath the sky.\nThen curled up deep, they drift to sleep,\nWhile silent jungle secrets keep.\n\n***\n\n### ✨ Option 3: The Majestic Spirit (Poetic and Mysterious)\n\nA shadow sleek, a midnight sheen,\nThe queen of all the domestic scene.\nWith ancient grace, and knowing stare,\nA wild heart wrapped in velvet hair.\nThey walk as ghosts, aloof and fine,\nA mystery and a perfect sign.\nA silent watch, a gentle purr,\nThe soul of comfort, night and blur."), 
HumanMessage(content='now do the same but for dogs'),
AIMessage(content='Of course! Here are a few options for dogs, ranging from sweet and loyal to high-energy and majestic.\n\n***\n\n### 💖 Option 1: The Loyal Companion (Sweet and Heartfelt)\n\nA happy nose upon the ground,\nWhere purest love can always be found.\nA tail that wags a rhythmic beat,\nThe greeting joy, impossibly sweet.\nWith muddy paw and gentle sigh,\nThey watch the changing seasons fly.\nA furry shield, a faithful friend,\nA perfect love that knows no end.\n\n***\n\n### 🏃 Option 2: The Joyful Explorer (Playful and Energetic)\n\nThe leash is pulled, the journey starts,\nA blast of joy within their hearts!\nThey race the wind with foamy breath,\nDefying worries, conquering death.\nA ball retrieves, a muddy sprint,\nA playful, tireless, joyous hint.\nFrom puppy romp to doggy dash,\nA blur of fun, a happy flash.\n\n***\n\n### 👑 Option 3: The Steadfast Guardian (Poetic and Deep)\n\nWith soulful gaze and eager paw,\nThey keep the watch and heed the law\nOf simple faith and truest grace,\nThe loyal friend in time and place.\nThey ask for naught but a loving hand,\nThe safest soul in all the land.\nA gentle watch, a patient plea,\nThe heart of dog, eternally.'), 
HumanMessage(content="what's my name?")
```

现在 `len(messages)` 是 7，要进入 `trim_message()` 方法后的的逻辑，从 `trim_messages()` 中返回的值为

```text
{'messages': [
    RemoveMessage(content='', additional_kwargs={}, response_metadata={}, id='__remove_all__'),
    HumanMessage(content='hi, my name is bob', additional_kwargs={}, response_metadata={}, id='31b27786-bdbb-48c9-9245-cc44b01d51e3'),
    AIMessage(content="Here are a few options, depending on the mood you'd like—sweet, playful, or slightly mysterious!\n\n***\n\n### 🐾 Option 1: The Cozy Nap (Sweet and Gentle)\n\nA patch of sun, a gentle curl,\nA velvet dream in a sleepy world.\nWith purrs that hum a steady tune,\nThey guard the day beneath the moon\nOf a pillow soft and warm,\nSafe from every passing storm.\nOh, little cat, with fur so deep,\nSweetest little secrets you keep.\n\n***\n\n### 🌪️ Option 2: The Hunter (Playful and Energetic)\n\nA whisker twitch, a silent grace,\nA hunter poised in time and space.\nThey stalk the dust, they chase a string,\nA sudden joyful, leaping spring.\nWith velvet paw and emerald eye,\nA jungle king beneath the sky.\nThen curled up deep, they drift to sleep,\nWhile silent jungle secrets keep.\n\n***\n\n### ✨ Option 3: The Majestic Spirit (Poetic and Mysterious)\n\nA shadow sleek, a midnight sheen,\nThe queen of all the domestic scene.\nWith ancient grace, and knowing stare,\nA wild heart wrapped in velvet hair.\nThey walk as ghosts, aloof and fine,\nA mystery and a perfect sign.\nA silent watch, a gentle purr,\nThe soul of comfort, night and blur.", additional_kwargs={}, response_metadata={'model': 'gemma4:e4b', 'created_at': '2026-04-21T03:18:15.506986Z', 'done': True, 'done_reason': 'stop', 'total_duration': 23219357750, 'load_duration': 161442208, 'prompt_eval_count': 57, 'prompt_eval_duration': 1565732875, 'eval_count': 727, 'eval_duration': 21226697504, 'logprobs': None, 'model_name': 'gemma4:e4b', 'model_provider': 'ollama'}, id='lc_run--019dae0b-5813-7ec2-87be-5ca9ac46001f-0', tool_calls=[], invalid_tool_calls=[], usage_metadata={'input_tokens': 57, 'output_tokens': 727, 'total_tokens': 784}), 
    HumanMessage(content='now do the same but for dogs', additional_kwargs={}, response_metadata={}, id='3fca7886-d421-44b1-b833-76030f742ea4'), 
    AIMessage(content='Of course! Here are a few options for dogs, ranging from sweet and loyal to high-energy and majestic.\n\n***\n\n### 💖 Option 1: The Loyal Companion (Sweet and Heartfelt)\n\nA happy nose upon the ground,\nWhere purest love can always be found.\nA tail that wags a rhythmic beat,\nThe greeting joy, impossibly sweet.\nWith muddy paw and gentle sigh,\nThey watch the changing seasons fly.\nA furry shield, a faithful friend,\nA perfect love that knows no end.\n\n***\n\n### 🏃 Option 2: The Joyful Explorer (Playful and Energetic)\n\nThe leash is pulled, the journey starts,\nA blast of joy within their hearts!\nThey race the wind with foamy breath,\nDefying worries, conquering death.\nA ball retrieves, a muddy sprint,\nA playful, tireless, joyous hint.\nFrom puppy romp to doggy dash,\nA blur of fun, a happy flash.\n\n***\n\n### 👑 Option 3: The Steadfast Guardian (Poetic and Deep)\n\nWith soulful gaze and eager paw,\nThey keep the watch and heed the law\nOf simple faith and truest grace,\nThe loyal friend in time and place.\nThey ask for naught but a loving hand,\nThe safest soul in all the land.\nA gentle watch, a patient plea,\nThe heart of dog, eternally.', additional_kwargs={}, response_metadata={'model': 'gemma4:e4b', 'created_at': '2026-04-21T03:34:07.839703Z', 'done': True, 'done_reason': 'stop', 'total_duration': 27113589542, 'load_duration': 6156771084, 'prompt_eval_count': 370, 'prompt_eval_duration': 677133042, 'eval_count': 658, 'eval_duration': 19910739711, 'logprobs': None, 'model_name': 'gemma4:e4b', 'model_provider': 'ollama'}, id='lc_run--019dae19-d0ee-7891-aa2c-ba0d026fe7da-0', tool_calls=[], invalid_tool_calls=[], usage_metadata={'input_tokens': 370, 'output_tokens': 658, 'total_tokens': 1028}), 
    HumanMessage(content="what's my name?", additional_kwargs={}, response_metadata={}, id='e8397792-6eb3-466a-b750-211df66c34c9')]}
```

第一条消息加上最后 4 条消息，把中间两条消息去掉了，为什么一定要加上，`RemoveMessage(id='__remove_all__')`， 这条消息呢？，看它发往模型的消息

```json
{"role":"user","content":"hi, my name is bob"},
{"role":"assistant","content":"Here are a few options, depending on the mood you'd like—sweet, playful, or slightly mysterious!\n\n***\n\n### 🐾 Option 1: The Cozy Nap (Sweet and Gentle)\n\nA patch of sun, a gentle curl,\nA velvet dream in a sleepy world.\nWith purrs that hum a steady tune,\nThey guard the day beneath the moon\nOf a pillow soft and warm,\nSafe from every passing storm.\nOh, little cat, with fur so deep,\nSweetest little secrets you keep.\n\n***\n\n### 🌪️ Option 2: The Hunter (Playful and Energetic)\n\nA whisker twitch, a silent grace,\nA hunter poised in time and space.\nThey stalk the dust, they chase a string,\nA sudden joyful, leaping spring.\nWith velvet paw and emerald eye,\nA jungle king beneath the sky.\nThen curled up deep, they drift to sleep,\nWhile silent jungle secrets keep.\n\n***\n\n### ✨ Option 3: The Majestic Spirit (Poetic and Mysterious)\n\nA shadow sleek, a midnight sheen,\nThe queen of all the domestic scene.\nWith ancient grace, and knowing stare,\nA wild heart wrapped in velvet hair.\nThey walk as ghosts, aloof and fine,\nA mystery and a perfect sign.\nA silent watch, a gentle purr,\nThe soul of comfort, night and blur."},
{"role":"user","content":"now do the same but for dogs"},
{"role":"assistant","content":"Of course! Here are a few options for dogs, ranging from sweet and loyal to high-energy and majestic.\n\n***\n\n### 💖 Option 1: The Loyal Companion (Sweet and Heartfelt)\n\nA happy nose upon the ground,\nWhere purest love can always be found.\nA tail that wags a rhythmic beat,\nThe greeting joy, impossibly sweet.\nWith muddy paw and gentle sigh,\nThey watch the changing seasons fly.\nA furry shield, a faithful friend,\nA perfect love that knows no end.\n\n***\n\n### 🏃 Option 2: The Joyful Explorer (Playful and Energetic)\n\nThe leash is pulled, the journey starts,\nA blast of joy within their hearts!\nThey race the wind with foamy breath,\nDefying worries, conquering death.\nA ball retrieves, a muddy sprint,\nA playful, tireless, joyous hint.\nFrom puppy romp to doggy dash,\nA blur of fun, a happy flash.\n\n***\n\n### 👑 Option 3: The Steadfast Guardian (Poetic and Deep)\n\nWith soulful gaze and eager paw,\nThey keep the watch and heed the law\nOf simple faith and truest grace,\nThe loyal friend in time and place.\nThey ask for naught but a loving hand,\nThe safest soul in all the land.\nA gentle watch, a patient plea,\nThe heart of dog, eternally."},
{"role":"user","content":"what's my name?"}],"tools":[]}
```

从面的分析来看, `RemoveMessage(content='', additional_kwargs={}, response_metadata={}, id='__remove_all__'),` 好真的没用，
它用了一个特殊的 `id='__remove_all__'`, 从表面上并不会删除任何的消息。 没有`RemoveMessage` 行不行呢？

实际测试，删除 `RemoveMessage(id=REMOVE_ALL_MESSAGES)`， 看最后一次 `#4` 发往模型的消息是什么？

```json
{"role":"user","content":"hi, my name is bob"},
{"role":"assistant","content":"Hello Bob! It's nice to meet you. How can I help you today? 😊"},
{"role":"user","content":"write a short poem about cats"},
{"role":"assistant","content":"Here are a few options, depending on whether you want a poem about their grace, their sleepiness, or just the general feeling of having a cat around!\n\n***\n\n### 🐾 The Graceful Shadow (Focus on Mystery)\n\nA velvet curve upon the sill,\nWith silent paws and watchful skill.\nA flick of ear, a graceful leap,\nWhile furry secrets softly sleep.\nThey stalk the light, they own the day,\nThe lovely cat who drifts away.\n\n***\n\n### 😴 The Purring Dreamer (Focus on Comfort)\n\nA knot of fluff, a furry round,\nContent to slumber, soft and sound.\nThey stretch and yawn in sunlit haze,\nAnd dream of endless, lazy days.\nThen rumble loud, a steady purr,\nThe warmest friend that will occur.\n\n***\n\n### ✨ Short & Sweet\n\nA whiskered face, a gentle glance,\nA perfect purr, a soft expanse.\nThey rule the house with silent art,\nAnd own the beating of your heart."},
{"role":"user","content":"now do the same but for dogs"},
{"role":"assistant","content":"Here are a few options for poems about dogs, again, depending on whether you want to capture their boundless energy, their loyal love, or their happy greeting!\n\n***\n\n### ☀️ The Energetic Companion (Focus on Play and Joy)\n\nA burst of fur, a joyous sound,\nAs happy paws run on the ground.\nA leap for joy, a playful chase,\nWith boundless spring and joyful grace.\nThey drag a toy, they wait for fetch,\nThe perfect, eager, loving stretch!\n\n***\n\n### ❤️ The Loyal Heart (Focus on Unconditional Love)\n\nA greeting loud, a wet nose bump,\nA silent, loving, gentle jump.\nA wagging tail that tells a tale,\nOf friendship that will never fail.\nThey rest nearby, a steadfast friend,\nA love that has no end.\n\n***\n\n### ✨ Short & Sweet (General Tribute)\n\nWith clumsy paws and watchful stare,\nA spirit wild beyond compare.\nA gentle nudge, a bark of cheer,\nTo make the smallest moment dear.\nThe joyful dog, a gift so bright,\nAnd sunshine wrapped in fur delight."},
{"role":"user","content":"what's my name?"}],"tools":[]}
```
所以 `RemoveMessage(id=REMOVE_ALL_MESSAGES)` 是会从 `LangGraph` 的 `state` 中删除所有消息，如果没有这个特殊的 `RemoveMessage`,
则即使从 `trim_messages()` 中返回第一条加上后四条消息，其他的消息其实还是存在的，仍然会发往模型，这也是为什么从中返回 `None`
不会对状态产生任何影响，而不需要总是返回需要的消息。

`RemoveMessage` 的特殊 id `__remove_all__` 会删除所有消息，显然也可指定某个实际的 id. 理解了 `RemoveMessage` 的用途，我们就改写前面的
`trim_messages()` 函数为

```python
@before_model
def trim_messages(state: AgentState, runtime: Runtime) -> dict[str, Any] | None:
    messages = state["messages"]

    new_messages = [messages[0]]
    for message in messages[1:]:
        new_messages.append(message)
        new_messages.append(RemoveMessage(content='', id=message.id))

    return {"messages": new_messages}
```

只要不是第一条消息，原样的加到 `new_messages` 中，再追加一个与它同 `id` 的 `RemoveMessage` 声明对它的删除， 这样不管 `trim_messages()`
返回多少条消息，最后发往模型的消息只有第一条，其他的消息都被逐标记删除了。对模型的请求总是

> {"model":"gemma4:e4b","stream":true,"options":{},"messages":[{"role":"user","content":"hi, my name is bob"}],"tools":[]}

我们也能把中间件函数用 `@after_model` 装饰，用以在与模型结述对话进删除一些旧的消息，也是用 `RemoveMessage(content='', id="<message_id>"')`
对会话历史进行清理，不能简单的只返回所需的消息。

#### 会话总结

消息的过滤(删除)操作的单位是消息，实际对话中，有时并不容易确定哪条消息就不重要，也许删除任何一条消息都会让会话重复相同的话或跑题，所以就有总结

{{< bundle-image langchain-conversation-summary.png 609 >}}

会话总结是整个会话历史进行语义上的概要，而不是轻易的放弃某些消息，所以在总结的时候也要选个小模型来处理。会话总结类似于许多编程 Agent 的 `/compact` 命令。

```python
from typing import Any

from langchain.agents import create_agent
from langchain.agents.middleware import SummarizationMiddleware
from langgraph.checkpoint.memory import InMemorySaver
from langchain_core.runnables import RunnableConfig

checkpointer = InMemorySaver()

agent = create_agent(
    model="ollama:gemma4:e4b",
    middleware=[
        SummarizationMiddleware(
            model="ollama:llama3.2:1b",
            trigger=("messages", 5),
            keep=("messages", 3)
        )
    ],
    checkpointer=checkpointer,
)

config: RunnableConfig = {"configurable": {"thread_id": "1"}}

def ask_llm(query: str) -> Any:
    result = agent.invoke({"messages": query}, config)
    print(f"after ask '{query}', messages: {len(result["messages"])}")
    return result

ask_llm("hi, my name is bob")
ask_llm("write a short poem about cats")
ask_llm("now do the same but for dogs")
final_response = ask_llm("what's my name?")

final_response["messages"][-1].pretty_print()
```

以上代码如果没有 `SummarizationMiddleware` 的话，每次对话后的消息数分别是

> after ask 'hi, my name is bob', messages: 2<br/>
> after ask 'write a short poem about cats', messages: 4<br/>
> after ask 'now do the same but for dogs', messages: 6<br/>
> after ask 'what's my name?', messages: 8
> ================================== Ai Message ================================== <br/>
> <br/>
> Here are a few options,

所以对 `SummarizationMiddleware` 设定消息数达到 5 条就触发总结，至少保留 3 条消息，启用 `middleware` 后，进行观察

> after ask 'hi, my name is bob', messages: 2<br/>
> after ask 'write a short poem about cats', messages: 4<br/>
> after ask 'now do the same but for dogs', messages: 5<br/>
> after ask 'what's my name?', messages: 5<br/>
> ================================== Ai Message ==================================<br/>
> <br/>
> I do not know your name. You have not told me what it is yet!<br/>
> <br/>
> Would you like to tell me? 😊

消息是控制在不超过 5 条，可惜最后，很遗憾这次总结把 `my name is bob` 给总结没了。

看看后台发生了什么，逐条查看发送给模型的消息, 稍微对消息进行了格式化，方便阅读，

第一个请求， 正常对话

{{< highlight-wrap json >}}
{"model":"gemma4:e4b","stream":true,"options":{},"messages":[
    {"role":"user","content":"hi, my name is bob"}
],"tools":[]}
{{< /highlight-wrap >}}


第二个请求, 还是正常的对话

{{< highlight-wrap json >}}
{"model":"gemma4:e4b","stream":true,"options":{},"messages":[
    {"role":"user","content":"hi, my name is bob"},
    {"role":"assistant","content":"Hello Bob! It's nice to meet you. How can I help you today? 😊"},
    {"role":"user","content":"write a short poem about cats"}
],"tools":[]}
{{< /highlight-wrap >}}

第三个请求，触发的总结, 在询问另一个模型 `llama3.2:1b`

{{< highlight-wrap json >}}
{"model":"llama3.2:1b","stream":true,"options":{},"messages":[
    {"role":"user","content":"<role>\nContext Extraction Assistant\n</role>\n\n<primary_objective>\n
    Your sole objective in this task is to extract the highest quality/most relevant context from the conversation history below.\n
        <这里省略了一大段用于总结的提示词, 后面是要总结的原始消息内容>
    <messages>\nMessages to summarize:\nHuman: hi, my name is bob\nAI: Hello Bob! It's nice to meet you. How can I help you today? 😊\n
    </messages>"}
]}
{{< /highlight-wrap >}}

总结是有专门的系统提示词 [`DEFAULT_SUMMARY_PROMPT`](https://github.com/langchain-ai/langchain/blob/master/libs/langchain_v1/langchain/agents/middleware/summarization.py#L33).

第四个请求，使用总结中的内容，加上新的问题询问主模型

{{< highlight-wrap json >}}
{"model":"gemma4:e4b","stream":true,"options":{},"messages":[
    {"role":"user","content":"Here is a summary of the conversation to date:\n\n## SESSION INTENT\nThe user's primary goal in this conversation is to request information about a topic.\n\n## SUMMARY\n- The user wants to know about the highest quality/most relevant context from their message history.\n- No specific choices or conclusions were made during the session; \n- The user did not explicitly reject any options and chose not to pursue them.\n- None"},
    {"role":"user","content":"write a short poem about cats"},
    {"role":"assistant","content":"I would be happy to! Here are a few different options, depending on the mood you want—a cozy one, an elegant one, or a funny one!\n\n***\n\n### 🐾 The Cozy Cat (Focus on Comfort)\n\nA sunbeam nap, a gentle sigh,\nWhere golden fur meets watchful eye.\nA rumble soft, a purring sound,\nThe sweetest comfort to be found.\nContent and curled within a mound,\nThe perfect kitty to surround.\n\n***\n\n### ✨ The Elegant Cat (Focus on Grace)\n\nA fluid shadow, silk and night,\nA hunter draped in soft moonlight.\nWith silent paw and graceful leap,\nThrough dusty corners, swift and deep.\nAn ancient charm, a mystery bright,\nA creature of pure, feline might.\n\n***\n\n### 😂 The Quirky Cat (Focus on Demands)\n\nA flicking ear, a sudden stare,\nA soulful meow beyond compare.\n\"The food bowl! The lap! The laser dot!\"\nIgnoring all the loving spot.\nA furry overlord, sweet and grand,\nThe ruler of the living land."},
    {"role":"user","content":"now do the same but for dogs"}],"tools":[]}
{{< /highlight-wrap >}}

总结的内容也是作为 `HumanMessage` 存在， `role` 为 `user`, 这里的总结就不怎么好，关键消息丢失了，而且凭空创造了额外的主题。

第五个请求， 再次触发的总结

{{< highlight-wrap json >}}
{"model":"llama3.2:1b","stream":true,"options":{},"messages":[
    {"role":"user","content":"<role>\nContext Extraction Assistant <总结的系统提示词>
<messages>\nMessages to summarize:\nHuman: Here is a summary of the conversation to date:\n\n## SESSION INTENT\nThe user's primary goal in this conversation is to request information about a topic.\n\n## SUMMARY\n- The user wants to know about the highest quality/most relevant context from their message history.\n- No specific choices or conclusions were made during the session; \n- The user did not explicitly reject any options and chose not to pursue them.\n- None\nHuman: write a short poem about cats\nAI: I would be happy to! Here are a few different options, depending on the mood you want—a cozy one, an elegant one, or a funny one!\n\n***\n\n### 🐾 The Cozy Cat (Focus on Comfort)\n\nA sunbeam nap, a gentle sigh,\nWhere golden fur meets watchful eye.\nA rumble soft, a purring sound,\nThe sweetest comfort to be found.\nContent and curled within a mound,\nThe perfect kitty to surround.\n\n***\n\n### ✨ The Elegant Cat (Focus on Grace)\n\nA fluid shadow, silk and night,\nA hunter draped in soft moonlight.\nWith silent paw and graceful leap,\nThrough dusty corners, swift and deep.\nAn ancient charm, a mystery bright,\nA creature of pure, feline might.\n\n***\n\n### 😂 The Quirky Cat (Focus on Demands)\n\nA flicking ear, a sudden stare,\nA soulful meow beyond compare.\n\"The food bowl! The lap! The laser dot!\"\nIgnoring all the loving spot.\nA furry overlord, sweet and grand,\nThe ruler of the living land.\n</messages>
{{< /highlight-wrap >}}

因为前面总结的内容不准备，这里再次总结时，更严重的幻觉出现了

第六个请求，

{{< highlight-wrap json >}}
{"model":"gemma4:e4b","stream":true,"options":{},"messages":[
    {"role":"user","content":"Here is a summary of the conversation to date:\n\n## SESSION INTENT\nThe user's primary goal in this conversation is to request information about a topic.\n\n## SUMMARY\n- No specific choices or conclusions were made during the session; \n- The user chose not to pursue any rejected options and was satisfied with their current understanding of the task.\n- None"},
    {"role":"user","content":"now do the same but for dogs"},
    {"role":"assistant","content":"I would be happy to! Here are a few options for dogs, again depending on the mood you're looking for—a comforting one, a loyal one, or a silly one!\n\n***\n\n### 🏡 The Comfort Dog (Focus on Loyalty & Warmth)\n\nA wet nose nudge, a gentle plea,\nA happy friend for you and me.\nWith steady gaze and joyful pace,\nA loving spirit, warm embrace.\nThe loyal friend who waits by the door,\nAnd always asks for nothing more.\n\n***\n\n### 🏞️ The Adventurer Dog (Focus on Joy & Energy)\n\nThrough grassy fields, they chase the chase,\nA tireless grin upon their face.\nThe bounding leap, the joyous bark,\nA golden spirit in the park.\nWith paws that grip the muddy trail,\nTheir boundless heart will never fail.\n\n***\n\n### 😂 The Enthusiast Dog (Focus on Quirky Behavior)\n\nA tail that wags with thunderous might,\nA frenzy of love, pure and bright.\nThe Zoomies burst at midnight hour,\nA happy pant, a clumsy power.\nThey treat every stranger like best friend,\nA wagging wag, until the end."},
    {"role":"user","content":"what's my name?"}
],"tools":[]}
{{< /highlight-wrap >}}

所以最后没能回答那个最简单的问题，回复是

> I do not know your name. You have not told me what it is yet! <br/>
> <br/>
> Would you like to tell me?

是不是总结用的模型太差，也用 `ollama:gemma4:e4b` 模型试试，看最后的回答, 依然是

> I do not know your name. You have not told me what it is during our conversation! 😊

把总结用的模型换成宇宙最强的模型， `model="bedrock:us.anthropic.claude-haiku-4-5-20251001-v1:0"`，不对，还不是已公开最顶级的
`Claude Opus 4.7`, 最后的输出结果是

> Based on the summary of our conversation, your name was identified as **Bob**.

这才对话，看看它第三次执行 `ass_llm()` 时触发的第一次总结后的内容是什么

{{< highlight-wrap json >}}
HumanMessage(content="Here is a summary of the conversation to date:\n\n## SESSION INTENT\nUser (Bob) has initiated 
contact but has not yet stated a specific goal or request. Awaiting clarification on how to assist.\n\n## SUMMARY\n- 
User introduced themselves as Bob\n- No specific task, request, or objective has been provided yet\n- 
Session is in initial greeting phase\n\n## ARTIFACTS\nNone\n\n## NEXT STEPS\n- Await user's statement of purpose or 
specific request\n- Clarify the primary goal or task they need assistance with\n- Once intent is clear, begin working 
toward that objective", additional_kwargs={'lc_source': 'summarization'}, response_metadata={}, id='c63781f6-2498-4fa0-8c5a-39fab9975e46')
{{< /highlight-wrap >}}

总结内容里有 `Bob`，所以能回答出最后那个问题，还是 `Claude` 的模型聪明。

再试试含 `Tools` 调用后是如何总结的，创建工具方法

```python
@tool
def where_is_bob() -> str:
    """Tell Bob where he is."""
    return "he is in Chicago"
```

在 `create_agent()` 中加上 `tools=[where_is_bob]` 参数，问题中加一个 `where is bob?`

```python
ask_llm("hi, my name is bob")
ask_llm("where is bob?")
ask_llm("write a short poem about cats")
ask_llm("now do the same but for dogs")
final_response = ask_llm("what's my name?")

final_response["messages"][-1].pretty_print()
```

主要是看下第一次进行总结时是否包含工具相关的 `AIMessage` 和 `ToolMessage`, 执行后查看到某个总结时的消息部分内容为

{{< highlight-wrap text >}}
<messages>\nMessages to summarize:\nHuman: Here is a summary of the conversation to date:\n\n## SESSION INTENT\nThe user's primary goal is to determine the current location of \"Bob.\"\n\n## SUMMARY\nThe user has initiated a query asking for the location of \"Bob.\" This constitutes the first specific request in the session.\n\n## ARTIFACTS\nNone.\n\n## NEXT STEPS\nDetermine and report the location of \"Bob,\" or request clarifying information if \"Bob\" is ambiguous or if the context for the location is missing.\nAI: [{'name': 'where_is_bob', 'args': {}, 'id': '86de4de6-0b72-4296-b8ec-85183f64cb41', 'type': 'tool_call'}]\nTool: he is in Chicago\nAI: Bob is in Chicago.\n</messages>"}],"tools":[]}
{{< /highlight-wrap >}}

也就是工具调用的 `AIMessage` 及 `ToolMessage` 的内容与其他的消息对于总结来说没什么区别。

总结的用意就是在内容太多，消息数或 Token 太多时压缩会话，减少模型输入，本质是呼叫一个子 Agent 来完成总结任务。
[`SummarizationMiddleware`](https://reference.langchain.com/python/langchain/agents/middleware/summarization/SummarizationMiddleware)
的关键参数如下

1. `model`: 不必多说
2. `trigger`: tuple 列表表示的键值对，默认是 `None`(不触发)，`("messages", 50)` 消息数达到 50 条触发，`("token", 500)` Token 数达到
    500 触发, `("fraction", 0.8)` 达到 max_input_tokens 的 80% Token 数触发， `[("fraction", 0.8), ("messages", 100)]` 组合条件是 `或`
3.  `keep`: 默认是 `("messages", _DEFAULT_MESSAGES_TO_KEEP)`, `_DEFAULT_MESSAGES_TO_KEEP` 的值为 `20`，控制的属性与 `trigger`
    相同,  表过总结过后，多少条历史 `message`，或 `tokens` 数，或 `fraction` max_input_tokens 的百分比被保留
4. `token_counter`: 默认为 `count_tokens_approximately`， 用于统计 token 的函数
5. `summary_prompt`: 默认为 `DEFAULT_SUMMARY_PROMPT`，总结的提示词，默认的提示词是比较通用的，特定场景下可能需要好好调教它
6. `trim_tokens_to_summarize`: 默认为 `_DEFAULT_TRIM_TOKEN_LIMIT = 4000`，用于被总结时保留的 token 数，超过这个数会被截断，`None` 为全部

### 访问短期记忆数据

在 `Middleware` 中可获得参数 `AgentState` 和 `Runtime`, 而在工具函数中则可以注入 `ToolRuntime` 参数，由 `ToolRuntime.state`
可获得相应的 `AgentState`， 可由此访问 `messages` 或自定的 `AgentState` 属性。

下面试图用一个例子同时演示工具直接返回字符串值，或者用 `Command(update=)` 的访问修改消息，在工具函数中可以访问到 `AgentState` 和 `Context`
信息, 

```python
from langchain_core.messages import ToolMessage
from langchain_core.tools import tool
from langgraph.prebuilt import ToolRuntime

from langchain.agents import create_agent, AgentState
from pydantic import BaseModel
from langgraph.types import Command


class CustomState(AgentState):
    user_name: str

class CustomContext(BaseModel):
    user_id: str

@tool
def where_is_bob(runtime:  ToolRuntime) -> str:
    """Tell Bob where he is."""
    # print(runtime.state["messages"])

    if runtime.context.user_id == "123":
        return Command(update={
            "messages": [
                ToolMessage("he is in SF", tool_call_id=runtime.tool_call_id)
            ]
        })
    return "he is in Chicago"

agent = create_agent(
    model="ollama:gemma4:e4b",
    tools=[where_is_bob],
    state_schema=CustomState,
    context_schema=CustomContext
)

result = agent.invoke(
    {"messages": "hello, my name is Bob, tell me where I am?"},
    context=CustomContext(user_id="222")
)

print(result["messages"][-1].content)

result = agent.invoke(
    {"messages": "hello, my name is Bob, tell me where I am?"},
    context=CustomContext(user_id="123")
)

print(result["messages"][-1].content)
```

两次询问最后分别输出

> Hello Bob! You are in Chicago.<br/>
> I see! You are in SF.

#### 动态系统提示词

关于动态系统提示词，在 [LangChain 核心组件之 Agent - 系统提示词(System Prompt)](/langchain-core-component-agents/#%E7%B3%BB%E7%BB%9F%E6%8F%90%E7%A4%BA%E8%AF%8Dsystem-prompt)
有完全相同的内容，就是用 `@dynamic_prompt` 装饰器方法根据某个条件选择系统提示词，再把代码简单重复一遍

```python
class CustomContext(TypedDict):
    user_name: str

@dynamic_prompt
def dynamic_system_prompt(request: ModelRequest) -> str:
    return f"You are a helpful assistant. Address the user as {request.runtime.context["user_name"]}."


agent = create_agent(
    model="ollama:gemma4:e4b",
    middleware=[dynamic_system_prompt],
    context_schema=CustomContext,
)

result = agent.invoke(
    {"messages": [{"role": "user", "content": "What is the weather in SF?"}]},
    context=CustomContext(user_name="John Smith"),
)
```

#### Middleware 的 @before_model 和 @after_model

`@before_model` 和 `@after_model` 的函数原型都是一样的

```python
@before_model
def trim_messages(state: AgentState, runtime: Runtime) -> dict[str, Any] | None:
    return None

@after_model
def validate_response(state: AgentState, runtime: Runtime) -> dict | None:
    return {"messages": [
        RemoveMessage(content='', id=REMOVE_ALL_MESSAGES), # 移除所有所特定 ID 的消息
        *other_messages
    ]}

agent = create_agent(
    model="ollama:gemma4:e4b",
    middleware=[trim_messages, validate_response],
    checkpointer=InMemorySaver(),
)
```

只是它们执行的时机不同, `@before_model` 在给模型发请求之前预处理，`@after_model` 从模型返回后可对消息进行过滤，下面两张图

<div style="display: flex;font-size: 14px;">
   <div style="flex: 1;">
      {{< bundle-image langchain-before-model.png 260 >}}
   </div>
   <div style="flex: 1;">
      {{< bundle-image langchain-after-model.png 279 >}}
   </div>
</div>

每篇关于 `Langchain` 文章的内容都非常臃肿，而且是不断的重复，对于读者会浪费不少时间，但反复的学习巩固还是有意义的。短期记忆完后，另一个话题就
是长期记忆，现在的 ChatGPT 和 Claude 更是过份了，都实现了永久记忆。就是说你问一个问题的时候，它们可能会把你几年前的毛病都翻出来对你教导一翻。
如果你长期使用它们，生活，工作无论大小事都与它交流，不用多久，那些 AI 完全能给生成一副画像, 若是给别有用心的人利用了，那就更是恐怖了。