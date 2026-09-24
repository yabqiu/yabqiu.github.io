---
title: "配置编程 Agents 远程使用自部署的 Ollama 服务"
url: /ollama-with-remote-code-agents/
date: 2026-09-22T20:20:20-05:00
featured: false
draft: false
type: post
toc: false
# menu: main
usePageBundles: true
thumbnail: "../images/logos/ai-logo.png"
categories:
  - AI
tags: 
  - Ollama
  - Claude Code
  - Codex
  - Copilot
  - OpenCode
  - Pi
comment: true
codeMaxLines: 80
showLastmod: true
lastmod:
---

ChatGPT 免费版聊天都不够用, 而订阅了 ChatGPT Plus 虽说能用 Codex, 但别指望用它来真正做一个项目, 基本是实现还没开始就会要求禁闭五小时, 
所以 ChatGPT Plus 也只能用当大排量的聊天机器人, 这方面比同价位的 Claude Pro 还不好, Claude Pro 勉强还能够稍稍推进一个编程项目.

地下室有张两年以前买来玩游戏的 RTX 4090, 一直躺在那儿, 被 ChatGPT 要求静默五小时候不得不又考虑激活那张 4090, 在其上运行一个本地开源模型, 
然后在别的机器上使用它. 订阅费是省了, 可是费电啊. 下面是关于 Ollama 及各编程 Agent 的配置, RTX 4090 运行在 Ubuntu Linux 操作系统中,
编程 Agent 是在 macOS 中运行的.

### 1. 工具及模型选择

用 AI 调研了一下, 模型推理服务首推 Ollama, 它本身是 llama.cpp 的包装, 另有适合高性能并发的推理服务有 [SGlang](https://www.sglang.io/)
和 [vLLM](https://vllm.ai/). 但 RTX 4090 也就那点能耐, 所以我们这里选用 Ollama.

开放权重的大语言模型选择上看来还是阿里的千问较为突出, 相对于 RTX 4090 的 24G 显存, 可选择模型有以下<!--more-->

1. qwen3-coder:30b(上下文 256K): 它是一个 MoE(混合专家模型), 总参数为 30B, 某些地方看到的模型名为 Qwen3-Coder-30B-A3B-Instruct, 
   A3B 中的 A 是 Active 的意思, 即每个 Token 只用其中的 3B 的参数, 这个模型有 128 个专家, 每个 Token 选择其中 8 个. 4-bit 量化版 19GB
2. qwen3.8:27B(上下文 256K): 一个通用多模态模型, 包含编程, 推理和视觉任务. 4-bit 量化版 18GB
3. devstral-Small-2:24B(上下文 384K): 法国公司 Mistral AI 推出的专门面向软件工程 Agent, 支持多文件修改的多模态模型, 4-bit 量化版 15GB. 
   它家另一款更小的模型 ministral-3:14b 也值得一试, 4-bit 量化 9GB
4. glm-4.7-flash(上下文 198K): 也是一个 30B-A3B 的 MoE 模型, 由最近来私传代码的公司智谱发布的, 4-bit 量化版 19GB
5. deepseek-coder:33b(上下文 16K): DeepSeek 的大语言模型, 4-bit 量化版 19GB

本文演示时将主要使用模型 `qwen3-coder:30b`

关于编程 Agent 的选择, 当我们无参数运行 `ollama` 时会提示可运行以下 Agent -- 当前 Ollama 版本为 0.30.6
 
Claude Code / Hermes Agent / OpenClaw / OpenCode / Hermess Desktop / Codex / Copilot CLI / OMP / Cline / Droid / Pi /
 Pool / Qwen Code

`Ollama` 向我们展示它有多丰富的 Agent 集成能力, 我们这里选几个知名的编程 Agent 来体验, 它们的代表是 Claude Code, Codex, Copilot CLI,
OpenCode Pi.

### 2. 运行 Ollama 服务

在 Linux 或 macOS 下, Ollama 的安装可用同一个命令

```bash
curl -fsSL https://ollama.com/install.sh | sh
```

`Ubuntu` 下还能用 `sudo snap install ollama` 安装.

安装完后, `ollama` 服务会自动启动. 可用 `sudo systemctl status ollama` 查看服务状态.

先测试一个小模型

```bash
ollama --verbose run qwen3.5:0.8b "what's the model name?"
```
这个模型只有 1G 大小, 第一次执行需要下载并加载模型参数, `--verbose` 最后能输出生成 Token 的统计信息. 比如这里的生成速度是 338.24 tokens/s.

`Ollama` 默认启动的服务监听在本地的 `127.0.0.1:11434` 端口

```bash
netstat -na | grep 11434
tcp        0      0 127.0.0.1:11434         0.0.0.0:*               LISTEN
```

所以该服务只能让本地的 Agent 使用, 要支持远程调用必须启动在某个外部接口(如 192.168.0.110), 或者简单配置为 0.0.0.0:11434. 这就要修改 `Ollama`
的服务配置.

在近两年前的一篇博文 [Ollama - 简化使用本地大语言模型](/ollama-simple-use-local-llm-model/) 也讲述过如何定制服务端口.

#### 2.1 前台运行 ollama serve 时监听端口

直接运行 `ollama serve` 会启动 `ollama` 服务在前台, 调试时能在 Agent 调用时看到详细的后台输出信息. 运行 `ollama serve --help`
会列出当前 `ollama` 版本支持的所有环境变量, 与上篇博文中对比又新增了少许的环境变量

```bash
ollama serve --help
Start Ollama

Usage:
  ollama serve [flags]

Aliases:
  serve, start

Flags:
  -h, --help   help for serve

Environment Variables:
      OLLAMA_DEBUG                  Show additional debug information (e.g. OLLAMA_DEBUG=1)
      OLLAMA_HOST                   IP Address for the ollama server (default 127.0.0.1:11434)
      OLLAMA_CONTEXT_LENGTH         Context length to use unless otherwise specified (default: 4k/32k/256k based on VRAM)
      OLLAMA_KEEP_ALIVE             The duration that models stay loaded in memory (default "5m")
      OLLAMA_MAX_LOADED_MODELS      Maximum number of loaded models per GPU
      OLLAMA_MAX_TRANSFER_STREAMS   Maximum parallel transfer streams for safetensors model pulls/pushes (default 4)
      OLLAMA_MAX_QUEUE              Maximum number of queued requests
      OLLAMA_MODELS                 The path to the models directory
      OLLAMA_NUM_PARALLEL           Maximum number of parallel requests
      OLLAMA_NO_CLOUD               Disable Ollama cloud features (remote inference and web search)
      OLLAMA_NOPRUNE                Do not prune model blobs on startup
      OLLAMA_ORIGINS                A comma separated list of allowed origins
      OLLAMA_SCHED_SPREAD           Always schedule model across all GPUs
      OLLAMA_FLASH_ATTENTION        Enabled flash attention
      OLLAMA_KV_CACHE_TYPE          Quantization type for the K/V cache (default: f16)
      OLLAMA_LLM_LIBRARY            Set LLM library to bypass autodetection
      OLLAMA_GPU_OVERHEAD           Reserve a portion of VRAM per GPU (bytes)
      OLLAMA_IGPU_ENABLE            Enable integrated GPUs
      LLAMA_ARG_FIT                 Enable llama.cpp automatic fit of unset memory options (default "on")
      LLAMA_ARG_FIT_TARGET          Target free VRAM margin per device for llama.cpp fit (MiB)
      OLLAMA_LOAD_TIMEOUT           How long to allow model loads to stall before giving up (default "5m")
```

指定监听的端口可使用 `OLLAMA_HOST` 环境变量, 如服务启动在前台并监听在 0.0.0.0:11434, 用如下命令

```bash
OLLAMA_HOST=0.0.0.0:11434 ollama serve
```

控制台中可以看到 ollama 服务所监听的端口

> time=2026-09-22T22:05:30.926-05:00 level=INFO source=routes.go:2031 msg="Listening on [::]:11434 (version 0.34.3)"

为使用 `ollama serve` 前台服务, 我们先用 `sudo systemctl stop ollama` 停掉后台的 `ollama` 服务, 否则会有端口冲突造成无法启动.

在另一个终端中用 `netstat` 命令查看

```bash
netstat -na|grep 11434
tcp6       0      0 :::11434                :::*                    LISTEN
```

现在可以在远端调用该服务了. Ollama 的服务兼容 OpenAI 和 Anthropic 客户端, 它的 API 请参考 [Ollama API Reference](https://docs.ollama.com/api/introduction).
在远端机器上我们测试一下, 假设 Ollama 服务的 IP 地址是 192.168.86.245

我们可以在另一机器下指定 `OLLAMA_HOST` 来使用 `ollama` 命令. 

```bash
OLLAMA_HOST=http://192.168.86.245:11434 ollama list
NAME    ID    SIZE    MODIFIED
```

看不到任何的模型, 那是因为直接 `ollam serve` 和 `sudo systemctl start ollama` 启用的服务的用户名不一样的, 所以它们不共享已下载的模型缓存, 
前者是当前某个用户, 后者是 `ollama` 用户, 如果执行 `ollama serve` 前切换到了 `ollama` 用户就会共享相同的模型缓存.

在远程用了 `OLLAMA_HOST` 之后可以像操作当前机器上的 `ollama` 一样, 如下载一个模型

```bash
OLLAMA_HOST=http://192.168.86.245:11434 ollama pull qwen3.5:0.8b
```

测试一下 Ollama API

```
curl http://192.168.86.245:11434/api/chat \
-d '{
  "model": "qwen3.5:0.8b",
  "messages": [{"role": "user", "content": "what is the model name?"}],
  "stream": false
}'
```

这样就能正常返回模型的回答.

`ollama serve` 方便调试, 为共享系统服务 `ollama` 的模型, 切换到 `ollama` 用户并启动 `ollama serve` 的完整命令如下

```bash
sudo -u ollama OLLAMA_HOST=0.0.0.0:11434 ollama serve
# 或者 -H env OLLAMA_HOST=0.0.0.0:11434
```

#### 2.1. 后台 ollama 服务指定监听端口

前面用 `ollama serve --help` 可看到 `Ollama` 所有支持的环境变量, 我们也可以把它们配置给 `ollama` 系统服务中, 有两种做法

##### 2.1.1. 直接编辑 /etc/systemd/system/ollama.service

```bash
sudo vi /etc/systemd/system/ollama.service
```

在 [Service] 下加上

```properties
Environment="OLLAMA_HOST=0.0.0.0"
```

[Service] 下可有多个 Environment, 它们会合并起来, 完后执行

```bash
sudo systemctl daemon-reload
sudo systemctl restart ollama
```

这样系统服务 `ollama` 就监听在了 `0.0.0.0:11434`, 如果修改 `/etc/systemd/system/ollama.service` 后直接运行 `sudo systemctl restart ollama`
的话就会有警告

> Warning: The unit file, source configuration file or drop-ins of ollama.service changed on disk. Run 'systemctl daemon-reload' to reload units.

也就是手动修改的 `/etc/systemd/system/ollama.service` 并未真正起效.

##### 2.1.2. 用 systemctl edit 来编辑 Ollama 服务参数

```bash
sudo EDITOR=vim systemctl edit ollama
```

上面命令打开后与 `sudo vi /etc/systemd/system/ollama.service` 显示的内容不同, 它会把默认的参数都注释掉了, 我们必须在提示的

```text
### Edits below this comment will be discarded
```

前面加上如下内容

```toml
[Service]
Environment="OLLAMA_HOST=0.0.0.0"
```

注: `EDITOR=vim` 指定用 `vim` 作为编辑器, 否则有可能使用 `nano` 编辑器, 我还不会用 `nano`.

保存后会看到控制台的输出

> Successfully installed edited file '/etc/systemd/system/ollama.service.d/override.conf'.

也就是会自动生效, 这时候就可以跳过 `sudo systemctl daemon-reload` 这一步, 直接重启 `ollama` 服务即可.

```bash
sudo systemctl restart ollama
```

最后我们发现用 `sudo EDITOR=vim systemctl edit ollama` 编辑的内容在 `/etc/systemd/system/ollama.service` 中看不到. 
用哪一种编辑方式就看个人喜好了, 本人偏爱于前者, 因为它可以看到注释的默认配置与定制的配置, 而且不需要额外的 daemon-reload 操作.

另外, 其他的环境变量也可以这么添加, 比如以下几个关键环境变量

1. OLLAMA_NUM_PARALLEL: 同时处理的请求数, 默认为 1, 有子 Agent 并行时可加大, 但同时要考虑显存大小
2. OLLAMA_KEEP_ALIVE: 请求束后, 模型继续驻留的时间, 默认为 5m
3. OLLMA_CONTENT_LENGTH: 根据 VARM 大小会选用默认的 4k/32k/256K. 24-48GB 显存大小时对应的是 32K 上下文

### 3. 配置编程 Agents

现在有了 http://192.168.86.245:11434 上的 Ollama 服务后, 继续把 `qwen3-coder:30b` 模型在服务器上拉下来

```bash
OLLAMA_HOST=http://192.168.86.245:11434 ollama pull qwen3-coder:30b
```

为便于以后远程管理 `Ollama` 服务, 可以客户端配置环境变量

```bash
export OLLAMA_HOST=http://192.168.86.245:11434
```

这和配置 `DOCKER_HOST` 使用 `docker` 命令类似.

接下来开始挨个配置一下流行的编程 Agent

#### 3.1. 配置 Claude Code 使用远程 Ollama

##### 3.1.1. Claude Code 的安装

```bash
curl -fsSL https://claude.ai/install.sh | bash
```

##### 3.1.2. 环境变量方式

```bash
export ANTHROPIC_BASE_URL="http://192.168.86.245:11434"
export ANTHROPIC_AUTH_TOKEN="ollama"
export ANTHROPIC_MODEL="qwen3-coder:30b"
export CLAUDE_CODE_MAX_CONTEXT_TOKENS=32768
```

`ANTHROPIC_API_KEY` 可省略, `CLAUDE_CODE_MAX_CONTEXT_TOKENS` 也是可选的, 最好与 `OLLAMA_HOST=http://192.168.86.245:11434 ollama ps`
命令看到的相应模型的 `CONTEXT` 一致.

```bash
OLLAMA_HOST=http://192.168.86.245:11434 ollama ps
NAME               ID              SIZE     PROCESSOR    CONTEXT    UNTIL
qwen3-coder:30b    06c1097efce0    21 GB    100% GPU     32768      4 minutes from now
```

用 `claude` 启动后不需要 `/login` 就能快乐的 Vibe Coding 了, 启动 `claude` 时不想使用环境变量 `ANTHROPIC_MODEL` 指定模型的话, 
可在命令中传入

```bash
claude --model gemma4:26b
```

不过在 `claude` 中不能用 `/model` 切换模型. `qwen3-coder:30b` 不是多模态的, 所以给它图片的话无法处理. 要处理图片可用 `qwen3.8:27B`
这个多模态模型, 在 RTX 4090 下上下文默认是 32k, 我把上下文窗口调到 64k 来试下.

要在 Ollama 服务端加上环境变量

```bash
sudo EDITOR=vim systemctl edit ollama
```

在 `[Service]` 下再加一个环境变量

```toml
Environment="OLLAMA_CONTEXT_LENGTH=65536"
```

再用命令 `sudo systemctl restart ollama` 重启 Ollama 服务

再回到 macOS 机器, 远端下命令让 Ollama 服务器拉取 `qwen3.8:27b` 的模型

```bash
OLLAMA_HOST=http://192.168.86.245:11434 ollama pull qwen3.8:27b
```

使用 `Claude Code` 前也要用环境变量设置与启动模型时一样大小的上下文参数

```bash
export CLAUDE_CODE_MAX_CONTEXT_TOKENS=65536
claude --model qwen3.8:27b 
```

{{< bundle-image claude-code-image.png 900 >}}

上图就是在 Claude Code 中使用 `qwen3.8:27b` 这一多模态模型的情景, 截了一张该容器左上角带 `Claude` 及三行文字的图片, 让模型去识别. 
表现还是不错的, 对于要进行界面调整, 或错误信息截图的问答可以使用该模型来处理.

加载模型后我们回过头来验证一下上下文窗口是否是 64k

```bash
OLLAMA_HOST=http://192.168.86.245:11434 ollama ps
NAME           ID              SIZE     PROCESSOR    CONTEXT    UNTIL
qwen3.8:27b    22130167c4c2    17 GB    100% GPU     65536      4 minutes from now
```

环境变量可以移入到相应 Shell 的配置文件中, 如 Bash 的 `~/.bashrc`, 或 Zsh 的 `~/.zshrc` 等.

##### 3.1.2. 配置文件方式

配置文件是 `~/.claude/settings.json`, 关于模型的配置内容如下

```json
{
  "model": "qwen3-coder:30b",
  "env": {
    "ANTHROPIC_BASE_URL": "http://192.168.86.245:11434",
    "ANTHROPIC_AUTH_TOKEN": "ollama",
    "CLAUDE_CODE_MAX_CONTEXT_TOKENS": "32768"
  }
}
```

也能在 `env` 中用 `ANTHROPIC_MODEL` 属性配置默认的模型名. 这样就能连接 `http://192.168.86.245:11434` 使用 Claude Code 了.

#### 3.2 配置 Codex 远程使用 Ollama

##### 3.2.1. Codex ClI 的安装

```bash
curl -fsSL https://chatgpt.com/codex/install.sh | sh
```

##### 3.2.2. 环境变量方式

```bash
export CODEX_OSS_BASE_URL="http://192.168.86.245:11434/v1"
codex --oss --local-provider ollama --model qwen3-coder:30b
```

启动后看到

```
╭───────────────────────────────────────────────────╮
│ >_ OpenAI Codex (v0.156.1)                        │
│                                                   │
│ model:     qwen3-coder:30b low   /model to change │
│ directory: ~/Desktop/demo                         │
╰───────────────────────────────────────────────────╯
```

仅仅用环境变量 `CODEX_OSS_BASE_URL` 启动 codex 有些麻烦

如果没有指定模型, 仅用命令

```bash
codex --oss --local-provider ollama
```

则会要求 `http://192.168.86.245:11434` 上的 `ollama` 服务下载模型 `gpt-oss:20b` 来使用. 该模型大小 13 GB, 也是一个不错的编程用模型.
未指定 `--local-provider` 会弹出 provider 的选择, 如 Ollama 或 LM Studio. 但必须指定  `--oss` 参数.

默认的推理深度是 `low`, 要设置 `high` 的推理深度的完整启动命令为

```bash
codex --oss --local-provider ollama --model qwen3-coder:30b -c 'model_reasoning_effort = "high"'
```

`-c` 为 `config`, 更多如何启动 `codex` 的参数请用 `codex --help` 查看.

使用环境变量的方式来使用远端的 Ollama, 还没法简单用无参数的 `codex` 命令来启动, 看来还得用配置文件的方式.

##### 3.2.3. 配置文件方式

`Codex` 的配置文件是 `~/.codex/config.toml`, 通过它可以进行丰富的参数配置. 下面是一个简单的配置文件示例

```toml
model_reasoning_effort = "medium"

model = "qwen3-coder:30b"
model_provider = "ollama_remote"
model_context_window = 32768

[model_providers.ollama_remote]
name = "Remote Ollama"
base_url = "http://192.168.86.245:11434/v1"
wire_api = "responses"
requires_openai_auth = false
```

现在只需要简单执行 `codex` 就能使用远端的 `Ollama` 服务

可以顺便测试它使用工具的能力

```
> create file test.txt under current folder with content 'hello world'
```

它就会运行 `echo 'hello world` > test.txt` 在当前目录中创建一个文件.

Codex 最主要的工具是 `exec_command`.

#### 3.3. 配置 GitHub Copilot 远程使用 Ollama

每次说到 Copilot 都要刻意与 Microsoft Copilot 区分开来, 必须说成是 GitHub Copilot, 微软这软件的命名真缺德, 又想起它的云服务名叫做
`Azure`, 简值就是一千个对这个单词有千个发音.

##### 3.3.1. Copilot CLI 的安装

```bash
curl -fsSL https://gh.io/copilot-install | bash
```

装好后的命令是 `copilot`, 这时候的 `copilot` 命令默认是 `GitHub Copilot` 的 CLI 了.

##### 3.3.2. 环境变量方式

设定环境变量

```bash
export COPILOT_PROVIDER_BASE_URL=http://192.168.86.245:11434/v1
export COPILOT_MODEL=qwen3-coder:30b

# 以下两个可选
export COPILOT_PROVIDER_MAX_PROMPT_TOKENS=24576
export COPILOT_PROVIDER_MAX_OUTPUT_TOKENS=8192
```

这是以 OpenAi 兼容的 API `/v1/completions` 来访问 Ollama 服务的, 后两个是可选的, 没有的话启动时会有相关的警告, 所以最好配置两个对应于
Ollama 服务端模型的参数值. 比如这里 Ollama 服务端模型最大上下文是 32768 的话, 分配到输入和输出分别为 24576 和 8192. 如果模型最大上下文窗口为
64k 时, 分别分配 57344, 8192.

直接命令 `copilot` 启动便会使用远端的 Ollama 服务.

尝试过用 `~/.copilot/providers.json` 和  `~/.copilot/settings.json` 两个配置文件的方式, 不可行. Copilot 使用 Ollama 还必须使用环境变量.
只能把前面的 `export` 命令写到相应 shell 的配置文件中去, 如 bash 的  `~/.bashrc`, 或 zsh 的  `~/.zshrc` 文件中.

就是有时候要注意 `~/.copilot/settings.json` 文件中的 `model` 配置别干扰了环境变量中的 `COPILOT_MODEL`. 此外 Copilot 也能用 `--model`
参数覆盖模型名.

#### 3.4. 配置 OpenCode 远程使用 Ollama

开源界有两个不错的编程 Agent, 那就是 OpenCode 和 Pi, Google 不光是大语言模型落后, 连他们的 Gemini CLI 也没人提了.

##### 3.4.1 OpenCode 的安装

```bash
curl -fsSL https://opencode.ai/v2/install | bash
```

OpenCode 还提供了桌面应用程序. OpenCode 的定制性也很强,它主要通过配置方式来定制, 与下面将要学到的 Pi 通过扩展的方式定制不同. 除了 CLI
和桌面板, `OpenCode` 还有 Web 版, 通过 `opencode serve` 启动, 控制台会输出 URL 和密码, 用户名是 `opencode`.

##### 3.4.2 环境变量方式

OpenCode V2 相比于 V1 有所不同, 这个链接是 V1 的环境变量 [Environment variables](https://opencode.ai/docs/cli/#environment-variables) 参考. 
在 V1 支持许多的环境变量,它主要可以通过 `OPENCODE_CONFIG` 指定配置文件或用 `OPENCODE_CONFIG_CONTENT` 直接配置 provider. 我们跳过
OpenCode V1 的配置, 过时了就不用学, 直接学习 V2 的配置.

查阅 OpenCode 官方在线文档时注意 V1 是白色背景, V2 是黑色背景.

OpenCode V2 只支持用配置文件的方式使用远端的 Ollama.

##### 3.4.3. 配置文件方式

OpenCode 支持两种配置文件名, opencode.json 和 opencode.jsonc, jsonc 为可带注释的 json 文件. 它可以放置的位置是

1. ~/.config/opencode/opencode.json(c)
2. <projects>/my-app/opencode.json(c)
3. <projects>/my-app/.opencode/opencode.json(c)
4. <projects>/my-app/packages/web/opencode.json(c)

启动 `OpenCode` 时它会加载所有搜索路径上的 `opencode.json(c)` 文件, 有重复配置时由近端覆盖无端的配置. 如果同一位置上有 `opencode.json`
和 `opencode.jsonc`, 那谁的优先级更高的, `OpenCode` 偏爱于 `jsonc` 文件, 不过还是拜托别把生活搞的太复杂了.

更准确的知道 `OpenCode` 使用了哪些配置文件或谁覆盖谁, 执行命令

```bash
opencode debug config
```

它们列出所有搜索路径上找到的 `opencode.json(c)` 文件, 并且后面的覆盖前面的.

现在来进行 `opencode.json(c)` 的配置, 也跟个时髦, 采用 `opencode.jsonc` 文件, 这里就把它放在用户目录中

`~/.config/opencode/opencode.jsonc`

内容为

```json
{
  "$schema": "https://opencode.ai/config.json",
  "model": "ollama/qwen3-coder:30b", // use provider name as prefix
  "providers": {
    "ollama": {
      "settings": {
        "baseURL": "http://192.168.86.245:11434/v1"
      }
    }
  }
}
```

`jsonc` 的注释方式与 Java 是一样的, `/* */` 多行注释, `//` 单行注释. 如何配置可查看 `$schema` 指定的链接. `model` 是指定启动 `OpenCode`
时默认的模型, 它为可选的, 因为 `OpenCode` 会通过 API `/api/tags` 和 `/api/show` 列出该 `Provider` 中的模型列表, 因此 `OpenCode` 使用
Ollama 时还支持 `/models` 命令.

先用 `opencode debug config` 查看一下

```bash
 opencode debug config
[
  {
    "type": "document",
    "path": "/Users/yanbin/.config/opencode/opencode.jsonc",
    "info": {
      "$schema": "https://opencode.ai/config.json",
      "model": {
        "providerID": "ollama",
        "model": "qwen3-coder:30b"
      },
      "providers": {
        "ollama": {
          "settings": {
            "baseURL": "http://192.168.86.245:11434/v1"
          }
        }
      }
    }
  },
  {
    "type": "directory",
    "path": "/Users/yanbin/.config/opencode"
  },
  {
    "type": "directory",
    "path": "/Users/yanbin/.opencode"
  }
]
```

读取到了相应的配置文件, 并把 `opencode.jsonc` 中的注释自动去除掉了.

现在启用 `OpenCode`

```bash
opencode
```

{{< bundle-image opencode-start.png 600 >}}

使用了配置的模型

当用 `/models` 选择模型时, 

{{< bundle-image opencode-models.png 878 >}}

`OpenCode` 在 TUI 与终端的集成也很出色, 不用 [Herdr](https://herdr.dev/) 这个辅助工具在终端显示的就非常友好.

#### 3.5. 配置 Pi 远程使用 Ollama

终于轮到最后一个编程 Agent 了 - Pi, 它也是一款完全开源的编程 Agent, 没有像其他编程工具那样绑定很多插件, 它的核心极小, 
甚至连多数人认为理所当然的功能都没有内置, 所以系统提示词也很精练, 要功能扩展的话就像 DeepSeek Harness
宣传的那样: 一切皆插件. Pi 中叫做 Package, 可在此搜索 [Package Catalog](https://pi.dev/packages). Pi 就像一个基本版的 Vim
或 Visual Studio Code, 趁不趁手, 全要靠自己打磨. 最终每个人都有自己独特的 Pi, 尤其是 Pi 的定制性极强, TUI 都能通过扩展任意修改.

##### 3.5.1. Pi 安装

```bash
curl -fsSL https://pi.dev/install.sh | sh
```

当前 Pi 的版本为 0.87.1, 它需要 Node.js 最小版本为 22.19.0. 上面安装命没找到符合要求的 Node.js 版本, 它会帮你安装 Node.js 22.19.0,
但安装完也可能会提示

> error: Pi requires Node.js 22.19.0 or newer. Found v22.17.1.

从而没能继续完成 `Pi` 本身的安装, 原因可能是你的 Node.js 是由 `nvm` 管理, 因此需要手动用 `Node.js` 安装并切换当前 Node.js 版本为最新的.

```bash
nvm install 22.19.0
nvm use 22.19.0
```

再重新运行前面 `Pi` 的安装命令才能成功.

##### 3.5.2. 配置文件方式

Pi 与 OpenCode 一样, 也不支持环境变量的方式来连接 Ollama 服务, 它只用文件配置, 想要快乐的只用 `pi` 命令启动的话需要两个配置文件, 它们分别是

`~/.pi/agent/models.json`

```json
{
  "providers": {
    "ollama": {
      "baseUrl": "http://192.168.86.245:11434/v1",
      "api": "openai-completions",
      "apiKey": "ollama",
      "models": [
        {
          "id": "qwen3-coder:30b",
          "name": "Qwen3 Coder 30B",
          "reasoning": false,
          "input": ["text"],
          "contextWindow": 32768,
          "maxTokens": 8192
        },
        {
          "id": "qwen3.8:27b",
          "name": "Qwen3.8 27B - VLM",
          "reasoning": true,
          "input": ["text", "image"],
          "contextWindow": 32768,
          "maxTokens": 8192
        }
      ]
    }
  }
}
```

这里配置了多个模型

`~/.pi/agent/settings.json`

```json
{
  "defaultProvider": "ollama",
  "defaultModel": "qwen3-coder:30b",
  "defaultThinkingLevel": "off"
}
```

启动 Pi

```bash
pi
```

进到 `Pi` 之后可以用 `/model` 在所配置的两个模型间选择

{{< bundle-image pi-ollama.png 871 >}}

没有 `~/.pi/agent/settings.json` 文件的话, 启动 `pi` 时需要加额外参数

```bash
pi --provider ollama --model qwen3-coder:30b
```

### 4. 简单小结

1. `Ollama` 服务默认监听在 `127.0.0.1:11434`, 需用 `sudo EDITOR=vim systemctl edit ollama` 加上 `OLLMA_HOST=0.0.0.0:11434`
    使其监听在外部接口上
2. `Claude Code` 和 `Codex` 都可以独立的使用环境变量或配置文件的方式, 让它们连接远端的 `Ollama`
3. `Copilot CLI` 可单独用环境变量配置, 用配置文件可以辅助设置, 但终也离开环境变量 `COPILOT_PROVIDER_BASE_URL`
4. `OpenCode`(V2开始) 和 `Pi` 索性就取消了环境变量配置的方式使用远端的 `Ollama`, 只能用配置文件
5. 各大编辑 Agent 还都在进化当中, 给给使用的带来一点选择上的焦虑, 不像 Java 的构建就是 Maven, IDE 最强是 `IntelliJ IDEA`, Rust
   的构建工具非 `Cargo` 莫属那么简单.