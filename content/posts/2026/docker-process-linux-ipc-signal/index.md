---
title:  Docker 内进程如何响应 Linux 的信号
url: /docker-process-linux-ipc-signal/
date: 2026-03-07T23:49:20-06:00
featured: false
draft: false
type: post
toc: false
# menu: main
usePageBundles: true
thumbnail: "../images/logos/docker-logo.png"
categories:
  - Docker
tags: 
  - Docker
  - Linux
comment: true
codeMaxLines: 50
---

本文主要专注在 Docker 内的应用进程如何与外部发过来的 Linux 信号进行响应。具体应用在当运行为一个 ECS 的 Docker 容器时，对 ECS 的 AutoScaling
以及部署时如何让应用能正确收到相应的信号。关于 Linux 的 Signal 请参考 Wikipedia 的 [Signal(IPC)](https://en.wikipedia.org/wiki/Signal_(IPC)).

其实相关的话题在以前三篇中都有涉及

1. [如何向 Docker 容器传递参数](/pass-arguments-to-docker-container/)
2. [AWS Batch 是如何向 Docker 容器传递参数](/how-aws-batch-pass-parameters-to-docker-container/)
3. [Dockerfile 中命令的两种书写方式的区别](dockerfile-difference-between-shell-exec-forms/)

不同 Dockerfile 的 `ENTRYPOINT` 或 `CMD` 会影响到 Docker 内进程的启动方式，额外参数的接收方式，以及信号的传递。本篇只关注在信号这一议题。
先学习本地的 Docker 容器内与对 `docker stop` 或 `docker kill` 发出的信号的响应，进而讲述运行在 ECS 的容器与 ASG，部署行为的信号响应。
<!--more-->

### 理解 `docker stop` 和 `docker kill` 命令

开始话题之前先要了解停止 Docker 容器的两个命令 `docker stop <container>` 和 `docker kill <container>`, 它们的官方文档是

1. [docker container stop](https://docs.docker.com/reference/cli/docker/container/stop/)
2. [docker container kill](https://docs.docker.com/reference/cli/docker/container/kill/)

#### `docker stop` 说明

`docker stop` 先给容器内的主进程(PID 1) 发送 `SIGTERM` 信号，等待某个 Timeout 时间后，容器内主进程仍未退出的话，再发送 `SIGKILL`
信号强制终止进程, 关闭容器。 第一个信号默认为 `SIGTERM`, 但可以通过以下几种方式修改

1. `docker stop` 的 `-s` 或 `--signal` 参数指定
2. 创建容器时用 `--stop-signal` 参数指定，如 `docker run --stop-signal SIGHUP <image>`
3. 创建镜像是在 `Dockerfile` 中用 `STOPSIGNAL` 指定，如 `STOPSIGNAL SIGHUP`
   
默认的 Timeout 为 10 秒, Windows 容器默认为 30 秒，有两种方式修改该值. 作为 ECS Task 时默认的 stopTimeout 为 30 秒，最大为 120 秒。
 
1. `docker stop` 的 `-t` 或 `--time` 参数修改
2. 或是创建容器时用 `--stop-timeout` 参数指定, 如 `docker run --stop-timeout 15 <image>`
3. 并且 timeout 值为 `-1` 时表示永远等待，直到容器内主进程退出，意味着不会发下一个 `SIGKILL` 信号。最后可能要 `docker kill` 主动终止。

#### `docker kill` 说明

`docker kill` 只做 `docker stop` 的第 2 步，默认直接发送 `SIGKILL` 信号强制终止容器贩的主进程(PID 1)。相当于 Linux 命令的 `kill -9 <pid>`.
默认信号也可以修改，只能能过 `docker kill` 的 `-s` 或 `--signal` 参数指定，如 `docker kill -s SIGHUP <container>`

#### 信号的表示与容器主进程(PID 1)

`docker stop` 或 `docker kill` 可以用三形式的表示方式，由 `trap -l` 显示出来的数值, 信号名或带前缀 `SIG` 的信号名。
以 `17) SIGSTOP` 为例, 下面的命令等效

1. `docker stop -s 17 <container>`
2. `docker kill --signal SIGSTOP <container>`
3. `docker stop -s STOP <container>`

向容器发送的信号会转发到容器内的主进程，即 PID 1 的进程，也就是 `ENTRYPOINT` 或 `CMD` 主指令对应的进程. 这时候就要注意各种写法了，
数组的方式一般没问题。 若是 shell 的方式，如在 Dockerfile 中写成

```Dockerfile
ENTRYPOINT /hello
```

<s>实际上容器内主进程为 `/bin/sh -c "/hello"`, 信号会发送到主进程 `/bin/sh`，但却不会转发到 `/hello` 子进程上。</s>
(可能与 Docker 的版本有别，我所测试用的 Docker 服务端与客户端都是 29.2.1, `/bin/sh` 会继续转发信号到 `/hello` 进程).
假如你的应用程序需要关心外部通过 `docker stop` 或 `docker kill` 发过来的信号，尤其要注意 `ENTRYPOINT` 和 `CMD` 的写法。
或者多做镜像与容器的 `inspect` 来确认容器内的主进程是什么。 也可用 `tini` 来转发容器外部发来的信号。

为什么更进一步了解 `docker stop` 和 `docker kill` 的具体行为呢？因为这有助于我们将来理解容器外头，比如是 ECS 的 AutoScaling 是如何对容器进行斩杀的。

### 实际测试容器主进程接收到信号

还是以上一文中的 C 代码 test-sig.c 为例

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

用 Linux 的 gcc, 如用镜像 gcc:15 编译生成 test-sig 可执行文件

```bash
gcc test-sig.c -o test-sig
```
创建一个 Dockerfile

```dockerfile
FROM busybox
COPY test-sig /test-sig
ENTRYPOINT ["/test-sig"]
```

构建镜像并运行容器

```bash
docker build -t test-sig .
docker run -it test-sig
PID: 1
```

没问题，主进程为 `/test-sig`, 它的进程为 1, 假设用 `docker ps` 找到该容器 ID, 并存为环境变量 `CID` 中。下面对该容器发送信号。

```shell
docker kill -s HUP $CID
docker kill -s 30 $CID
docker kill -s TERM $CID
docker stop -s HUP $CID
```

Docker 容器端的输出为

```shell
bash-3.2$ docker run -it test-sig
PID: 1
received signal: 1: Hangup
received signal: 30: Power failure
received signal: 15: Terminated
^Creceived signal: 2: Interrupt
^Zreceived signal: 20: Stopped
received signal: 1: Hangup
bash-3.2$
```

`docker stop` 的 `TERM` 不会让容器退出，但是紧续其他的 `SIGKILL` 就不客气了，而且 `SIGKILL` 直接被内核处理，终止进程，无需通知主进程。

把 `ENTRYPOINT` 换成 shell 的方式

```dockerfile
ENTRYPOINT "/test-sig"
```

重新构建镜像，假如 tag 为 1，现在启动

```shell
docker build -t test-sig:1 .
bash-3.2$ docker run -it test-sig:1
PID: 1
^Creceived signal: 2: Interrupt
```

`/test-sig` 进程 PID 还是 1, 按下 `ctrl+c` 后 `/test-sig` 可以接收到信号号，说明在 [docker container kill](https://docs.docker.com/reference/cli/docker/container/kill/)
中的描述有误。可能与 Docker 的版本有别，我所测试用的 Docker 服务端与客户端都是 29.2.1, API 版本是 1.53.

用 `docker inspect <container-id>` 查看到容器实际的 ENTRYPOINT 如下

```json
   "Entrypoint": [
       "/bin/sh",
       "-c",
       "\"/test-sig\""
   ]
```

到容器内后看到的进程是

```shell
bash-3.2$ docker exec -it 95 sh
/ # ps
PID   USER     TIME  COMMAND
    1 root      0:00 /test-sig
   13 root      0:00 sh
```

#### 引入 `tini` 管理进程

tini 是一个极简的专为容器设计的 init 进程，它作为容器中主进程(PID 1), 有两个特殊职责：1) 转发信号到其负责的进程, 2) 回收僵尸进程.
当然我们需要在 Dockerfile 中用与基础镜像对应的命令来安装 tini

对于 `busybox`, 相应的 Dockerfile 为

```dockerfile
FROM busybox

ADD https://github.com/krallin/tini/releases/download/v0.19.0/tini-static /tini
RUN chmod +x /tini
COPY test-sig /test-sig

ENTRYPOINT ["/tini", "--", "/test-sig"]
```

构建并运行

```shell
docker build -t test-sig:2 .
docker run -it test-sig:2
PID: 7
^Creceived signal: 2: Interrupt
```

虽然 `/test-sig` 的 PID 不是 1, 但它能正确接收到信号了。

现在容器的 COMMAND 是 `/tini -- /test-sig`, 容器里进程为

```shell
/ # ps
PID   USER     TIME  COMMAND
    1 root      0:00 /tini -- /test-sig
    7 root      0:00 /test-sig
```

`tini` 作为主进程后，它会把收到的信号发送到它的子进程上, 如果是 ECS Task 把 Task Definition 中的 `initProcessEnabled` 配置为 `true`

```json
"linuxParameters": {  
  "initProcessEnabled": true  
}
```

即使是正常写的 `ENTRYPOINT` 为

```dockerfile
ENTRYPOINT ["java", "-jar", "/app.jar" ]
```

ECS 任务容器启动后内容进程是下面那样子的

```shell
sh-4.2# ps -aux
USER         PID %CPU %MEM    VSZ   RSS TTY      STAT START   TIME COMMAND
root           1  0.0  0.0   1112     4 ?        Ss   15:50   0:00 /sbin/docker-init -- java -jar /app.jar
root           7 14.8  3.9 27671424 1283536 ?    Sl   15:50   0:11 java -jar /app.jar
```

所以 `/sbin/docker-init` 也会把收到的信号转发到子进程(PID 7)。`/sbin/docker-init` 实质上就是  `tini`, 后面还会讲到。

### 实战 Java 应用的 Docker 容器

对于通常的 Java 应用，多是用 ShutdownHook 来处理关闭前的事宜, 它可以捕捉到 `SIGTERM` 和 `SIGINT`(ctrl+c 产生) 信号. Java 收到
`SIGTERM` 或 `SIGINT` 会从当前代码出退出，并执行 ShutdownHook 中注册的(如果有的话)线程来做一些清理工作。 比如 `docker stop` 
的时候可最后作些清理工作， 如释放资源，缓存落盘，打个日志，发个通知什么。程序正常退出时也会触发的 ShutdownHook.

*除非用低级别的 `sun.misc.Signal` 来注册一些其他的信号， 如此方可用 `docker kill -s USR1` 这样的方式在外部与容器内主进程直接通信，
这样做其实是有实际应用场景的，比如约定收到某个类型的信号，执行某个操作，但不退出，就如 `kill -3` 所触发打印线程栈这样的操作，
这可以避免每次外部需要与 JVM 通信时都不得不创建一个特定的 `API`*.

像现代的 SpringBoot 应用，最后一般是打包成 `jar` 包，再用  `java -jar fat.jar` 的方式启动，下面用一个类来模拟这样的方式

*SimWeb.java*

```java
import java.util.Arrays;

public class SimWeb {
    static {
        Runtime.getRuntime().addShutdownHook(new Thread(() -> {
            System.out.println("start execute shutdown hook");
            try {
                Thread.sleep(2000);
            } catch (InterruptedException ignored) {
            }
            System.out.println("finished execute shutdown hook");
        }));
    }

    public static void main(String[] args) throws InterruptedException {
        System.out.println("args:" + Arrays.toString(args));
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

编译并打成 `jar` 包

```shell
javac SimWeb.java
jar cfe app.jar SimWeb SimWeb.class
```

这样就生成了一个可执行的 `app.jar` 包。

下面尝试几种 Docker 镜像的 `ENTRYPOINT`

#### ENTRYPOINT ["java", "-jar", "/app.jar"]

```Dockerfile
FROM amazoncorretto:21-al2023
COPY app.jar /app.jar
ENTRYPOINT ["java", "-jar", "/app.jar"]
```

这是非常标准的方式，`java` 进程将会是 PID 1, 但是无法使用非标准的 `JAVA_OPTS` 环境变量传来的 JVM 参数。我们更应该用 `JAVA_TOOL_OPTIONS`
(since JDK 5) 或 `JDK_JAVA_OPTIONS`(since JDK 9) 作为环境变量配置 JVM 参数，随后的 `java` 命令将会自动使用它们相应的设置。

下面在启动容器的控制台中使用用 `ctrl+c` 发送 `SIGINT` 信号进行快速测试，它与用 `docker kill -s TERM` 有同样的效果，因为 Java 进程在收到
`SIGTERM` 或 `SIGINT` 会作出相同的反应。

```shell
bash-3.2$ docker run -it simweb:1 arg1 arg2
args:[arg1, arg2]
service is running ...
PID: 1
service is running ...
service is running ...
^Cstart execute shutdown hook
service is running ...
service is running ...
service is running ...
service is running ...
finished execute shutdown hook
bash-3.2$
```

符合预期，Java 是主进程(PID 1), 在容器内

```shell
sh-5.2# ps -aux
USER       PID %CPU %MEM    VSZ   RSS TTY      STAT START   TIME COMMAND
root         1  1.2  0.8 5135376 68632 pts/0   Ssl+ 21:20   0:00 java -jar /app.jar arg1 arg2
```

#### ENTRYPOINT ["sh", "-c", "java $JAVA_OPTS -jar /app.jar \"$@\"", "--"]

`ENTRYPOINT` 换成

```dockerfile
ENTRYPOINT ["sh", "-c", "java $JAVA_OPTS -jar /app.jar \"$@\"", "--"]
```

这样的一个好处是可以使用环境变量 $JAVA_OPTS 作为 JVM 参数，有 `JAVA_TOOL_OPTIONS` 或 `JDK_JAVA_OPTIONS` 也算得是什么特别的优点。

执行新构建出的镜像 simweb:2, 效果

```shell
bash-3.2$ docker run -it simweb:2 arg1 arg2
args:[arg1, arg2]
service is running ...
PID: 1
service is running ...
service is running ...
^Cstart execute shutdown hook
service is running ...
service is running ...
service is running ...
service is running ...
finished execute shutdown hook
```
虽然 `java` 进程仍然是 PID 1, 它不是 `sh` 的子进程吗？ 查看容器内容的进程

```shell
sh-5.2# ps -aux
USER       PID %CPU %MEM    VSZ   RSS TTY      STAT START   TIME COMMAND
root         1  1.7  0.8 5135536 66616 pts/0   Ssl+ 21:25   0:00 java -jar /app.jar arg1 arg2
```

`ENTRYPOINT ["sh", "-c", "java $JAVA_OPTS -jar /app.jar \"$@\"", "--"]` 和 `ENTRYPOINT ["java", "-jar", "/app.jar"]` 
的效果原来是一样的，"sh" 只承担了解析环境变量的功能后，就没它什么事了。

#### ENTRYPOINT java $JAVA_OPTS -jar /app.jar "$0" "$@"

再试下 Dockerfile 中换成

```dockerfile
ENTRYPOINT java $JAVA_OPTS -jar /app.jar "$0" "$@"
```

运行构建出 simweb:3, 启动容器

```shell
bash-3.2$ docker run -it simweb:3 arg1 arg2
args:[arg1, arg2]
service is running ...
PID: 1
service is running ...
service is running ...
^Cstart execute shutdown hook
service is running ...
service is running ...
service is running ...
service is running ...
service is running ...
finished execute shutdown hook
```

Java 仍然为 PID 1, 能正常处理信号。 无论是 `docker inspect simweb:3` 看到镜像还是 `docker inspect <container-id>` 看到容器的
`ENTRYPOINT` 都是

```json
"Entrypoint": [
    "/bin/sh",
    "-c",
    "java $JAVA_OPTS -jar /app.jar \"$0\" \"$@\""
]
```

可是观察容器内部的进程却没有了 `/bin/sh` 的影子, Java 进程直接脱去了 `/bin/sh` 的外衣，成为了 PID 1, 这很理想，但与官方
[docker container kill](https://docs.docker.com/reference/cli/docker/container/kill/) 文档中描述不相符，只能接受现实，但在不同 Docker 版本情况下需多加留意。

```shell
sh-5.2# ps -aux
USER       PID %CPU %MEM    VSZ   RSS TTY      STAT START   TIME COMMAND
root         1  6.8  0.8 5135472 66264 pts/0   Ssl+ 21:34   0:00 java -jar /app.jar arg1 arg2
```

最后测试下来，前面的三种 ENTRYPOINT 写法并没有本质上的区别，它们最后都能使得 Java 成为容器内的主进程(PID 1). 因此下面的 ENTRYPOINT 效果一致：

1. ENTRYPOINT ["java", "-jar", "/app.jar"]
2. ENTRYPOINT ["sh", "-c", "java $JAVA_OPTS -jar /app.jar \"$@\"", "--"]
3. ENTRYPOINT java $JAVA_OPTS -jar /app.jar "$0" "$@"

#1 因为没有 `sh`, 所以不能解析环境变量 $JAVA_OPTS, 但标准的 `JAVA_TOOL_OPTIONS` 或 `JDK_TOOL_OPTIONS` 完全可以替代, 本人认为是更推荐的方式。
三种方式接收参数的方式都没问题，比如用 `CMD` 传送参数的方式。#3 `shell` 的方式使用时需保持谨慎。

###  测试用 `tini` 来管理 Java 进程

`tini` 的功能前面有简单介绍, 在 `amazonlinux:2023` 中安装方式有所不同。

```dockerfile
RUN curl -fsSL https://github.com/krallin/tini/releases/download/v0.19.0/tini-amd64 \
    -o /usr/bin/tini && chmod +x /usr/bin/tini
```

然后以上三种 ENTRYPOINT 的写法分别为

1. ENTRYPOINT ["/usr/bin/tini", "--", "java", "-jar", "/app.jar"]
2. ENTRYPOINT ["/usr/bin/tini", "--", "sh", "-c", "java $JAVA_OPTS -jar /app.jar \"$0\" \"$@\""]
3. ENTRYPOINT ["/usr/bin/tini", "--", "sh", "-c", "java $JAVA_OPTS -jar /app.jar \"$0\" \"$@\""]

注意到 #2 和 #3 是一样的，也就是说采用  tini + array 的方式，也就是两种写法。假如以上分别构建为镜像 simweb:tini-1, simweb:tini-2.

下面来观察它们对信号的反应，以及在容器中的进程长什么样. 首先它们的外在表现都是一样的，Java 进程(PID 7) 能接收到外部通过主进程 tini(PID 1) 
转发过来的信号。所以执行效果 wibweb:tini-1, wibweb:tini-2 是完全一样的, 都是

```shell
bash-3.2$ docker run -it simweb:tini-1 arg1 arg2
args:[arg1, arg2]
service is running ...
PID: 7
service is running ...
service is running ...
^Cstart execute shutdown hook
service is running ...
service is running ...
service is running ...
service is running ...
finished execute shutdown hook
```

只看下容器内部进程的样式，分别是

simweb:tini-1:

```shell
sh-5.2# ps -aux
USER       PID %CPU %MEM    VSZ   RSS TTY      STAT START   TIME COMMAND
root         1  0.0  0.0 289628  4452 pts/0    Ss   21:59   0:00 /usr/bin/tini -- java -jar /app.jar
root         7  1.3  0.8 5135308 68496 pts/0   Sl+  21:59   0:00 java -jar /app.jar arg1 arg2
```

simweb:tini-2:

```shell
sh-5.2# ps -aux
USER       PID %CPU %MEM    VSZ   RSS TTY      STAT START   TIME COMMAND
root         1  0.0  0.0 289628  4452 pts/0    Ss   22:04   0:00 /usr/bin/tini -- sh -c java $JAVA_OPTS -jar /app.jar "$0" "$@" arg1 arg2
root         7  1.5  0.8 5135332 68464 pts/0   Sl+  22:04   0:00 java -jar /app.jar arg1 arg2
```

### ECS 中运行的 Docker 容器

从前面我们明白，以上三种 `ENTRYPOINT` 写法，没有 `tini` 的情况下，Java 为主进程(PID 1), 有 `tini` 的情况下，Java 进程(PID 7) 作为子进程，
由 `tini` 进程(PID 1) 管理。所以，Java 进程(PID 7) 能接收到外部通过 tini(PID 1) 转发过来的信号。

在 ECS 中运行的 Docker 容器，只要在 Task Definition 中配置了 `initProcessEnabled` 为 `true` 的话，

```json
"linuxParameters": {  
  "initProcessEnabled": true  
}
```

ECS 会自动为 Java 进程引入类似于 `/sbin/docker-init` 来管理 Java 进程，并为之转发外部来的 Linux 信号, 其实 `/sbin/docker-init`
就是 `tini`. 只不过是被重命了名。

而设置了 `"initProcessEnabled": true` 就相当于是 `docker run` 添加了 `--init` 参数。

所以我们加上 `--init` 重新试下前面的 `simweb:1`, `simweb:2`, 和 `simweb:3`, 执行行为都是一样的，相应的 `/sbin/docker-init` 进程为 PID 1,
Java 进程为 PID 7, Java 能正确接收到外部发来的信号。

```shell
bash-3.2$ docker run -it --init simweb:1 arg1 arg2
args:[arg1, arg2]
service is running ...
PID: 7
service is running ...
^Cstart execute shutdown hook
service is running ...
service is running ...
service is running ...
service is running ...
finished execute shutdown hook
bash-3.2$
```

重新审视它们在容器中的进程

simweb:1: ENTRYPOINT ["java", "-jar", "/app.jar"]

```shell
sh-5.2# ps -aux
USER       PID %CPU %MEM    VSZ   RSS TTY      STAT START   TIME COMMAND
root         1  0.0  0.0    864   468 pts/0    Ss   22:20   0:00 /sbin/docker-init -- java -jar /app.jar arg1 arg2
root         7  0.8  0.9 5135392 74828 pts/0   Sl+  22:20   0:00 java -jar /app.jar arg1 arg2
```

simweb2: ENTRYPOINT ["sh", "-c", "java $JAVA_OPTS -jar /app.jar \"$@\"", "--"]

```shell
sh-5.2# ps -aux
USER       PID %CPU %MEM    VSZ   RSS TTY      STAT START   TIME COMMAND
root         1  0.0  0.0    864   464 pts/0    Ss   22:23   0:00 /sbin/docker-init -- sh -c java $JAVA_OPTS -jar /app.jar "$@" -- arg1 arg2
root         7  2.5  0.8 5135268 66344 pts/0   Sl+  22:23   0:00 java -jar /app.jar arg1 arg2
```

simweb3: ENTRYPOINT java $JAVA_OPTS -jar /app.jar "$0" "$@"

```shell
sh-5.2# ps -aux
USER       PID %CPU %MEM    VSZ   RSS TTY      STAT START   TIME COMMAND
root         1  0.0  0.0    864   464 pts/0    Ss   22:24   0:00 /sbin/docker-init -- /bin/sh -c java $JAVA_OPTS -jar /app.jar "$0" "
root         7  3.7  0.8 5135252 66192 pts/0   Sl+  22:24   0:00 java -jar /app.jar arg1 arg2
```

关于 Docker 内的主进程，或是由主进程 `tini`(PID 1) 管理的子进程(PID 7) 能如何响应外部发来的 Linux 信号，我们已经做了比较全面的测试，
结论是无论是 `ENTRYPOINT` 的哪种写法，还是是否引入 `tini` 来管理进程，自己的应用进程总能正确收到外部发来的信号。

罗嗦了前面一大堆， 关于讲述运行在 ECS 的容器与 ASG，部署行为的信号响应还要继续写一篇了，敬请期待。