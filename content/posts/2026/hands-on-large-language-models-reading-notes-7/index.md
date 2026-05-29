---
title: "《Hands-On Large Language Models》阅读笔记(七)"
url: /hands-on-large-language-models-reading-notes-7/
date: 2026-05-19T11:54:49-05:00
featured: false
draft: false
type: post
toc: false
# menu: main
usePageBundles: true
thumbnail: "../images/logos/hands-on-llm.jpg"
categories:
  - LLM
tags: 
  - Machine Learning
  - LLM
comment: true
codeMaxLines: 50
showLastmod: true
lastmod:
---

#### 第八章：语义搜索与 RAG

在 BERT 之前 Google 只是关键词搜索，当你把程序的出错信息(特别是带有本地文件路径)往 Google 一贴，什么也搜索不出来，此时就能体现低级和高级程序员的不同水平。
现在形势不同，自从一篇 BERT 的论文(Pre-Training of Deep Bidirectional Transformers for Language Understanding) 发表(2018年) 数月后, 
Google 把`BERT` 整合进它的搜索引擎中，特别是加上各种 AI 工具，从此程序员不用再为出错信息而交流了。这就是语义搜索(semantic search)的威力. 

很早就听说过 `RAG`(Retrieval-Augmented Generation) 已过时的论调，然而实际学习 AI 时总也避不开 `RAG`. 因为微调模型的少， 再加上模型的上下文窗口太小，
所以有了 `RAG`, 持 `RAG` 过时之说大概是因为模型支持的上下文不断增大，原本以 `RAG` 片段作为提示词一部分的内容可以全部塞进上下文窗口中，
但对于长短期记忆和大量的资料库的检索，`RAG` 仍然有其存在的价值。

语义搜索和 RAG 是否也可以集成到 Lucene, Solr, Elasticsearch 等搜索引擎中，以提升搜索的准确性和效率呢？

语义搜索的几个概念和通常做法

- 稠密搜索(dense retrieval): 从向量数据库中搜索与当前文本向量相似的一些文档，比如 10 个文档. 稠密搜索就是粗泛的大面积快速搜索。
- 重排序(reranking): 对稠密搜索结果重排序(更精细的比较相似度)，比如取前 3 个文档. 稠密搜索筛选，重排序进行精选，重排序也可让 LLM 介入.
  有一个相关的论文 [Multi-Stage Document Ranking with BERT](https://arxiv.org/pdf/1910.14424), 常称作 monoBERT 方法
- RAG：通过包括资料分片，向量化存储，对输入向量化后检索相似文档，作为输入提供给 AI 模型，让 AI 基于它们生成结果的全过程<!--more-->

关于 RAG, 本人之前写过一篇详细介绍它的日志 [简单例子用 Python + PostgreSQL 演示 RAG](/rag-python-postgresql-pgvector/)

稠密检索的主要缺陷

- 无关结果问题: 当文本中根本不存在答案时，系统仍会返回最近邻结果。这时要设置相关性距离阈值
- 无法精准匹配特定短语: 语义检索对精确短语匹配效果差，这类场景更适合关键词匹配。因此推荐使用混合搜索（语义搜索 + 关键词搜索）来弥补这一不足。
- 跨领域性能下降: 模型在训练数据之外的专业领域（如法律、医疗）表现会显著退化，因为领域词汇和语义分布与训练数据差异较大

生成嵌入向量前对文档如何分片是个技术活，以句子为单位分块，粒度太小，导致向量无法捕捉足够的上下文信息; 以段落为单位更优一些，如果段落太大可考虑细分;
块与块之间增加重叠部分，可有效的保留上下文信息; 甚至分块的时候也利用 LLM 实现动态智能分块。

最近邻搜索算法，对于数千至万量级的向量，用 NumPy 即可高效完成这种计算，当处理百万级的向量时，建议采用 Annoy 或 FAISS 等最近邻(ANN: Approximate
nearest neighbor), 可能要用到 GPU 加速。另一类解决方案是用到专门的向量数据库(如 Weaviate, Pinecone), 它们有索引，或其他的过滤条件来优化搜索。

面向稠密检索时，对嵌入模型微调可显著提升 LLM 在特定任务中的表现，微调过程的目标是使这些查询的嵌入向量更接近目标句子的嵌入向量。同时，
微调中也需引入与查询无关的负样本，帮助模型区分相关与不相关内容.

##### RAG

用 `Cohere` 托管的 LLM, 向量库，可用它的 API 提供搜索到的 `documents`, 代码如下(需要 Cohere API KEY, 未进行验证)

```python
query = "income generated"
results = search(query)

docs_dict = [{'text': text} for text in results['texts']]
response = co.chat(
    message = query,
    documents=docs_dict
)
print(response.text)
```

下面实践一下 `llama-cpp-python` 与 `LangChain`(改用版本 1.x) 实现的 RAG, 先要安装依赖

> uv add llama-cpp-python langchain-community langchain-huggingface faiss-cpu   # 有 CUDA 的可安装 faiss-gpu

```python
from langchain_community.vectorstores import FAISS
from langchain_huggingface import HuggingFaceEmbeddings

from llama_cpp import Llama

embedding_model =  HuggingFaceEmbeddings(
    model_name="BAAI/bge-small-en-v1.5",
)

texts = ["""Interstellar is a 2014 epic science fiction film co-written, directed, and produced by Christopher Nolan.
It stars Matthew McConaughey, Anne Hathaway, Jessica Chastain, Bill Irwin, Ellen Burstyn, Matt Damon, and Michael Caine.
Set in a dystopian future where humanity is struggling to survive, the film follows a group of astronauts who travel through a wormhole near Saturn in search of a new home for mankind.""",

"""Brothers Christopher and Jonathan Nolan wrote the screenplay, which had its origins in a script Jonathan developed in 2007.
Caltech theoretical physicist and 2017 Nobel laureate in Physics[4] Kip Thorne was an executive producer, acted as a scientific consultant, and wrote a tie-in book, The Science of Interstellar.
Cinematographer Hoyte van Hoytema shot it on 35 mm movie film in the Panavision anamorphic format and IMAX 70 mm.
Principal photography began in late 2013 and took place in Alberta, Iceland, and Los Angeles.
Interstellar uses extensive practical and miniature effects and the company Double Negative created additional digital effects.""",

"""Interstellar premiered on October 26, 2014, in Los Angeles.
In the United States, it was first released on film stock, expanding to venues using digital projectors.
The film had a worldwide gross over $677 million (and $773 million with subsequent re-releases), making it the tenth-highest grossing film of 2014.
It received acclaim for its performances, direction, screenplay, musical score, visual effects, ambition, themes, and emotional weight.
It has also received praise from many astronomers for its scientific accuracy and portrayal of theoretical astrophysics. Since its premiere, Interstellar gained a cult following,[5] and now is regarded by many sci-fi experts as one of the best science-fiction films of all time.
Interstellar was nominated for five awards at the 87th Academy Awards, winning Best Visual Effects, and received numerous other accolades"""
]

db = FAISS.from_texts(texts=texts, embedding=embedding_model)

question = "Income generated"

prompt_tpl = """Relevant information:
{context}

Provide a concise answer to the following question using the relevant information provided above:
{question}
"""

llm = Llama.from_pretrained(
    repo_id="microsoft/Phi-3-mini-4k-instruct-gguf",
    filename="Phi-3-mini-4k-instruct-q4.gguf",
    n_gpu_layers=-1,
    n_ctx=4096,
    verbose=False,
)

docs = db.similarity_search(question, k=2)
context = "\n\n".join(doc.page_content for doc in docs)

prompt = prompt_tpl.format_map({"context": context, "question": question})

response = llm.create_chat_completion(messages=[{"role": "user", "content": prompt}])

print(response['choices'][0]['message']['content'])

llm.close()
```

得到的结果是

> Interstellar generated a worldwide gross of over $677 million, making it the tenth-highest grossing film of 2014.
> It also earned $773 million with subsequent re-releases. Additionally, it received numerous accolades, including an
> Academy Award for Best Visual Effects. However, specific income generated from ticket sales or other revenue streams
> is not provided in the given information.

##### 高级 RAG 技术

- RAG 改写，使用 LLM 将原始查询转化为更利于检索的简单形式， 如

> 用户提问：“我们明天有一篇关于动物的作文要交。我喜欢企鹅，可以写关于企鹅的。但我也可以写海豚。它们是动物吗？也许是吧。我们写海豚吧。比如，它们生活在哪里？”

通过 LLM 可改写为

> 查询：“海豚生活在哪里”

- 多查询 RAG

> 用户提问：“比较 NVIDIA 2020 年与 2023 年的财报。 ”

LLM 知道要分别对 NVIDIA 2020 年财报和 NVIDIA 2023 年财报进行 RAG 查询，再对比

- 多跳 RAG

> 用户提问：“2023 年排名最靠前的汽车制造商有哪几个？它们是否都生产电动汽车？”

LLM 会分多步，先查到得到相应汽车制造商的名称，再分别查询它们是否生产电动汽车，最后综合回答。

其实这些都可以通过工具(Tools) 来实现。

##### RAG 效果评估

有以下相关指标

- 流畅性（fluency） 生成文本的语言流畅度与逻辑连贯性。
- 感知效用（perceived utility） 回答内容的信息价值与实用价值。
- 引用召回率（citation recall） 外部事实陈述中获得完整引证支持的比例。
- 引用精确率（citation precision） 引用内容对相关论断的支持的有效性。
- 忠实度（faithfulness） 答案与所提供上下文的一致性程度。
- 答案相关性（answer relevance） 答案与提问主题的契合度。