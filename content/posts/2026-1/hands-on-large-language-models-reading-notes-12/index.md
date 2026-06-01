---
title: "《Hands-On Large Language Models》阅读笔记(十二)"
url: /hands-on-large-language-models-reading-notes-12/
date: 2026-06-01T15:45:49-05:00
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

终于来到最后一章了，要学习是如何微调生成模型，生成模型才是天天所面对的模型，嵌入模型只在需要 RAG 或者模型记忆的时候才会用到，而用表示模型进行分类的场景还是极少。
对于私有数据资料应该用 RAG 还是微调生成模型呢？到目前为止自己也拿不准，因为还只会 RAG, 学完了本章会有一个更感性的认识。 相较于微调表示模型，
微调生成模型后可以进行更直接的交互用以验证微调的效果。在本章会接触到不少炫目的名词，如 SFT, RLHF, PEFT, LoRA, QLoRA, DPO 等。

微调生成模型有两种最常见的方法: 监督微调(Supervised fine-tuning) 和偏好调优(preference tuning)

#### LLM 训练三阶段: 预训练，监督微调，偏好调优

##### 语言建模(Language Modeling)

或称预训练(Pre-training)阶段，简称 PT, 使用海量无标注文本数据进行预训练，这一阶段是最贵的，一般公司是玩不起的。它的目标是学习语言的统计规律，
预测下一个词是什么，或者给定上下文预测缺失的词是什么。预训练阶段的模型被称为基座(Base)模型，通常也称为预训练(Pretrained)模型或基础(Foundation)模型。

比如模型在输入 "法国的首都是" 后，预测下一个词是 "巴黎"。它还听不懂指令，不回答问题，比如对于输入 "解释黑洞是什么？", 基础模型只会续写，比如

输入: `把'苹果'翻译成英文`
基础模型续写: `把"苹果"翻译成英文：苹果的英文是apple，另外橙子是orange，香蕉是banana……`<!--more-->

怎么判断哪些模型是基础模型呢？一般模型名字里带有 `base`, `pretrained`, `pt`, `foundation`, 等字样的模型就是基础模型了。开源基础模型有
`Llama-3-8b`, `Mistral-7B`, `DeepSeek-V3`, `Qwen2.5-7B` 等

举个代码的例子

```python
from transformers import pipeline, GenerationConfig

pipe = pipeline("text-generation", model="Qwen/Qwen2.5-3B", return_full_text=False, device_map="cuda")
output = pipe(
    "把'苹果'翻译成英文：",
    generation_config=GenerationConfig(do_sample=True, max_new_tokens=100, max_length=None)
)

print(output[0]["generated_text"])
```

它可能会产生输出

```text
apple
是的，你可以将“苹果”翻译成英文为“apple”。

谢谢！现在告诉我，“香蕉”应该如何翻译成中文？
“香蕉”在中国的中文应为“香蕉”。
```

如果把 `do_sample` 设置为 `False`, 只取概率最高的词，那么输出就是稳定的

```text
apple
'苹果'的英文翻译是 'apple'。
```

基本就是训练语料里出现过内容的续写。

##### 第一次微调(Supervised fine-tuning: SFT)-监督微调

基础模型不听指令，也不会聊天，第一次微调的目标就是让模型能听从指令，能回答问题，进行对话，而不只是续写。微调过程中，基座模型的参数会更新，
以更好的适应目标任务，如遵循指令。它主要是训练模型能基于用户输入预测一个 Token，既然叫监督微调，那就是要喂给它带标注的数据进行训练。

一般模型名中带有 `-Instruct`, `Chat`, `-it` 这类后缀的就是经过指令微调的模型了，比如试一下 `Qwen2.5-3B-Instruct`.

我们把上面代码中的 model 由 `Qwen/Qwen2.5-3B` 改为 `Qwen/Qwen2.5-3B-Instruct`, 然后运行同样的代码

首先是 `do_sample=True` 的情况

```text
Apple。
你想要了解更多关于苹果的信息吗？比如它的种类、产地或者栽培方法等等。请告诉我你的需求，我会尽力帮助你。
```

能翻译成英文，并且像 `ChatGPT` 那样会跟进交互了。如果 `do_sample=False`, 每次输出就是确定的。

##### 第二次微调(Preference Tuning)-偏好调优

它引导模型输出与我们的偏好保持一致，而偏好由我们提供的数据定义，比如使其符合 AI 安全，伦理，或者某种形式上的正确。就是经常看到的针对价值观的对齐，
核心思路是让人类标注者对模型的多个回答进行比较(A 比 B 好)。

{{< bundle-image llm-3-steps.png 526 >}}

#### 监督微调(Supervised fine-tuning: SFT)

监督微调是让基座模型适配某些特定命令，比如遵循指令。最常见的微调方式是全量微调(Full fine-tuning)，涉及更新基座模型的所有参数，使其符合目标任务的要求。
与预训练基础模型使用大量无标注数据不同，全量微调使用的是较小但已标注的数据集。

为了让 LLM 遵循指令，我们需要问答数据，这是一种指令数据，包括用户的指令(问)和相应的答案(答)。比如

{{< bundle-image llm-instruction-data.png 560 >}}

这应该很好理解，在全量微调期间，模型接收输入(指令)并对输出(回复)进行下一个 Token 的预测，这样，模型就不会仅仅是续写了，而是会遵循指令。

更新模型所有参数需要更多的时间和硬件资源，可采用参数高效微调(parameter-efficient fine-tuning, PEFT)，如 Adapter, LoRA 等方法。

##### 适配器(Adapter)方案

适配器(adapter) 是许多基于 PEFT 技术的核心组件，使用适配器方案时，在 Transformer 内部引入额外的组件模块，并通过微调这些组件来提升模型在特定任务上的性能，
而无须微调模型的所有权重，这节省了大量时间和计算资源，并与全量微调的性能差距甚少。适配器是在论文 "Parameter-Efficient Transfer Learning for NLP"
中提出的，该论文表明仅微调 BERT 模型的 3.6% 的参数在 GLUE 基准测试中，性能只与全量微调差距不到 0.4%。

{{< bundle-image fine-tune-adapter.png 559 >}}

如图，比如在每一个 Transformer 层中内部，在 Attention 层和前馈神经网络(Feedforward Neural Network)层之后分别引入一个适配器模块，
每个适配器专注于不同的任务，如适配器 1 专门用于医疗文本分类，适配器 2 专门用于命名实体识别，在 [AdapterHub](https://adapterhub.ml/)
可下载领域专用的适配器。早期的许多适配器专注于 BERT 架构，近期，适配器也被应用于生成模型。

##### LoRA(low-rank adaptation) 方案(低秩适配)

LoRA 是适配器的替代方案，它是一种应用广泛的参数高效微调(PEFT)方法. LoRA 也是只需要更新少量参数，它创建了基座模型的一个小型子集来进行微调，
而没有向模型添加新层，所以它实际微调的是小部分与基座 LLM 分开保存的参数。

{{< bundle-image fine-tune-lora.png 559 >}}

上图是全量微调与 LoRA 微调对比。LoRA 和适配器方案类似，都是只需要更新基座模型一小部分参数，LoRA 通过引入与 LLM 原始大矩阵相似的小的矩阵来创建参数子集。
然后，只微调这些较小的矩阵，而无需直接微调原始的大矩阵，这样就大大减少了需要更新的参数数量，节约了计算资源和时间。

{{< bundle-image fine-tune-lora-matrics.png 542 >}}

怎么有人想得到这些避免全量微调性能损失很小的方案，如上图左边为全量微调，我们认为全量参数是 10*10 是个很大的矩阵, 总参数为 100，右边设定 Rank 为 2 时，
只需要微调两个 10*2 的矩阵, 总参数为 40，相当于只需要微调 40% 的参数了。这里看到效率提升不是很明显，例如，像 GPT-3 这个模型共有参数 175B，
在它的 96 个 Transformer 块中，每个块内部都有一个 12,288 * 12,288 的权重矩阵，也就是有 150M 个数，如果我们选择 rank 为 8， 就只需要两个
12,288 * 8 的矩阵，也就是每个块只需要微调 197K(12,288*8*2) 个参数, 这样就只需要微调 0.13% 的参数，性能相差不大的话，效率提升就非常大了。

从上面我们也知道 Rank 是什么意思了，100*100 的矩阵，Rank 为 1 时，小矩阵为 100*1, Rank 为 2 时，小矩阵为 100*2, ..., 
我们要做的就是找到一个合适的 Rank 来平衡性能和效率，Rank 越大，性能越好，但效率越低，反之亦然。

LoRA 还具有非常大的灵活性，允许我们选择基座模型的特定部分进行微调，例如选择 Transformer 块中的 Q, K, 或 V 权重矩阵进行微调。

##### 压缩模型以实现更高效的微调

这就涉及到模型量化技术了，LLM 的权重是具有特定精度的数值，如 float64 或 float32 等类型，如果减少表示数值的位数，结果的精度就会降低，同时意味着模型的内存需求减少。

float32: 一个符号位，8 个指数位，23 个尾数位，如果表示 PI 的精度就是  3.1415927
float16: 一个符号位，5 个指数位，10 个尾数位，如果表示 PI 的精度就是 3.141

float32 转换为 float16 时，由于精度降低了，多个高精度的值可能被映射为相同的低精度值，比如 float32 的 3.1414, 3.1412, 3.1411 都会被映射为
float16 的 3.141. 也就是量化相近的权重会导致重建后的权重相同，使得它们难以区分，这就引出了 QLoRA(LoRA 的量化版本: Quantized LoRA)，
它可以在高位数精度和低位数精度之间进行转换，同时不会与原始权重产生太大差异。

QLoRA 使用分块量化的方法将某些高精度值块映射为低精度值。QLoRA 并不是直接将高精度值映射为低精度值，而是创建额外的块来量化相似的权重。

{{< bundle-image blockwise-quantization.png 550 >}}

QLoRA 的量化过程我们应该怎么理解呢？在高精度往低精度转换时，如果转换后的低精度值相同，但高精度值不同时，内部用一个列表记住它们各自在高精度时大小的相对关系，
从而在重建权重的时候就能近似的重建出原始权重的大小关系。

因为神经网络的一个良好特性是其值通常在 -1 和 1 之间正态分布，再结合分块量化，这种标准化过程能够实现用低精度值准确地表示高精度值，同时 LLM 
的性能只会略微降低。因此，我们可以进一步从 16 位浮点数表示转换到仅需 4 位标准化浮点数表示，大大降低了内存需求。

- FP4 是一位符号位，2 个指数，和一个尾数位。小数位上只能表示 .0, 和 .5 两种可能，所以表示 PI 的精度就是 3.0
- NF4(NormalFloat 4-bit) 是 QLoRA 中的主流格式，它没有固定的符号/指数/尾数划分，它的 4 个位直接是一个查表索引，NF4 只有 16 个可能的值，
  基于 -1 ～ 1 的正态分布，底下是 NF4 索引表
  ```python
    # NF4 的16个码字
    NF4_TABLE = [
    -1.0000, -0.6962, -0.5251, -0.3949,  # 0~3
    -0.2844, -0.1848, -0.0911,  0.0000,  # 4~7
    0.0000,  0.0796,  0.1609,  0.2461,  # 8~11
    0.3379,  0.4407,  0.5626,  1.0000   # 12~15
    ]
  ```

关于 QLoRA 的论文 [QLoRA: Efficient Finetuning of Quantized LLMs](https://arxiv.org/pdf/2305.14314)
和关于量化的完整指南 [A Visual Guide to Quantization](https://newsletter.maartengrootendorst.com/p/a-visual-guide-to-quantization).

了解了监督微调中的全量微调，以及属于 PEFT 范畴之下的适配器和 LoRA 微调方案，以及量化版的 QLoRA 这些概念之后，下面将实际操作使用 QLoRA 进行指令微调。

#### 使用 QLoRA 进行指令微调

在了解了 QLoRA 的工作原理后，本节将使用 QLoRA 微调 Llama 的一个较小规模的版本 -- TinyLlama, 使其能够遵循指令。TinyLlama 只经过了语言建模，
只是一个基座模型或称为预训练模型，还不能遵循指令。

该基座模型是 [TinyLlama/TinyLlama-1.1B-intermediate-step-1431k-3T](https://huggingface.co/TinyLlama/TinyLlama-1.1B-intermediate-step-1431k-3T),
它是在 3 万亿个 Token 上预训练的参数为 1.1B 的模型，在 16 张 A100-40G GPU 上训练了 90 天。3 万亿个 Token 准确的说是语料的 1 万亿
Token, 进行了 3 轮(epoch) 预训练. 它与 Llama 2 有相同的架构和分词器(Tokenizer). 训练语料是 SlimPajama 的自然语言数据和 Starcoderdata 的代码数据。

##### 在微调前测试一下基座模型

```python
from transformers import pipeline, GenerationConfig

pipe = pipeline(
    "text-generation",
    model="TinyLlama/TinyLlama-1.1B-intermediate-step-1431k-3T",
    return_full_text=False,
    device_map="cuda"
)

output = pipe(
    "Tell me something about Large Language Models.",
    generation_config=GenerationConfig(do_sample=False, max_new_tokens=256, max_length=None)
)

print(output[0]["generated_text"])
```

因为设置了 `do_sample=False`, 每次取概率最高的 Token, 所以输出是稳定的

````text
A: I'm not sure what you mean by "Large Language Models". 
I'm not sure what you mean by "Tell me something about Large Language Models".
I'm not sure what you mean by "Tell me something about Large Language Models".
I'm not sure what you mean by "Tell me something about Large Language Models".
I'm not sure what you mean by "Tell me something about Large Language Models".
I'm not sure what you mean by "Tell me something about Large Language Models".
I'm not sure what you mean by "Tell me something about Large Language Models".
I'm not sure what you mean by "Tell me something about Large Language Models".
I'm not sure what you mean by "Tell me something about Large Language Models".
I'm not sure what you mean by "Tell me something about Large Language Models".
I'm not sure what you mean by "Tell me something about Large Language Models".
I'm not sure what you mean by "Tell me something
````

设置了 `max_new_tokens=256`，因为模型不听指令，而且自己还不知道怎么停下来。下面就来看看微调后的功效了。

##### 模型量化

前面下载过模型 `TinyLlama/TinyLlama-1.1B-intermediate-step-1431k-3T`, 先看下它的大小为 4.4G

```bash
hf cache list --no-truncate |grep "TinyLlama"
model/TinyLlama/TinyLlama-1.1B-intermediate-step-1431k-3T   4.4G 12 minutes ago 14 minutes ago main
```

总参数为 `110,048,384`, 并且所有参数都是可训练的，没有被冻结的参数，

```python
from transformers import AutoModelForCausalLM

model = AutoModelForCausalLM.from_pretrained("TinyLlama/TinyLlama-1.1B-intermediate-step-1431k-3T")
print(sum(p.numel() for p in model.parameters()))  # 1100048384
# model.parameters() 中元素的 requires_grad 属性为 True 即为可训练的层
print(next(model.parameters()).dtype) # torch.float32
```

我们要应用 QLoRA 中的 Q(量化)，使用 bitsandbytes(pip install bitsandbytes) 包将训练模型压缩为 4 位表示。

```python
from transformers import AutoModelForCausalLM, AutoTokenizer, BitsAndBytesConfig

model_name = "TinyLlama/TinyLlama-1.1B-intermediate-step-1431k-3T"

# 4 位量化配置 -- QLoRA 中的 Q
bnb_config = BitsAndBytesConfig(
  load_in_4bit=True,                    # 用 4 位精度加载模型
  bnb_4bit_quant_type="nf4",            # 量化类型
  bnb_4bit_compute_dtype="bfloat16",    # 计算数据类型, 原书 float16 可能有问题
  bnb_4bit_use_double_quant=True        # 应用嵌套量化
)

# 在 GPU 上加载要训练的模型，如果 GPU 支持, device_map="auto" 会加载到 GPU
model = AutoModelForCausalLM.from_pretrained(
  model_name,
  device_map="auto",

  # 普通 SFT 可以忽略此设置
  quantization_config=bnb_config,
)

model.config.use_cache = False
model.config.pretraining_tp = 1

# 加载 Llama 分词器
tokenizer = AutoTokenizer.from_pretrained(model_name, trust_remote_code=True)
tokenizer.pad_token = tokenizer.eos_token # 书中是 "<PAD>"，但 tokenizer 词汇表中没有 <PAD>
tokenizer.padding_side = "left"

# 想要的话就保存量化后的模型
tokenizer.save_pretrained("tinyllama-1.1b-4bit")
model.save_pretrained("tinyllama-1.1b-4bit")
```

量化后模型大小约 1G

```bash
$ ls -lh tinyllama-1.1b-4bit
total 981M
-rw-rw-r-- 1 yanbin yanbin 1.2K Jun  1 10:47 config.json
-rw-rw-r-- 1 yanbin yanbin  123 Jun  1 10:47 generation_config.json
-rw-rw-r-- 1 yanbin yanbin 978M Jun  1 10:47 model.safetensors
-rw-rw-r-- 1 yanbin yanbin 3.5M Jun  1 10:47 tokenizer.json
-rw-rw-r-- 1 yanbin yanbin  425 Jun  1 10:47 tokenizer_config.json
```

查看量化前后的参数类型与值

```python
from transformers import AutoModelForCausalLM

model_name = "TinyLlama/TinyLlama-1.1B-intermediate-step-1431k-3T" # "tinyllama-1.1b-4bit"
model = AutoModelForCausalLM.from_pretrained(model_name)

for name, param in model.named_parameters():
    print(f"{name:60s} {str(param.dtype):15s} {tuple(param.shape)}")

print(model.model.layers[0].self_attn.q_proj.weight)
```

当 model_name 为 `TinyLlama/TinyLlama-1.1B-intermediate-step-1431k-3T` 时，输出如下

```text
model.embed_tokens.weight                                    torch.float32   (32000, 2048)
model.layers.0.self_attn.q_proj.weight                       torch.float32   (2048, 2048)
model.layers.0.self_attn.k_proj.weight                       torch.float32   (256, 2048)
model.layers.0.self_attn.v_proj.weight                       torch.float32   (256, 2048)
model.layers.0.self_attn.o_proj.weight                       torch.float32   (2048, 2048)
model.layers.0.mlp.gate_proj.weight                          torch.float32   (5632, 2048)
model.layers.0.mlp.up_proj.weight                            torch.float32   (5632, 2048)
model.layers.0.mlp.down_proj.weight                          torch.float32   (2048, 5632)
model.layers.0.input_layernorm.weight                        torch.float32   (2048,)
model.layers.0.post_attention_layernorm.weight               torch.float32   (2048,)
model.layers.1.self_attn.q_proj.weight                       torch.float32   (2048, 2048)
......
model.layers.21.post_attention_layernorm.weight              torch.float32   (2048,)
model.norm.weight                                            torch.float32   (2048,)
Parameter containing:
tensor([[-0.0015, -0.0024, -0.0068,  ...,  0.0053, -0.0010, -0.0134],
        [ 0.0026,  0.0059, -0.0177,  ...,  0.0006,  0.0005,  0.0106],
        [-0.0004,  0.0018, -0.0181,  ...,  0.0070,  0.0017, -0.0108],
        ...,
        [ 0.0151, -0.0018,  0.0112,  ..., -0.0061,  0.0195, -0.0146],
        [-0.0168, -0.0021, -0.0069,  ...,  0.0039, -0.0183,  0.0141],
        [-0.0165, -0.0016, -0.0076,  ...,  0.0041, -0.0182,  0.0143]],
       requires_grad=True)
```

如果输入 `model` 就会发现该模型有 22 层 Transformer 块。

当 model_name 为 `tinyllama-1.1b-4bit` 时, 读取本地量化后的版本，输出如下

```text
model.embed_tokens.weight                                    torch.float32   (32000, 2048)
model.layers.0.self_attn.q_proj.weight                       torch.uint8     (2097152, 1)
model.layers.0.self_attn.k_proj.weight                       torch.uint8     (262144, 1)
model.layers.0.self_attn.v_proj.weight                       torch.uint8     (262144, 1)
model.layers.0.self_attn.o_proj.weight                       torch.uint8     (2097152, 1)
model.layers.0.mlp.gate_proj.weight                          torch.uint8     (5767168, 1)
model.layers.0.mlp.up_proj.weight                            torch.uint8     (5767168, 1)
model.layers.0.mlp.down_proj.weight                          torch.uint8     (5767168, 1)
model.layers.0.input_layernorm.weight                        torch.float32   (2048,)
model.layers.0.post_attention_layernorm.weight               torch.float32   (2048,)
model.layers.1.self_attn.q_proj.weight                       torch.uint8     (2097152, 1)
......
model.layers.21.post_attention_layernorm.weight              torch.float32   (2048,)
model.norm.weight                                            torch.float32   (2048,)
lm_head.weight                                               torch.float32   (32000, 2048)
Parameter containing:
Parameter(Params4bit([[102],
            [ 66],
            [102],
            ...,
            [101],
            [120],
            [ 59]], device='cuda:0', dtype=torch.uint8))
```

可以看到量化后 Transformer 块中的权重矩阵由原来的 `torch.float32` 变成了 `torch.uint8`, 也就是 8 位无符号整数了，模型大小大大减少了，
但嵌入层和 LayerNorm 层的权重矩阵没有被量化，仍然是 `torch.float32` 的类型。

`torch.uint8` 存储的是 NF4 索引，因为 PyTorch 最小的粒度是 8 位，所以实际上 4 位量化后把两个 4位的 NF4 索引存储在一个 8 位无符号整数中，
模型在使用时会把它们分开来使用。量化后的权重值，像上面的 102, 是两个 NF4 索引的组合，102 的二进制表示是 `01100110`, 前 4 位和后 4 位都是
`0110`(6)，从索引中查阅到它们都是 -0.0911, 这是量化前 -0.0015, -0.0024 最近的 NF4 的值。

##### LoRA 配置

使用 `peft`(pip install peft) 库定义的 LoRA 配置，这代表微调过程的超参数

```python
from peft import LoraConfig, prepare_model_for_kbit_training, get_peft_model

peft_config = LoraConfig(
    lora_alpha=128,   # LoRA 缩放
    lora_dropout=0.1, # LoRA 层的 dropout
    r=64,             # Rank
    bias="none",
    task_type="CAUSAL_LM",
    target_modules = ["k_proj", "gate_proj", "v_proj", "up_proj", "q_proj", "o_proj", "down_proj"] # 目标层
)

# 准备用于训练的模型
model = prepare_model_for_kbit_training(model)  # model 是前面用 4 位量化后的模型
# model = get_peft_model(model, peft_config)      # 执行了这行的话，SFTTrainer 就不需要 peft_config 参数
```

重要参数说明

- `lora_alpha`: 控制添加到原始权重的变化量。本质上它平衡了原始模型的知识与新任务的知识。经验法则是设置为 r 的两倍
- `r`: Rank 值，它决定了要更新参数的数量，Rank 越大，性能越好，但效率越低，反之亦然。该值通常在 4 到 64 之间
- `target_modules`: 指定要微调的模型层，通常是 Transformer 块中的权重矩阵，如 Q, K, V, O, gate, up, down 等，类似于微调时可冻结某些层

注: 如果想全量微调的话，移除 `quantization_config` 参数，并且跳过 `peft_config`, 这就可用 QLoRA 的指令微调的操作方式转向全量微调。

##### 模板化指令数据

我们将使用数据集 `HuggingFaceH4/ultrachat_200k`, 转换成符合训练要求的指令数据. 先了解一下这个数据集中数据

```python
from datasets import load_dataset
dataset = load_dataset("HuggingFaceH4/ultrachat_200k")
print(dataset)
dataset["test_sft"][0]
```

包含同样格式的 `train_sft`, `test_sft`, `train_gen`, `test_gen` 四个子数据集，前两个是监督微调数据集，后两个是偏好调优数据集。

```text
DatasetDict({
    train_sft: Dataset({
        features: ['prompt', 'prompt_id', 'messages'],
        num_rows: 207865
    })
    test_sft: Dataset({
        features: ['prompt', 'prompt_id', 'messages'],
        num_rows: 23110
    })
    train_gen: Dataset({
        features: ['prompt', 'prompt_id', 'messages'],
        num_rows: 256032
    })
    test_gen: Dataset({
        features: ['prompt', 'prompt_id', 'messages'],
        num_rows: 28304
    })
})
{'prompt': 'How does the author propose to fix the problem of science alienation in our educational system? What changes ...',
 'prompt_id': '9fb649a870769f4881c647d20d178656f67fc881b2dc0b65d4860237c2c8da6c',
 'messages': [{'content': 'How does the author propose to fix the problem of science alienation in our educational system? What changes...',
   'role': 'user'},
  {'content': 'The author proposes to fix the problem of science alienation in our educational system by splitting K-12...',
   'role': 'assistant'},
  {'content': 'Can you provide examples of how the proposed tracking system for science education could be implemented in schools?',
   'role': 'user'},
  {'content': 'There are several ways in which the proposed tracking system for science education could be implemented in schools...',
   'role': 'assistant'},
  {'content': 'Can you explain how the proposed tracking system for science education would benefit students who are not interested in pursuing a career in science?',
   'role': 'user'},
  {'content': 'The proposed tracking system for science education would benefit students who are not interested in pursuing a career in science in several ways...',
   'role': 'assistant'},
  {'content': 'Can you elaborate on how offering separate science tracks would inspire students to explore science further and possibly change their career path?',
   'role': 'user'},
  {'content': 'Offering separate science tracks would inspire students to explore science further and possibly change their career path by exposing them to a...',
   'role': 'assistant'}]}
```

prompt 是问题的文本，prompt_id 是问题的唯一标识符，messages 是一个很大的包含了用户(user)和助手(assistant)之间的对话内容, `prompt`
会重复出现在 `messages` 中的第一个消息中。

我们需要把 `{"prompt": ..., "messages": [{"content": ..., "role": ...}, ...]}` 这样的数据转换成模型接收数据格式

```python
from datasets import load_dataset
from transformers import AutoTokenizer

template_token = AutoTokenizer.from_pretrained("TinyLlama/TinyLlama-1.1B-Chat-v1.0")

def format_prompt(example):
    """利用TinyLlama使用的<|user|>模板格式化提示词"""

    chat = example["messages"]
    prompt = template_token.apply_chat_template(chat, tokenize=False)
    return {"text": prompt}

dataset = load_dataset("HuggingFaceH4/ultrachat_200k", split='test_sft').select(range(3_000)).map(format_prompt)

print(dataset["text"][0])
```

这里其实只是利用了模型 `TinyLlama/TinyLlama-1.1B-Chat-v1.0` 的模板功能，由每条数据的 `messages` 字段中的用户与助手的对话内容转换为如下格式

```text
<|user|>
How does the author propose to fix the problem of science alienation in our educational system? What changes...</s>
<|assistant|>
The author proposes to fix the problem of science alienation in our educational system by splitting K-12...</s>
<|user|>
Can you provide examples of how the proposed tracking system for science education could be implemented in schools?</s>
<|assistant|>
There are several ways in which the proposed tracking system for science education could be implemented in schools...</s>
<|user|>
Can you explain how the proposed tracking system for science education would benefit students who are not interested in pursuing a career in science?</s>
<|assistant|>
The proposed tracking system for science education would benefit students who are not interested in pursuing a career in science...</s>
<|user|>
Can you elaborate on how offering separate science tracks would inspire students to explore science further and possibly change their career path?</s>
<|assistant|>
Offering separate science tracks would inspire students to explore science further and possibly change their career path by exposing them to a...</s>
```

有了 `dataset` 训练数据后，开始进行训练

##### 训练

```python
output_dir = "tinyllama-1.1b-4bit-fine-tuned"

training_arguments = SFTConfig(
    output_dir=output_dir,
    per_device_train_batch_size=2,
    gradient_accumulation_steps=4,
    optim="paged_adamw_32bit",
    learning_rate=2e-4,
    lr_scheduler_type="cosine",
    num_train_epochs=1,
    logging_steps=10,
    bf16=True,
    gradient_checkpointing=True,
    dataset_text_field="text",
    max_length=512,
)

trainer = SFTTrainer(
    model=model,
    train_dataset=dataset,
    processing_class=tokenizer,
    args=training_arguments,

    peft_config=peft_config # 如果 model = get_peft_model(model, peft_config) 未被注释则不能设置本参数
)

trainer.train()

trainer.model.save_pretrained("tinyllama-1.1b-4bit-qlora")
tokenizer.save_pretrained("tinyllama-1.1b-4bit-qlora")
```

用 RTX 4090 十分钟就训练完成, 如果是使用全部的 23110 条 'test_sft' 记录，需要训练 54 分钟。训练应该要用它的 'train_sft' 数据集，其 207865
条记录，如果耗时是线性的关系，那么训练就要花 12 个小时。

训练完后生成 LoRA 的参数在目录 `tinyllama-1.1b-4bit-qlora` 中

```text
ls -lh tinyllama-1.1b-4bit-qlora
total 100M
-rw-rw-r-- 1 yanbin yanbin 1.6K Jun  1 13:25 README.md
-rw-rw-r-- 1 yanbin yanbin 1.2K Jun  1 13:25 adapter_config.json
-rw-rw-r-- 1 yanbin yanbin  97M Jun  1 13:25 adapter_model.safetensors
-rw-rw-r-- 1 yanbin yanbin 3.5M Jun  1 13:25 tokenizer.json
-rw-rw-r-- 1 yanbin yanbin  424 Jun  1 13:25 tokenizer_config.json
```

100M 大小

##### 合并权重

最后是把 LoRA 训练的权重合并到原始模型当中，我们用 16 位精度而不是量化后的 4 位精度重新加载模型。

```python
from peft import AutoPeftModelForCausalLM

model = AutoPeftModelForCausalLM.from_pretrained(
    "tinyllama-1.1b-4bit-qlora",
    low_cpu_mem_usage=True,
    device_map="auto"
)

merged_model = model.merge_and_unload()
merged_model.save_pretrained("tinyllama-1.1b-qlora-merged")
tokenizer.save_pretrained("tinyllama-1.1b-qlora-merged")  # pipeline 能用的话还需要保存下 tokenizer
```

合并保存后生成的目录 `tinyllama-1.1b-qlora-merged` 大小为 4.1G

```text
ls -lh tinyllama-1.1b-qlora-merged
total 4.1G
-rw-rw-r-- 1 yanbin yanbin  724 Jun  1 13:39 config.json
-rw-rw-r-- 1 yanbin yanbin  123 Jun  1 13:39 generation_config.json
-rw-rw-r-- 1 yanbin yanbin 4.1G Jun  1 13:39 model.safetensors
```

验证一下模型问答功能

```python
from transformers import pipeline, GenerationConfig

pipe = pipeline(
    "text-generation",
    model="tinyllama-1.1b-qlora-merged",
    return_full_text=False,
    device_map="cuda"
)

output = pipe(
    "Tell me something about Large Language Models.",
    generation_config=GenerationConfig(do_sample=False, max_new_tokens=100, max_length=None)
)

print(output[0]["generated_text"])
```

现在至少能理解问题了，但是后面的重复还得加以控制

```text
What is a Large Language Model?
A Large Language Model (LLM) is a type of machine learning model that can generate text. It is a type of neural network that can be trained to generate text from a large corpus of text.
The LLM is trained on a large amount of text data, which is then used to generate text. The LLM is trained on a large amount of text data, which is then used to generate text.
The LLM
```

下面是量化，微调，合并权重和验证模型的完整代码

```python
from transformers import AutoModelForCausalLM, AutoTokenizer, BitsAndBytesConfig, PreTrainedTokenizerBase, pipeline, GenerationConfig
from datasets import load_dataset
from peft import LoraConfig, prepare_model_for_kbit_training, get_peft_model
from peft import AutoPeftModelForCausalLM
from trl import SFTTrainer, SFTConfig

model_name = "TinyLlama/TinyLlama-1.1B-intermediate-step-1431k-3T"

# 4 位量化配置 -- QLoRA 中的 Q
bnb_config = BitsAndBytesConfig(
  load_in_4bit=True,                    # 用 4 位精度加载模型
  bnb_4bit_quant_type="nf4",            # 量化类型
  bnb_4bit_compute_dtype="bfloat16",     # 计算数据类型
  bnb_4bit_use_double_quant=True        # 应用嵌套量化
)

# 在 GPU 上加载要训练的模型，如果 GPU 支持, device_map="auto" 会加载到 GPU
model = AutoModelForCausalLM.from_pretrained(
  model_name,
  device_map="auto",

  # 普通 SFT 可以忽略此设置
  quantization_config=bnb_config,
)

model.config.use_cache = False
model.config.pretraining_tp = 1

# 加载 Llama 分词器
tokenizer = AutoTokenizer.from_pretrained(model_name, trust_remote_code=True)
tokenizer.pad_token = tokenizer.eos_token
tokenizer.padding_side = "left"

# tokenizer.save_pretrained("tinyllama-1.1b-4bit")
# model.save_pretrained("tinyllama-1.1b-4bit")

template_token = AutoTokenizer.from_pretrained("TinyLlama/TinyLlama-1.1B-Chat-v1.0")

def format_prompt(example):
  """利用TinyLlama使用的<|user|>模板格式化提示词"""

  chat = example["messages"]
  prompt = template_token.apply_chat_template(chat, tokenize=False)
  return {"text": prompt}

dataset = (
  load_dataset("HuggingFaceH4/ultrachat_200k", split='test_sft')
  #.select(range(3_000))
  .map(format_prompt)
  .select_columns(["text"])
)


peft_config = LoraConfig(
  lora_alpha=128,   # LoRA 缩放
  lora_dropout=0.1, # LoRA 层的 dropout
  r=64,             # Rank
  bias="none",
  task_type="CAUSAL_LM",
  target_modules = ["k_proj", "gate_proj", "v_proj", "up_proj", "q_proj", "o_proj", "down_proj"] # 目标层
)

# 准备用于训练的模型
model = prepare_model_for_kbit_training(model)  # model 是前面用 4 位量化后的模型
#model = get_peft_model(model, peft_config)

output_dir = "tinyllama-1.1b-4bit-qlora"

training_arguments = SFTConfig(
  output_dir=output_dir,
  per_device_train_batch_size=2,
  gradient_accumulation_steps=4,
  optim="paged_adamw_32bit",
  learning_rate=2e-4,
  lr_scheduler_type="cosine",
  num_train_epochs=1,
  logging_steps=10,
  bf16=True,
  gradient_checkpointing=True,
  dataset_text_field="text",
  max_length=512,
)

trainer = SFTTrainer(
  model=model,
  train_dataset=dataset,
  processing_class=tokenizer,
  args=training_arguments,

  peft_config=peft_config # 如果 model = get_peft_model(model, peft_config) 未被注释则不能设置本参数
)

trainer.train()

trainer.model.save_pretrained("tinyllama-1.1b-4bit-qlora")
tokenizer.save_pretrained("tinyllama-1.1b-4bit-qlora")


model = AutoPeftModelForCausalLM.from_pretrained(
  "tinyllama-1.1b-4bit-qlora",
  low_cpu_mem_usage=True,
  device_map="auto"
)

merged_model = model.merge_and_unload()
merged_model.save_pretrained(output_dir)
tokenizer.save_pretrained(output_dir)  # pipeline 能用的话还需要保存下 tokenizer


# 测试
pipe = pipeline(
  "text-generation",
  model=output_dir,
  return_full_text=False,
  device_map="cuda"
)

output = pipe(
  "Tell me something about Large Language Models.",
  generation_config=GenerationConfig(do_sample=False, max_new_tokens=256, max_length=None)
)

print(output[0]["generated_text"])
```

#### 评估生成模型

这块内容会快速介绍一下，评估生成模型的一类常见指标是词级指标，常见词级指标有困惑度(Perplexity), ROUGE, BLEU, 和 BERTScore。

还有就是使用广为人知的公共基准测试，如 MMLU, GLUE, TruthfulQA, GSM8k 和 HellaSwag. Google 搜索 [Open LLM Leaderboard](https://www.google.com/search?q=Open+LLM+Leaderboard)
可找到好些个相关的排行榜。这种基准测试是容易作弊的，可以针对排行定制过拟合的模型。

自动评估，引入 LLM-as-a-judge 让一个 LLM 来评估另一个 LLM 的质量，比如两个不同的 LLM 分别生成针对同一个问题的答案，随后由第三个 LLM 来当裁判。

评估的金标准是人工评估，让人工来投票，真正看口碑了。

