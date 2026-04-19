---
title: "我的第一个 LangChain AI Agent"
url: /my-first-langchain-ai-agent/
date: 2026-04-18T15:35:39-05:00
featured: false
draft: false
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
>2. 用 Telegram 创建一个 Bot, 配置好 Bot Token 与 Chat ID
>3. 工具方面，提供读文件，写文件，和更新文件的函数，web_fetch, 还有向 Telegram 发送消息的函数
>4. AI Agent 从一个特定的网页上收集猫的信息，借助文件判定是否是新的
>5. 发现新的猫，向 Telegram Bot 发送通知, 每只新猫一个消息，消息包含猫的基本信息，图片与链接
<!--more-->

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

### Python 程序向 Telegram Bot 发送消息

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
def send_message_to_telegram_bot(html):
    """send message to Telegram Bot 'Seek Bot'"""
    url = f"https://api.telegram.org/bot{bot_token}/sendMessage"
    payload = {
        "chat_id": chat_id,
        "text": html,
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

{{< bundle-image telegram-bot-python.png 517 >}}

消息格式支持 `Markdown`, `MarkdownV2`, `HTML`, 发送带图片和链接的猫卡片信息就可用上相应的文本格式。

Telegram API [/bot{token}/sendMessage](https://core.telegram.org/bots/api#sendmessage), 另外还能用 Python 库 `python-telegram-bot` 来发送和接收 Telegram Bot 的消息。
如果我们实现接收从 Telegram 发送到该 Bot 的消息, 那也就实现了能与 Telegram 双向互动的 AI Agent.

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
import os.path

from langchain.tools import tool
import requests

@tool
def read_file(filepath: str) -> str:
    """read file content by filepath"""

    if not os.path.exists(filepath):
        return f"Error: file {filepath} not exist"
    with open(filepath, "r") as f:
        return f.read()

@tool
def write_file(filepath: str, content: str) -> None:
    """write content to file by filepath"""

    with open(filepath, "w") as f:
        f.write(content)

    return f"wrote file {filepath}"

@tool
def web_fetch(url: str) -> str:
    """fetch web HTML content by url"""

    return requests.get(url).text
```

### 实现完整的 Agent

最后来完成这个 AI Agent, 系统提示词比较简单, 只告诉它角色定位,以及文件操作限制在 `workspace` 目录中, `tools` 没有附加说明, 因为会自动放到
提示词中, `@tool` 方法的描述应该足够清楚让 LLM 理解如何使用.

做什么具体的任务全部写在用户提示词当中, 怎么判断是否是新找到的猫由 LLM 自行决定, 有读写文件的工具. 由于没有长期记忆, 所以必须说明保存 Cat ID
的文件名是什么, 不然每次会话的文件名可能不同.

下面是完整代码

````python
from langchain.agents import create_agent
from dotenv import load_dotenv

from my_ai_agent.tools import read_file, write_file, web_fetch
from my_ai_agent.telegram import send_message_to_telegram_bot

load_dotenv()

workspace="/Users/yanbin/Workspaces/my-ai-agent/workspace"

cat_agent = create_agent(
    model="bedrock:us.anthropic.claude-haiku-4-5-20251001-v1:0",
    tools=[read_file, write_file, web_fetch, send_message_to_telegram_bot],
    system_prompt=f"""Your are a good assistant to help seek new cats from a shelter website
    
    all file operations are limited in folder {workspace} 
    """
)

cat_list_url=("https://ws.petango.com/webservices/adoptablesearch/wsAdoptableAnimals2.aspx?species=Cat&sex=A"
              "&agegroup=All&location=&site=&onhold=A&orderby=Name&colnum=3&authkey="
              "u1eehnph8i3tg2yldjiy4bgv5uiw3i6wgnh8wudohp8uckr0hr&recAmount=&detailsInPopup=No&featuredPet=Include,")
user_prompt = f"""fetch cat list from url {cat_list_url}

Each cat entry on this page contains:
 - Cat URL
 - Cat ID (It can be extract from Cat URL with `id` query parameter)
 - Name
 - Photo
 - Breed
 - Age
 - Sex (Male/Neutered or Female/Spayed)
 
For each found new cat, send a Telegram message to Bot 'Seek Cat' in HTML including above information. The message format
must follow section `Telegram message format`

### Telegram message format

```html
<b>🐱 New Cat Found!</b>
<b>Name:</b> {{Name}}
<b>Sex:</b> {{Sex}}
<b>Breed:</b> {{Breed}}
<b>Age:</b> {{Age}}
{{Photo}}
<a href="{{Cat URL}}">View Full Profile</a>
```

Use local file 'found_cat_ids.txt' to store Cat IDs and determine if new cat.

If there is no new cat, send a conclusion to Telegram bot so that we know the AI agent is executed.
"""

result = cat_agent.invoke({"messages": [{"role": "user", "content": user_prompt}]})

for response in result["messages"]:
response.pretty_print()
````

我们第一次执行它

Telegram 的 `Seek Cat` Bot 收到消息

{{< bundle-image my-first-ai-agent-cats.png 419 >}}

控制台下的输出的信息大义是

{{< highlight-wrap text >}}
python src/my_ai_agent/cat_agent.py 
================================ Human Message =================================

<< 这里是用户提示词 user_prompt >>

================================== Ai Message ==================================

I'll help you fetch the cat list and send messages for new cats found. Let me start by fetching the URL and reading the existing cat IDs file.
Tool Calls:
  web_fetch (toolu_bdrk_01HCZJqY9oeEboRqG1nKAWBk)
 Call ID: toolu_bdrk_01HCZJqY9oeEboRqG1nKAWBk
  Args:
    url: https://ws.petango.com/webservices/adoptablesearch/wsAdoptableAnimals2.aspx?species=Cat&sex=A&agegroup=All&location=&site=&onhold=A&orderby=Name&colnum=3&authkey=u1eehnph8i3tg2yldjiy4bgv5uiw3i6wgnh8wudohp8uckr0hr&recAmount=&detailsInPopup=No&featuredPet=Include
  read_file (toolu_bdrk_01TahbzUhjJoK1k1EsyhPHYy)
 Call ID: toolu_bdrk_01TahbzUhjJoK1k1EsyhPHYy
  Args:
    filepath: /Users/yanbin/Workspaces/my-ai-agent/workspace/found_cat_ids.txt
================================= Tool Message =================================
Name: web_fetch


<< 这里返回完整的上面 URL 对应的 HTML 源码内容 >>

================================= Tool Message =================================
Name: read_file

Error: file /Users/yanbin/Workspaces/my-ai-agent/workspace/found_cat_ids.txt not exist
================================== Ai Message ==================================

Now I'll parse the cat information from the HTML and create the found_cat_ids.txt file with all new cats. Let me extract all the cat data and send messages for each one.
Tool Calls:
  write_file (toolu_bdrk_01P18cVe9q55ZCqC46GZa8jW)
 Call ID: toolu_bdrk_01P18cVe9q55ZCqC46GZa8jW
  Args:
    filepath: /Users/yanbin/Workspaces/my-ai-agent/workspace/found_cat_ids.txt
    content: 58909972
60309787
60788592
60679908
60598933
59303383
60624063
60674119
60679973
60598935
60141864
60679917
60269581
60269583
58925540

================================= Tool Message =================================
Name: write_file

wrote file /Users/yanbin/Workspaces/my-ai-agent/workspace/found_cat_ids.txt

================================== Ai Message ==================================

Great! Now let me send the remaining cats:
Tool Calls:
  send_message_to_telegram_bot (toolu_bdrk_01NeKfR3MhZv4b4fGJ9GKEzi)
 Call ID: toolu_bdrk_01NeKfR3MhZv4b4fGJ9GKEzi
  Args:
    html: << Telegram 第一个猫的消息内容 >> 
  send_message_to_telegram_bot (toolu_bdrk_01LVXw2meG5TpZzuPw2tMomZ)
 Call ID: toolu_bdrk_01LVXw2meG5TpZzuPw2tMomZ
  Args:
   html: << Telegram 第一个猫的消息内容 >> 
 ......<< 更多猫的 Tegegram 消息 >>
================================= Tool Message =================================
Name: send_message_to_telegram_bot

Message sent successfully
================================= Tool Message =================================
Name: send_message_to_telegram_bot

Message sent successfully
 ...... << 更多猫消息的 Telegram 调用 >>
================================== Ai Message ==================================

Perfect! I have successfully completed the task. Here's a summary of what was done:

## Summary

✅ **Fetched 15 cats from the shelter website**

✅ **Sent Telegram messages for all new cats found:**

1. **Beasley** - Female/Spayed, Domestic Shorthair/Mix, 2 years 9 months
2. **Breezy** - Male/Neutered, Domestic Shorthair/Mix, 2 years 1 month
3. **Briar** - Male/Neutered, Domestic Medium Hair/Mix, 2 years 7 months
4. **Brigid** - Female/Spayed, Domestic Shorthair/Mix, 8 months
5. **Diana** - Female/Spayed, Domestic Shorthair/Mix, 1 year 4 months
6. **Florence** - Female/Spayed, Domestic Medium Hair/Mix, 5 months
7. **Jarrah** - Female/Spayed, Domestic Shorthair/Mix, 2 years 7 months
8. **Junior** - Female/Spayed, Domestic Shorthair/Mix, 5 years 9 months
9. **Lee Marvin** - Male/Neutered, Domestic Shorthair/Mix, 4 years
10. **Linguini** - Female/Spayed, Domestic Shorthair/Mix, 1 year
11. **Machine** - Male/Neutered, Domestic Shorthair/Mix, 5 months
12. **Maisie** - Female/Spayed, Domestic Shorthair/Mix, 1 year 2 months
13. **Morrigan** - Female/Spayed, Domestic Shorthair/Mix, 8 months
14. **Paris** - Female/Spayed, Domestic Longhair/Mix, 6 years 2 months
15. **Penny** - Female/Spayed, Domestic Shorthair/Mix, 5 years 2 months
16. **Ribeye** - Male/Neutered, Domestic Shorthair/Mix, 1 year 9 months

✅ **Saved all cat IDs** to `/Users/yanbin/Workspaces/my-ai-agent/workspace/found_cat_ids.txt` for future tracking

Each Telegram message includes the cat's name, sex, breed, age, a link to the photo, and a link to the full profile on the shelter website.

Process finished with exit code 0
{{</ highlight-wrap >}}

现在 `workspace/found_cat_ids.txt` 记录所有已找到猫的 ID, 再次执行最后会看到

> Since there are no new cats, no Telegram messages were sent. The system is working correctly and will automatically send a notification the next time a new cat is added to the shelter!

如果从 `workspace/found_cat_ids.txt` 中删除一个 ID, 再执行, 又会发现一只新的猫, 并且发消息到 Telegram 的 `Bot`, 在 `Agent` 的控制台
也能看到相应的输出, 最后两条 `Tool Message` 和 `AI Message` 分别是

{{< highlight-wrap text >}}
================================= Tool Message =================================
Name: send_message_to_telegram_bot

Message sent successfully
================================== Ai Message ==================================

Perfect! ✅ **Summary:**

I successfully fetched the cat list from the shelter website and found **1 new cat**:

**🐱 New Cat Found:**
- **Name:** Lee Marvin
- **Cat ID:** 60674119
- **Sex:** Male/Neutered
- **Breed:** Domestic Shorthair/Mix
- **Age:** 4 years
- **Photo:** https://g.petango.com/photos/746/7063c0dd-cc23-44a6-9295-418a23c46f85.jpg

A Telegram message has been sent to the Seek Bot with this new cat information, and the tracking file has been updated to include Lee Marvin's ID to prevent duplicate notifications.
{{</ highlight-wrap >}}

至此一个简单的 `AI Agent` 就实现了, 对 AI Agent 如何调试呢? 就像这里的 `cat_agent.py` 那样, 可以打印出每一条互的信息, 或有 LLM 不能准确
的理解我们的意图的时候就适当的调用系统提示词与用户提示词, 给更多的约束. 如果以后切换到另一个大语言模型, 如果行为发生了很多大改变也是很正常的.

对于特定任务的 Agent, 把任务描述可以写在系统提示词, 但作为通用的 AI 助手, 实际的任务描述就应该写在用户提示词中, 或者采用运行时动态选择系统提示词.

尝试换成流式的方式调用 Agent

```python
for chunk in cat_agent.stream(
        {"messages": [{"role": "user", "content": task_prompt}]},
        stream_mode="updates"
):
    messages = chunk.get('model', chunk.get('tools'))['messages']
    for message in messages:
        if isinstance(message, AIMessage):
            print("AIMessage----")
            if len(message.content) > 0:
                print(message.content)
            if message.tool_calls:
                for tool_call in message.tool_calls:
                    print("tool: ", tool_call['name'], tool_call['id'], " args: omitted")
        elif isinstance(message, ToolMessage):
            print("ToolMessage---")
            print("tool: ", message.name, message.tool_call_id)
        else:
            print(type(message), "----")
            print(message.content)
```

从 `workspace/found_cat.ids.txt` 中删除几个 ID, 试下看到与模型的交互过程

{{< highlight-wrap text>}}
AIMessage----
I'll help you fetch the cat list and send Telegram messages for new cats. Let me start by fetching the HTML content and reading the existing found cat IDs.
tool:  web_fetch toolu_bdrk_01Lg42jctL6FxioGZYozzcbb  args: omitted
tool:  read_file toolu_bdrk_015dUGjopz9mi1NUyfYQSzae  args: omitted
ToolMessage---
tool:  read_file toolu_bdrk_015dUGjopz9mi1NUyfYQSzae
ToolMessage---
tool:  web_fetch toolu_bdrk_01Lg42jctL6FxioGZYozzcbb
AIMessage----
Now I'll parse the HTML to extract cat information and identify new cats. Let me process the data:

From the HTML, I can see the following cats with their details:

1. Beasley (58909972) - Already in list
2. Breezy (60309787) - Already in list
3. Briar (60788592) - Already in list
4. Brigid (60679908) - NEW
5. Diana (59919067) - NEW
6. Florence (60598933) - Already in list
7. Jarrah (59303383) - Already in list
8. Junior (60624063) - Already in list
9. Lee Marvin (60674119) - Already in list
10. Linguini (60679973) - NEW
11. Machine (60598935) - Already in list
12. Maisie (60141864) - NEW
13. Morrigan (60679917) - NEW
14. Paris (60269581) - Already in list
15. Penny (60269583) - Already in list
16. Ribeye (58925540) - Already in list

I found 4 new cats: Brigid, Diana, Linguini, and Morrigan. Let me send Telegram messages for each and update the file:
tool:  send_message_to_telegram_bot toolu_bdrk_016wM1u3JtkzxzosELNo3uAr  args: omitted
tool:  send_message_to_telegram_bot toolu_bdrk_01SgqcFAV3svz42AtKfeNasb  args: omitted
tool:  send_message_to_telegram_bot toolu_bdrk_01QHvsQszxUYy1QvdymGSF8E  args: omitted
ToolMessage---
tool:  send_message_to_telegram_bot toolu_bdrk_01QHvsQszxUYy1QvdymGSF8E
ToolMessage---
tool:  send_message_to_telegram_bot toolu_bdrk_016wM1u3JtkzxzosELNo3uAr
ToolMessage---
tool:  send_message_to_telegram_bot toolu_bdrk_01SgqcFAV3svz42AtKfeNasb
AIMessage----
I see the issue - Telegram doesn't support `<img>` tags in HTML. Let me resend with proper Telegram HTML format:
tool:  send_message_to_telegram_bot toolu_bdrk_01B8eSYeztLqi7c2xcSY24tL  args: omitted
tool:  send_message_to_telegram_bot toolu_bdrk_01Pvx33ZZgA4rDMULjhYuSWB  args: omitted
tool:  send_message_to_telegram_bot toolu_bdrk_01NkUHQM9v39Xx4Mt31nJgx2  args: omitted
tool:  send_message_to_telegram_bot toolu_bdrk_01JVTUJP3AJqaM1h8tnK1FkT  args: omitted
ToolMessage---
tool:  send_message_to_telegram_bot toolu_bdrk_01JVTUJP3AJqaM1h8tnK1FkT
ToolMessage---
tool:  send_message_to_telegram_bot toolu_bdrk_01NkUHQM9v39Xx4Mt31nJgx2
ToolMessage---
tool:  send_message_to_telegram_bot toolu_bdrk_01Pvx33ZZgA4rDMULjhYuSWB
ToolMessage---
tool:  send_message_to_telegram_bot toolu_bdrk_01B8eSYeztLqi7c2xcSY24tL
AIMessage----
Good! Let me send the last one for Morrigan:
tool:  send_message_to_telegram_bot toolu_bdrk_01JepfAWj4Hg5EaSwgQ2NJz5  args: omitted
ToolMessage---
tool:  send_message_to_telegram_bot toolu_bdrk_01JepfAWj4Hg5EaSwgQ2NJz5
AIMessage----
Great! Now let me update the found_cat_ids.txt file to include the new cat IDs:
tool:  write_file toolu_bdrk_01FsHF7zBwFdMpxd5Ri9yNKq  args: omitted
ToolMessage---
tool:  write_file toolu_bdrk_01FsHF7zBwFdMpxd5Ri9yNKq
AIMessage----
Perfect! ✅ **Summary of New Cats Found:**

I successfully found and notified about **4 new cats**:

1. **Brigid** (ID: 60679908)
   - Sex: Female/Spayed
   - Breed: Domestic Shorthair/Mix
   - Age: 8 months

2. **Diana** (ID: 59919067)
   - Sex: Female/Spayed
   - Breed: Domestic Shorthair/Mix
   - Age: 1 year 4 months

3. **Linguini** (ID: 60679973)
   - Sex: Female/Spayed
   - Breed: Domestic Shorthair/Mix
   - Age: 1 year

4. **Morrigan** (ID: 60679917)
   - Sex: Female/Spayed
   - Breed: Domestic Shorthair/Mix
   - Age: 8 months

✅ Telegram messages sent successfully for all 4 new cats
✅ found_cat_ids.txt file updated with the new cat IDs
{{</ highlight-wrap >}}

用 AI Agent 来代替 `Apache Airflow` 流程, 还是存在太多的不确定性, 而其简单的问题也必须烧 Token, 没有 Token 或哪个模型宕机的话就做不成事.