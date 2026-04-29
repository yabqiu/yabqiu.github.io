---
title: "LangChain 高级用法之长期记忆"
url: /langchain-advanced-usage-long-term-memory/
date: 2026-04-28T22:40:48-05:00
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

关于短期记忆已写过两篇 [LangChain - 关于会话记忆](/langchain-about-session-memory/) 和 [LangChain 核心组件之短期记忆](/langchain-core-component-short-term-memory/).
有短期记忆就有长期，记忆的短与长的区分标准是看记忆是否能跨越会话，与选择的存储介质, 时效性，中途模型切换都无关。知期记忆限定在同一个会话当中，
只要没跨会话，即使是一年前聊过的天，重新拣起来继续聊也是短期记忆; 而长期记忆是专指跨越会话的，在一个会话中聊过的，重开一个新的会话，`Agent` 
还能知道你在别的会话中聊过的内容, 这就是长期记忆。即便这种记忆用内存保存数据，`Agent` 重启数据会丢失，但只要能跨会话就是长期记忆。

所谓的会话就是像 `ChatGPT`, `Claude` 桌面应用对应的 `Chat`, `New chat` 就创建了一个新的会话，短期记忆局限于同一个 `Chat`, 长期记忆则跨越 `Chat`。
同一个会话中聊天，`Agent` 的回答一直有当前会话上下文中，是好理解的。长期记忆则是无论你 `New chat` 重开了一个新的 `Chat`，`Agent` 都知道你在其他会话中聊过什么。

现在的 `ChatGPT` 和 `Claude` 都具有了长期记忆，这带来一个恐怖的事情，随着你使用它们的时间越来越长，`AI` 可能比你还更了解你，它有了你的隐私，
能描绘你的性格，甚至能预测你的下一步行动。看来不想 `AI` 介入的太多，难道要经常切换着帐号来使用某个 `AI` 工具？还得探索能不能要求删除长期记忆。
如果它们像广告那样粘住你的设备与 IP 就更可怕了。<!--more-->

长期记忆赋予了 `AI` 能像人与人那样的对话，无论多久，或如何切换话题，两个人之间的对话总会与所有的对话有所关联。业界有少专门实现 `Agent` 长期记忆的组件，
如 [Mem0](https://mem0.ai/), [ZEP](https://www.getzep.com/), LangChain Memory, LlamaIndex Memory, [Letta](https://www.letta.com/),
[Cognee](https://www.cognee.ai/), [SuperMemory](https://supermemory.ai/) 等。

短期记忆的上下文虽说相对较短，但经多轮对话后，很快就会超过 `Agent` 的上下文长度，短期记忆可以通过 `Summarization` 来压缩上下文。
而对于大量会话的长期记忆来说，通过总结肯定是无法有效压缩上下文的，看到一般的实现是把历史会话向量化，然后在对话过程中用 `RAG` 取加相关片断。
这也是为什么 `LangChain` 中把文档 `Long-term memory` 放在 `Retrieval` 之后，这里假定我们对 `RAG` 有所了解之后来学习 `LangChain`
的长期记忆。

若对 `RAG` 不熟悉的话，可参考本人写过的一篇 [简单例子用 Python + PostgreSQL 演示 RAG](/rag-python-postgresql-pgvector/).

回到 `LangChain` 的长期记忆实现，它是内置于 [LangGraph stores](https://docs.langchain.com/oss/python/langgraph/persistence#memory-store)
的功能，它用 Namespace 和 Key 来组织的 `JSON` 文件存储长期记忆。

`LangChain` 实现长期记忆是使用 `create_agent()` 的 `store` 参数，`LangChain` 的基类 `BaseStore` 有两个实现，`InMemoryStore` 和
`InMemoryByteStore`.

```python
store = InMemoryStore()

agent = create_agent(
    model="ollama:gemma4:e4b",
    store=store
)
```

回顾 `LangChain` 的短期记忆是指定 `create_agent()` 的参数如 `checkpointer=InMemorySaver()`，所以 `checkpointer`: 短期记忆，`store`: 长期记忆。
官方文档在演示长期记忆时使用的是 `InMemoryStore` 和 `PostgresStore`, 而本文为了方便使用了 `SqliteStore`, 需先安装 Python 库

```bash
uv add langgraph-checkpoint-sqlite
```

下面是一个完整的例子，阅读时提前知道几个要点

- 对存入 `store` 的消息进行向量化，嵌入模型是 `ollama:embeddinggemma:latest`
- `@before_model` 中把用户消息存入长期记忆，并从长期记忆中取出与当前问题相关的消息，一并发给模型。取长期记忆数据时用了 `(APP, user_id)`
  作为 namespace
- `@after_model` 中把 `AI` 非工具调用的消息存入长期记忆，以备后面唤醒长期记忆

```python
from typing import Any

from langchain.agents.middleware import before_model, after_model
from langchain_core.messages import HumanMessage, AIMessage
from langgraph.runtime import Runtime
import hashlib

from dataclasses import dataclass

from langchain.agents import create_agent, AgentState
from langchain.embeddings import init_embeddings
from langgraph.store.sqlite import SqliteStore
from langgraph.store.sqlite.base import SqliteIndexConfig


def gen_key(text: Any) -> str:
  return hashlib.md5(str(text).encode()).hexdigest()


@dataclass
class Context:
  user_id: str


APP = "chat_app"

with (SqliteStore.from_conn_string(
        "langchain_memory.db",
        index=SqliteIndexConfig(embed=init_embeddings(model="ollama:embeddinggemma:latest")))
as store):
  @before_model
  def retrieve_long_term_memory(state: AgentState, runtime: Runtime) -> dict[str, Any] | None:
    user_id = runtime.context.user_id
    additional_messages = []

    last_message = state["messages"][-1]
    if isinstance(last_message, HumanMessage):
      for related in store.search((APP, user_id), query=str(last_message.content), limit=2):
        additional_messages.append(related.value)
      store.put((APP, user_id), gen_key(last_message.content),
                {"role": "user", "content": last_message.content})

    new_messages = {"messages": state["messages"][0:-1] + additional_messages + [last_message]}
    return new_messages


  @after_model
  def save_long_term_memory(state: AgentState, runtime: Runtime) -> None:
    user_id = runtime.context.user_id
    last_message = state["messages"][-1]
    if isinstance(last_message, AIMessage):
      if len(last_message.content) > 0:
        store.put((APP, user_id), gen_key(last_message.content),
                  {"role": "assistant", "content": last_message.content})


  agent = create_agent(
    model="ollama:gemma4:e4b",
    store=store,
    middleware=[save_long_term_memory, retrieve_long_term_memory],
    context_schema=Context,
  )

  user_id = "user_123"

  agent.invoke({
    "messages": [{"role": "user", "content": "My name is Yanbin"}]},
    context=Context(user_id="user_123"),
  )

  result = agent.invoke({
    "messages": [{"role": "user", "content": "Please just state my name?"}]},
    context=Context(user_id="user_123"),
  )

  for message in result["messages"]:
    message.pretty_print()
```

当第一次执行时，会生成一个新的 `sqlite` 数据库文件 `langchain_memory.db`, 最后执行的输出结果是

```text
================================ Human Message =================================

Please just state my name
================================ Human Message =================================

My name is Yanbin
================================== Ai Message ==================================

Hello Yanbin! It's nice to meet you.

How can I help you today? 😊
================================== Ai Message ==================================

Yanbin
```

我们学习过短期记忆的话，如果没有短期记忆的话，以上第二次问 `Please state my name?` 的时候，`AI` 是没有上下文的，无法回答出这个问题来。
所以这是长期记忆在起作用。上面的代码注释掉 `store=store` 后，第二次问 `Please state my name?` 的时候，`AI` 就答不上来了。

### 分析交互过程

第一次 `agent.invoke()` 时

在 `@before_model` 中的 `store.search((APP, user_id), query=str(last_message.content), limit=2)` 时也会请求一次嵌入模型，然后从
长期记忆中搜索是否有与问题 `My name is Yanbin` 相近的结果。此时，数据库为空, 所以没有相关联的数据

发送给嵌入模型的请求数据是

{{< highlight-wrap json >}}
{"model":"embeddinggemma:latest","input":["My name is Yanbin"],"options":{"mirostat":null,"mirostat_eta":null,"mirostat_tau":null,"num_ctx":null,"num_gpu":null,"num_thread":null,"repeat_last_n":null,"repeat_penalty":null,"temperature":null,"stop":null,"tfs_z":null,"top_k":null,"top_p":null}}
{{</ highlight-wrap >}}

还是在 `@before_model` 函数中，`store.search()` 完后，会把把消息 `My name is Yanbin`, 存入到长期记忆. 在存入之前会对它进行向量化，
再存入向量数据库。所以发现向嵌入模型 `embeddinggemma:latest`, 又发送了与上面完全一样的请求, 从嵌入模型收到的是一堆向量化后的数值，然后存入
`sqlite` 数据库 `langchain_memory.db` 中, 后面我们会知道是存入到 `store_vectors` 这个表中的。

经过 `@before_model` 后只是把第一条消息存入到长期记忆中，最终发送了主模型的请求只有 `My name is Yanbin`.

模型返回后，在 `@after_model` 中把回复的 `Hello Yanbin! It's nice to meet you.` 直接消息存入到长期记忆中

第二次 `agent.invoke()` 问 `Please just state my name?` 这个问题时，在 `@before_model` 中也会对该问题向量化后从长期记忆中查询相关消息，
搜索到两条相关数据，添加到 `messages` 中作为问题补充。同样会把最后的问话向量化后存入长期记忆中。

所以这时候发给 `gemma4:e4b` 模型的请求包含了三条消息

{{< highlight-wrap json >}}
{"model":"gemma4:e4b","stream":true,"options":{},"messages":[{"role":"user","content":"Please just state my name"},{"role":"user","content":"My name is Yanbin"},{"role":"assistant","content":"Hello Yanbin! It's nice to meet you.\n\nHow can I help you today? 😊"}],"tools":[]}
{{</ highlight-wrap >}}

因此模型有这个上下文才能回答出 `please state my name?` 这个问题

有了长期记忆，以后只执行第二个 `agent.invoke()`

```python
    result = agent.invoke({
        "messages": [{"role": "user", "content": "Please just state my name"}]},
        context=Context(user_id="user_123"),
    )
```

模型也能作出正确的回答，因为在 `@before_model` 方法中会从长期记忆中查询相关消息，并把相关消息发给模型。我们这里使用长期记忆时使用的
`Namespace` 是 `("chat_app", "user_123")`, 如果第二次 `agent_invoke()` 时换一个 `Namespace`, 比如不同的 `APP`, 或 `user_id`,
那么模型就没有相关的上下文，只能说

```text
================================ Human Message =================================

Please just state my name
================================== Ai Message ==================================

I do not know your name. You haven't told me what it is.
```

本例中，在使用长期记忆时，如果知道某个其他用户的 `user_id`，还能访问别的用户的长期记忆，这会造成严重的用户信息泄漏。

### BaseStore 相关的 API

前面的例子用到了 `store` 的两个 API, 分别是  `put()` 和 `search()`, 除此之外还有 `get()`, `delete()`, `batch()` 操作，以及函数对应的
async 版本，用前缀 `a`， 如 `aget()`, `asearch()` 等。

`put()`, `get()` 和 `search()` 的方法原型分别是

```python
    def put(
        self,
        namespace: tuple[str, ...],
        key: str,
        value: dict[str, Any],
        index: Literal[False] | list[str] | None = None,
        *,
        ttl: float | None | NotProvided = NOT_PROVIDED,
    ) -> None:
        ...

    def get(
        self,
        namespace: tuple[str, ...],
        key: str,
        *,
        refresh_ttl: bool | None = None,
    ) -> Item | None:

    def search(
        self,
        namespace_prefix: tuple[str, ...],
        /,
        *,
        query: str | None = None,
        filter: dict[str, Any] | None = None,
        limit: int = 10,
        offset: int = 0,
        refresh_ttl: bool | None = None,
    ) -> list[SearchItem]:
        ...
```

记住，LangChain 的长期记忆 `value` 是一个 `dict[str, Any]`, 可认为是一个 `JSON` 对象。应自己规划好 `namespace` 和 `key` 如何隔离数据。
如前面的例子，`namespace('<app_name>', '<user_id>')` 来限定应用程序及用户 ID 分类，`key` 可用来进一步划分的小类，比如用 `project` 为 `key`.

以上方法有参数 `ttl` 或 `refresh_ttl`，用于控制记忆保存多久时间，或查询出来的结果缓存多久。`search(query="<str>")` 
时查询是用向量化后再以相似度进行匹配, 所以我们前面的 `@before_model` 进行了两个 `embedding`, 这是可以优化的地方。`embedding` 是为了能
快速进行相似度的查询，如果只需用 `namespace` 和 `key` 或 `filter` 查询的话，可不使用嵌入模型对内容进行向量化。

这里是只用 `filter` 来查询的例子， 比如 `put()` 的值为

```python
    store.put(
        namespace,
        "a-memory",
        {
            "rules": [
                "User likes short, direct language",
                "User only speaks English & python",
            ],
            "my-key": "my-value",
        },
    )
```

查询时用

```python
items = store.search(
        namespace, filter={"my-key": "my-value"}, query="language preferences"
    )
```

如果 `put()` 时用的 `namespace` 和 `key` 都是一样的，则会覆盖之前的值.

### 查看长期记忆中的数据

用 `sqlite` 查看其中的数据

```bash
sqlite langchain_memory.db
sqlite> .table
store              store_migrations   store_vectors      vector_migrations
sqlite> .schema store
CREATE TABLE store (
    -- 'prefix' represents the doc's 'namespace'
    prefix text NOT NULL,
    key text NOT NULL,
    value text NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, expires_at TIMESTAMP, ttl_minutes REAL,
    PRIMARY KEY (prefix, key)
);
CREATE INDEX store_prefix_idx ON store (prefix);
CREATE INDEX idx_store_expires_at ON store (expires_at)
WHERE expires_at IS NOT NULL;
sqlite> .schema store_vectors
CREATE TABLE store_vectors (
    prefix text NOT NULL,
    key text NOT NULL,
    field_name text NOT NULL,
    embedding BLOB,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (prefix, key, field_name),
    FOREIGN KEY (prefix, key) REFERENCES store(prefix, key) ON DELETE CASCADE
);
sqlite> .schema store_migrations
CREATE TABLE store_migrations (
                    v INTEGER PRIMARY KEY
                );
sqlite> .schema vector_migrations
CREATE TABLE vector_migrations (
                        v INTEGER PRIMARY KEY
                    );
sqlite> .headers on
sqlite> select * from store limit 3;
prefix|key|value|created_at|updated_at|expires_at|ttl_minutes
chat_app.user_123|635067f4f6ee04146299a6e4b39fdd94|{"role":"user","content":"My name is Yanbin"}|2026-04-28 22:21:59|2026-04-28 22:21:59||
chat_app.user_123|91f32494d462a8e3c4c70f859af0c34c|{"role":"assistant","content":"Hello Yanbin! It's nice to meet you.\n\nHow can I help you today? 😊"}|2026-04-28 22:22:00|2026-04-28 22:22:00||
chat_app.user_123|c5fcad1b11486cd15f73f31aa4b684b9|{"role":"user","content":"Please just state my name"}|2026-04-28 22:34:28|2026-04-28 22:34:28||
```

### 总结

本文的例子只是演示了长期记忆能实现什么，实际项目应该不会以这种方式来存储长期记忆，而且在这个会话中只需用短期记忆即可。具体实现长期记忆可以考虑把
会话定期，或结束时刷入到长期记忆中，至于对话当中如何从长期记忆中获取内容传给模型，可以附加到系统提示词，后面可以研究一下 `Agent` 是如何动态使用
`Agent Skills` 的。

还需进一步借鉴其他的长期记忆实现框架, 比如尝试在 `LangChain` 实现的 `Agent` 中使用 `Mem0` 作为长期记忆。