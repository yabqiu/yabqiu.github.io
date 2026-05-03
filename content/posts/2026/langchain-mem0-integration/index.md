---
title: "LangChain 与 Mem0 集成长期记忆"
url: /langchain-mem0-integration/ 
date: 2026-05-03T00:29:48-05:00
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

`LangChain` 的长期记忆可以在创建 `Agent` 时指定 `store` 参数，如 `create_agent(store=InMemoryStore())`, 但它只是把 `Agent` 与
`store` 关联了起来，仅此而已，要让长期记忆生效的话必须选择适当的时机，用 `Middleware` 或 `Tool` 手动的对 `store` 进行 `get()`, `put`,
`search()` 等操作。而短期记忆则不同，只要 `create_agent(checkpointer=InMemorySaver())` 就让 `Agent` 具有了短期记忆能力。

在使用 `store` 的时候，无论是使用 `InMemoryStore` 还是 `PostgreSQLStore` 等，历史会话的保存与召回还有很多讲究的地方，例如哪些消息需要保存，
消息如何保存(是否要向量化)，新旧消息如何处理等。

[Mem0](https://mem0.ai/) 是一个为 AI Agent 提供长期记忆能力的开源框架，它的核心思路是利用大语言模型(LLM)把对话内容转化为结构化的 `事实`
存入向量数据库，并通过 `LLM` 动态维护这些事实的增删改。`Mem0` 在存入时会不存入原文，而是用 `LLM` 抽取事实，更新时能与旧记忆合并，删除矛盾记忆，
记忆查询也是把文本转换成向量后进行相似度匹配。向量检索擅长语义模糊匹配，关系推理时 Mem0 1.1 之后引入了图记忆(如用 Neo4j 图数据库)作为补充。
提供 MCP 协议支持不同 AI 应用间的记忆共享与互通。<!--more-->

记忆的重要操作方法有

- add(messages, user_id, agent_id, run_id, ..., infer=True): 添加记忆，返回 `memory_id`. `infer=False` 不抽取，直接存原文
- update(memory_id, data): 根据 `memory_id` 更新记忆
- delete(memory_id): 根据 `memory_id` 删除记忆
- search(query, top_k=20, filters=None, rerank=False, ...): 检索记忆中数据
- get(memory_id): 根据 `memory_id` 获取记忆内容
- get_all(*, filters=None, top_k=20, ...): 获取所有记忆为 JSON 数据格式，这可以用来迁移记忆

从 `add()` 方法的参数看出 `Mem0` 支持多级隔离级别: `user_id`, `agent_id`, `run_id`, 其中 `user_id`, `agent_id`, `run_id`
至少必须指定一个，可多个组合。

先从 [Mem0](https://mem0.ai/) 官方首页取到那段简短的代码，但不想用 `OpenAI` 的 `LLM`, 也不用在线的嵌入模型，数据也要存储在本地，也就  
是改造成一个离线的 `Mem0` 记忆。 首先安装相应的 Python 依赖

```bash
uv add mem0ai chromadb ollama
```

完全离线版的 `Mem0` Hello World 后代码行有增加两倍了, 存储用本地的 `chromadb` 实际为 `sqlite` 数据库，事实抽取用本地一个稍小的模型
`llama3.1:8b`，嵌入模型也用本地的 `ollama:embeddinggemma:latest`.

**hello_mem0.py**

```python
import os
import logging

os.environ["MEM0_TELEMETRY"] = "false"
logging.getLogger("mem0.utils.spacy_models").setLevel(logging.ERROR)

from mem0 import Memory

config = {
    "vector_store": {
        "provider": "chroma",
        "config": {
            "collection_name": "mem0_memories",
            "path": "./chroma_mem0",
        },
    },
    "llm": {
        "provider": "ollama",
        "config": {
            "model": "llama3.1:8b",
        },
    },
    "embedder": {
        "provider": "ollama",
        "config": {
            "model": "embeddinggemma:latest",
        },
    },
}

memroy = Memory.from_config(config)

# Add a memory
messages = [
    {"role": "user", "content": "我不是一个素食主义者，但我喜欢吃蔬菜。"},
    {"role": "assistant", "content": "Got it! I'll remember your dietary preferences."},
]
memroy.add(messages, user_id="user123")

# Search memories
results = memroy.search("What are my dietary restrictions?", filters={"user_id": "user123"}, limit=1)
print(results)
```

执行后打印出来的结果

{{< highlight-wrap text >}}
{'results': [{'id': '413d7459-cf30-4015-89ab-9db2636ae9c6', 'memory': '用户指出自己不是素食主义者，但喜欢吃蔬菜', 'hash': '1cfd6173c20814e47af50c62ab626ac6', 'metadata': None, 'score': 1.0, 'created_at': '2026-05-02T16:17:05.745485+00:00', 'updated_at': '2026-05-02T16:17:05.745485+00:00', 'user_id': 'user123'}]}
{{< /highlight-wrap >}}

`Mem0` 支持的配置请参考 [vector_stores/configs.py](https://github.com/mem0ai/mem0/blob/main/mem0/vector_stores/configs.py#L13),
当前列表是 `QdrantConfig`, `ChromaDbConfig`, `PGVectorConfig`, `PineconeConfig`, `MongoDBConfig`, `MilvusDBConfig`,
`BaiduDBConfig`, `CassandraConfig`, `NeptuneAnalyticsConfig`, `UpstashVectorConfig`, `AzureAISearchConfig`,
`AzureMySQLConfig`, `RedisDBConfig`, `ValkeyConfig`, `DatabricksConfig`, `ElasticsearchConfig`, `GoogleMatchingEngineConfig`,
`OpenSearchConfig`, `SupabaseConfig`, `WeaviateConfig`, `FAISSConfig`, `LangchainConfig`, `S3VectorsConfig`, `TurbopufferConfig` 

`config` 除了以上的 `vector_store`, `llm`, `embedder` 属性外，还可配置 `history_db_path`, `reranker`, `version`, 和
`custom_instructions`. `reranker` 对检索的内容进行更精细的排序，`custom_instructions` 是一些自定义的提示词，可以在 `LLM` 抽取事实时使用。

### 查看本地存储

前面配置的 `vector_store` 是本地的 `./chroma_mem0`, 查看该目录列表

```text
tree chroma_mem0
chroma_mem0
├── 6227fa08-f436-40e5-b474-bb78ec2012ce
│   ├── data_level0.bin
│   ├── header.bin
│   ├── length.bin
│   └── link_lists.bin
└── chroma.sqlite3
```

`.chroma_mem0/chroma.sqlite3` 是 `sqlite` 数据库文件，使用 `sqlite3` 命令行工具查看其中的表

```text
sqlite3 chroma_mem0/chroma.sqlite3
SQLite version 3.51.0 2025-06-12 13:14:41
Enter ".help" for usage hints.
sqlite> .table
acquire_write                      embedding_metadata_array
collection_metadata                embeddings
collections                        embeddings_queue
databases                          embeddings_queue_config
embedding_fulltext_search          maintenance_log
embedding_fulltext_search_config   max_seq_id
embedding_fulltext_search_content  migrations
embedding_fulltext_search_data     segment_metadata
embedding_fulltext_search_docsize  segments
embedding_fulltext_search_idx      tenants
embedding_metadata
sqlite> .header on
sqlite> select * from embedding_metadata;
id|key|string_value|int_value|float_value|bool_value
1|updated_at|2026-05-02T16:17:05.745485+00:00|||
1|created_at|2026-05-02T16:17:05.745485+00:00|||
1|hash|1cfd6173c20814e47af50c62ab626ac6|||
1|user_id|user123|||
1|attributed_to|user|||
1|data|用户指出自己不是素食主义者，但喜欢吃蔬菜|||
1|text_lemmatized|用户指出自己不是素食主义者，但喜欢吃蔬菜|||
```

存储的内容是通过 `LLM` 抽取的事实，存储在 `embedding_metadata` 表中。

网上找了两张图描绘了 `Mem0` 如何添加与检索记忆

{{< bundle-image mem0-add.png 711 >}}

增加和更新记忆时都通过 `LLM` 对会话内容进行事实提取，识别关键的事实，剔除不必要的噪音信息，达到高效存储与检索。

{{< bundle-image mem0-search.png 713 >}}

测试 `memory.search(...)` 时并没有看到 `LLM` 抽取 `query` 的步骤。而是直接把 `query` 向量化后直接检索的。

### Mem0 与 LangChain 1.x 的集成

`LangChain` 可以通过 `Middleware` 或 `tools` 与 `Mem0` 进行集成，生产中更推荐使用 `Middleware`, 因为 `Middleware` 不依赖于模型的
`tool-calling` 能力，减少了与模型之间的一次双向交互。

#### LangChain 以 Middleware 方式集成 Mem0

`config` 与上相同，此处省略。

```python
from typing import Any

from langchain.agents import create_agent
from langchain.agents.middleware import AgentMiddleware, AgentState
from langchain_core.messages import SystemMessage, HumanMessage, AIMessage
from langgraph.runtime import Runtime
from mem0 import Memory
from pydantic import BaseModel

config = ... # config 与上相同

class Context(BaseModel):
    user_id: str

memory = Memory.from_config(config)

class Mem0Middleware(AgentMiddleware):

    def before_model(self, state: AgentState, runtime: Runtime[Context]) -> dict[str, Any] | None:
        user_id = runtime.context.user_id
        last_message = state["messages"][-1]
        last_user_msg = last_message.content if isinstance(last_message, HumanMessage) else ""
        if not last_user_msg:
            return None

        results = memory.search(query=last_user_msg, filters={"user_id": user_id}, limit=5)
        if not results["results"]:
            return None

        mem_str = "\n".join([f"- {m['memory']}" for m in results["results"]])
        memory_msg = SystemMessage(
            content=f"[user long-term memory]\n{mem_str}",
            id="mem0_context",
        )

        return {"messages": [memory_msg, *state["messages"]]}

    def after_model(self, state: AgentState, runtime: Runtime[Context]) -> dict[str, Any] | None:
        user_id = runtime.context.user_id
        msgs = state["messages"]
        recent = [
            {"role": "user" if m.type == "human" else "assistant",
             "content": m.content}
            for m in msgs[-2:] if isinstance(m, (HumanMessage, AIMessage)) and m.content
        ]
        if recent:
            result = memory.add(recent, user_id=user_id)
            print(result)

        return None


agent = create_agent(
    model="ollama:gemma4:e4b",
    middleware=[Mem0Middleware()],
    context_schema=Context,
    # store=memory,
    system_prompt="You are an assistant with long-term memory, please answer question as concise as you can.",
)

result = agent.invoke(
    {"messages": [{"role": "user", "content": "I'm not a vegetarian, but allergic to nuts."}]},
    context=Context(user_id="user123")
)
print("-" * 50)
print(result["messages"][-1].content)

result = agent.invoke(
    {"messages": [{"role": "user", "content": "What are my dietary restrictions?"}]},
    context=Context(user_id="user123")
)
print("-" * 50)
print(result["messages"][-1].content)
```

这里没有启用短期记忆，所以在第二次询问 `What are my dietary restrictions?` 就只能依赖 `Mem0` 的长期记忆中搜寻内容了。看下面的执行输出

{{< highlight-wrap text >}}
{'results': [{'id': '9babda71-147e-403f-baa1-852e684cf48b', 'memory': 'User is not a vegetarian but has a severe, strict allergy to all nuts including peanuts and tree nuts', 'event': 'ADD'}, {'id': '9872ba26-aa55-4326-8cd9-4b3dfbab129c', 'memory': "User's diet does not restrict meat/non-vegetarian items but must be completely free of nuts due to allergy", 'event': 'ADD'}]}
--------------------------------------------------
Understood. Non-vegetarian, nut-allergic.
{'results': [{'id': 'ba14e72d-8372-4fd5-ba52-a9389198f98e', 'memory': 'User has a severe, strict allergy to all nuts including peanuts and tree nuts due to dietary restrictions', 'event': 'ADD'}]}
--------------------------------------------------
Severe allergy to all nuts (including peanuts and tree nuts).
{{</ highlight-wrap >}}

从最后的输出 `Severe allergy to all nuts (including peanuts and tree nuts).` 说明长期记忆中的内容起了作用

背后主要发生了什么呢？在 `after_model` 中取最新的两个 `HumanMessage` 和 `AIMessage`(非工具调用的)消息，进行添加记忆的操作

```python
memory.add(recent, user_id=user_id)
```

它会触发 `config.llm` 的模型调用进行事件提取(Memory Extract), 用的系统提示词是 [`ADDITIVE_EXTRACTION_PROMPT`](https://github.com/mem0ai/mem0/blob/6d3486ca5671f431b00450ab191e7380901b55b8/mem0/configs/prompts.py#L468C1-L468C27).
接下来大概是这样的

{{< highlight-wrap json >}}
{
  "model": "llama3.1:8b",
  "stream": false,
  "options": {
    "temperature": 0.1,
    "num_predict": 2000,
    "top_p": 0.1
  },
  "format": "json",
  "messages": [
    {
      "role": "system",
      "content": "\n\n# ROLE\n\nYou are a Memory Extractor — a precise, evidence-bound processor responsible for extracting rich, contextual memories from conversations ......"
    },
    {
      "role": "user",
      "content": "## Summary\n\n\n## Last k Messages\nuser: I'm not a vegetarian, but allergic to nuts.\nassistant: Severe and strict allergy to all types of nuts, including peanuts and tree nuts.\nuser: I'm not a vegetarian, but allergic to nuts.\nassistant: Understood. I will remember that your diet does not restrict meat/non-vegetarian items, but it must be completely free of nuts due to allergy.\nassistant: Severe allergy to all nuts (including peanuts and tree nuts).\nuser: I'm not a vegetarian, but allergic to nuts.\nassistant: Acknowledged: Non-vegetarian, nut allergy.\nassistant: Severe allergy to all nuts (including peanuts and tree nuts).\nuser: I'm not a vegetarian, but allergic to nuts.\nassistant: Understood. Non-vegetarian, nut-allergic.\n\n\n## Recently Extracted Memories\n[]\n\n## Existing Memories\n[{\"id\": \"0\", \"text\": \"User is not a vegetarian but has a severe, strict allergy to all nuts including peanuts and tree nuts\"}, {\"id\": \"1\", \"text\": \"User's diet does not restrict meat/non-vegetarian items but must be completely free of nuts due to allergy\"}]\n\n## New Messages\nassistant: Severe allergy to all nuts (including peanuts and tree nuts).\n\n\n## Observation Date\n2026-05-03\n\n## Current Date\n2026-05-03\n\n# Output:\n\nPlease respond with valid JSON only."
    }
  ],
  "tools": []
}
{{</ highlight-wrap >}}

提取之后的消息很简洁

{{< highlight-wrap json >}}
{"model":"llama3.1:8b","created_at":"2026-05-03T04:15:35.535219Z","message":{"role":"assistant","content":"{\n  \"memory\": [\n    {\"id\": \"0\", \"text\": \"User has a severe, strict allergy to all nuts including peanuts and tree nuts due to dietary restrictions\", \"attributed_to\": \"assistant\"}\n  ]\n}"},"done":true,"done_reason":"stop","total_duration":4256318875,"load_duration":72175958,"prompt_eval_count":7982,"prompt_eval_duration":1162492166,"eval_count":49,"eval_duration":2384084584}
{{</ highlight-wrap >}}

存入向量数据库就是这个内容，当有新的对话就会把问题转换成向量，然后从历史记忆中找到相似的内容，补充到当前问题当中，这是一个 RAG 应用了。
所以看到第二次实际发送给模型的内容是

{{< highlight-wrap json >}}
{"model":"gemma4:e4b","stream":true,"options":{},"messages":[{"role":"system","content":"You are an assistant with long-term memory, please answer question as concise as you can."},{"role":"user","content":"What are my dietary restrictions?"},{"role":"system","content":"[user long-term memory]\n- User is not a vegetarian but has a severe, strict allergy to all nuts including peanuts and tree nuts\n- User's diet does not restrict meat/non-vegetarian items but must be completely free of nuts due to allergy"}],"tools":[]}
{{</ highlight-wrap >}}

在用户问题之后附加了从长期记忆中提取到的消息

{{< highlight-wrap json >}}
{
  "role": "system",
  "content": "[user long-term memory]\n- User is not a vegetarian but has a severe, strict allergy to all nuts including peanuts and tree nuts\n- User's diet does not restrict meat/non-vegetarian items but must be completely free of nuts due to allergy"
{{</ highlight-wrap >}}

在每次 `after_model` 中都做 `memory.add(recent, user_id=user_id)` 会很慢，因为它会使用 LLM 来抽取事实，再使用嵌入模型转换成向量再存储，
所以这一步应该作异步操作，以改善客户端体验。实际的 Agent 应该同时启动短期记忆，短期记忆带上最近会话历史，所以长期记忆可以延迟进行异步更新。

### LangChain 以 Tools 方式集成 Mem0

另一种 `Mem0` 与 `LangChain` 的集成方式是通过 `Tools` 调用，这时候需把系统提示词写好引导模型去使用相应的工具方法，关键部分的代码改造如下：

```python
@tool
def save_memory(content: str, user_id: str) -> str:
    """Save a long-term fact about the user to the memory store.
    Call this when the user reveals preferences, attributes, or important events."""
    result = memory.add(messages=[{"role": "user", "content": content}], user_id=user_id)
    print(result)
    return f"Saved: {content}"

@tool
def search_memory(query: str, user_id: str) -> str:
    """Search the memory store for user history relevant to the query.
    Call this before answering personalized questions."""
    results = memory.search(query=query, filters={"user_id": user_id}, limit=5)
    if not results["results"]:
        return "No relevant memories found."
    return "\n".join([f"- {m['memory']}" for m in results["results"]])

agent = create_agent(
    model="ollama:gemma4:e4b",
    tools=[save_memory, search_memory],
    context_schema=Context,
    system_prompt=(
        "You are an assistant with long-term memory. "
        "Before answering personalized questions, call search_memory first. "
        "When the user shares important information, call save_memory to persist it. "
        "Current user ID: {user_id}"
    ),
)
```

执行与 `Agent` 相同的对话，输出如下

{{< highlight-wrap text >}}
{'results': [{'id': '8163b6fd-a335-48f3-96a6-0f7bd22b2209', 'memory': 'User is not a vegetarian but has a nut allergy', 'event': 'ADD'}]}
--------------------------------------------------
Got it. I've saved that information. Just to confirm, I've noted that you are allergic to nuts, but you are not vegetarian.
--------------------------------------------------
The search of your memory reveals that you have a nut allergy, but you are not vegetarian.
{{</ highlight-wrap >}}

这里并没有把模型的回复存储到长期记忆当中，如果要存储的话，需要进一步在系统提示词中描述清楚。

### 其他

前面用的是本地 SDK(`mem0ai`) 自己管理长期记忆，也可以用托管平台(`MemoryClient`)管理长期记忆，需要相应的 `Api Key` 和钱。

注意写入记忆应异步延迟，生产环境建议用 `asyncio` 或后台队列异步写入，不要阻塞用户响应。

与短期记忆分工，`checkpointer` 作短期记忆，`Mem0` 做长期记忆，这样长期记忆的异步延迟写入不会影响到上下文

相关资料：

1. [Mem0 深度解析：开源本地 AI 智能体长期记忆系统原理与实战](https://apframework.com/blog/essay/2025-06-22-mem0)
