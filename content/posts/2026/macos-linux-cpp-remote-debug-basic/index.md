---
title: "MacOS/Linux C++ GDB 远程调试基础"
url: /macos-linux-cpp-remote-debug-basic/
date: 2026-01-03T22:05:00-05:00
featured: false
draft: false
type: post
toc: false
# menu: main
usePageBundles: true
thumbnail: "/images/logos/cpp-logo.png"
categories:
  - cpp
tags: 
  - C++
  - MacOS
  - Linux
comment: true
codeMaxLines: 50
# additional
lastmod: 
---

有那么一个古老的项目，编译用的是 g++, 目标平台的是 Linux x86_64， 构建工具是 make. 本地开发环境是 macOS, 硬件为 Arm64 的苹果芯片 M4，
IDE 尝试用 CLion, 如果喜欢的话，也可以选择 VSCode.

想要单步调试的目的就是想让代码跑起来，然后逐步理解代码执行的逻辑。所以不想对现有项目构建过程进行改造，如换成 CMake, 使用 LLVM 编译之类的。
只想使用远程 Linux 机器, 仍然使用原来的 `make` 命令编译，然后在 Linux 下启动程序，由 Clion 连到的 Linux 上进程， 关联源代码进行调试。

因为编译用的 g++, 所以采用 gdbserver 和 gdb 进行远程调用，llvm 相应的解决方案是 lldb-server + lldb. 下面是关键的步骤

. 本地 Clion 中编辑代码
. 代码传到远程 Linux 机器
. 远程 Linux 机器上编译，编译的二进制代码要保留符号信息
. 在 Linux 机器上的符号文件要下载给 Clion
. 在远程用 gdbserver 指定端口启动程序
. 本地 Clion 配置用 GDB 连接到远程 Linux 机器上的 gdbserver
. 在 Clion 中断点单步调试

上面是基本的操作，当我们用某个客户端工具(比配置运行 Clion 的 Remote GDB Server) 就自动完成以上的某些操作，如自动双向同步源代码与二进制/符号文件,
自动驱动远程端的的编译，启动 gdbserver. 远程 Linux 还能进一步使用 Docker 容器替代。但是在苹果芯片的 macOS 中启动 `x86_64` 的容器中运行的
`gdbserver` 在 macOS 中用 gdb 是连不上的。 <!--more-->

### Clion 远程调试 Linux C++ 代码最简示例

#### 准备远程 Linux 机器

以一个 AWS EC2 为例，选择的 AMI 是 Amazon Linux 2023, 我们需要在其上安装以下软件

```shell
yum install gcc-c++ gdb make gdb-gdbserver
```

#### 创建本地项目

我们可以用 CLion 创建一个 C++ 项目，不过它们把 CMake 的内容带进来，我们不使用 CMake 的任何功能。或者直接创建一个目录, 如
`~/ClionProjects/linux-remote`, 然后在其中新建文件 `main.cpp`, 内容为

```cpp
#include <iostream>

int main()
{
    auto lang = "C++";
    std::cout << "Hello and welcome to " << lang << "!\n";

    return 0;
}
```

#### 上传代码到远程 Linux 机器

通过某种方式把代码上传到远程 Linux 机器，如 scp, rsync。 比如上传后的文件在 `/work/main.cpp`

#### 在远程机器上编译

SSH 登陆上远程 Linux 机器，进到 `main.cpp` 所在的目录 `/work`, 执行

```shell
g++ -g -o0 main.cpp -o main
````

`-g` 是在进二进制文件中包含调试信息，如执行指令所对应源代码中的行号等。`-g` 是最小调试信息，用 `g3` 会包含最详细的调试信息(包含宠定义).

`-o0` 是明确不进行优化，易于调试跟踪。默认就是 `-o0`, 不同的优化级别是 `-o1`, `-o2`,`-o3`, `-Os`, `-Ofast`, 从左至右优化越激进。
有时候还可加上 `fomit-frame-pointer` 来保留帧指针

这样我们生成的二进制文件 `main` 中带有符号信息. 如果要从 `main` 中提取出调试用的符号信息，可执行命令

```shell
objcopy --only-keep-debug main main.debug
```

#### 下载符号信息文件

如果二进制文件中包含符号信息，可直接下载二进制文件本身，如这里的 `main`, 如果有单独的符号文件，如从 `main` 中分离出来的 `main.debug`, 
只要下载它就行

*如果不下载符号文件, Clion 也能在调试时通过远程的 gdbserver 取得符号信息, 但这样每次调试都会慢一些。*

#### 远程 Linux 上启动 gdbserver

首先确定一个可用的不被防火墙拦截的端口，这里用端口号 `6379`, 然后执行 `gdbserver` 命令

```shell
gdbserver :6379 ./main 
```

如果 `main` 需要命令行参数，可在 `./main` 后面添加, 即 `gdbserver :6379 ./main arg1 arg2`. `:6379` 是在 `0.0.0.0` 上监听，
假如要指定网络接口，就用 `10.255.61.50:6379` 的格式。

`gdbserver` 成功启动后显示

```shell
gdbserver :6379 ./main
Process ./main created; pid = 339488
Listening on port 6379
```

在 `6370` 端口号上等待 `gdb` 客户端的连接。 其实先启动 `gdbserver` 和 `gdb` 客户端的先后顺序是无所谓的。

#### 配置 Clion `Remote Debug`

{{< bundle-image clion-remote-debug-config.png 857 >}}

这里选择 `Bundled GDB` 即 Clion 自带的 GDB, 还可用系统中自己安装的 GDB 客户端. 另有 `LLDB` 的相关选项。 

- `'target remote' args`: 就是远程 Linux 机器的连接信息。
- `Symbol file`: 因为二进制文件 `main` 中包含有调试信息，所以可选择 `main`, 或者可选择从 `main` 用 `objcopy` 分离出来的 `main.debug`。 
- `Sysroot`: 我填入的是本地项目的路径

有时需要配置 `Path mappings` 配置在本地与远程 Linux 机器的路径映射, 如远程的 `/work` 与本地的 `~/ClionProjects/linux-remote` 映射。

*如果只是指定 `target remote' args` 为 `tcp:10.255.61.50:6379`, 而不指定 `Symbol file`， `Sysroot`, 也是可以的, 但每次调试时
Clion 都要从远程的 gdbserver 取得符号信息, 会慢一些。*

#### 启动 Clion 调试

在源代码中打上断点，然后点击 `Debug 'Linux Remote'`, 成功连接上远程的 `gdbserver` 后，在远程的控制台可以看到输出

> Remote debugging from host 172.28.24.48, port 52733

执行就会停留在所设置的断点处， 这就可以 Clion 中进行快乐的单步调试了。

{{< bundle-image clion-remote-debug-run.png 900 >}}

像调试本地代码一样的，在 `GBD` 标签中显示的是 `(gdb)` 控制台。


远程调试基本的原理就是这样了， 由此延伸就是可以使用一个 Docker 容器卷自动映射，不用代码或符号文件上下手动传， Clion 的 `Remote GDB Server`
就实现了这一功能，通过配置的 `SSH/SCP` 同步文件. 或者使用 Clion 的 `Settings/Build, Execution, Deployment/Toolchains` 下的
`Remote Host` 或 `Docker` 的选项自动同步文件和编译。

### 本地 Docker container 作为远程主机的尝试

试着把真正远程的 Linux 机器换成一个本地的 Docker 容器试下. 首先本机需用 macOS 的 Apple 芯片(Arm64), 但配置了环境变量
`DOCKER_DEFAULT_PLATFORM=linux/amd64`, 所以构建，执行都是用的 `x86_64` 的架构，相当于执行 `docker` 命令时带了
`--platform linux/amd64` 参数。

先构建一个含有 g++ 编译器和 gdbserver 的镜像，Dockerfile 为

```docker
FROM ubuntu:24.04

WORKDIR /work

RUN apt update -y && apt install -y gcc g++ make gdb gdbserver && \
    rm -rf /var/lib/apt/lists/*
```

没有直接基于官方 `gcc` 的基础镜像构建。

构建好镜像后，来到项目所在目录 `～/ClionProjects/linux-remote`, 然后启动容器, 并编译

```shell
docker run -it -v ./:/work -p 6379:6739 gcc:custom bash
g++ -g -o0 main.cpp -o main
gdbserver :6379 ./main
```

未能启动 `gdbserver`, 而是直接执行了 `./main`, 在容器中输出的信息是

```text
gdbserver: linux_ptrace_test_ret_to_nx: Cannot PTRACE_GETREGS: Input/output error
gdbserver: linux_ptrace_test_ret_to_nx: PC 0x555500000000 is neither near return address 0x7fffff7c4000 nor is the return instruction 0x55555559ea71!
gdbserver: Error disabling address space randomization: Operation not permitted
Couldn't write debug register: Input/output error.
Exiting
root@a8cabcdb6940:/work# Hello and welcome to C++!
```

在容器中执行  `uname -m` 的输出是 `x86_64`. 

ChatGPT 后再尝试启动容器的命令

```shell
docker run -it -v ./:/work -p 6379:6379 --cap-add=SYS_PTRACE --security-opt seccomp=unconfined gcc:custom bash
```

此时执行 `gdbserver :6379 ./main` 仍有错误

```text
root@4f09dbe80075:/work# gdbserver :6379 ./main
gdbserver: linux_ptrace_test_ret_to_nx: Cannot PTRACE_GETREGS: Input/output error
gdbserver: linux_ptrace_test_ret_to_nx: PC 0x555500000000 is neither near return address 0x7fffff7c4000 nor is the return instruction 0x55555559ea71!
Couldn't write debug register: Input/output error.
Exiting
root@4f09dbe80075:/work# Hello and welcome to C++!
```

即使采用高级授权

```shell
docker run -it -v ./:/work -p 6379:6379 --privileged gcc:custom bash
```

在其中执行 `gdbserver :6379 ./main` 还是一样的问题。

最后的定论就是由于使用 Arm64 的 macOS 运行 x86_64 的容器是用 `QEMU` 或 `Rosetta` 模拟出来的，模拟的还没到位，无法使用 `PTRACE`。
除非在 macOS 中启动 `aarch64` 架构的容器，这是硬件原生支持的。

比如如下命令重新构建镜像

```shell
docker build -t gcc:arm64 --platform linux/arm64 .
```

启动用命令

```shell
docker run -it --privileged --platform linux/arm64 -v ./:/work -p 6379:6379 gcc:arm64 bash
```

然后在容器中运行

```shell
root@8dd5f72b2b3e:/work# uname -m
aarch64
root@8dd5f72b2b3e:/work# g++ -g -o0 main.cpp -o main
root@8dd5f72b2b3e:/work# gdbserver :6379 ./main
Process ./main created; pid = 17
Listening on port 6379
```

这就可以调试了，所以在 Apple 芯片的 macOS 只能用 Docker 容器调试 Arm64 的 C/C++ 程序。还注意到在 Arm64 的容器中编译会自动产生符号文件,
如 `main.dSYM`.

#### 配置 DOCKER_HOST 使用远端 Docker 容器

如果有一个 Docker 主机， 我们可以配置环境变量，如

```shell
DOCKER_HOST=tcp://10.255.61.50:2375
```

后续执行的所有 `docker` 命令都会作用在该远程 Docker 主机上。但构建好镜像后，执行

```shell
docker run -it -v /work:/work -p 6379:6379 --privileged gcc:custom bash
```

卷和端口映射都是基本 DOCKER_HOST, 所以上面的映射是

> 10.255.61.50:/work:<containter>/work
> 10.255.61.50:6379:<containter>:6379

我们需要自己实现的是要把本地的文件与 `10.255.61.50`  之间同步，如用 `rsync` 命令(前提是要 10.255.61.50 开启了 SSH), 或用 Clion 的
`Build, Execution, Deployment / Deployment/ SFTP` 在本地与远端间自动同步，或都更笨拙的 `docker cp` 命令。

不过这种文件的同步操作都可以通过配置 Clion 来完成。

下篇预告：在使用了自定义静态库和动态库的 C++ 程序应如何远程单步调试。

链接

1. [使用Clion远程自动同步和远程调试c++](https://juejin.cn/post/7185063838016864317)
2. [CLion 实现远程调试](https://leehao.me/CLion-%E5%AE%9E%E7%8E%B0%E8%BF%9C%E7%A8%8B%E8%B0%83%E8%AF%95/)