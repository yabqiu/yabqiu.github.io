---
title: "《从零构建大模型》阅读笔记(一)"
url: /build-a-large-language-model-from-scratch-reading-notes-1/
date: 2026-06-02T16:49:20-05:00
featured: false
draft: true
type: post
toc: false
# menu: main
usePageBundles: true
thumbnail: "../images/logos/build-llm.jpg"
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

读完《Hands-On Large Language Models》又回头重新看《Build a Larget Language Model from Scratch》一书，阅读时零乱记点什么。

大语言模型构建通常包括两个阶段: 预训练(pre-training)和微调(fine-tuning), 又时也把偏好调优当作一个单独阶段，其实也是一种形式的微调。
预训练使用未标注的大量文本数据理解语言结构，即自监督学习，预训练生成的模型为基础模型(base/foundation model), 这能够完成文本被全任务。
再经过指令微调或分类任务微调后就能回答问题或进行分类了。

Transformer 架构最初由 Google 于 2017 年的论文 "Attention is All You Need" 提出，它已成为了现代大语言模型的基础架构，基本上 LLM 指的就是这种架构。
Transformer 的两个子模块：编码器(encoder)和解码器(decoder), 一大关键组件是自注意力机制(self-attention), 使模型能够捕捉输入序列中不同位置之间的关系和依赖性。
Transformer 的并行处理能力和长距离依赖建模能力使其成为了训练大规模语言模型的理想选择。Transformer 的变体有 BERT(Bidirectional Encoder Representations from Transformers)、
GPT(Generative Pre-trained Transformer)等。

GPT-3 基础模型预训练数据集为过滤后的 Common Crawl, WebText2, Book1, Books2, 和 Wikipedia, 在共 3000 亿 Token 上进行训练，
文本压缩后大小为 700G 左右。Llama 扩展了它的训练数据范围，包括 Arxiv 论文(92G)和 StackExchange 上的代码问答(78G)。训练基础模型成本极高，
GPT-3 的预训练成本高达 460 万美元。

词嵌入相关的概念有 BPE(byte pair encoding), word2vec. GPT-2 参数 117M, 词嵌入维度是 768, GPT-3 参数 175B, 它的词嵌入维度是 12,288。