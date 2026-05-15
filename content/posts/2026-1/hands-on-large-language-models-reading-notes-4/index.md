---
title: "《Hands-On Large Language Models》阅读笔记(四)"
url: /hands-on-large-language-models-reading-notes-4/
date: 2026-05-14T22:28:20-05:00
featured: false
draft: true
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

继续巩固 LLM 的基础知识，文本聚类与主题建模是 NLP 中的两个概念，文本聚类就文本按语义分类，如猫狗一块，苹果西瓜在一起，足球篮球放一堆，
而它们的分类名就是主题了，如动物，水果，体育。主题范围有大有小，水果可以延展到食物，体育可以缩小到球类，语义越相近，就离得越近，这其实就是物以类聚。

可用嵌入模型进行聚类，与文本聚类相关的主题建模方法 BERTopic. 下面是用 Hugging Face 上的 `arxiv_nlp` 数据集来进行本章的学习，Hugging Face
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

只要训练集的数据，包括标题，摘要，年份，分类四个字段。

文本聚类不仅可以发现已知的数据模型，更可挖掘出未知的数据模式，文本聚类当前流行的做法主要包含以下三个步骤

1. 使用嵌入模型(embedding model)将输入文档转换为嵌入向量. 将使用 `thenlper/gte-small` 模型
2. 使用降维模型(dimensionality reduction model) 将嵌入向量降至更低的维度. PCA(Principal Component Analysis) 和 
   UMAP(Uniform Manifold Approximation and Projection) 是著名的两种降给方法，将使用 UMAP
3. 使用聚类模型(cluster model) 将降维后的向量进行聚类. k 均值聚类(k-means) 和基于密度的算法，如 HDBSCAN(Hierarchical Density-Based
   Spatial Clustering of Applications with Noise)，将使用 HDBSCAN, 它是  DBSCAN 的变体。

单说一下降维，就是降维打击的降维，降维可以理解为一种压缩算法，例如三维空间中多点之间的远近关系，把它们降维到二维平面的梳密分布就一目了然了。
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
得到的结果 `clusters` 是与数据集同等大小(44949)的的数组, 值就是对应的聚类标签(在 -1 ~ 154 之间)。也就是上面代码把这 44949 篇文章的摘要分到了
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

#### 从文聚类和主题建模

前面实践了把 44949 篇文章摘要转成嵌入向量，降维后再分成发 156 个类别，至于哪个类别具体是什么主题就不知道了，或者要查看类别人为标注。
现在要扩展到主题建模，这一过程可以为每个聚类确定最具代表性的关键词，从而协助人工更准确定义类别名称。BERTopic 是一个高度模块化的文本聚类和主题建模框架。

