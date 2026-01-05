---
title: "MacOS/Linux C++ GDB 远程调试动态库和静态库"
url: /macos-linux-cpp-remote-debug-library/
date: 2026-01-04T17:03:41-05:00
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

前篇 [MacOS/Linux C++ GDB 远程调试基础](/macos-linux-cpp-remote-debug-basic/) 演示了如在 macOS 开发调试远程 Linux 下的 C++ 程序，
本篇将结合 [C 语言静态库与动态库的生成和使用](/c-static-dynamic-library/) 练习如何调试含动态库，静态库的 C++ 程序, 并了解如何指定符号文件。

### 示例源码

本文将用以下的目录结构, 分别验证主程序与动态库，静态库的源代码断点调试，以及把源文件放在不同目录中是否有特别之处。

```sh
├── dynamic        # 动态库
│   └── add.cpp
├── static         # 静态库
│   └── sub.cpp
└── main.cpp       # 主程序
```
<!--more-->

源码文件内容分别为

#### dynamic/add.cpp

```cpp
int add(int a, int b)
{
    return a + b;
}
```

#### static/sub.cpp

```cpp
int sub(int a, int b)
{
    return a - b;
}
```

#### main.cpp

```cpp
#include <stdio.h>

int add(int a, int b);
int sub(int a, int b);

int main()
{
    int m = add(5, 2);
    int n = sub(5, 2);

    printf("%1$d+%2$d=%3$d, %1$d-%2$d=%4$d\n", 5, 2, m, n);
    return 0;
}
```

如目录所示，`add.cpp` 和 `sub.cpp` 分别是动态库和静态库的实现，`main.cpp` 是主程序，它会调用动态库和静态库中的函数。

### 上传文件到 Linux 机器

这次用 `rsync` 在本与远程 Linux 机器间同步文件，要求在两端都要安装 `rsync`, 并用 SSH 证书的方式认证。假定在本地机器上有证书文件
`~/ssh.pem`, 上传本地目录所有程序文件的到远程机器的命令是

```shell
rsync -avz -e "ssh -i ~/ssh.pem" --exclude '.idea' ./ ec2-user@10.255.61.50:~/yanbin/work/
```

如果要从远程机器上同步文件到本用 `rsync` 反过来就行了。

或用 Clion 的 `Build, Execution, Deployment / Deployment/ SFTP` 在本地与远端间自动同步.

### 在远程 Linux 机器上编译

`SSH` 登陆到远程机器， 并切换到目录 `~/yanbin/work`, 执行下面的命令

```shell
g++ -g -fPIC -shared -o dynamic/libadd.so dynamic/add.cpp  # 生成动态库 libadd.so

g++ -g -c static/sub.cpp -o static/sub.o
ar rcs static/libsub.a static/sub.o                    # 生成静态库
rm static/sub.o

g++ -g main.cpp -Ldynamic -Lstatic -ladd -lsub -o main
```

注意，在 Linux 上编译时，`main.cpp` 放在最后面写成 `g++ -g -Ldynamic -Lstatic -ladd -lsub -o main main.cpp` 可能会出现错误
`undefined reference to 'sub(int, int)'`。

编译参数解释：

- `-g`: 生成带调试信息的动态库或执行文件， 静态库的符号会在最终的可执行文件中。不带 `-g` 的话调试时将无法定位到源代码
- `-Ldynamic -Lstatic`: 设置编译时加载动态库或静态库的路径
- `-ladd -lsub`: 链接动态库或静态库，以 `-ladd` 为例，按序试图链接 `libadd.so` 和  `libadd.a`. 因为只有 `dynamic/libadd.so` 和
  `static/libsub.a`, 所以 `-ladd` 和 `-lsub` 分别使用动态库 `libadd.so` 和静态库 `libsub.a`

编译后的内容目录是

```text
├── dynamic
│   ├── add.cpp
│   └── libadd.so
├── main
├── main.cpp
└── static
    ├── libsub.a
    └── sub.cpp
```

在 Linux 上测试执行

```shell
LD_LIBRARY_PATH=dynamic/ ./main
5+2=7, 5-2=3
```

动态库是运行时加载的，所以如果不指定 `LD_LIBRARY_PATH` 变量，则无法加载 `libadd.so`, 会提示类似下面的错误

```shell
./main
./main: error while loading shared libraries: libadd.so: cannot open shared object file: No such file or directory
````

### Linux 上启动 `gdbserver`

编译完，验证可执行后，在远程 Linux 机器上启动 `gdbserver`, 假定使用端口号 `6379`, 执行命令

```shell
export LD_LIBRARY_PATH=dynamic/
[ec2-user@lin-0aff3d32 work]$ gdbserver :6379 ./main
Process ./main created; pid = 1856414
Listening on port 6379
```

### 从 Linux 同步文件到本地 macOS

用 CLion 远程调试时需要用到编译后的符号文件，在 Linux 上动态库的符号信息在 `libadd.so` 中，静态库, 动态库和可执行文件的符号信息都在可执行文件
`main` 中,  如果是 `macOS` 下用 `g++ -g` 编译会产生单独的 `dSYM` 目录， 如 `dynamic/libadd.so.dSYM` 和 `main.dSYM` 目录。

我们用不着在 `Linux` 编译出来的所有中间对象文件，只需要下载含有符号信息的文件就行，比如这里的可执行文件 `main`， 即只需要做

```shell
scp -i ~/ssh.pem ec2-user@10.255.61.50:~/yanbin/work/main ./
```

或者是在 `Linux` 上编译后从 `main` 文件中把符号信息分离出来

```shell
objcopy --only-keep-debug main main.dSYM
```

那么只需把该 `main.dSYM` 目录下载到本地, 配置 CLion `Remote Debug` 时指定为 `Symbol file` 的文件即可。

为什么不需要静态库或是动态库的符号信息呢？因为 `Linux` 上的动态库和静态库的符号信息都包含在可执行文件中，我们在后面将会实际体验到，
所以只需要下载可执行文件或从中剥离出来的符号信息文件即可。

*如果不下载符号文件, Clion 也能在调试的时候通过远程的 gdbserver 取得符号信息, 但这样每次调试都会慢一些。*

### 在 macOS 上用 CLion 配置远程调试

在 CLion 中选择 `Remote Debug`, 采用与上一篇完全一样的配置

{{< bundle-image clion-remote-debug-config.png 857 >}}

这里只指定了符号文件为 `/Users/yanbin.qiu/CLionProjects/linux-remote/main`。如果有用 `objcopy` 从 `main` 中剥离出来的符号信息文件
`main.dSYM` 则指定 `main.dSYM` 文件也行。

*如果只是指定 `target remote' args` 为 `tcp:10.255.61.50:6379`, 而不指定 `Symbol file`， `Sysroot`, 也是可以的, 但每次调试时
Clion 都要从远程的 gdbserver 取得符号信息, 会慢一些。*

在 `add.cpp`, `sub.cpp` 和 `main.cpp` 中分别打上断点，开始调试。

既能在动态库的源代码中断点调试

{{< bundle-image clion-linux-remote-debug-dynamic.png 985 >}}

也能在静态库的源代码上断点高度

{{< bundle-image clion-linux-remote-debug-static.png 985 >}}

更别说调试主程序文件

{{< bundle-image clion-linux-remote-debug-main.png 985 >}}

这与当初的预期还要理想， 开始以为选择执行文件 `main` 作为符号文件只能调试主程序与静态库的源代码，无法定位到动态库的源代码，原来是多虑了。

### 划重点

- 无论是编译静态库，还是动态库都需要带上 `-g` 编译参数，如此符号信息才会最终进到可执行文件中
- 从 Linux 机器上中只需要下载包含符号信息的可执行文件或者从可执行文件中剥离出来的符号信息文件
- `Remote Debug` 的 `Symbol file` 指定了在 Linux 上编译的可执行文件或符号信息文件， 就能断点调试整个项目的代码
- 如果不下载带符号信息的文件到本地，Clion `Remote Debug` 时不指定 `Symbol file`, 调试时也会从远程的 gdbserver 取得符号信息，但会更慢

### 附：macOS 的编译执行过程

如果在 `macOS` 上用 `g++` 编译运行的命令如下

```shell
g++ -g -fPIC -shared -o dynamic/libadd.so dynamic/add.cpp   # 生成动态库 dynamic/libadd.so

g++ -g -c static/sub.cpp -o static/sub.o
ar rcs static/libsub.a static/sub.o  # 生成静态库 static/libsub.a

g++ -g -Ldynamic -Lstatic -lsub -ladd -o main main.cpp   # 编译生成可执行文件 main

./main  # 输出 5+2=7, 5-2=3
```

`./main` 会自动从 dynamic 目录中加载 libadd.so, 因为编译时指定了 -Ldynamic, 如果 libadd.so 不在 dynamic 目录中, 可用
`DYLB_LIBRARY_PATH` 指定搜索路径，例如

```shell
DYLB_LIBRARY_PATH=<path-to-libadd.so> ./main
```