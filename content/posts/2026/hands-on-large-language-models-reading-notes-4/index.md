---
title: "《Hands-On Large Language Models》阅读笔记(四)"
url: /hands-on-large-language-models-reading-notes-4/
date: 2026-05-15T22:00:20-05:00
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

#### 第五章：文本聚类与主题建模(Text Clustering and Topic Modeling)

继续巩固 LLM 的基础知识，文本聚类与主题建模是 NLP 中的两个概念，文本聚类就是文本按语义分类，如猫狗一块，苹果西瓜在一起，足球篮球放一堆，
而它们的分类名就是主题了，如动物，水果，体育。主题范围有大有小，水果可以延展到食物，体育可以缩小到球类，语义越相近，就离得越近，这其实就是物以类聚。

可用嵌入模型进行聚类，与文本聚类相关的主题建模方法是 BERTopic. 下面是用 Hugging Face 上的 `arxiv_nlp` 数据集来进行本章的学习，Hugging Face
还真是一个宝藏啊，估计一些禁书都能在里面找到。`arxiv_nlp` 包含了 1991 年到 2024 年来自 ArXiv cs.CL(计算与语言)板块 44949 篇摘要。

```python
from datasets import load_dataset
dataset = load_dataset("maartengr/arxiv_nlp")
```

得到的 dataset 是<!--more-->

```text
DatasetDict({
    train: Dataset({
        features: ['Titles', 'Abstracts', 'Years', 'Categories'],
        num_rows: 44949
    })
})
```

只取训练集的数据，包括标题，摘要，年份，分类四个字段。

文本聚类不仅可以发现已知的数据模式，更可挖掘出未知的数据模式，文本聚类当前流行的做法主要包含以下三个步骤

1. 使用嵌入模型(embedding model)将输入文档转换为嵌入向量. 将使用 `thenlper/gte-small` 模型
2. 使用降维模型(dimensionality reduction model) 将嵌入向量降至更低的维度. PCA(Principal Component Analysis) 和 
   UMAP(Uniform Manifold Approximation and Projection) 是著名的两种降维方法，将使用 UMAP
3. 使用聚类模型(cluster model) 将降维后的向量进行聚类. k 均值聚类(k-means) 和基于密度的算法，如 HDBSCAN(Hierarchical Density-Based
   Spatial Clustering of Applications with Noise)，将使用 HDBSCAN, 它是  DBSCAN 的变体。

单说一下降维，就是降维打击的降维，降维可以理解为一种压缩算法，例如三维空间中多点之间的远近关系，把它们降维到二维平面的疏密分布就一目了然了。
降维并不是简单删除维度，而是要把多维信息压缩到低维向量中。

下面来看完整的代码

```python
from datasets import load_dataset
from sentence_transformers import SentenceTransformer
from umap import UMAP  # 依赖 uv add umap-learn
from hdbscan import HDBSCAN # 依赖 uv add hdbscan

dataset = load_dataset("maartengr/arxiv_nlp")["train"] # [Titles, Abstracts, Years, Categories]

abstracts = dataset["Abstracts"]
titles = dataset["Titles"]

# text embeddings
embedding_model = SentenceTransformer("thenlper/gte-small", device="cuda")
embeddings = embedding_model.encode(abstracts, batch_size=10, device="cuda", show_progress_bar=True)
print(embeddings.shape)  # (44949, 384)

# dimensionality reduction, 从 384 降到 5
umap_model = UMAP(n_components=5, min_dist=0.0, metric="cosine", random_state=42)
reduced_embeddings = umap_model.fit_transform(embeddings)
print(reduced_embeddings.shape)  # (44949, 5)

# clustering
hdbscan_model = HDBSCAN(
    min_cluster_size=50, metric="euclidean", cluster_selection_method="eom"
).fit(reduced_embeddings)
clusters = hdbscan_model.labels_  # clusters 中每条数据对应的聚类标签，[1,3,-1,143]
print(len(set(clusters)))  # 156 个 cluster, cluster {0,1,2,3,...,153,154,-1}
```

`embedding_model.encode(, device="mps")` 在 Apple M3 Pro(内存 36G) 下非常的慢，所以转到我的 4090 上运行，有近 100 倍的提升。
得到的结果 `clusters` 是与数据集同等大小(44949)的数组, 值就是对应的聚类标签(在 -1 ~ 154 之间)。也就是上面代码把这 44949 篇文章的摘要分到了
156 个类别当中，至于每个类型是关于什么就要人工来标注了。根据 `clusters` 中的索引与标签值可查看同类型的几篇文章概要。

把嵌入向量降维到 2 维，分类后用 `matplotlib` 可生成一张图片

```python
import pandas as pd
import matplotlib.pyplot as plt

# dimensionality reduction, 从 384 降到 2
umap_model = UMAP(n_components=2, min_dist=0.0, metric="cosine", random_state=42)
reduced_embeddings = umap_model.fit_transform(embeddings)
print(reduced_embeddings.shape)  # (44949, 2)

hdbscan_model = HDBSCAN(
    min_cluster_size=50, metric="euclidean", cluster_selection_method="eom"
).fit(reduced_embeddings)
clusters = hdbscan_model.labels_

df = pd.DataFrame(reduced_embeddings, columns=["x", "y"])
df["title"] = titles
df['cluster'] = [str(c) for c in clusters]

# 选择离群点和非离群点(聚类)
clusters_df = df.loc[df.cluster != "-1", :]
outliers_df = df.loc[df.cluster == "-1", :]

# 生成图片
plt.scatter(outliers_df.x, outliers_df.y, alpha=0.05, s=2, c="grey")
plt.scatter(
    clusters_df.x, clusters_df.y, c=clusters_df.cluster.astype(int),
    alpha=0.6, s=2, cmap="tab20b"
)
plt.axis("off")

plt.savefig("cluster_chart.png", dpi=300, bbox_inches="tight", format="png", transparent=True)
```

{{< bundle-image clustering_chat.png 800 >}}

标签为 `-1` 的离群点用灰色表示。

#### 从文本聚类和主题建模

前面实践了把 44949 篇文章摘要转成嵌入向量，降维后再分成 156 个类别，至于哪个类别具体是什么主题就不知道了，或者要查看类别人为标注。
现在要扩展到主题建模，这一过程可以为每个聚类确定最具代表性的关键词(主题表示)，从而协助人工更准确定义类别名称。

BERTopic 是一个高度模块化的文本聚类和主题建模框架, BERTopic 先要求像前面那样完成聚类, 然后从同一个簇中确认它们相似语义的关键字(
比如根据实词的频度来计算), 把聚类和主题表示放在一起就是

{{< bundle-image cluster-to-topic.png 550 >}}

BERTopic 可以使用与聚类不同的嵌入模型, 当然也能用一样的, 下面我们在前面创建好了嵌入模型(embedding_model), 文本嵌入(text embeddings),
降维模型(umap_model), 和聚类模型(hdbscan_model) 后由 BERTopic 来真正执行前面实际的降维, 聚类, 和创建主题的操作

```python
from bertopic import BERTopic

topic_model = BERTopic(
    embedding_model=embedding_model,
    umap_model=umap_model,
    hdbscan_model=hdbscan_model,
    verbose=True,
).fit(abstracts, embeddings)
```

这一步执行很快, 真正慢的是前一步对 44949 篇文章摘要逐一转换成嵌入向量才慢, 刚刚从 Mac 的 M3 Pro(内存36G) 转到用 M4 Pro(内存48G)后, 
对摘要的嵌入向量的编码还是快多了(device="mps"), 原来 2it/s, 现在是 35it/s.

主题建模后 `topic_model.get_topic_info()` 的内容为

| Topic | Count | Name                                    | Representation                                                                                 |
|-------|-------|-----------------------------------------|------------------------------------------------------------------------------------------------|
| -1    | 14252 | `-1_the_and_of_to`                      | [the, and, of, to, in, we, for, that, language, on]                                            |
| 0     | 2207  | `0_speech_asr_recognition_end`          | [speech, asr, recognition, end, acoustic, audio, speaker, the, wer, error]                     |
| 1     | 1268  | `1_medical_clinical_biomedical_patient` | [medical, clinical, biomedical, patient, notes, healthcare, health, patients, and, extraction] |
| ...   | ...   | ...                                     | ...                                                                                            |
| 152   | 51    | `152_long_context_window_length`        | [long, context, window, length, llms, memory, contexts, extension, extensible, llm]            |

每个分类都提取出了最具代表性的关键字, 第一个主题标记为 `-1`, 就是无法归类, 被视为离群点(outliers), 可用 BERTopic 的 reduce_outliers()
将离群点重新分配到主题中. 比如看到 Topic 为 0 主题是关于 语音识别 的文章.

用 `topic_model.get_topic(topic_id)` 还能看到特定主题各关键字的评分, 由高到低排列. 有了这些关键字, 大致看下摘要内容就能为主题准确命名了.

有多种可视化图可显示, 下图用代码展示成两种类型的图

```python
# 平面聚焦分布图
reduced_embeddings = umap_model.fit_transform(embeddings)

fig = topic_model.visualize_documents(
    titles,
    reduced_embeddings=reduced_embeddings,
    width=1800,
    hide_annotations=True
)

fig.update_layout(font=dict(size=12))

# 条形图
topic_model.visualize_barchart(title=titles, n_words=10, autoscale=True, top_n_topics=4)
```

{{< bundle-image documents-and-topics.png 1000 >}}
{{< bundle-image barchart.png 600 >}}

以上主题中的关键字没有考虑语义结构,BERTopic 还有微调的表示模型对主题进一步优化, 如 `KeyBERTInspired`, 这种表示模型可以叠加应用, 如下面的
MMR 表示模型.

```python
from bertopic.representation import KeyBERTInspired

representation_model = KeyBERTInspired()
topic_model.update_topics(abstracts, representation_model=representation_model)

topic_model.visualize_barchart(title=titles, n_words=10, autoscale=True, top_n_topics=4)
```

{{< bundle-image KeyBERTInspired_barchat.png 600 >}}

与前面主题对比一下, 是不是更容易阅读, 自己斟酌.

再用最大边际相关性(MMR - maximal marginal relevance) 处理, 过滤掉冗余词, 只保留对主题表示有新贡献的词

```python
from bertopic.representation import MaximalMarginalRelevance

representation_model = MaximalMarginalRelevance(diversity=0.2)
topic_model.update_topics(abstracts, representation_model=representation_model)

topic_model.visualize_barchart(title=titles, n_words=10, autoscale=True, top_n_topics=4)
```

{{< bundle-image mmr_barchart.png 600 >}}

经过 KeyBERTInspired 和 MMR 处理后, 主题仍然不是很理解. 下面用生成模型生成更符合语义的主题, 书中使用的是 `google/flan-t5-small` 和
`GPT` 作了测试, 这里打算用本地 Ollama 的 `gemma4:e4b` 模型.

实现方式是喂给生成模型几个相关文档, 列出主题建模时确定的关键字列表, 让生成模型给出一个更有表现力的主题

```python
import openai
from bertopic.representation import OpenAI

prompt = """
I have a topic that contains the following documents:
[DOCUMENTS]

The topic is described by the following keywords: [KEYWORDS]

Based on the information above, extract a short topic label in the following
format:
topic: <short topic label>
"""

client = openai.OpenAI(base_url="http://localhost:11434/v1", api_key="any")
representation_model = OpenAI(
    client, model="gemma4:e4b", exponential_backoff=True, chat=True, prompt=prompt,
    generator_kwargs={"stop": "xxxxx"}
)
topic_model.update_topics(abstracts, representation_model=representation_model)
```

使用 `Ollama` 兼容的 `OpenAI` 的 API, 这里定义了一个提示词模板 `prompt`, `OpenAI(...)` 在应用 `prompt` 模版时会在 `[DOCUMENTS]`
位置安放几篇(通常 4 篇) 最能代表这个主题的摘要,`[KEYWORDS]` 处替换为前面主题建模生成的关键字列表, 所以模板中的 `[DOCUMENTS]` 和
`[KEYWORDS]` 的固定写法. 如果不给 `OpenAI()` 函数指定 `prompt`, 它会使用自己默认的提示词模板(bertopic.representation._openai.DEFAULT_CHAT_PROMPT),
所以 `prompt` 是可选的. 另外 `BERTopic` 可选择的模型还有 `llamacpp`, `langchain`(还是 0.x 版), `cohere`.

提示词发送给生成模型, 生成模型回一个简短的标签名. 

在通过 `OpenAI` 的 SDK 使用 `Ollama` 模型时有一点要注意, 必须设置 `generator_kwargs={"stop": "xxxxx-or-any"}`, 否则会向 `Ollama`
模型发送请求如下

```json
{
  "messages": [
    {
      "role": "system",
      "content": "You are an assistant that extracts high-level topics from texts."
    },
    {
      "role": "user",
      "content": "<包括几篇相关摘要的和关键字的提示词>"
    }
  ],
  "model": "gemma4:e4b",
  "stop": "\n"
}
```

有 `"stop": "\n"` 的话, `Ollama` 的回复是空, 不能简单去掉 `stop` 字段(除非定制一个 Ollama representation 类),
但可用 `generator_kwargs` 参数覆盖, 设置 `"stop": "xxxxx-or-any"` 后就能得到如下的完全自然语言的标签

| Topic | Count | Name                                                                                    | Representation                                                                                                                                                        |
|-------|-------|-----------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| -1    | 14252 | -1,14252,-1_Advanced Natural Language Processing (NLP) Techniques and Model Development | -1,14252,-1_Advanced Natural Language Processing (NLP) Techniques and Model Development,[Advanced Natural Language Processing (NLP) Techniques and Model Development] |
| 0     | 2207  | 0_End-to-End Automatic Speech Recognition and Machine Translation                       | [End-to-End Automatic Speech Recognition and Machine Translation]                                                                                                     |
| 1     | 1268  | 1_Biomedical Information Extraction from Clinical Notes and Records                     | [Biomedical Information Extraction from Clinical Notes and Records]                                                                                                   |
| ...   | ...   | ...                                                                                     | ...                                                                                                                                                                   |
| 152   | 51    | 152_Long-Context LLM Extension, Retrieval Augmentation, and Evaluation                  | [Long-Context LLM Extension, Retrieval Augmentation, and Evaluation]                                                                                                  |

使用上了生成模型, 一般不用担心它不能给你生成一个适合人类阅读的标签, 即使难以归纳它也会发挥它的幻觉优势给你创造出来.

有这个列表就没必要再用图展示那些分类标签了.

##### 小结

学习了生成模型和表示模型如何无监督(没有标注数据的情况下)地对文本进行分类, 对主题建模. 再次简单回顾一下文本聚类和主题建模的全过程

文本嵌入(用嵌入模型) -> 向量降维(用降维模型, 如 PCA 或 UMAP 方法) -> 进行聚类(用聚类模型, 如 K-Means 或 HDBSCAN) -> 
主题建模(如 BERTopic) -> 表示模型或生成模型对主题名称求精.