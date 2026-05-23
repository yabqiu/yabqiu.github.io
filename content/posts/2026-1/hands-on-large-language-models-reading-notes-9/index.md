---
title: "《Hands-On Large Language Models》阅读笔记(九)"
url: /hands-on-large-language-models-reading-notes-9/
date: 2026-05-20T13:08:49-05:00
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
codeMaxLines: 80
showLastmod: true
lastmod:
---

#### 第三部分: 训练和微调语言模型 - 构建文本嵌入模型

终于来到了可以真正实战的部分，前面的章节都是关于理解和使用大语言模型的知识，现在可以开始动手实践了。涉及到训练和微调模型，
不过本书只讲了如何训练一个嵌入模型，要学习训练一个生成模型的知识还得看
[Build a Large Language Model (From Scratch)](https://www.manning.com/books/build-a-large-language-model-from-scratch)
这本书，或者参考 Karpathy 的 [nonoGPT](https://github.com/karpathy/nanogpt) 项目。

先开始从构建一个文本嵌入模型开始吧。嵌入模型 NLP 的基础，它可应用于多种场景, 如监督分类(supervised classification), 
无监督分类(unsupervised classification), 语义搜索等, 甚至为 ChatGPT 赋予记忆功能。

嵌入模型的功能就是把非结构化的文本转换为数值表示的向量，这样才可计算，这一转换过程称为嵌入(embedding)。对输入进行嵌入通常由 LLM 执行，
这样的模型就是嵌入模型。嵌入模型可针对多种目的进行训练，如基于语义，或情感的分类等，比如通过微调使嵌入模型关注情感倾向，通过向模型展示足够多的语义相似文档，
引导模型向语义分析的方向发展，使用情感救命则向情感分析的方向发展。

训练，微调和引导嵌入模型的方法很多，其中最强大的且应用最广泛的技术是对比学习。

#### 对比学习

对比学习是训练和微调文本嵌入模型的一种主要技术。对比学习的目标是训练嵌入模型，使相似文档在向量空间中距离更近，而不相似文档相距更远。
对比学习的基本理念是，向模型输入相似的和不相似的文档对作为示例，这是学习文档之间的相似性或差异性并构建相关模型的最佳方式。

在自然语言处理领域，对比学习的一个框架是 sentence-transformers, 相应的技术是 SBERT(Sentence-BERT)，它减小了原始 BERT 的计算开销。在
SBERT 之前句子嵌入通通用交叉编码器(cross-encoder) 架构，并结合 BERT 模型来实现，交叉编码器计算 n 个句子两两相似度，需要 n(n-1)/2 次计算。

SBERT 有一种更快的创建可进行语义比较的嵌入向量，采用双编码器的方式，并通过损失函数对句子嵌入进行优化，这种方法比交叉编码更快。

#### 构建嵌入模型

在预训练嵌入模型时，可能听说过自然语言推理 NLI(natural language inference), 即判断前提(premise) 和假设(hypothesis)
两个句子间的三种关系：蕴含(正例), 矛盾(负例), 和不相关(中性)关系。

1. 蕴含(Entailment): "小明今天骑自行车去了图书馆" 和 "小明今天骑自行车了" 之间的关系是蕴含的，两句向量应该接近
2. 矛盾(Contradiction): "小明今天骑自行车去了图书馆" 和 "小明今天哪儿也没去" 的关系就是矛盾的，两句向量应该很远
3. 中间(Neutral): "小明今天骑自行车去了图书馆" 和 "小明今天买了三斤肉" 的关系是中立的，两句向量距离应该适中，去图书馆与买肉没有关系

我们将使用 GLUE(**G**eneral **L**anguage **U**nderstanding **E**valuation benchmark) 基准数据集来创建和微调嵌入模型。它包含了
392,702 带在推理关系标注(蕴含, 矛盾, 中立)的句子对。我们将使用其中的 5 万对来训练模型，剩下的用来评估模型性能。

这还得开动我的 4090 来构建这样的嵌入模型了, 在有 4090 的 Linux 机器下准备

```bash
mkdir jupyter-lab && cd jupyter-lab
python3.14 -m venv .venv
source venv/bin/activate
pip install jupyterlab
jupyter lab --ip=0.0.0.0 --port=8888 --no-browser
```

`jupyter lab` 启动后显示的 `http://127.0.0.1:8888/lab?token=a0ce201633d4e0e52e60bddf84ee39b39e8e7f20d625bd7b` 这样的链接，把
127.0.0.1 替换为实际的 IP 地址就可以在浏览器中打开，当前目录为其工作目录。或者在 `IntelliJ IDEA` 中创建 Jupyter Notebook 文件 `*.ipynb`,
然后打开文件后选择连接到该 `External Server`.




