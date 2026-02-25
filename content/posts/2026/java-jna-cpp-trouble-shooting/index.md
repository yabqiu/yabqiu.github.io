---
title: Java 使用 JNA 调用 C++ 动态库问题诊断 
url: /java-jna-cpp-trouble-shooting/
date: 2026-02-22T22:29:37-06:00
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
comment: true
codeMaxLines: 50
---

在 Java 应用程序通过 JNA 调用 C++ 动态库时，C++ 代码运行在与 Java 同一进程中，当 C++ 代码 Crash 的时候，将会导致整个 Java 应用程序崩溃。
对于一个 Web 应用，这不是我们期望的结果，由于某一个请求输入的数据导致 C++ 代码崩溃了当前 Java 进程，从而造成该 Web 服务已接受到的所有请求全部失败，
这是非常糟糕的用户体验。如果是 Java 代码本身的异常我们可用 try/catch 进行保护，影响只限制在当前请求。如果是 C++ 代码崩溃的话，Java
应用程序无法捕获到这个异常，以致于整个 Java 应用程序崩溃，甚至发生这种情况时连 hs_err.log 文件都来不及生成，更别说生成 HeapDump, 或 CoreDump 了。

如果是用原始 JNI 的方式来调用动态库，我们还能在 JNI 相关的 C++ 代码中捕获到异常，并抛给 Java 去处理。而用 JNA, 我们贪图了它的方便，
比如一个 Java 进程中同时加载同一接口的不同动态库版本(JNI 要同样的实现必须用自定义的 ClassLoader)，但在 C++ 代码崩溃时， Java 就显得无能为力了，
只能跟随着立即死亡, 并且在控制台下找不到关于 C++ 因何失败的线索。比如 C++ 中内存被多次释放，或地址越界访问破坏了内存数据等。 <!--more-->

下面我们来用 JNA 的方式来调用 C++ 动态库，演示当 C++ 代码崩溃时会发生什么，并试图找到好的诊断办法。以下演示在 Linux 下进行, 并且 Linux
发行版是 Amazon Linux 2023.

刚刚完成的一篇日志 [C++ 调用 C++ 动态库时问题诊断](/cpp-shared-library-trouble-shooting/) 算是个开味菜，现在开始要与 Java/JNA 的结合了。

先回顾一下上篇的总结部分，两个要点

1. `ulimit -c unlimited` 或某个足够大的值才会产生 core dump 文件， 对应的配置文件是 `/etc/security/limits.conf`
2. `sysctl -w kernel.core_pattern=/crash_logs/core.%p.%t` 配置直接生成 core dump 文件到某个具体的目录，而不是用 coredumpctl 来管理，
它对应的配置文件是 `/etc/sysctl.conf`(直接修改后，需 sysctl -p)，运行时文件是 `/proc/sys/kernel/core_pattern`。

有了这两个知识储备后，以后如果应用程序运行在 Docker 容器中的话，比如 ECS Container，就可以通过配置 Docker 容器的参数来实现，
比如 `--ulimit core=-1` 来开启 core dump，`--sysctl kernel.core_pattern=/crash_logs/core.%p.%`。

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

### 测试 C++ 崩溃时是否生成 core dump

确保 ulimit -c 和 kernel.core_pattern 的值可以产生 coredump 并直接生成 core dump 文件，所以我们先执行

```shell
ulimit -c unlimited
sysctl -w kernel.core_pattern=core.%p.%t
```

注：切记, 如果写在某个目录中一定要确保该目录存在，并且有写入权限，比如 `sysctl -w kernel.core_pattern=/crash_logs/core.%p.%t`, 
若目录 /crash_logs 不存在，或无写入权限，可触发错误时，看不到 `(core dumped)`, 会错认为是别的原因导致 core dump 文件未能生成。
我本人就深受其苦, 写完本文的大部分内容，一直未能生成 core dump 文件而四处找原因，也使原本写成的后面大部分内容进行了重写。

运行 Java 程序

```shell {hl_lines=1}
java -cp jna-5.18.1.jar  Test.java
loaded shared library
Server started on port 8080
```

在另一终端中访问

```shell
curl http://localhost:8080/ok
```

服务端控制台输出

>hello world

调用

```shell
url http://localhost:8080/fail
```

服务端输出

```text
hello devil
buf: you are devil!
free(): double free detected in tcache 2
Aborted (core dumped)
```

看到 `(core dumped)`, 非常好，现在可以开始分析该 core dump 文件的内容进行 C++ 错误代码定位。

### C++ 错误代码分析

上一步在当前目录中生成了文件 `core.91607.1771819640`, 上 gdb, 然后 `bt`

```shell
gdb java core.91607.1771819640
.......
(gdb) bt
#0  0x00007f4ac588d02c in __pthread_kill_implementation () from /lib64/libc.so.6
#1  0x00007f4ac583fb86 in raise () from /lib64/libc.so.6
#2  0x00007f4ac5829873 in abort () from /lib64/libc.so.6
#3  0x00007f4ac582a1b2 in __libc_message.cold () from /lib64/libc.so.6
#4  0x00007f4ac58970d7 in malloc_printerr () from /lib64/libc.so.6
#5  0x00007f4ac58993ab in _int_free () from /lib64/libc.so.6
#6  0x00007f4ac589b923 in free () from /lib64/libc.so.6
#7  0x00007f4ac57431eb in sayHello (name=0x7f49ec018430 "devil") at hello.cpp:15
#8  0x00007f4a95c16052 in ?? ()
#9  0x0000000000000000 in ?? ()
```
问题出在

> sayHello (name=0x7f49ec018430 "devil") at hello.cpp:15

其实都不需要虚拟机参数 `-XX:+CreateCoredumpOnCrash` 默认就会生成 core dump 文件. 不过没见到 hs_err.log 文件。尝试下面的命令

```shell
java -XX:+CreateCoredumpOnCrash \
     -XX:ErrorFile=hs_err_%p.log \
     -cp jna-5.18.1.jar Test.java
```

仍然没见到 hs_err_<PID>.log Hot Spot 错误文件，不过不重要了，本文件想要知道的是 Java 通过 JNA 调用 C++ 动态库出错时哪段 C++ 代码出问题了。

### 另一定位 C++ 地址相关问题

Claude 介绍了针对 double free 的 ASAN, 即在编译动态库时用

```shell
g++ -shared -fPIC -fsanitize=address -g -o libhello.so hello.cpp
```
然后执行

```shell
LD_PRELOAD=$(gcc -print-file-name=libasan.so) \
java -cp jna-5.18.1.jar Test.java
```
如果加了 `-fsantitize=address` 参数编译生成的 `libhello.so`, 不指定 `LD_PRELOAD` 的话，Test.java 将找不到 `libhello.so` 文件。

现在访问

```shell
curl http://localhost:8080/fail
```

在服务端的控制台会出现一大片的信息

```text
hello devil
buf: you are devil!
=================================================================
==86041==ERROR: AddressSanitizer: attempting double-free on 0x606000062000 in thread T25:
    #0 0x7f740fcb1ac7 in operator delete[](void*) (/usr/lib/gcc/x86_64-amazon-linux/11/libasan.so+0xb1ac7)
    #1 0x7f740c201233 in sayHello /root/hello.cpp:15
    #2 0x7f731e016051  (/root/.cache/JNA/temp/jna4011246694615712467.tmp+0x16051)
    #3 0x7f731e0151cb  (/root/.cache/JNA/temp/jna4011246694615712467.tmp+0x151cb)
    #4 0x7f731e015507  (/root/.cache/JNA/temp/jna4011246694615712467.tmp+0x15507)
    #5 0x7f731e00aa59  (/root/.cache/JNA/temp/jna4011246694615712467.tmp+0xaa59)
    #6 0x7f731e00af20  (/root/.cache/JNA/temp/jna4011246694615712467.tmp+0xaf20)
    #7 0x7f73fa5439bf  (<unknown module>)

0x606000062000 is located 0 bytes inside of 64-byte region [0x606000062000,0x606000062040)
freed by thread T25 here:
    #0 0x7f740fcb1ac7 in operator delete[](void*) (/usr/lib/gcc/x86_64-amazon-linux/11/libasan.so+0xb1ac7)
    #1 0x7f740c201220 in sayHello /root/hello.cpp:14
    #2 0x7f731e016051  (/root/.cache/JNA/temp/jna4011246694615712467.tmp+0x16051)
    #3 0x7f731d0a5f0f  (<unknown module>)

previously allocated by thread T25 here:
    #0 0x7f740fcb1107 in operator new[](unsigned long) (/usr/lib/gcc/x86_64-amazon-linux/11/libasan.so+0xb1107)
    #1 0x7f740c2011d3 in sayHello /root/hello.cpp:9
    #2 0x7f731e016051  (/root/.cache/JNA/temp/jna4011246694615712467.tmp+0x16051)
    #3 0x7f731d0a5f0f  (<unknown module>)

Thread T25 created by T1 here:
    #0 0x7f740fc57736 in pthread_create (/usr/lib/gcc/x86_64-amazon-linux/11/libasan.so+0x57736)
    #1 0x7f740b50452d in os::create_thread(Thread*, os::ThreadType, unsigned long) (/usr/lib/jvm/java-21-amazon-corretto.x86_64/lib/server/libjvm.so+0xd0452d)
    #2 0x7f740b1f2bee in JVM_StartThread (/usr/lib/jvm/java-21-amazon-corretto.x86_64/lib/server/libjvm.so+0x9f2bee)
    #3 0x7f73fa5439bf  (<unknown module>)
    #4 0x7f73fa53f17f  (<unknown module>)
    #5 0x7f73fa53f17f  (<unknown module>)
    #6 0x7f73fa53f17f  (<unknown module>)
    #7 0x7f73fa53f17f  (<unknown module>)
    #8 0x7f73fa53f17f  (<unknown module>)
    #9 0x7f73fa53f17f  (<unknown module>)
    #10 0x7f73fa53f2f1  (<unknown module>)
    #11 0x7f73fa53f2f1  (<unknown module>)
    #12 0x7f73fa53f2f1  (<unknown module>)
    #13 0x7f73fa53f797  (<unknown module>)
    #14 0x7f73fa53f2f1  (<unknown module>)
    #15 0x7f73fa53f17f  (<unknown module>)
    #16 0x7f73fa53f17f  (<unknown module>)
    #17 0x7f73fa537cc5  (<unknown module>)
    #18 0x7f740b114834 in JavaCalls::call_helper(JavaValue*, methodHandle const&, JavaCallArguments*, JavaThread*) (/usr/lib/jvm/java-21-amazon-corretto.x86_64/lib/server/libjvm.so+0x914834)
    #19 0x7f740b1baebc in jni_invoke_static(JNIEnv_*, JavaValue*, _jobject*, JNICallType, _jmethodID*, JNI_ArgumentPusher*, JavaThread*) [clone .constprop.1] (/usr/lib/jvm/java-21-amazon-corretto.x86_64/lib/server/libjvm.so+0x9baebc)
    #20 0x7f740b1bd43a in jni_CallStaticVoidMethod (/usr/lib/jvm/java-21-amazon-corretto.x86_64/lib/server/libjvm.so+0x9bd43a)
    #21 0x7f741079f163 in JavaMain (/usr/lib/jvm/java-21-amazon-corretto.x86_64/bin/../lib/libjli.so+0x5163)
    #22 0x7f74107a1bb8 in ThreadJavaMain (/usr/lib/jvm/java-21-amazon-corretto.x86_64/bin/../lib/libjli.so+0x7bb8)
    #23 0x7f740f88b2e9 in start_thread (/lib64/libc.so.6+0x8b2e9)

Thread T1 created by T0 here:
    #0 0x7f740fc57736 in pthread_create (/usr/lib/gcc/x86_64-amazon-linux/11/libasan.so+0x57736)
    #1 0x7f74107a27ad in CallJavaMainInNewThread (/usr/lib/jvm/java-21-amazon-corretto.x86_64/bin/../lib/libjli.so+0x87ad)
    #2 0x7f741079fa7c in ContinueInNewThread (/usr/lib/jvm/java-21-amazon-corretto.x86_64/bin/../lib/libjli.so+0x5a7c)
    #3 0x7f74107a04cf in JLI_Launch (/usr/lib/jvm/java-21-amazon-corretto.x86_64/bin/../lib/libjli.so+0x64cf)
    #4 0x55f716db61fe in main (/usr/lib/jvm/java-21-amazon-corretto.x86_64/bin/java+0x11fe)
    #5 0x7f740f82a60f in __libc_start_call_main (/lib64/libc.so.6+0x2a60f)

SUMMARY: AddressSanitizer: double-free (/usr/lib/gcc/x86_64-amazon-linux/11/libasan.so+0xb1ac7) in operator delete[](void*)
==86041==ABORTING
```

最前面就提示了出错的代码行在

> 1 0x7f740c201233 in sayHello /root/hello.cpp:15

即第二个

> delete[] buf;

不想信息输出来控制台，可以输出到文件，命令是

```shell
LD_PRELOAD=$(gcc -print-file-name=libasan.so) \
ASAN_OPTIONS=abort_on_error=0:log_path=/crash_logs/asan.log \
java -cp jna-5.18.1.jar Test.java
```

但是其他非地址的问题该如何解决呢？加了 `-fsanitize=address` 编译参数对性能影响有多大呢？Claude 的答案是

> CPU 速度慢 2x 左右, 内存占用增加 2~3x, 二进制体积增大（含调试符号）

看来生产环境中是不可接受的。

### 尝试用 JNI 的方式调用动态库

#### 创建带 native 方法的 Greeter 类

```java
public class Greeter {

    static {
        System.load("/root/libgreeter.so"); // 或用 System.loadLibrary("greeter"); 会依 LD_LIBRARY_PATH 寻找 libgreeter.so
    }

    public native void sayHello(String name);

    public static void main(String[] args) {
        Greeter greeter = new Greeter();
        String name = args.length > 0 ? args[0] : "world";
        greeter.sayHello(name);
    }
}
```

#### 编译并生成相应的 C++ 头文件 Greeter.h

```shell
javac -h ./ Greeter.java
```

生成 Greeter.class 和 Greeter.h

#### 实现 Greeter.cpp

```cpp
#include <jni.h>
#include <stdio.h>
#include <string.h>

extern "C" {

JNIEXPORT void JNICALL
Java_Greeter_sayHello(JNIEnv* env, jobject thiz, jstring jname) {

    const char* name = env->GetStringUTFChars(jname, nullptr);
    if (name == nullptr) return;

    printf("hello %s\n", name);

    if (strcmp(name, "devil") == 0) {
        char* buf = new char[64];
        strcpy(buf, "you are devil!");

        printf("buf: %s\n", buf);

        delete[] buf;
        delete[] buf;
    }

    env->ReleaseStringUTFChars(jname, name);
}
}
```

#### 编译生成 C++ 动态库 libgreeter.so

```shell
export JAVA_HOME=/usr/lib/jvm/java-21-amazon-corretto
g++ -shared -fPIC -I $JAVA_HOME/include -I $JAVA_HOME/include/linux -o libgreeter.so Greeter.cpp
```

#### 执行 Greeter

```shell
[root@lin-0aff3de6 ~]# java Greeter
hello world
[root@lin-0aff3de6 ~]# java Greeter devil
hello devil
buf: you are devil!
free(): double free detected in tcache 2
Aborted (core dumped)
```
同样会产生 core dump 文件，诊断办法同上。

### 试下 Python

```python
import ctypes
import os

lib = ctypes.CDLL("./libhello.so")

lib.sayHello.argtypes = [ctypes.c_char_p]
lib.sayHello.restype = None

name = "world"
lib.sayHello(name.encode("utf-8"))
lib.sayHello(b"devil")
```

执行

```shell
python3 test.py
hello world
hello devil
buf: you are devil!
free(): double free detected in tcache 2
Aborted (core dumped)
```

诊断办法用 `gdb python3 core.92636.1771820629`, 然后输入 `bt` 就可以看到出错的 C++ 代码行了。

看来也要收住篇幅了，后来的研究任务是假如 Java 运行在 Docker 容器中, 以及作为 ECS 部署时，它所调用的 C++ 代码崩溃时应如何获得 core dump 进行问题分析。