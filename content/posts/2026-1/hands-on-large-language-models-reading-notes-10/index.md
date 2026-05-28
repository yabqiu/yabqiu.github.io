---
title: "《Hands-On Large Language Models》阅读笔记(十)"
url: /hands-on-large-language-models-reading-notes-10/
date: 2026-05-27T23:41:49-05:00
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
codeMaxLines: 80
showLastmod: true
lastmod:
---

#### 第三部分: 训练和微调语言模型 - 微调嵌入模型

仍然是第 10 章的部分，会学习到微调嵌入模型，监督学习(Supervised Learning)和无监督学习(Unsupervised Learning)的内容。
前面学习的其实也是在一个基础模型的基础上创建一个嵌入模型。

本书把以 `bert-base-uncased` 作为基础模型创建嵌入模型叫做从头开始训练嵌入模型，有点不理解。`bert-base-uncased` 本身就是一个预训练的基础模型，
它的训练语料是 BookCorpus + English Wikipedia, 共约 33 亿词。而所谓的微调嵌入模型就是把 `bert-base-uncased` 换为另一个预训练的
sentence-transformers 模型。这与前面的训练过程又有何区别呢？

因为 `bert-base-uncased` 就可以直接用来对句子生成嵌入向量，比如下面的代码可得到句子的嵌入向量

```python
from sentence_transformers import SentenceTransformer

model = SentenceTransformer('bert-base-uncased', device="cuda")
embeddings = model.encode(["The weather is nice today"])
```

所以本章的微调嵌入模型时所谓的监督学习不过是把前面的基本模型换成了另一个专门用对比学习预训练过的 `all-MiniLM-L6-v2` 模型，其他代码完全就没变。
选基础模型的时候还得了解它是怎么训练的，训练语料是什么样的，训练目标是什么样的，这样才能知道它适不适合用来微调嵌入模型。<!--more-->

使用了有标签的数据集进行训练就是有监督的学习, 下面是书中的例子

```python
from datasets import load_dataset
from sentence_transformers import SentenceTransformer
from sentence_transformers.sentence_transformer import losses
from sentence_transformers.sentence_transformer.evaluation import EmbeddingSimilarityEvaluator
from sentence_transformers.sentence_transformer.training_args import SentenceTransformerTrainingArguments
from sentence_transformers.sentence_transformer.trainer import SentenceTransformerTrainer

# 加载训练数据
train_dataset = load_dataset(
    "nyu-mll/glue", "mnli", split="train"
).select(range(50_000)).remove_columns("idx")

# 选择基座模型
embedding_model = SentenceTransformer('sentence-transformers/all-MiniLM-L6-v2', device="cuda")

# 定义损失函数
train_loss = losses.MultipleNegativesRankingLoss(model=embedding_model)

# 定义评估器，使用语义文本相似度基准(Semantic Textual Similarity Benchmark, STSB)
# 这是一个由人工标注的句子对数据集，相似度分数在 1 ~ 5 之间
val_sts = load_dataset("nyu-mll/glue", "stsb", split="validation")
evaluator = EmbeddingSimilarityEvaluator(
    sentences1=val_sts["sentence1"],
    sentences2=val_sts["sentence2"],
    scores=[score/5 for score in val_sts["label"]], # 值转换为 0~1 之间
    main_similarity="cosine"
)

# 定义训练参数
args = SentenceTransformerTrainingArguments(
    output_dir="finetuned_embedding_model",
    num_train_epochs=1,
    per_device_train_batch_size=32,
    per_device_eval_batch_size=32,
    warmup_steps=100,
    fp16=True,
    eval_steps=100,
    logging_steps=100
)

# 训练模型
trainer = SentenceTransformerTrainer(
    model=embedding_model,
    args=args,
    train_dataset=train_dataset,
    loss=train_loss,
    evaluator=evaluator
)

trainer.train()
print(evaluator(embedding_model))
embedding_model.save("finetuned_embedding_model")
```

但是不太明白这个训练过程，首先选的损失函数是 `MultipleNegativesRankingLoss`，是一个多负例损失函数，它假定训练数据集的三个列依次为
`anchor`, `positive` 和 `negative`，而这里的 `train_dataset` 的结构是三列分别为 `premise`, `hypothesis` 和 `label`. 只可能是
`MultipleNegativesRankingLoss` 损失函数把 `train_dataset` 数据集作了如下映射

- `premise` -> `anchor`
- `hypothesis` ->  `positive`

把 `hypothesis` 当作  `premise` 的正例还说得过去，训练时只用了前两列作为全正例数据，而 `label` 列应该是被忽略了的。

但是训练后 `evaluator(embedding_model)` 的结果却是 `{'pearson_cosine': 0.8490628613847147, 'spearman_cosine': 0.8485285321978133}`,
还是比较高。

##### 增强型 SBERT(Augmented SBERT)

训练或微调这些嵌入模型的一个挑战是需要大量的训练数据，尤其是正例和负例对，比如要用超过十亿个句子对训练。增强型 SBERT 可以让我们在只有少量标注数据(
比如几千个标注数据) 的情况下也能微调嵌入模型。**增强型 SBERT 是利用速度较慢但更精确的交叉编码器架构(BERT) 来增强和标注更大的输入对集合**，
这些新标注的数据对随后被用于微调双编码器(SBERT), 又要回顾一下什么是交叉编码器(cross-encoder)和双编码器了，
交叉编码器是把两个输入句子拼接在一起输入到模型中进行处理，而双编码器则是分别对两个输入句子进行编码，然后计算它们的相似度。

<div style="display: flex;">
   <div style="flex: 5;">
      {{< bundle-image bert-cross-encoder.png 800 >}}
   </div>
   <div style="flex: 3;">
      {{< bundle-image sbert-bi-encoder.png 320 >}}
   </div>
</div>

左边是交叉编码器架构，右边是双编码器架构, 双编码器架构计算效率更高，因为它可以预先计算和存储句子的嵌入向量，而交叉编码器需要在每次比较时重新计算两个句子的表示。

增强型 SBERT 包含以下步骤:

1. 使用小型标注数据集(黄金数据集) 微调交叉编码器(BERT)
2. 创建新的句子对(说到句子对就是对比学习)
3. 使用微调过的交叉编码器标注新的句子对(白银数据集)
4. 在扩展数据集(黄金数据集 + 白银数据集) 上训练双编码器(SBERT)

黄金数据集是一个规模较小但完全标注的数据集，包含真实标注。白银数据集也是完全标注的，但不一定是真实标注，因为它是通过交叉编码器的预测生成的。

明白了上面的工作原理后，我们开始实施

```python
import numpy as np
import pandas as pd
from datasets import load_dataset, Dataset
from sentence_transformers import SentenceTransformer, InputExample
from sentence_transformers.sentence_transformer import losses
from sentence_transformers.cross_encoder import CrossEncoder
from sentence_transformers.sentence_transformer.datasets import NoDuplicatesDataLoader
from sentence_transformers.sentence_transformer.evaluation import EmbeddingSimilarityEvaluator
from sentence_transformers.sentence_transformer.training_args import SentenceTransformerTrainingArguments
from sentence_transformers.sentence_transformer.trainer import SentenceTransformerTrainer

# 加载训练数据
train_dataset = load_dataset(
    "nyu-mll/glue", "mnli", split="train"
).select(range(50_000))

mapping = {2: 0, 1: 0, 0: 1}

gold_dataset = train_dataset.select(range(10_000))
gold_examples = [
    InputExample(texts=[row["premise"], row["hypothesis"]], label=mapping[row["label"]])
    for row in gold_dataset]
gold_dataloader = NoDuplicatesDataLoader(gold_examples, batch_size=32)

# 暂存 gold 数据集，待与 silver 数据集合并
gold = pd.DataFrame(
    {
        "sentence1": gold_dataset["premise"],
        "sentence2": gold_dataset["hypothesis"],
        "label": [mapping[label] for label in gold_dataset["label"]]
    }
)

# 在黄金数据集中训练交叉编码器, num_labels=2 很重要，不然 output 全是 1
cross_encoder = CrossEncoder('bert-base-uncased', num_labels=2, device="cuda")
cross_encoder.fit(
    train_dataloader=gold_dataloader,
    epochs=1,
    show_progress_bar=True,
    warmup_steps=100,
    use_amp=False
)

# 取 40,000 条数据组成未标注的数据集
silver_dataset = train_dataset.select(range(10_000, 50_000))
pairs = list(zip(silver_dataset["premise"], silver_dataset["hypothesis"]))

# 使用经过微调的交叉编码器标注句子对
output = cross_encoder.predict(pairs, apply_softmax=True, show_progress_bar=True)
# 获得 silver 数据集，待与 gold 数据集合并
silver = pd.DataFrame(
    {
        "sentence1": silver_dataset["premise"],
        "sentence2": silver_dataset["hypothesis"],
        "label": np.argmax(output, axis=1)
    }
)

# 最终的已标注的数据, 10,000 条 gold 数据 + 40,000 条 silver 数据(由交叉编码器标注的)
data = pd.concat([gold, silver], ignore_index=True, axis=0)
data = data.drop_duplicates(subset=["sentence1", "sentence2"], keep="first")
train_dataset = Dataset.from_pandas(data, preserve_index=False)

# 后面的训练过程就是一样的了 ------------

embedding_model = SentenceTransformer("bert-base-uncased")

# 定义损失函数
train_loss = losses.CosineSimilarityLoss(model=embedding_model)

# 定义评估器，使用语义文本相似度基准(Semantic Textual Similarity Benchmark, STSB)
# 这是一个由人工标注的句子对数据集，相似度分数在 1 ~ 5 之间
val_sts = load_dataset("nyu-mll/glue", "stsb", split="validation")
evaluator = EmbeddingSimilarityEvaluator(
    sentences1=val_sts["sentence1"],
    sentences2=val_sts["sentence2"],
    scores=[score/5 for score in val_sts["label"]], # 值转换为 0~1 之间
    main_similarity="cosine"
)

# 定义训练参数
args = SentenceTransformerTrainingArguments(
    output_dir="augmented_embedding_model",
    num_train_epochs=1,
    per_device_train_batch_size=32,
    per_device_eval_batch_size=32,
    warmup_steps=100,
    fp16=True,
    eval_steps=100,
    logging_steps=100
)

# 训练模型
trainer = SentenceTransformerTrainer(
    model=embedding_model,
    args=args,
    train_dataset=train_dataset,
    loss=train_loss,
    evaluator=evaluator
)

trainer.train()
embedding_model.save("augmented_embedding_model")
```

训练完 `evaluator(embedding_model)`

>{'pearson_cosine': 0.7093859109290936, 'spearman_cosine': 0.7151919508569721}

与上一篇用余弦相似度损失函数经过 50,000 条数据训练的结果

> {'pearson_cosine': 0.7250524707965853, 'spearman_cosine': 0.7289073126352569}

毕竟它只用了 10,000 条进行了良好标注的训练数据，其余 40,000 条是由交叉编码器标注的，虽然总数量一样，但质量还是不如前者，所以结果也就差了一点。

##### 无监督学习(Unsupervised Learning)

前面不管是黄金数据还是白银数据都是有标签的，所以前面的叫做监督学习，如果我们使用没有标注过的数据来训练模型，这就叫无监督学习，类似于前面的无监督分类。
无监督学习有许多种方法，如

- SimCSE(Simple Contrastive Learning of Sentence Embeddings, 句子嵌入的简单对比学习)
- CT(Contrastive Tension, 对比张力)
- TSDAE(Transformer-based Sequential Denoising Auto-Encoder, 基于 Transformer 的序列去噪自编码器)
- GPL(Generative Pseudo-Labeling, 生成式伪标签)

下面重点学习 TSDAE, 它的基本思路是通过删除输入句子中一定比例的词来为其添加噪声，这个 "受损" 的句子被输入编码器，编码器的上方有一个池化层，
将其映射为句子嵌入。基于这个句子嵌入，解码器尝试重建原始句子，但不包含人为添加的噪声。这里的核心概念是：句子嵌入越准确，重建的句子就越准确。
这种方法与掩码语言建模相似，但它是针对整个句子进行的，而不是单个词, TSDAE 的训练目标是最小化重建损失。

{{< bundle-image TSDAE.png 425 >}}

从图片看起来很直截了当，训练过程的代码如下

```python
import nltk
import torch
from datasets import Dataset, load_dataset
from sentence_transformers import SentenceTransformerTrainingArguments, SentenceTransformerTrainer
from sentence_transformers.sentence_transformer.datasets import DenoisingAutoEncoderDataset
from sentence_transformers.sentence_transformer.evaluation import EmbeddingSimilarityEvaluator
from sentence_transformers.sentence_transformer import model, SentenceTransformer, losses

nltk.download('punkt_tab')  # 下载 punkt_tab 分词器

mnli = load_dataset(
    "nyu-mll/glue", "mnli", split="train"
).select(range(25_000))
flat_sentences = list(mnli["premise"]) + list(mnli["hypothesis"]) # 共 50,000 条句子

# 为输入数据添加噪声
damaged_data = DenoisingAutoEncoderDataset(flat_sentences)

train_dataset = {"damaged_sentence": [], "original_sentence": []}
for data in damaged_data:  # 这里会用到 `punkt_tab` 分词器
    train_dataset["damaged_sentence"].append(data.texts[0])  # data.texts[0] 是受损的句子
    train_dataset["original_sentence"].append(data.texts[1])
train_dataset = Dataset.from_dict(train_dataset)

print(train_dataset[3])

# 定义评估器，使用语义文本相似度基准(Semantic Textual Similarity Benchmark, STSB)
# 这是一个由人工标注的句子对数据集，相似度分数在 1 ~ 5 之间
val_sts = load_dataset("nyu-mll/glue", "stsb", split="validation")
evaluator = EmbeddingSimilarityEvaluator(
    sentences1=val_sts["sentence1"],
    sentences2=val_sts["sentence2"],
    scores=[score/5 for score in val_sts["label"]], # 值转换为 0~1 之间
    main_similarity="cosine"
)

# 使用基础模型，使用 `[CLS]` token 作为池化策略，而不是对所有 token 的平均池化，因为平均池化会丢失位置信息
word_embedding_model = model.Transformer("bert-base-uncased")
pooling_model = model.Pooling(word_embedding_model.get_embedding_dimension(), "cls")
embedding_model = SentenceTransformer(modules=[word_embedding_model, pooling_model], device="cuda")

# 专门的损失函数，去噪自编码器损失函数，训练目标是最小化重建损失
# transformers 5.0.0 之后，tie_encoder_decoder 必须为 False, 并设置  decoder_name_or_path
# transformers 当前版本 5.8.0
train_loss = losses.DenoisingAutoEncoderLoss(
    embedding_model,
    decoder_name_or_path="bert-base-uncased",
    tie_encoder_decoder=False
)
train_loss.decoder = train_loss.decoder.to("cuda")

# 定义训练参数
args = SentenceTransformerTrainingArguments(
    output_dir="tsdae_embedding_model",
    num_train_epochs=1,
    per_device_train_batch_size=16,
    per_device_eval_batch_size=16,
    warmup_steps=100,
    fp16=True,
    eval_steps=100,
    logging_steps=100,
)

# 训练模型，这个训练过程比较慢，在 RTX 4090 上花了 5 分钟，前一篇几个训练只要 2 分钟
trainer = SentenceTransformerTrainer(
    model=embedding_model,
    args=args,
    train_dataset=train_dataset,
    loss=train_loss,
    evaluator=evaluator
)
trainer.train()
embedding_model.save("tsdae_embedding_model")

# 保存解码器的权重，后续用来测试受损句子重建的效果
torch.save(train_loss.decoder.state_dict(), "tsdae_embedding_model/decoder.pt")
```

`train_dataset[3]` 的输出为

{{< highlight-wrap text >}}
{'damaged_sentence': 'do you know this is information',
'original_sentence': 'How do you know? All this is their information again.'}
{{< /highlight-wrap >}}

`damaged_sentence` 是从 `original_sentence` 中删除了一些词得到的。训练完评估 `evaluator(embedding_model)`, 结果为

> {'pearson_cosine': 0.7186646635148979, 'spearman_cosine': 0.7280383796512555}

准确度与前面使用余弦相似度损失函数训练的结果相当， `pearson_cosine` 0.719 比 0.725, 都不如用多负例排序损失函数训练的效果。

##### 测试由受损句子重建的效果

上面训练生成了 `tsdae_embedding_model` 模型，并且用 `torch.save(train_loss.decoder.state_dict(), "tsdae_embedding_model/decoder.pt")`
保存了训练好的解码器权重，下面我们来测试一下受损句子重建的效果

```python
import torch
from datasets import load_dataset
from sentence_transformers import SentenceTransformer
from sentence_transformers.sentence_transformer import losses
from sentence_transformers.sentence_transformer.datasets import DenoisingAutoEncoderDataset
from transformers import AutoTokenizer

# 加载编码器
embedding_model = SentenceTransformer("tsdae_embedding_model", device="cuda")

# 初始化解码器结构
train_loss = losses.DenoisingAutoEncoderLoss(
    embedding_model,
    decoder_name_or_path="bert-base-uncased",
    tie_encoder_decoder=False
)

# 加载已保存的解码器权重
decoder_path = "tsdae_embedding_model/decoder.pt"
train_loss.decoder.load_state_dict(torch.load(decoder_path, map_location="cuda"))
train_loss.decoder = train_loss.decoder.to("cuda")
train_loss.decoder.eval()

tokenizer = AutoTokenizer.from_pretrained("bert-base-uncased")

def reconstruct_sentence(damaged_sentence: str, max_length=64) -> str:
    with torch.no_grad():
        embedding = embedding_model.encode(
            damaged_sentence,
            convert_to_tensor=True,
            device="cuda"
        ).unsqueeze(0).unsqueeze(1)  # [1, 1, hidden_dim]

        input_ids = torch.tensor([[tokenizer.cls_token_id]], device="cuda")

        for _ in range(max_length):
            outputs = train_loss.decoder(
                input_ids=input_ids,
                encoder_hidden_states=embedding,
            )
            next_token_id = outputs.logits[:, -1, :].argmax(dim=-1, keepdim=True)
            if next_token_id.item() == tokenizer.sep_token_id:
                break
            input_ids = torch.cat([input_ids, next_token_id], dim=-1)

        return tokenizer.decode(input_ids[0][1:], skip_special_tokens=True)


# 测试
mnli = load_dataset("nyu-mll/glue", "mnli", split="train").select(range(3))
test_sentences = list(mnli["premise"]) + list(mnli["hypothesis"])
damaged_dataset = DenoisingAutoEncoderDataset(test_sentences)

for data in damaged_dataset:
    damaged, original = data.texts[0], data.texts[1]
    reconstructed = reconstruct_sentence(damaged)
    print(f"Damaged:  {damaged}")
    print(f"Original: {original}")
    print(f"Rebuilt:  {reconstructed}")
    print("-" * 60)
```

下面是测试结果

```text
Damaged:  has two dimensions and geography.
Original: Conceptually cream skimming has two basic dimensions - product and geography.
Rebuilt:  the two dimensions of the two dimensions have a dual dimension.
------------------------------------------------------------
Damaged:  during the and i guess your level uh lose to the next level the parent team decide to a A guy up him a replace
Original: you know during the season and i guess at at your level uh you lose them to the next level if if they decide to recall the the parent team the Braves decide to call to recall a guy from triple A then a double A guy goes up to replace him and a single A guy goes up to replace him
Rebuilt:  and uh i think the first one i get to get to the first one to get a new one and then i get to get a new one to get a new one and then i get to the next one and then i get to get the first one to get a new one and then i get to get the next
------------------------------------------------------------
Damaged:  One of carry your instructions
Original: One of our number will carry out your instructions minutely.
Rebuilt:  you have to carry your own one of your own.
------------------------------------------------------------
Damaged:  what work.
Original: Product and geography are what make cream skimming work. 
Rebuilt:  what do you think about what work?
------------------------------------------------------------
Damaged:  You lose to the level the.
Original: You lose the things to the following level if the people recall.
Rebuilt:  you can lose the chance to lose the money.
------------------------------------------------------------
Damaged:  A member of will execute immense precision
Original: A member of my team will execute your orders with immense precision.
Rebuilt:  the commission will be able to execute a thorough execution of the commission ' s work.
------------------------------------------------------------
```

从测试结果可以看出，受损句子重建的效果基本能还原关键词。

##### 使用 TSDAE 进行领域适配(Domain Adaptation)

当我们只有很少或完全没有标注数据时，通常使用无监督学习来创建文本嵌入模型，但无监督学习表现肯定不如监督学习好，还难以学习特定领域的概念。
领域适配的目标是将现有嵌入模型适配到不同于源领域主题的特定文本领域，例如当前模型领域是 SQL, Python, Java, Rust 等编程领域要拓展到金融领域，
如 Bond, Fund, Stock, Option 等概念, 就要使用域内或域外的训练数据集对该模型进行微调，目标领域的数据更重要。

#### 本章小结

本章中学习了多种训练和微调嵌入模型的方法，许多嵌入模型的基础技术就是对比学习，标注或未标注的句子对作为语料。用标注好的数据集进行训练是监督学习，
用未标注的数据集学习就叫做无监督学习。

我们创建嵌入模型并没有完全从头开始，而是选择了一个 BERT 模型，并用到了多种损失函数，如 softmax, 余弦相似度损失函数, 多负例排序损失函数, 
去噪自编码器损失函数等。增强型 SBERT 是一种半监督学习方法, 用少量良好的标注数据微调模型的交叉编码器，利用虽然慢但能力较强的交叉编码器来标注更多的训练数据，
从而使用少量黄金数据和交叉编码器标注的大量白银数据最终来训练模型。

TSDAE 是一种无监督学习方法，通过删除输入句子中的词来添加噪声，训练一个去噪自编码器来重建原始句子, TSDAE 的训练目标是最小化重建损失。
此种方法可微调模型以适应其他特定领域的文本数据，尤其是在缺乏标注数据的情况下。

学习完后感觉本章都是在讲嵌入模型的微调技术，还要进入到下一章才是更清晰的理解什么是模型微调。