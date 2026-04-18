---
title: "我的第一个 LangChain AI Agent"
url: /my-first-langchain-ai-agent/
date: 2026-04-18T13:10:39-05:00
featured: false
draft: true
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
comment: true
codeMaxLines: 50
showLastmod: true
lastmod:
---

学习了一段时间 LangChain，了解到用 `create_agent()` 创建的 Agent 的用法，以及底层 `init_chat_model()` 与模型的交互，决定以古法的方式
亲自创建一个 AI Agent, 要实现的功能是把原来用 Apache `Airflow` 做的一个从猫收留网站上查看有没有新进入收容所的猫，有则发邮件通知。换成 AI
Agent 的话，功能列表是

>1. 使用 AWS Bedrock 上的一个 Claude 模型
>2. 用 Telegram 创建一个 Bot, 与该 AI Agent 完成配对
>3. 工具方面，提供读文件，写文件，和更新文件的函数，web_fetch, 还有向 Telegram 发送消息的函数
>4. AI Agent 从一个特定的网页上收集猫的信息，借助文件判定是否是新的
>5. 发现新的猫，向 Telegram Bot 发送通知, 每只新猫一个消息，消息包含猫的基本信息，图片与链接

### 准备项目

创建项目，`workspace` 目录(`AI Agent` 在其中操作文件), 引入依赖

```bash
uv init --lib my-ai-agent
cd my-ai-agent
mkdir workspace
uv add langchain dotenv langchain-aws
```
完后整个目录结构是

```text
my-ai-agent
├── pyproject.toml
├── README.md
├── src
│   └── my_ai_agent
│       ├── __init__.py
│       └── py.typed
├── uv.lock
└── workspace
```

### Python 程序与 Tegertam Bot 配对

在 `Telegram` 中找到 `BotFather`, 点击 `Open`，然后用 `Create a New Bot` 来创建一个 `Bot`, 名称 `Seek Cat`, 用户名为
`t.me/seek_cat_bot`, 也可与 `BotFather` 对话时用 `/newbot` 命令创建。创建 Bot 后，会得到一个 `Token`, 把它存入到 `.env` 中

```properties
TELEGRAM_BOT_TOKEN=<your-bot-token>
```

再寻找 `Chat ID`, 可访问 `https://api.telegram.org/bot{BOT_TOKEN}/getUpdates`, 响应中能找到 `result[0]/message/chat/id` 值，
把它也存入 `.env` 文件中，现在该 `.env` 中有两个值

```properties
TELEGRAM_BOT_TOKEN=<your-bot-token>
TELEGRAM_CHAT_ID=<your-chat-id>
```

向 `Telegram` Bot 发送消息进行测试，创建 `src/my_ai_agent/telegram.py` 文件，内容如下

```python
from langchain.tools import tool
import requests

from dotenv import load_dotenv
import os

load_dotenv()

bot_token = os.getenv("TELEGRAM_BOT_TOKEN")
chat_id = os.getenv("TELEGRAM_CHAT_ID")

@tool
def send_message_to_telegram_bot(text):
    """send message to Telegram Bot"""
    url = f"https://api.telegram.org/bot{bot_token}/sendMessage"
    payload = {
        "chat_id": chat_id,
        "text": text,
        "parse_mode": "HTML"
    }
    result = requests.post(url, json=payload).json()
    if result["ok"]:
        return  "Message sent successfully"
    else:
        return "Error sending message, " + result["description"]

if __name__ == "__main__":
    response = send_message_to_telegram_bot("hello, find a new cat!")
    print(response)
```

这里给方法加上 `@tool` 以备后面 `Agent` 用。

能成功发送消息给 Telegram Bot 的话，输出类似于

> Message sent successfully

Telegram 中

{{< bundle-img telegram-bot-python.png 517 >}}

消息格式支持 `Markdown`, `MarkdownV2`, `HTML`, 发送带图片和链接的猫卡片信息就可用上相应的文本格式。

Telegram API [/bot{token}/sendMessage](https://core.telegram.org/bots/api#sendmessage), 另外还能用 Python 库 `python-telegram-bot`
来发送和接收 Telegram Bot 的消息。

### 使用 AWS Bedrock 模型的

先要配置好 AWS Credentials，用 Profile 的方式，或者直接配置几个环境变量，如 `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`,
`AWS_SESSION_TOKEN`, 可以把这几个环境变量写入到 `.env` 文件中，而后用 `dotenv` 加载，`.env` 的内容为

```properties
AWS_ACCESS_KEY_ID=<your-access-key-id>
AWS_SECRET_ACCESS_KEY=<your-secret-access-key>
AWS_SESSION_TOKEN=<your-session-token>
```

创建 `src/my_ai_agent/cat_agent.py` 文件，内容如下

```python
from langchain.agents import create_agent
from dotenv import load_dotenv

load_dotenv()

cat_agent = create_agent(
    model="bedrock:us.anthropic.claude-haiku-4-5-20251001-v1:0",
)

response = cat_agent.invoke({"messages": [{"role":"user", "content": "how are you?"}]})
print(response)
```

执行正常并且能打印出类似如下的信息, 就说明模型，Agent 能正常工作。

{{< highlight-wrap text >}}
{'messages': [HumanMessage(content='how are you?', additional_kwargs={}, response_metadata={}, id='51b2487f-f767-40c3-8ab0-43f6b702c247'), AIMessage(content="I'm doing well, thanks for asking! I'm here and ready to help with whatever you need. How are you doing today?", additional_kwargs={'usage': {'prompt_tokens': 11, 'completion_tokens': 30, 'cache_read_input_tokens': 0, 'cache_write_input_tokens': 0, 'total_tokens': 41}, 'stop_reason': 'end_turn', 'model_id': 'us.anthropic.claude-haiku-4-5-20251001-v1:0'}, response_metadata={'usage': {'prompt_tokens': 11, 'completion_tokens': 30, 'cache_read_input_tokens': 0, 'cache_write_input_tokens': 0, 'total_tokens': 41}, 'stop_reason': 'end_turn', 'model_id': 'us.anthropic.claude-haiku-4-5-20251001-v1:0', 'model_provider': 'bedrock', 'model_name': 'us.anthropic.claude-haiku-4-5-20251001-v1:0'}, id='lc_run--019da1ff-63fa-7bc3-a84b-3429af38f40d-0', tool_calls=[], invalid_tool_calls=[], usage_metadata={'input_tokens': 11, 'output_tokens': 30, 'total_tokens': 41, 'input_token_details': {'cache_creation': 0, 'cache_read': 0}})]}
{{< /highlight-wrap >}}

### 创建 Tools 方法

Agent 需要能浏览网上，操作文件，所以我们创建文件 `src/my_ai_agent/tools.py`, 并在其中加入方法

```python
from langchain.tools import tool
import requests

@tool
def read_file(filepath: str) -> str:
    """read file content by filepath"""
    with open(filepath, "r") as f:
        return f.read()

@tool
def write_file(filepath: str, content: str) -> None:
    """write content to file by filepath"""
    with open(filepath, "w") as f:
        f.write(content)

@tool
def web_fetch(url: str) -> str:
    """fetch web HTML content by url"""
    return requests.get(url).text
```

### 实现完整的 Agent