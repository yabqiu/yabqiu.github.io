---
title: "C 语言，Bash, Java 如何响应 Linux Signal" 
url: /c-bash-java-react-to-ipc-signal/
date: 2026-03-06T05:04:20-06:00
featured: false
draft: false
type: post
toc: false
# menu: main
usePageBundles: true
thumbnail: "../images/logos/linux-logo.png"
categories:
  - Linux
tags: 
  - Java
  - Linux
comment: true
codeMaxLines: 30
---

本文初衷是要专注在 Docker 内的应用进程如何与外部发过来的 Linux 信号进行响应。具体应用在当运行为一个 ECS 的 Docker 容器时，对 ECS 的 AutoScaling
以及部署时如何让应用能正确收到相应的信号。可是一提及这一话题，思维就产生了极大的发散，很想好好捋一捋进程与 Linux 信号之间关系和相关的概念。
关于 Linux 的 Signal 请参考 Wikipedia 的 [Signal(IPC)](https://en.wikipedia.org/wiki/Signal_(IPC)).

本文以渐进的方式，从信号的简介，本地 C, Bash, Java 程序与信号，然后将在下一篇学习本地 Docker 容器内进程与信号的处理，ECS 的容器与 ASG，部署行为的信号响应。
<!--more-->

### 关于 Linux/Unix 的信号

这里所说的信号，就是在 Linux 用 `kill -l` 或 `trap -l` 能列出的那些信号，就是像我们对一个进程的 `kill`, `ctrl+c`, `ctrl+z` 产生的信号。
以 Bash 的 `trap -l` 列表为例，其中列出所有的信号

```shell
bash-3.2$ trap -l
 1) SIGHUP	 2) SIGINT	 3) SIGQUIT	 4) SIGILL
 5) SIGTRAP	 6) SIGABRT	 7) SIGEMT	 8) SIGFPE
 9) SIGKILL	10) SIGBUS	11) SIGSEGV	12) SIGSYS
13) SIGPIPE	14) SIGALRM	15) SIGTERM	16) SIGURG
17) SIGSTOP	18) SIGTSTP	19) SIGCONT	20) SIGCHLD
21) SIGTTIN	22) SIGTTOU	23) SIGIO	24) SIGXCPU
25) SIGXFSZ	26) SIGVTALRM	27) SIGPROF	28) SIGWINCH
29) SIGINFO	30) SIGUSR1	31) SIGUSR2
```
每个信号都有一个编译，例如我们用 `kill <pid>` 会向应用程序发送 `SIGTERM` 信号。`kill -9 <pid>` 发送 `SIGKILL`, 该信号直接被内核处理掉。
`ctrl+c` 产生的是 `SIGINT` 中断信号，`ctrl+z` 产生 `SIGTSTP` 信号。程序正常退出(exit(0)) 不会产生任何信号。

更多 `kill` 发送信号的方式

```shell
kill -15 <pid>
kill -s SIGINT <pid>
kill -ABRT <pid>
```

### C 语言处理信号

前一节讲了如何用键盘和 `kill` 命令发送信号，那么进程在收到信号之后，能否识别是何种类型的信号，并作出相应的反应。用低一级别的汇编或 C 
语言应对信号的能力肯定要强, 下面以 C 为例，只捕获收到的信号并输出信号 ID 及相应的名称。

test-sig.c

```c
#include <stdio.h>
#include <signal.h>
#include <unistd.h>
#include <string.h>

void handler(int signum) {
    printf("received signal: %d: %s\n", signum, strsignal(signum));
}

int main() {
    for (int i = 1; i < 32; i++) {
        signal(i, handler);  // register handler for signal 1~31
    }

    printf("PID: %d\n", getpid());
    while (1) pause();
}
```

编译并运行

```shell
gcc -o test-sig test-sig.c
./test-sig
PID: 28095
```

测试它，在另一个终端执行

```shell
export PID=28095
kill -TERM $PID
kill $PID
kill -s SIGHUP $PID
kill -30 $PID
kill -2 $PID
```

然后在 `./test-sig` 所在终端按依次按 `ctrl+c` 和 `ctrl+z`, 最后用 `kill -9 $PID` 强制结束它. 在 `./test-sig` 终端的输出为

```text
received signal: 15: Terminated: 15
received signal: 15: Terminated: 15
received signal: 1: Hangup: 1
received signal: 30: User defined signal 1: 30
received signal: 2: Interrupt: 2
^Creceived signal: 2: Interrupt: 2
^Zreceived signal: 18: Suspended: 18
Killed: 9
```

### trap 命令处理信号

Linux 的 `trap` 命令也能对部分信号进行监听处理，用法为

```shell
trap <命令> <空格分开的信号列表，名称或数字>
```

如 test.sh

```shell
trap 'echo "received signal: SIGUP"' SIGHUP

cleanup() {
    echo "do cleanup"
    exit 0
}

trap cleanup SIGTERM SIGINT 18

echo "PID: $$"

while true; do
    sleep 1
done
```

执行 `bash test.sh` 产生的 PID 是 32799. 然后执行

```shell
kill -HUP 32799
kill -18 32799
```

就能看到 `bash test.sh` 的输出

```shell
bash-3.2$ bash a.sh
PID: 32799
received signal: SIGUP
do cleanup
```

但是 `ctrl+z` 却无法结束它，并且之后再用 `kill -18 <pid>` 也不能结束。`trap - SIGTERM` 还能恢复针对该信号的默认行为。

### Java 处理信号

Java 离操作系统还隔着一层虚拟机，所以它接收处理信号的能力较弱。高层的 API 接口只有一个，而且还不能分辨是哪种类型的信号，只知收到了信号

```java
Runtime.getRuntime().addShutdownHook(new Thread(() -> {
    System.out.println("Received signal");
}));
```

下面用一段 Java 代码来演示运行着服务，并注册了 ShutdownHook, 测试它能捕获到什么信号

Test.java

```java
public class Test {
    static {
        Runtime.getRuntime().addShutdownHook(new Thread(() -> {
            System.out.println("start execute shutdown hook");
            try { Thread.sleep(3000); } catch (InterruptedException ignored) {}
            System.out.println("finished execute shutdown hook");
        }));
    }

    public static void main(String[] args) throws InterruptedException {
        Thread worker = new Thread(() -> {
            while (true) {
                try {
                    System.out.println("service is running ...");
                    Thread.sleep(500);
                } catch (InterruptedException e) {
                    System.out.println("service interrupted！");
                    return;
                }
            }
        });
        worker.start();

        System.out.println("PID: " + ProcessHandle.current().pid());
        Thread.sleep(5 * 60 * 1000);
        System.out.println("main thread end");
    }
}
```

启动它

```shell
java Test.java
service is running ...
PID: 42209
service is running ...
service is running ...
......
```

打印出进程 ID, 并持续输出模拟服务的运行。

#### 测试 `ctrl+c`

```shell
^Cstart execute shutdown hook
service is running ...
service is running ...
service is running ...
service is running ...
service is running ...
service is running ...
finished execute shutdown hook
bash-3.2$
```
主线程从当前位置结束，信号可被捕获到并成功退出程序，ShutdownHook 执行期间服务仍然运行，至到 ShutdownHook 结束并终止程序。

#### 测试 `kill <pid>`

行为与 `ctrl+c` 一致，除了在控制台中不会看到 `^C` 之外。

#### 测试 `kill -9 <pid>`

```text
service is running ...
service is running ...
Killed: 9
```

直接退出进程，ShutdownHook 得不到执行。

#### 测试程序正常结束

因为主线程是 Sleep 五分钟，观察五分钟后主线程结束时的输出。

```shell
service is running ...
service is running ...
main thread end
service is running ...
service is running ...
........
```

子线程一直在运行，程序得不到退出，所以也不会触发 ShutdownHook. 原因是子线程的 Daemon 为 false. 代码中加上

```java
worker.setDaemon(true);
```

再自然运行五分钟，这时候的输出为

```shell
service is running ...
service is running ...
main thread end
start execute shutdown hook
service is running ...
service is running ...
service is running ...
service is running ...
service is running ...
service is running ...
finished execute shutdown hook
bash-3.2$
```

非守护线程会阻止进程的退出，主线程结束，在没有非守护子线程时触发 ShutdownHook 捕捉到，ShutdownHook 执行期间，子线程仍然活跃。

#### `kill -s <pid>` SIGQUIT

对 Java 进程执行 `kill -s <pid>` 只会在控制台打印线程栈信息，不管是否注册了 ShutdownHook. 并且 `kill -3` 的 `SIGQUIT` 信号不会被
ShutdownHook 捕获到, 更不会让程序退出。

小结一下，Java 的 ShutdownHook 可以捕获到的信号只有 `SIGTERM`，`SIGINT`，或是在程序自然退出时触发。`SIGTERM`, `SIGINT` 的触发过程是

1. 让主线程在当前位置终止
2. 转而开始执行 ShutdownHook
3. ShudownHook 执行期，子线程服务仍在运行
4. ShudownHook 结束前，子线程服务结束
5. ShutdownHook 也退出
6. 整个进程退出

`SIGTERM` 和 `SIGINT` 也在乎其他子线程是否是守护线程，保证要退出程序。

另外在 ShutdownHook 本身是一个线程，如果在其中再启动子线程的话，还是能够得到执行，无论其是否 Daemon.

```java
    static {
    Runtime.getRuntime().addShutdownHook(new Thread(() -> {
        System.out.println("start execute shutdown hook");
        try { Thread.sleep(3000); } catch (InterruptedException ignored) {}
        System.out.println("finished execute shutdown hook");

        var thread = new Thread(()->{
            System.out.println("xxxxxxxx");
        });
        thread.setDaemon(true); // 有无这行都没关系
        thread.start();
    }));
}
```

用 `ctrl+c` 的终止效果为

```text
service is running ...
service is running ...
^Cstart execute shutdown hook
service is running ...
service is running ...
service is running ...
service is running ...
service is running ...
service is running ...
finished execute shutdown hook
xxxxxxxx
bash-3.2$
```

Java 的 ShutdownHook 也没有超时，如果在其中做太多事情，退不出来的话程序就卡在 ShutdownHook 当中，一般来说只在 ShutdownHook 做些清理操作。

对于 SpringBoot 应用，有一个属性

```text
spring.lifecycle.timeout-per-shutdown-phase: 30s
```

并且它的默认值就是 30 秒，也就是你的 ShutdownHook 执行超过 30s 的话，程序将被强行终止。

#### Java 低级别的 Signal API

下面的 Java 代码使用了自  1.2 引入的 `sun.misc.Signal`，它能支持的信号基本就这些。

```java
import sun.misc.Signal;

public class JSignal {

    public static void main(String[] args) throws Exception {
        String[] signals = new String[]{"HUP", "INT", "TRAP", "ABRT", "EMT", "SYS",
            "PIPE", "ALRM", "TERM", "URG", "TSTP", "CONT", "CHLD", "TTIN", "TTOU", 
            "IO", "XCPU", "XFSZ", "VTALRM", "PROF", "WINCH", "INFO", "USR1", "USR2"};

        for (String signal : signals) {
            Signal.handle(new Signal(signal), signalName -> {
                System.out.println("received " + signalName);
            });
        }

        System.in.read();
    }
}
```

对于不能注册的信号，启动时会提示错误，例如

```text
Exception in thread "main" java.lang.IllegalArgumentException: Signal already used by VM or OS: SIGSEGV
	at java.base/jdk.internal.misc.Signal.handle(Signal.java:172)
	at jdk.unsupported/sun.misc.Signal.handle(Signal.java:157)
```

上面代码的执行行为与最前面的 C 代码差不多。

执行端

```shell
bash-3.2$ java JSignal.java
^Creceived SIGINT
^Zreceived SIGTSTP
received SIGHUP
received SIGUSR1
received SIGTERM
Killed: 9
bash-3.2$
```

以上信息是由 `ctrl+c`, `ctrl+z` 产生的 `SIGINT` 和 `SIGTSTP` 和下面 `kill` 命令造成的

```shell
kill -1 59838
kill -30 59838
kill  59838
kill -9 59838
```

假定 `java JSignal.java` 的进程 ID 是 59838.