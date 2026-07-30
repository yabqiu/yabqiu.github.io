---
title: "《从零构建大模型》阅读笔记(四) - "
url: /build-a-large-language-model-from-scratch-reading-notes-4/
date: 2026-06-09T09:18:20-05:00
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

前面学习过大语言模型的分词，词嵌入，以及各种注意力机制，现在阅读到第四章: 从头实现 GPT 模型进行文本生成，将学习如何实现大语言模型的其他构建块，
并将它们组装成一个类 GPT 模型，再到下一章才会开始训练它来生成文本。

#### 构建一个大语言模型架构

大语言模型，如 GPT(生成式预训练 Transformer) 旨在预测下一个 token 的大型深度神经网络架构。本章将建立一个参数量为 124M 规模的小型 GPT-2 模型.
模型 "参数" 指的是模型的可训练权重，这些权重本质上是模型的内部变量，在训练过程中通过调整和优化来最小化特定的损失函数。

>OpenAI 已公开 GPT-2 模型的架构和权重，它的参数量为 1.5B, GPT-3 的参数量为 175B. GPT-2 是学习实现大语言模型架构的一个很好的参考，
>可以在单台电脑上试验，而要训练出 GPT-3 则要依赖 GPU 集群，根据 Lambda 实验室的说法，在单 V100 的 GPU 中训练 GPT-3 需要 355 年，在
>RTX 8000 GPU 上则要 665 年.

下图是我们将要实现的 GPT 模型的架构图, 以及各个需要实现的部件, 有些前面已经实现过，这里将把它们拼凑起来<!--more-->

<div style="display: flex;">
   <div style="flex: 40;">
      {{< bundle-image gpt-model.png 400 >}}
   </div>
   <div style="flex: 64;">
      {{< bundle-image gpt-model-components.png 640 >}}
   </div>
</div>


定义一个小型 GPT-2 模型的配置

```python
GPT_CONFIG_124M = {
  "vocab_size": 50257,      # BPE 分词器用的词汇表大小 50,257
  "context_length": 1024,   # 上下文长度
  "emb_dim": 768,           # 嵌入维度，每个 token 转化为 768 维的向量
  "n_heads": 12,            # 注意力头的数量
  "n_layers": 12,           # 层数，表示模型中 Transformer 块的数量
  "drop_rate": 0.1,         # dropout 率，防止过拟合
  "qkv_bias": False         # 是否在多头注意力机制的纯种层中添加一个偏置向量，用于 Q, K, V 的计算
}
```

先创建一个留有占位符的 GPT 主干类，我们慢慢给它填充内容

```python
import torch
import torch.nn as nn


class DummyGPTModel(nn.Module):
    def __init__(self, cfg):
        super().__init__()
        self.tok_emb = nn.Embedding(cfg["vocab_size"], cfg["emb_dim"])  # 初始化一个覆盖整个词汇表的矩阵(向量空间)
        self.pos_emb = nn.Embedding(cfg["context_length"], cfg["emb_dim"]) # 绝对位置编码，把序列中每个位置映射成同维向量
        self.drop_emb = nn.Dropout(cfg["drop_rate"])
        self.trf_blocks = nn.Sequential(  # trf: transformer, 使用占位符初始一个 n_layers 层的 transformer 块
            *[DummyTransformerBlock(cfg) for _ in range(cfg["n_layers"])]
        )
        self.final_norm = DummyLayerNorm(cfg["emb_dim"])  # 使用占位符初始一个层归一化，让权重数值保持稳定的分布
        self.out_head = nn.Linear(  # 把 `emb_dim` 维特征投影回词汇表大小，输出每个 token 位置的 logits(原始分数)
            cfg["emb_dim"], cfg["vocab_size"], bias=False
        )

    def forward(self, in_idx):
        batch_size, seq_len = in_idx.shape
        tok_embeds = self.tok_emb(in_idx)  # 把输入序列嵌入到向量空间
        pos_embeds = self.pos_emb(torch.arange(seq_len, device=in_idx.device))  # 嵌入位置编码
        x = tok_embeds + pos_embeds  # 词嵌入 + 位置嵌入，让每个向量同时携带词义和位置信息
        x = self.drop_emb(x)  # Dropout，随机遮盖一些元素来防止过拟合
        x = self.trf_blocks(x)  # 依次经过所有 Transformer 块，每个块包含多头自注意力 + 前馈网络
        x = self.final_norm(x)  # 对最后的输出做层妆一化
        logits = self.out_head(x)  # 把 `emb_dim` 维的向量投影到 `vocab_size` 维，得到词汇表中每个 token 的预测值（logits）
        return logits


class DummyTransformerBlock(nn.Module):
    def __init__(self, cfg):
        super().__init__()

    def forward(self, x):
        return x


class DummyLayerNorm(nn.Module):
    def __init__(self, normalized_shape, eps=1e-5):
        super().__init__()

    def forward(self, x):
        return x
```

这是一个类似 GPT2 模型的基本框架，使用的参数也是与 GPT2 一样，例如相同的分词器，所以词汇表大小也是一样的 50,257, 嵌入维度为 768, 12 个
Transformer 块，每个块中有 12 个注意力头，dropout 率为 0.1，防止过拟合, 也就防止泛化能力差。

`DummyTransformerBlcok` 和 `DummyLayerNorm` 都没有实现，只是简单返回输入。下面来使用一下这个未完全实现的模型

```python
import tiktoken

tokenizer = tiktoken.get_encoding("gpt2")

txt1 = "Every effort moves you"
txt2 = "Every day holds a"

batch = torch.stack([  # 两段文本组成一个 batch
    torch.tensor(tokenizer.encode(txt1)),
    torch.tensor(tokenizer.encode(txt2))
], dim=0)

# 查看每段文本怎么分词的
print(tokenizer.decode_tokens_bytes(tokenizer.encode(txt1)))
print(tokenizer.decode_tokens_bytes(tokenizer.encode(txt2)))

torch.manual_seed(123)
model = DummyGPTModel(GPT_CONFIG_124M)  # 使用前面创建的 DummyGPTModel
logits = model(batch)

# 输出每个 token 对应词汇表中每个 token(共 50257 个 token)的预测值（logits)
print(f"Output shape: {logits.shape}\nlogits:\n{logits}")
```

输出为

```text
[b'Every', b' effort', b' moves', b' you']
[b'Every', b' day', b' holds', b' a']
Output shape: torch.Size([2, 4, 50257])
logits:
tensor([[[-1.2034,  0.3201, -0.7130,  ..., -1.5548, -0.2390, -0.4667],
         [-0.1192,  0.4539, -0.4432,  ...,  0.2392,  1.3469,  1.2430],
         [ 0.5307,  1.6720, -0.4695,  ...,  1.1966,  0.0111,  0.5835],
         [ 0.0139,  1.6754, -0.3388,  ...,  1.1586, -0.0435, -1.0400]],

        [[-1.0908,  0.1798, -0.9484,  ..., -1.6047,  0.2439, -0.4530],
         [-0.7860,  0.5581, -0.0610,  ...,  0.4835, -0.0077,  1.6621],
         [ 0.3567,  1.2698, -0.6398,  ..., -0.0162, -0.1296,  0.3717],
         [-0.2407, -0.7349, -0.5102,  ...,  2.0057, -0.3694,  0.1814]]],
       grad_fn=<UnsafeViewBackward0>)
```

从以上的信息我们怎么能计算出它的参数量呢？如果目前为止让我来算总的参数的话，只会算到 12 个 Transformer 块，每个块 12 个注意力头，
Transformer 块中的所有注意力共享 3 个 768 *768 形状 {{<sub W q>}}, {{<sub W k>}}, 和 {{<sub W v>}} 的可训练权重矩阵，
于是计算如下

```text
768 * 768 * 3 * 12 = 21,233,664
再加词嵌入
50257 * 768 = 38,576,256
总 21,233,664 + 38,576,256 = 59,809,920
```

离 124M 还很远，下面是 Claude Sonnet 4.6 给出的正确计算过程

{{< bundle-image gpt2-num-params.png 716 >}}

把其他部分都实现后应该可以用下面的代码计算出总的参数量

```python
total_params = sum(p.numel() for p in model.parameters())
print(f"{total_params:,}")  # 应输出 163,009,536

total_params_gpt2 = total_params - sum(p.numel() for p in model.out_head.parameters())
print(f"{total_params_gpt2:,}")  # 应输出 124,412,160
```

#### 实现层归一化

层归化是为了解决梯度消失或梯度爆炸等问题，以提高神经网络的训练稳定性和收敛速度。层归一化的主要思想是调整神经网络层的激活(输出)，使其均值为 0
且方差为 1，层归一化它通常在多头注意力模块的前后进行。

