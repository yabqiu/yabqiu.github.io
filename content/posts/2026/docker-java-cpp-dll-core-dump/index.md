---
title:  Docker 容器中 Java 调用 C++ 动态库问题诊断
url: /docker-java-cpp-dll-core-dump/
date: 2026-02-24T15:04:20-06:00
featured: false
draft: false
type: post
toc: false
# menu: main
usePageBundles: true
thumbnail: "../images/logos/java-logo.png"
categories:
  - Java/JEE
tags: 
  - JNA
  - Docker
comment: true
codeMaxLines: 30
---

本想用一篇日志记录下在 Docker 容器中使用 Java 调用 C++ 动态库时，当 C++ Crash 时如何自动生成 core dump, 不想分成了至少三篇来完成这一研究。
可以回顾一下前两篇日志

1. [C++ 调用 C++ 动态库时问题诊断](/cpp-shared-library-trouble-shooting/)
2. [Java 使用 JNA 调用 C++ 动态库问题诊断](/java-jna-cpp-trouble-shooting/)

本文是基于第二篇进一步推进，继续探索如何在 Docker 容器中 Java 调用 C++ 动态库时的 core dump 如何生成。首先测试的平台依然是 AWS EC2 实例，
OS 为 Amazon Linux 2023, Docker 版本为 25.0.14。为叙事方便，本文所用代码与上篇一样，但还是重复一遍，省却了连接跳转。<!--more-->

### 准备能触发 Crash 的动态库

C++ 动态还是用上篇一样的例子，当输入为 `devil` 时产生重复释放内存从而导致 Crash。

```cpp {linenos=table,hl_lines=15}
#include <stdio.h>
#include <string.h>

extern "C" {
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
}
```
编译生成动态库 `libhello.so`，带上 `-g` 参数让代码行信息保留在二进制文件中，命令如下

```shell
g++ -g -shared -fPIC -o libhello.so hello.cpp
```

### 创建使用 C++ 动态库的 Java 代码

我们将采用 JNA, 只依赖于单一文件，当前版本为 5.18.1, 可从 [5.18.1](https://repo1.maven.org/maven2/net/java/dev/jna/jna/5.18.1/jna-5.18.1.jar)
下载. JDK 版本将使用版本 21, 应用 Java 18 的 Simple Web Server API 来搭建一个简单的 Web Server。

Test.java

```java
import com.sun.jna.*;
import com.sun.net.httpserver.*;

import java.net.InetSocketAddress;

public class Test {
    interface HelloLibrary extends Library {
        void sayHello(String name);
    }

    public static void main(String[] args) throws Exception {
        var library = Native.load("./libhello.so", HelloLibrary.class);
        System.out.println("loaded shared library");

        HttpServer server = HttpServer.create(new InetSocketAddress(8080), 0);

        server.createContext("/ok", exchange -> {
            library.sayHello("world");
            exchange.sendResponseHeaders(200, 0);
            exchange.close();
        });
        server.createContext("/fail", exchange -> {
            library.sayHello("devil");
            exchange.sendResponseHeaders(500, 0);
            exchange.close();
        });

        server.start();
        System.out.println("Server started on port 8080");
    }
}
```

### 构建 Docker 镜像

先创建一个 Dockerfile, 其内容为

```dockerfile
FROM amazoncorretto:21

COPY jna-5.18.1.jar ./
COPY Test.java ./
COPY libhello.so ./

ENTRYPOINT java $JAVA_OPTS -cp jna-5.18.1.jar Test.java
```

创建 Docker 镜像

```shell
docker build -t java-web .
```

### 测试 Docker 容器

在不确定 Docker 容器中运行的 Java 进程(其实就是宿主机上的一个进程) 产生 core dump 与否是不是与宿主机的 ulimit 等设置有关，但在宿主机上执行

```shell
ulimit -c unlimited
sysctl -w kernel.core_pattern=/data/crash_logs/core.%p
```

然后启动容器中的 Web 服务，映射端口号

```shell
docker run -it -p 8080:8080 java-web:latest
loaded shared library
Server started on port 8080
```

在另一个终端中访问 `/ok` 和 `/fail` 两个接口

```shell
[root@lin-0aff3de6 ~]# curl http://localhost:8080/ok
[root@lin-0aff3de6 ~]#
[root@lin-0aff3de6 ~]# curl http://localhost:8080/fail
[root@lin-0aff3de6 ~]# curl http://localhost:8080/fail
[root@lin-0aff3de6 ~]# curl http://localhost:8080/fail
curl: (52) Empty reply from server
```

测试 `http://localhost:8080/fail` 后服务器还不会立即死的，进行了几 次 /fail 请求后最终看到服务器支撑不住了, 其实第一个 /fail 
调用就触发要崩溃，只是产生了延迟。服务端的输出是

```text
hello world
hello devil
buf: you are devil!
hello devil
buf: you are devil!
double free or corruption (fasttop)
#
# A fatal error has been detected by the Java Runtime Environment:
#
#  SIGSEGV (0xb) at pc=0x00007f29394a323b, pid=1, tid=32
#
# JRE version: OpenJDK Runtime Environment Corretto-21.0.10.7.1 (21.0.10+7) (build 21.0.10+7-LTS)
# Java VM: OpenJDK 64-Bit Server VM Corretto-21.0.10.7.1 (21.0.10+7-LTS, mixed mode, sharing, tiered, compressed oops, compressed class ptrs, g1 gc, linux-amd64)
# Problematic frame:
# C  [libc.so.6+0x3523b]  abort+0x23b
#
# Core dump will be written. Default location: /data/crash_logs/core.1
#
# An error report file with more information is saved as:
# //hs_err_pid1.log
#
# If you would like to submit a bug report, please visit:
#   https://github.com/corretto/corretto-21/issues/
# The crash happened outside the Java Virtual Machine in native code.
# See problematic frame for where to report the bug.
#
```

后面几行给出了两个信息，可以生成 core dump 文件，在 `/data/crash_logs/core.1`, 还生成了 hs_err.log 文件，在 `//hs_err_pid1.log`,
因为指定的  `/data/crash_logs/core.%p`, 所以无论生成的 core dump 是在容器内还是宿主机上，都是在 `/data/crash_logs` 目录下，可是没有。

用  `docker ps -a` 找到已死的容器 ID, `docker start <container-id>` 复活它，再  `docker exec -it <container-id> bash`
进到容器中，可以看到 `/hs_err_pid1.log` 文件。 从该 `/hs_err_pid1.log` 可看到引起 crash 的原因

```text
j  com.sun.jna.Library$Handler.invoke(Ljava/lang/Object;Ljava/lang/reflect/Method;[Ljava/lang/Object;)Ljava/lang/Object;+390
j  $Proxy0.sayHello(Ljava/lang/String;)V+16
j  Test.lambda$main$1(LTest$HelloLibrary;Lcom/sun/net/httpserver/HttpExchange;)V+3
j  Test$$Lambda+0x00007f28bc181a28.handle(Lcom/sun/net/httpserver/HttpExchange;)V+5
```

如果我们起动容器时加上 `-e JAVA_TOOL_OPTIONS="-XX:ErrorFile=/data/crash_logs/hs_err_%p.log"` 将来能看在宿主机的 /data/crash_logs
目录下生成 `hs_err_1.log` 文件.

用 `docker exec -it <container-id> bash` 进到容器，观察两个值

```shell
bash-4.2# ulimit -c
unlimited
bash-4.2# cat /proc/sys/kernel/core_pattern
/data/crash_logs/core.%p
```

从上面看到的 `/data/crash_logs/java-core.1` 说明真的是宿主机上的 `kernel.core_pattern=java-core.%p` 影响着容器中运行的 Java 进程。
还是那句话， 说是容器的进程，本就是宿主机上的的进程，因为加了进程 namespace. 比如运行了 `docker run -it -p 8080:8080 java-web:latest`,
我们用 `docker exec -it <container-id> sh` 进到容器后看到的 Java 进程是

```shell
sh-4.2# ps -ef|grep Test.java
root           1       0  2 14:40 pts/0    00:00:01 java -cp jna-5.18.1.jar Test.java
```

而在宿主机上， 看到的 Java 进程是

```shell
[root@lin-0aff3de6 ~]# ps -ef|grep Test.java
root      124386  124360  0 14:40 ?        00:00:02 java -cp jna-5.18.1.jar Test.java
```
容器内 PID:1 与 容器外 PID: 124386 实际是同一个进程，只是生成 core dump 时用了容器内的 PID:1. 假如启动 Docker 容器时用了 `--pid=host`
参数，那么容器里外看到的该 Java 进程 PID 值就完全一样了。

### 测试宿主机 ulimit -c 对容器的影响

试图用 ulimit -c 0

```shell
[root@lin-0aff3de6 ~]# ulimit -c 0
[root@lin-0aff3de6 ~]# ulimit -c
0
[root@lin-0aff3de6 ~]# docker run -it -p 8080:8080 java-web:latest
```

观察容器内部

```shell
[root@lin-0aff3de6 ~]# docker exec 7f2 bash -c "ulimit -c && cat /proc/sys/kernel/core_pattern"
unlimited
java-core.%p
```

有两种情况，要么 docker 命令启动容器时不受 `ulimit -c 0` 会话的影响，或者 `docker` 启动时默认设置了 `ulimit -c unlimited`. 又试了修改
`/etc/security/limits.conf` 中的 soft, hard 都为 0 后在容器上的 `ulimit -c` 都是 `unlimited`. 

因为 Docker 容器设置 `ulimit -c unlimited` 是默认行为，除非启动 Docker 容器时用了 `docker run --ulimit core=0` 参数重置 ulimit -c.

### 阶段小结

学到这里，我们有必要来个小节, 两个关键知识点

1. Docker 默认 `ulimit -c unlimited`, 除非用 `--ulimit core=0` 明确指定值
2. 容器与宿主机共享相同的 `/proc/sys/kernel/core_pattern` 配置，可以在启动容器时用命令
  `sysctl -w kernel.core_pattern=java-core.%p` 配置，或修改 `/etc/sysctl.conf` 使之永久有效

加了 `-XX:+CreateCoredumpOnCrash` 也没有 coredump 文件生成

```shell
sysctl -w kernel.core_pattern=/data/crash_logs/java-core.%p
docker run -it -v /data/crash_logs:/data/crash_logs \
  -e JAVA_TOOL_OPTIONS="-XX:ErrorFile=/crash_logs/hs_err_%p.log -XX:+CreateCoredumpOnCrash" \
  -p 8080:8080 java-web:latest
```

后来还试过启动 Docker 容器的加更多的参数，如

```shell
sysctl -w kernel.core_pattern=/data/crash_logs/java-core.%p
docker run -v /data/crash_logs:/data/crash_logs \
   --ulimit core=-1 --privileged --cap-add=SYS_PTRACE --security-opt seccomp=unconfined \
  -e JAVA_TOOL_OPTIONS="-XX:ErrorFile=/data/crash_logs/hs_err_%p.log -XX:+CreateCoredumpOnCrash" \
  -p 8080:8080 java-web:latest
```

通通都不管用, 怎么都不生成 core dump 文件, 即使 Docker 基础镜像由 `amazoncorretto:21` 改成 `amazoncorretto:21-2023` 也无济于事。

### 尝试非 Web Java 应用

修改之前的 Java 代码，改为

```java
import com.sun.jna.*;

public class Test {
    interface HelloLibrary extends Library {
        void sayHello(String name);
    }

    public static void main(String[] args) throws Exception {
        var library = Native.load("./libhello.so", HelloLibrary.class);
        System.out.println("loaded shared library");
        library.sayHello(args[0]);
    }
}
```

Dockerfile 也相应修改

```dockerfile
FROM amazoncorretto:21

COPY jna-5.18.1.jar ./
COPY Test.java ./
COPY libhello.so ./


ENTRYPOINT ["java", "-cp", "jna-5.18.1.jar", "Test.java"]
```

因为要通过命令行参数来触发 Crash. 生新构建 `docker build -t java-console .` 镜像后，

执行

```shell
docker run java-console world
loaded shared library
hello world

sysctl -w kernel.core_pattern=/data/crash_logs/java-core.%p
docker run -v /data/crash_logs:/data/crash_logs   \
  --ulimit core=-1 --privileged --cap-add=SYS_PTRACE --security-opt seccomp=unconfined  \
  -e JAVA_TOOL_OPTIONS="-XX:ErrorFile=/data/crash_logs/hs_err_%p.log -XX:+CreateCoredumpOnCrash" \
  java-dll-core-dump:latest devil
Picked up JAVA_TOOL_OPTIONS: -XX:ErrorFile=/data/crash_logs/hs_err_%p.log -XX:+CreateCoredumpOnCrash
loaded shared library
double free or corruption (fasttop)
#
# A fatal error has been detected by the Java Runtime Environment:
#
#  SIGSEGV (0xb) at pc=0x00007f116926523b, pid=1, tid=7
#
# JRE version: OpenJDK Runtime Environment Corretto-21.0.10.7.1 (21.0.10+7) (build 21.0.10+7-LTS)
# Java VM: OpenJDK 64-Bit Server VM Corretto-21.0.10.7.1 (21.0.10+7-LTS, mixed mode, sharing, tiered, compressed oops, compressed class ptrs, g1 gc, linux-amd64)
# Problematic frame:
# C  [libc.so.6+0x3523b]  abort+0x23b
#
# Core dump will be written. Default location: /data/crash_logs/java-core.1
#
# An error report file with more information is saved as:
# /data/crash_logs/hs_err_1.log
#
# If you would like to submit a bug report, please visit:
#   https://github.com/corretto/corretto-21/issues/
#
```

coredump 没有，还是没有。

### 换成 C++ 调动动态库的方式

Dockerfile 的内容为

```dockerfile
FROM amazonlinux:2023

COPY hello ./
COPY libhello.so ./
ENV LD_LIBRARY_PATH=./

ENTRYPOINT ["./hello"]
```

运行

```shell
sysctl -w kernel.core_pattern=/data/crash_logs/core.%p
docker run cpp-chello world
hello world
docker run -v /data/crash_logs:/data/crash_logs cpp-hello devil
free(): double free detected in tcache 2
```

可是在 `/data/crash_logs` 目录下生成了 core.1 文件。

`gdb ./hello core.1`, 再 `bt` 就能看到调用栈了

```text
Missing rpms, try: dnf --enablerepo='*debug*' install libstdc++-debuginfo-14.2.1-7.amzn2023.0.2.x86_64 glibc-debuginfo-2.34-231.amzn2023.0.3.x86_64 libgcc-debuginfo-14.2.1-7.amzn2023.0.2.x86_64
(gdb) bt
#0  0x00007f6381b24918 in abort () from /lib64/libc.so.6
#1  0x00007f6381b251b2 in __libc_message.cold () from /lib64/libc.so.6
#2  0x00007f6381b920d7 in malloc_printerr () from /lib64/libc.so.6
#3  0x00007f6381b943ab in _int_free () from /lib64/libc.so.6
#4  0x00007f6381b96923 in free () from /lib64/libc.so.6
#5  0x00000000004011f9 in sayHello (name=0x7fffc33cdf74 "devil") at hello.cpp:14
#6  0x0000000000401244 in main (argc=2, argv=0x7fffc33cd3d8) at hello.cpp:24
```

注意把 `hello` 和 `libhello.so` 都放在与 `core.1` 同一目录下，否则 `gdb` 无法解析出调用栈中的函数名和行号信息。

看来是 JNA 在从中作祟。

### 恢复让 coredumpctl 管理的方式

```shell
# 这是改的内存中的值
echo '|/usr/lib/systemd/systemd-coredump %P %u %g %s %t %c %h' | sudo tee /proc/sys/kernel/core_pattern

# 如果在 /etc/sysctl.conf 没有相关的配置, 执行 sysctl --system 也能恢复到上面的值
cat /proc/sys/kernel/core_pattern
|/usr/lib/systemd/systemd-coredump %P %u %g %s %t %c %h
```

现在执行 Java Web 的 Docker 镜像, 访问 `/fail` 接口出错后在服务端显示为

```text
docker run -p 8080:8080 java-web
...
# Core dump will be written. Default location: Core dumps may be processed with "/usr/lib/systemd/systemd-coredump %P %u %g %s %t %c %h" (or dumping to //core.1)
```
说明被 systemd-coredump 接管了，但这次测试用 `coredumpctl` 没有看到任何记录，还有些问题。查看 systemd-coredump 的日志

```shell
[root@lin-0aff3de6 ~]# journalctl -u "systemd-coredump*" --no-pager
Feb 24 03:04:12 lin-0aff3de6.example.com systemd[1]: systemd-coredump.socket: Deactivated successfully.
Feb 24 03:04:12 lin-0aff3de6.example.com systemd[1]: Closed systemd-coredump.socket - Process Core Dump Socket.
-- Boot 5bcfe0586e0643dbbef47fe92eb6483e --
Feb 25 03:03:33 lin-0aff3de6.example.com systemd[1]: systemd-coredump.socket: Deactivated successfully.
Feb 25 03:03:33 lin-0aff3de6.example.com systemd[1]: Closed systemd-coredump.socket - Process Core Dump Socket.
```

大概是 Java 进程退的太快了。

如果用 `journalctl -u "systemd-coredump*" --no-pager` 看到 Timeout 的信息时，因为默认 5 分钟，但 core dump 太多，超时就会放弃。
那就得修改 RuntimeMaxSec 的值，改成无限制，或某个更大的值，修改方法如下

```shell
sudo mkdir -p /etc/systemd/system/systemd-coredump@.service.d/
sudo tee /etc/systemd/system/systemd-coredump@.service.d/override.conf <<EOF
[Service]
RuntimeMaxSec=infinity
EOF

sudo systemctl daemon-reload

# 查看修改后的值，如果显示为 infinity 就说明修改成功了，出现多个值的话，最后那个值生效
systemctl cat systemd-coredump@.service | grep RuntimeMaxSec
```

### -fsanitize=address 仍然有效

在编译 `hello.cpp` 时加上 `-fsanitize=address` 参数，生成的动态库在触发 Crash 后能看到更详细的错误信息. `-fsanitize=address` 
要避免和高优化的 `-O2` 或 `-O3` 一起使用，通常还会与 `-fno-omit-frame-pointer` 一同使用，保留桢指针。`-fsanitize` 除了有 `address`,
还支持 `leak`, `thread`, `undefined`, `memory`， 如 `-fsanitize=address,leak`.

```shell
g++ -shared -fPIC -fsanitize=address -g -o libhello.so hello.cpp
```

Dockerfile 中要安装 `libasan` 包，通过 `yum search libasan` 安装相应的版本，或安装 `gcc` 时会自动安装 `libasan` 包. 
现在的 Dockerfile 如下

```dockerfile
FROM amazoncorretto:21

COPY jna-5.18.1.jar ./
COPY Test.java ./
COPY libhello.so ./

RUN yum install -y libasan10

ENTRYPOINT LD_PRELOAD=/lib64/libasan.so.6.0.0 java -cp jna-5.18.1.jar Test.java
```

用 `amazoncorrectto:21-2023` 基础镜像也行，安装 `libasan` 包的命令是 `yum install -y libasan`. 还有在配置 `LD_PRELOAD` 时要
使用实际的文件，不能是链接文件, `/lib64/libasan.so.6` 是无效的.

构建并启动

```shell
docker build -t java-web .
docker run -it -p 8080:8080 java-web
```

访问 `http://localhost:8080/fail` 接口后，能看到更详细的错误信息

在 Docker 容器即服务端控制台会输出

```text
[root@lin-0aff3de6 ~]# docker run -it -p 8080:8080 java-web
loaded shared library
Server started on port 8080
hello devil
buf: you are devil!
=================================================================
==1==ERROR: AddressSanitizer: attempting double-free on 0x606000087ec0 in thread T25 (HTTP-Dispatcher):
    #0 0x7f0b23be4617 in operator delete[](void*) (/lib64/libasan.so.6.0.0+0xb2617)
    #1 0x7f0a3e0cc273 in sayHello /root/hello.cpp:17
    #2 0x7f0a3e17f051  (/root/.cache/JNA/temp/jna6772144201813723775.tmp+0x16051)
    #3 0x7f0a3e17e1cb  (/root/.cache/JNA/temp/jna6772144201813723775.tmp+0x151cb)
    #4 0x7f0a3e17e507  (/root/.cache/JNA/temp/jna6772144201813723775.tmp+0x15507)
    #5 0x7f0a3e173a59  (/root/.cache/JNA/temp/jna6772144201813723775.tmp+0xaa59)
    #6 0x7f0a3e173f20  (/root/.cache/JNA/temp/jna6772144201813723775.tmp+0xaf20)
    #7 0x7f0b0cea19bf  (<unknown module>)

0x606000087ec0 is located 0 bytes inside of 64-byte region [0x606000087ec0,0x606000087f00)
freed by thread T25 (HTTP-Dispatcher) here:
    #0 0x7f0b23be4617 in operator delete[](void*) (/lib64/libasan.so.6.0.0+0xb2617)
    #1 0x7f0a3e0cc260 in sayHello /root/hello.cpp:16
    #2 0x7f0a3e17f051  (/root/.cache/JNA/temp/jna6772144201813723775.tmp+0x16051)
    #3 0x7f0a3dc33faf  (<unknown module>)

previously allocated by thread T25 (HTTP-Dispatcher) here:
    #0 0x7f0b23be3b17 in operator new[](unsigned long) (/lib64/libasan.so.6.0.0+0xb1b17)
    #1 0x7f0a3e0cc213 in sayHello /root/hello.cpp:11
    #2 0x7f0a3e17f051  (/root/.cache/JNA/temp/jna6772144201813723775.tmp+0x16051)
    #3 0x7f0a3dc33faf  (<unknown module>)

Thread T25 (HTTP-Dispatcher) created by T1 here:
    #0 0x7f0b23b8a1a1 in pthread_create (/lib64/libasan.so.6.0.0+0x581a1)
    #1 0x7f0b1e532112 in os::create_thread(Thread*, os::ThreadType, unsigned long) (/usr/lib/jvm/java-21-amazon-corretto/lib/server/libjvm.so+0xcff112)
    #2 0x7f0b1e226d81 in JVM_StartThread (/usr/lib/jvm/java-21-amazon-corretto/lib/server/libjvm.so+0x9f3d81)
    #3 0x7f0b0cea19bf  (<unknown module>)
    #4 0x7f0b0ce9d17f  (<unknown module>)
    #5 0x7f0b0ce9d17f  (<unknown module>)
    #6 0x7f0b0ce9d17f  (<unknown module>)
    #7 0x7f0b0ce9d17f  (<unknown module>)
    #8 0x7f0b0ce9d17f  (<unknown module>)
    #9 0x7f0b0ce9d17f  (<unknown module>)
    #10 0x7f0b0ce9d2f1  (<unknown module>)
    #11 0x7f0b0ce9d2f1  (<unknown module>)
    #12 0x7f0b0ce9d2f1  (<unknown module>)
    #13 0x7f0b0ce9d797  (<unknown module>)
    #14 0x7f0b0ce9d2f1  (<unknown module>)
    #15 0x7f0b0ce9d17f  (<unknown module>)
    #16 0x7f0b0ce9d17f  (<unknown module>)
    #17 0x7f0b0ce95cc5  (<unknown module>)
    #18 0x7f0b1e149363 in JavaCalls::call_helper(JavaValue*, methodHandle const&, JavaCallArguments*, JavaThread*) (/usr/lib/jvm/java-21-amazon-corretto/lib/server/libjvm.so+0x916363)
    #19 0x7f0b1e1f3cae in jni_invoke_static(JNIEnv_*, JavaValue*, _jobject*, JNICallType, _jmethodID*, JNI_ArgumentPusher*, JavaThread*) [clone .isra.156] [clone .constprop.261] (/usr/lib/jvm/java-21-amazon-corretto/lib/server/libjvm.so+0x9c0cae)
    #20 0x7f0b1e1f5ede in jni_CallStaticVoidMethod (/usr/lib/jvm/java-21-amazon-corretto/lib/server/libjvm.so+0x9c2ede)
    #21 0x7f0b23711736 in JavaMain (/usr/lib/jvm/java-21-amazon-corretto/bin/../lib/libjli.so+0x5736)
    #22 0x7f0b23714528 in ThreadJavaMain (/usr/lib/jvm/java-21-amazon-corretto/bin/../lib/libjli.so+0x8528)
    #23 0x7f0b234f544a in start_thread (/lib64/libpthread.so.0+0x744a)

Thread T1 created by T0 here:
    #0 0x7f0b23b8a1a1 in pthread_create (/lib64/libasan.so.6.0.0+0x581a1)
    #1 0x7f0b237150cc in CallJavaMainInNewThread (/usr/lib/jvm/java-21-amazon-corretto/bin/../lib/libjli.so+0x90cc)
    #2 0x7f0b2371212c in ContinueInNewThread (/usr/lib/jvm/java-21-amazon-corretto/bin/../lib/libjli.so+0x612c)
    #3 0x7f0b23712b7e in JLI_Launch (/usr/lib/jvm/java-21-amazon-corretto/bin/../lib/libjli.so+0x6b7e)
    #4 0x557760e00a4d in main (/usr/lib/jvm/java-21-amazon-corretto/bin/java+0xa4d)
    #5 0x7f0b22f5e139 in __libc_start_main (/lib64/libc.so.6+0x21139)

SUMMARY: AddressSanitizer: double-free (/lib64/libasan.so.6.0.0+0xb2617) in operator delete[](void*)
==1==ABORTING
```

这个非常有用. 编译动态库时还要加上 `-g` 参数, 这样在输出的错误信息中就能看到具体的行号了. 对于优化级别 `-O`, 0, 1, 2, 3 就看行号准不准确了。

### 总结

使用 Docker 容器运行 Java 应用时，触发 C++ 动态库崩溃后，无论是非用 systemd-coredump 管理 core dump 都未能成功生成 core dump 文件，
不过实际的一个 SpringBoot Web 应用确实在所用 C++ 代码崩溃后由 systemd-coredump 生成了 core dump 文件，尚不知问题出在哪里.

在此就不在测试 SpringBoot Web 应用了。

最后用，但可能性能会下降一半的方式就是用 `-fsanitize=address -g` 编译 C++ 代码，虽然不能生成 core dump 文件，但能在控制台输出更详细的错误信息了。