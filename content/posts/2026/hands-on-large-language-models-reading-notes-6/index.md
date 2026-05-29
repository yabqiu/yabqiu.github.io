---
title: "《Hands-On Large Language Models》阅读笔记(六)"
url: /hands-on-large-language-models-reading-notes-6/
date: 2026-05-18T23:59:49-05:00
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

#### 第七章：高级文本生成技术与工具

在上一章中已经从 `AutoModelForCausalLM`, `AutoTokenizer`, `pipeline` 过渡到了稍微高那么一层的 `llama-cpp-python` 的使用，
这一章将继续学习 `LLM` 的使用, 到真正能训练，微调模型还远着呢。 其中大部分的内容都在学习 `LangChain` 的过程中有所掌握，包括记忆机制，
智能体工具调用等，所以这方面的内容没有具体展开。

本章所覆盖的在不对模型作微调的情况下提升文本生成质量的方法论与技术:

1. 模型输入/输出：模型加载与调用, 用 llama-cpp-python 演示
2. 记忆机制：增强模型的上下文记忆能力，查看 `LangChain` 短期记忆相关日志 [LangChain 核心组件之短期记忆](/langchain-core-component-short-term-memory/)
3. 智能体系统：整合外部工具实现复杂行为，用 `LangChain` 1.0 后的 `create_agent()` 将会非常简单
4. 链式架构：模块化方法与组件的衔接组合, 这是 `LangChain` 0.x 的架构，1.0 后不再使用链式架构

本章进到 `LangChain` 的学习当中，本人对 `LangChain` 已经有了一定程度的了解，由于 `LangChain` 1.0 于 2025 年 10 月份才正式发布，
显然写作本书的时候用的还是 `LangChain 0.x` 的版本，而 `LangChain` 1.0 带来了巨大的变化，所以学习当中会把书中的例子改写为 `LangChain 1.x` 的版本。

下载 llama-cpp 的 GGUF 单文件模型: [Phi-3-mini-4k-instruct-q4.gguf](https://huggingface.co/microsoft/Phi-3-mini-4k-instruct-gguf/resolve/main/Phi-3-mini-4k-instruct-q4.gguf?download=true)
<!--more-->

再安装 Python 依赖

> uv add langchain langchain-community llama-cpp-python

以上三个组件的当前版本依次为 1.3.1, 0.4.1, 和 0.3.23

```python
from langchain_community.llms import LlamaCpp

llm = LlamaCpp(
    model_path="<path-to-your>/Phi-3-mini-4k-instruct-q4.gguf",
    n_gpu_layers=-1,
    max_tokens=500,
    n_ctx=4096,
    seed=42,
    verbose=False,
)

response = llm.invoke("Hi! My name is Maarten. What is 1 + 1?")
print(response)
```

书中的例子输出为空白，而我的执行是有输出的

````text
<|assistant|> Hello Maarten! The answer to 1 + 1 is 2.
```

This response directly answers the user's question while maintaining a polite and friendly tone, suitable for an introductory conversation.
````

但这个输出是有问题的，输出中不应该再看到模型的特殊 `Token`, `<|assistant|>`.

`LangChain` 0.x 版本内部实现是 `Chain`, 而 `LangChain` 1.0 后内部是 `GraphState`. 

所谓的提示词模板就是能把

```json
[
  {"role": "user", "content": "Hey"},
  {"role": "assistant", "content": "Hey yourself!"}
]
```

易于阅读的消息格式转换成

```text
<|user|>
Hey<|end|>
<|assistant|>
Hey yourself!<|end|>
<|endoftext|>
```

注意，不同的模型有不同的特殊 `Token`, 比如有些时候能看到 `<s>`, `<SEP>` 等。其实以后应该不会直接面对 `<|user|>`, `<|assistant|>` 这些关键字了，
这个抽象已经在模型的服务层屏蔽了，如使用 `Ollama` 的服务 `http://localhost:11434` 时直接传递 JSON 格式的数据.

看了一下用 `LangChain` 0.x 的 `Chain` 还真是麻烦

```python
template = """<|user|>
{input_prompt}<|end|>
<|assistant|>"""
prompt = PromptTemplate(
    template=template,
    input_variables=["input_prompt"]
)

basic_chain = prompt | llm # | 是一个链操作，重载的 `__or__()` 函数

response = basic_chain.invoke({"input_prompt": "Hi! My name is Maarten. What is 1 + 1?"})
print(response)
```

如果有多个链多个提示词的话

```python
from langchain_community.chains import LLMChain
from langchain_core.prompts import PromptTemplate

template = """<s><|user|>
Create a title for a story about {summary}. Only return the title.<|end|>
<|assistant|>"""
title_prompt = PromptTemplate(template=template, input_variables=["summary"])
title = LLMChain(llm=llm, prompt=title_prompt, output_key="title")

...
character = LLMChain(llm=llm, prompt=character_prompt, output_key="character")

...
story = LLMChain(llm=llm, prompt=story_prompt, output_key="story")

llm_chain = title | character | story
llm_chain.invoke("how are you?")
```

真够麻烦的

后面的代码应该以如下为蓝本

```python
from llama_cpp import Llama

llm = Llama.from_pretrained(
    repo_id="microsoft/Phi-3-mini-4k-instruct-gguf",
    filename="Phi-3-mini-4k-instruct-q4.gguf",
    n_gpu_layers=-1,
    max_tokens=500,
    n_ctx=4096,
    verbose=False,
)

response = llm.create_chat_completion(messages=[
    {"role": "user", "content": "Hi! My name is Maarten. What is 1 + 1?"}
])
print(response['choices'][0]['message']['content'])
```

输出

> Hello Maarten! 1 + 1 equals 2. It's a basic arithmetic operation.

由 `llm` 可得到 `tokenizer`, `input_ids`, 如调试

```python
llm.tokenizer().decode(llm.input_ids)
```

输出

```text
"<|user|> Hi! My name is Maarten. What is 1 + 1?<|assistant|> Hello Maarten! 1 + 1 equals 2. It's a basic arithmetic operation."
```

这是与 llama-cpp 模型的交互文本. 上面的 `llm` 中可以查看到不少有用的信息，例如 `llm.token_bos()` 为 `1`, `llm.token_eos` 为 `32000`,
`llm.token_nl()` 是 `13`, 用 `llm.tokenizer().decode([1])` 解码, 它们分别是 `''`, `''`, 和 `\n`, `1` 和 `32000` 是一样的。

`llm` 还有 `generate()` 方法. `llm.metadata['tokenizer.chat_template']` 是它所用的模板

```text
{{ bos_token }}{% for message in messages %}{% if (message['role'] == 'user') %}{{'<|user|>' + '
' + message['content'] + '<|end|>' + '
' + '<|assistant|>' + '
'}}{% elif (message['role'] == 'assistant') %}{{message['content'] + '<|end|>' + '
'}}{% endif %}{% endfor %}
```

`bos_token` 为空字符串。在 HuggingFace 站点上也可以查看到模型相应的 `chat_template`, 如 [Phi-3-mini-4k-instruct-q4.gguf](https://huggingface.co/microsoft/Phi-3-mini-4k-instruct-gguf)
中的 `tokenizer.chat_template`; [google/gemma-4-E4B-it](https://huggingface.co/google/gemma-4-E4B-it/blob/main/chat_template.jinja).

#### LLM 的记忆

LLM 本身没有记忆，它所谓的记忆全是对话中提供的信息，比如你把相关系统先告诉它，或者把对话历史(user, assistant 之间的交互)给它重复，
它才知道之前聊过的内容，其实与 LLM 的每一次交互发给模型的所有内容就是完整的提示词。 当上下文大小不断增大时，必须裁剪对话历史，或由另一个
LLM 进行历史信息总结摘要，这些在 `LangChain` 1.0 后都可以通过 `Middleware` 来处理。

#### 构建智能体

`LLM` 结合外部工具调用就能实现一个智能体，它的核心驱动为 ReAct(Reasoning and Acting) – 三个阶段完成认知闭环: Thought, Action, Observation.

用 `LangChain` 1.0 以上的版本, `create_agent()`, 或者 `init_chat_model()` 再 `bind_tools()` 实现方法调用都非常的简单，下面尝试用
`llama_cpp` 通过系统提示词手工来实现工具调用。

首先要选择一个能使用工具的模型，这里选择 `GGUF` 格式的 [unsloth/gemma-4-E4B-it-GGUF](https://huggingface.co/unsloth/gemma-4-E4B-it-GGUF).
没有参照它的 `chat_template` 模板 [gemma-4 tokenizer.chat_template](https://huggingface.co/unsloth/gemma-4-E4B-it-GGUF/blob/main/gemma-4-E4B-it-Q4_0.gguf),
它的这个模板看起来非常的复杂。

完整代码如下

```python
import re
import inspect

from llama_cpp import Llama

llm = Llama.from_pretrained(
    repo_id="unsloth/gemma-4-E4B-it-GGUF",
    filename="gemma-4-E4B-it-Q4_0.gguf",
    n_gpu_layers=-1,
    n_ctx=8192,
    flash_attn=True,
    verbose=False,
)

def get_my_location() -> str:
    print(f"  [get_location]")
    return 'Chicago'

def get_weather(city: str) -> str:
    print(f"  [get_weather] {city}")
    return f'It\'s sunny in {city}, temperature is 25°C.'

TOOLS = {"get_my_location": get_my_location, "get_weather": get_weather}

def _tool_signature(name: str, fn) -> str:
    params = list(inspect.signature(fn).parameters.keys())
    return f"{name}({', '.join(params)})"

TOOL_SIGNATURES = {name: _tool_signature(name, fn) for name, fn in TOOLS.items()}

SYSTEM_PROMPT = f"""You are a helpful assistant. Answer the user's question using the available tools.

Available tools:
- get_my_location(): get my current location, returns the city name
- get_weather(city): get the weather for a given city, returns weather information

You must follow this loop until you have a final answer:

Thought: <your reasoning about what to do>
Action: <tool_name>(<argument>)
Observation: <tool result>

When you have enough information:
Thought: I now know the final answer.
Final Answer: <your answer>

Important:
- Call only one tool per step.
- Tool names must be exactly one of: {list(TOOLS.keys())}
- For tools with no parameters write: Action: get_my_location()
- For tools with parameters write positional values only, no keyword names: Action: get_weather(Chicago)
- Never fabricate an Observation. Always wait for the real result.
"""

def parse_action(text: str):
    match = re.search(r"Action:\s*(\w+)\(([^)]*)\)", text)
    if match:
        arg = match.group(2).strip().strip("\"'")
        arg = re.sub(r"^\w+=", "", arg).strip().strip("\"'")
        return match.group(1), arg
    return None, None

def call_tool(name: str, arg: str):
    fn = TOOLS[name]
    params = list(inspect.signature(fn).parameters.keys())
    if params:
        return fn(arg)
    return fn()

def react(question: str, max_steps: int = 6):
    print(f"\nQuestion: {question}\n")

    messages = [
        {"role": "system", "content": SYSTEM_PROMPT},
        {"role": "user", "content": question},
    ]
    scratchpad = ""

    for step in range(max_steps):
        current_messages = messages.copy()
        if scratchpad:
            current_messages.append({"role": "assistant", "content": scratchpad})

        response = llm.create_chat_completion(
            messages=current_messages,
            stop=["Observation:"],
            max_tokens=256,
            temperature=0.1,
        )

        chunk = response["choices"][0]["message"]["content"].strip()
        print(f"--- step {step + 1} ---\n{chunk}")

        if "Final Answer:" in chunk:
            final = chunk.split("Final Answer:")[-1].strip()
            print(f"\n=== Final Answer: {final} ===")
            return final

        tool_name, tool_arg = parse_action(chunk)
        if tool_name and tool_name in TOOLS:
            observation = call_tool(tool_name, tool_arg)
        elif tool_name:
            observation = f"Unknown tool '{tool_name}'. Use one of: {list(TOOLS.keys())}"
        else:
            observation = "No valid Action found. Follow the format: Action: tool_name(argument)"

        print(f"Observation: {observation}\n")
        scratchpad += chunk + f"\nObservation: {observation}\n"

    return "Max steps reached without a final answer."

react("Where am I, and how about the weather there?")

llm.close()
```

下面是代码执行结果

```text
Question: Where am I, and how about the weather there?

--- step 1 ---
Thought: I need to find the user's current location first, and then use that location to get the weather. I will start by calling `get_my_location()`.
Action: get_my_location()
  [get_location]
Observation: Chicago

--- step 2 ---
Thought: I have the user's location, which is Chicago. Now I need to get the weather for Chicago using the `get_weather` tool.
Action: get_weather(Chicago)
  [get_weather] Chicago
Observation: It's sunny in Chicago, temperature is 25°C.

--- step 3 ---
Thought: I have successfully retrieved the user's location (Chicago) and the weather for that location (sunny, 25°C). I now have all the necessary information to answer the user's request.
Final Answer: You are in Chicago, and the weather there is sunny with a temperature of 25°C.

=== Final Answer: You are in Chicago, and the weather there is sunny with a temperature of 25°C. ===
```

输出中很清楚的输出每一步

- Thought: 模型分析问题，是否要调用工具，是的话工具名和相应参数是什么, 没有工具调用则跳出循环
- Action: 客户端调用工具，得到工具的调用结果
- Observation: 工具调用的结果又附加传给模型，重新进入 Thought 步骤

#### 小结

由于本书中的 `LangChain` 还是 0.x 的版本，1.0 有了非常大的变化，所以不在实验 `LangChain` 0.x 的相关代码。在学习本章主要借机练习了
`llama-cpp-python` 的使用，和如何实现一个原始的 ReAct 框架。

关于 `ReAct` 的论文 [Shunyu Yao et al. “ReAct: Synergizing Reasoning and Acting in Language Models.” arXiv preprint arXiv:
2210.03629 (2022)](https://arxiv.org/abs/2210.03629)