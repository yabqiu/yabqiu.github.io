---
title: C++ 调用 C++ 动态库时问题诊断 
url: /cpp-shared-library-trouble-shooting/
date: 2026-02-22T02:25:26-06:00
featured: false
draft: false
type: post
toc: false
# menu: main
usePageBundles: true
thumbnail: "../images/logos/cpp-logo.png"
categories:
  - Java/JEE
tags: 
  - JNA
comment: true
codeMaxLines: 50
---

本文初衷是为了解决 Java 应用程序通过 JNA 调用 C++ 动态库时，C++ 代码运行崩溃导致整个 Java 应用程序崩溃而进行的研究。从一个 C++ 调用 C++
写的动态库起步，记录它在什么情况下产生 core dump 文件，如何分析 core dump 文件等过程。可惜篇幅无法控制，不足以再加入 Java->JNA->C++
动态库内容了，所以不得不单列此篇，并更名为 'C++ 调用 C++ 动态库时问题诊断'. 关于 Java JNA 到 C++ 的问题诊断只能另立一篇了。

下面我们来用 JNA 的方式来调用 C++ 动态库，演示当 C++ 代码崩溃时会发生什么，并试图找到好的诊断办法。以下演示在 Linux 下进行, 并且 Linux 
发行版是 Amazon Linux 2023. <!--more-->

先写一个 C++ 动态库，创建一个条件让它会 crash, 比如输入为 `devil` 时产生重复释放内存。

```cpp {linenos=table,hl_lines=14}
#include <stdio.h>
#include <string.h>

void sayHello(char* name) {
    printf("hello %s\n", name);

    if (strcmp(name, "devil") == 0) {
        char* buf = new char[64];
        strcpy(buf, "you are devil!");

        printf("buf: %s\n", buf);

        delete[] buf;
        delete[] buf;
    }
}
```

我们把它编译成动态库，命名为 `libhello.so`，编译命令如下

```shell
g++ -g -shared -fPIC -o libhello.so hello.cpp
```

先用 C++ 来使用该动态库，编写一个测试程序 `test.cpp`，内容如下

```cpp
#include <stdio.h>

extern void sayHello(char* name);

int main(int argc, char* argv[]) {
    if (argc < 2) {
        printf("usage: %s <name>\n", argv[0]);
        return 1;
    }

    sayHello(argv[1]);
    return 0;
}
```

编译并执行

```shell
g++ -g -o test test.cpp -L. -lhello
LD_LIBRARY_PATH=. ./test cpp
```
输出为

> hello cpp

没问题，现在来输入 `devil` 让它崩溃

```shell
LD_LIBRARY_PATH=. ./test devil
```
>LD_LIBRARY_PATH=. ./test devil<br/
>hello devil<br/>
>buf: you are devil!<br/>
>free(): double free detected in tcache 2<br/>
>Aborted (core dumped)

提示说生成的 core dumped 文件，可是在当前目录中没有任何 core 文件。实际上 core dump 文件被 `systemd` 接管了，要用 `coredumpctl` 管理。

### 关于 ulimit -c 的值

在正式检视生成的 core dump 文件之前，还有个知识需补充，之所以会自动生成 core dump 文件是因为系统的 ulimit -c 足够大

```shell
ulimit -c
unlimited
```

修改它的值只对当前 shell 有效，要永久生效的话需修改 `/etc/security/limits.conf` 中的值，

```shell
cat /etc/security/limits.conf
* hard core 0
```

我们可以设置它为更小一点的值，如 1024，设置 0 的话则会禁止生成 core dump 文件，下面测试一下
```shell
ulimit -c 0
LD_LIBRARY_PATH=. ./test devil
hello devil
buf: you are devil!
free(): double free detected in tcache 2
Aborted (core dumped)
```

好像看到也生成的 core dump 文件，但用 `coredumpctl list` 看到的是

```shell
coredumpctl list
TIME                          PID UID GID SIG     COREFILE EXE             SIZE
Sun 2026-02-22 20:44:13 UTC 67032   0   0 SIGABRT none     /host/root/test
```

COREFILE 为 none, 即在 `/var/lib/systemd/coredump/` 下没有相应的 coredump 文件, 实际上就是没能生成相应的 coredump 文件。

用 `coredumpctl info` 或 `coredumpctl gdb` 会看到提示

```text
       Storage: none
       Message: Process 67032 (test) of user 0 terminated abnormally without generating a coredump.
```

执行 `ulimit -c unlimited` 恢复当前会话的的 ulimit -c 为无上限。

### 查看 core dump 文件的内容

假如我们在 `ulimit -c unlimited` 时生成了 core dump 文件，为什么默认被 `systemd` 的 `coredumpctl` 接管了呢？因为文件
`/proc/sys/kernel/core_pattern` 中的内容应该是

```shell
cat /proc/sys/kernel/core_pattern
|/usr/lib/systemd/systemd-coredump %P %u %g %s %t %c %h
```

我们可以修改文件的内容使其显式的在某个目录下生成 core dump 文件，这是后话。首先看如何用 `coredumpctl` 查看 core dump 文件

```shell
coredumpctl list
TIME                          PID UID GID SIG     COREFILE EXE               SIZE
Sun 2026-02-22 21:04:53 UTC 68106   0   0 SIGABRT present  /host/root/test 720.4K
```

`coredumpctl info` 查看最近一个 core dump 的信息

```shell
coredumpctl info
           PID: 68106 (test)
           UID: 0 (root)
           GID: 0 (root)
        Signal: 6 (ABRT)
     Timestamp: Sun 2026-02-22 21:04:52 UTC (6min ago)
  Command Line: ./test devil
    Executable: /host/root/test
 Control Group: /system.slice/docker-9dea312e67c6fc26b72e672f7d5b18aa489278832105d31c5fe91d3b2ea8a61d.scope
          Unit: docker-9dea312e67c6fc26b72e672f7d5b18aa489278832105d31c5fe91d3b2ea8a61d.scope
         Slice: system.slice
       Boot ID: 6faddeadbb624f478e4d6ba74ed272ff
    Machine ID: ec2f3eafec356ee3b6fff46819df8912
      Hostname: lin-0aff3de6.xxx.com
       Storage: /var/lib/systemd/coredump/core.test.0.6faddeadbb624f478e4d6ba74ed272ff.68106.1771794292000000.xz (present)
  Size on Disk: 720.4K
       Message: Process 68106 (test) of user 0 dumped core.
```

`coredumpctl gdb` 进到 `gdb` 提示符下再输入 `bt` 就能清楚的看到问题所在了

```shell {hl_lines=9}
(gdb) bt
#0  0x00007f97f248d02c in __pthread_kill_implementation () from /lib64/libc.so.6
#1  0x00007f97f243fb86 in raise () from /lib64/libc.so.6
#2  0x00007f97f2429873 in abort () from /lib64/libc.so.6
#3  0x00007f97f242a1b2 in __libc_message.cold () from /lib64/libc.so.6
#4  0x00007f97f24970d7 in malloc_printerr () from /lib64/libc.so.6
#5  0x00007f97f24993ab in _int_free () from /lib64/libc.so.6
#6  0x00007f97f249b923 in free () from /lib64/libc.so.6
#7  0x00007f97f2c361fb in sayHello (name=0x7ffe72aaf7f9 "devil") at hello.cpp:14
#8  0x000000000040117e in main (argc=2, argv=0x7ffe72aaee18) at test.cpp:11
```

很清楚了，问题出在 `hello.cpp:14` 第十四行 `delete[] buf;`.

这是用 `g++` 编译时加了 `-g` 参数，所以能定位到具体的行号。如果没有 `-g` 参数，相当于 `-g0`, 二进制代码中不生成任何调试信息，
`-g1`, `-g2`, `-g3` 分别表示生成更丰富的调试信息， `-g` 相当于 `-g2` 包含行号，变量，宏等信息，`-g3` 还包含宏定义信息。

### core dump 文件的导出

`coredumpctl` 或 `coredumpctl list` 看到的文件实际存储在 `/var/lib/systemd/coredump/` 目录下。

```shell
ls -l /var/lib/systemd/coredump/
total 1456
-rw-r-----. 1 root root 737736 Feb 22 21:04 core.test.0.6faddeadbb624f478e4d6ba74ed272ff.68106.1771794292000000.xz
-rw-r-----. 1 root root 737592 Feb 22 21:13 core.test.0.6faddeadbb624f478e4d6ba74ed272ff.68586.1771794822000000.xz
```

一种办法是，用 `xz` 命令解压相应的文件，如

```shell
xz -dck /var/lib/systemd/coredump/core.test.0.6faddeadbb624f478e4d6ba74ed272ff.68106.1771794292000000.xz > core.68106
```
或用 `coredumpctl dump` 命令导出 core dump 文件

```shell
coredumpctl dump 68106 -o core.68106
```

对这个文件可直接用 `gdb` 来查看

```shell
gdb ./test core.68106
(gdb) bt
```

能看到与前面 `coredumpctl gdb`, `bt` 一样的输出信息。

### 编译动态库没带 `-g` 会如何

我们重新编译动态库和，但是不加 `-g` 参数，如

```shell
g++ -g -shared -fPIC -o libhello.so hello.cpp
```

再执行 `LD_LIBRARY_PATH=. ./test devil' 让它生成 core dump 文件

再用 `coredumpctl gdb`, `bt` 来查看 core dump 文件的内容

```shell
(gdb) bt
#0  0x00007fa6cc08d02c in __pthread_kill_implementation () from /lib64/libc.so.6
#1  0x00007fa6cc03fb86 in raise () from /lib64/libc.so.6
#2  0x00007fa6cc029873 in abort () from /lib64/libc.so.6
#3  0x00007fa6cc02a1b2 in __libc_message.cold () from /lib64/libc.so.6
#4  0x00007fa6cc0970d7 in malloc_printerr () from /lib64/libc.so.6
#5  0x00007fa6cc0993ab in _int_free () from /lib64/libc.so.6
#6  0x00007fa6cc09b923 in free () from /lib64/libc.so.6
#7  0x00007fa6cc7391fb in sayHello(char*) () from ./libhello.so
#8  0x000000000040117e in main (argc=2, argv=0x7fff1926a1c8) at test.cpp:11
```

由于编译 `test.cpp` 时带了 `-g` 参数，所以能看到出错时 test.cpp 的行号为 11，在动态库 `libhello.so` 中就一无所知了。

### core dump 文件的清理

命令如下

```shell
ls -l /var/lib/systemd/coredump/
rm -f /var/lib/systemd/coredump/*
journalctl --rotate
journalctl --vacuum-time=1s
```

这时候看到的 `coredumpctl list` 列表就为空了。

### 直接生成 core dump 文件

从前面我们了解了 `ulimit -c` 控制了是否能顺利生成 core dump 文件，要确保有 `ulimit -c` 的值不为 `0`，配置为 `ulimited` 就行了。
同时它是一个会话级别的值，要永久生效可以修改 `/etc/security/limits.conf` 中的值。比如修改其内容为

```text
* hard core unlimited
* soft core unlimited
```

同时 `/proc/sys/kernel/core_pattern` 的内容

```shell
cat /proc/sys/kernel/core_pattern
|/usr/lib/systemd/systemd-coredump %P %u %g %s %t %c %h
```
决定了 core dump 由 `coredumpctl` 来管理，如果只想生成 core dump 文件在特定的目录中

如果要生成 core dump 在当前目录中，执行

```shell
sysctl -w kernel.core_pattern=core.%p
```

可用的占位符有 %p 进程 PID, %e 可执行文件名, %t 时间戳, %s导致崩溃的信号编号, %u 用户 ID

此时看到的 `/proc/sys/kernel/core_pattern` 内容为

```shell
cat /proc/sys/kernel/core_pattern
core.%p
```

测试

```shell
LD_LIBRARY_PATH=. ./test devil
```

当前目录下生成了一个 `core.71686` core dump 文件，`71686` 为 `%p` 代表的进程号。

如查要生成另一个指定目录就用

```shell
sysctl -w kernel.core_pattern=/tmp/cores/core.%p
```

同样的，修改 `kernel.core_pattern` 的值会影响 `/proc/sys/kernel/core_pattern` 的值，但它也是临时的，作用于当前会话。要永久有效的话需修改
`/etc/sysctl.conf` 文件的内容，在其中添加

```text
kernel.core_pattern = /tmp/cores/core.%p
```

然后执行 `sysctl -p` 使其立即生效，或者机器重启后依然保持有效。

最后做个测试，我们结合 `ulimit -c 0`, 看是否生成 core dump 文件

```shell
ulimit -c 0
sysctl -w kernel.core_pattern=core.%p
LD_LIBRARY_PATH=. ./test devil
hello devil
buf: you are devil!
free(): double free detected in tcache 2
Aborted
```

无法生成 core dump 文件， 因为 `ulimit -c 0` 禁止了 core dump 文件的生成。如果设置 `ulimit -c 1024` 还是有机会生成 core dump 文件的。

### 总结

以上测试是为 Java 通过 JNA 调用 C++ 打基础的，具体情形与选择的 Linux 发行版本会有比较大的关系。本文件选择的 Linux 版本是 Amazon Linux 2023.

具体为

```shell
uname -a
Linux lin-0aff3de6.xxx.com 6.1.161-183.298.amzn2023.x86_64 #1 SMP PREEMPT_DYNAMIC Tue Jan 27 05:01:22 UTC 2026 x86_64 x86_64 x86_64 GNU/Linux
cat /etc/os-release
NAME="Amazon Linux"
VERSION="2023"
ID="amzn"
ID_LIKE="fedora"
VERSION_ID="2023"
PLATFORM_ID="platform:al2023"
PRETTY_NAME="Amazon Linux 2023.10.20260202"
ANSI_COLOR="0;33"
CPE_NAME="cpe:2.3:o:amazon:amazon_linux:2023"
HOME_URL="https://aws.amazon.com/linux/amazon-linux-2023/"
DOCUMENTATION_URL="https://docs.aws.amazon.com/linux/"
SUPPORT_URL="https://aws.amazon.com/premiumsupport/"
BUG_REPORT_URL="https://github.com/amazonlinux/amazon-linux-2023"
VENDOR_NAME="AWS"
VENDOR_URL="https://aws.amazon.com/"
SUPPORT_END="2029-06-30"
```

在 C++ 崩溃时有两个参数决定了能否生成 core dump 文件，以及如何管理

- `ulimit -c` 限制了 core dump 文件的大小，如果为 0 则禁止生成 core dump 文件，设置为 `unlimited` 则不限制大小
- `kernel.core_pattern` 决定了 core dump 文件的生成路径和文件名，如果为 `core` 则生成在当前目录下，文件名为 `core
- `ulimit -c` 要永久生效需修改 `/etc/security/limits.conf` 文件
- `kernel.core_pattern` 的修改会临时影响到 `/proc/sys/kernel/core_pattern` 的内容，我们知道 `/proc` 下的内容是动态的。要让
   `kernel.core_pattern` 的修改要永久生效需修改 `/etc/sysctl.conf` 文件
