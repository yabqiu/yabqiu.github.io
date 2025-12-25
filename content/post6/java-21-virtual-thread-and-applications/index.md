---
title: Java 21 之虚拟线程深入学习及应用场景
url: /java-21-virtual-thread-and-applications/
date: 2025-12-25T15:00:00-05:00
featured: false
type: post
draft: false
toc: false
# menu: main
usePageBundles: true
thumbnail: "../images/logos/java-logo.png"
categories:
  - java
  - new features
tags:
  - virtual threads
  - thread
  - java 21
comment: true
codeMaxLines: 50
# additional
wpPostId: 14464 
wpStatus: draft
views: 
lastmod: 2025-11-02T14:59:01-06:00
---

篇首说明: 本文十分冗长, 语言组织混乱, 如果觉得 TLDR, 就直接跳到 [关于虚拟线程的总结](#conclusion) 部分看要点,
若对总结上中的某些要素点仍有兴趣的话请倒查本文中其他部分的内容. 个人对 Java 虚拟线程的主动研究是为了在项目中更有效的使用它.

### 关于线程的概要

Java 21 于两年前 2023 年 9 月份放出，它是一个 LTS(long term support) 版本，个人基本就是把 LTS 当作能在正式项目中使用的版本。 Java
21 有几个增进编程体验的特性，像 Sequenced Collections, Record Patterns, 和 Pattern Matching for switch, 而对于性能改进的，
也是 Java 21 最具代表的特性无疑就是 Virtual Threads -- 虚拟线程。本文单列出它来，着重感受一下虚拟线程是什么，以及我们应该如何使用它。

其实在之前的 [Java 19, 20 新特性学习](/java-19-20-new-features/) 就有一定的笔墨介绍了于 Java 19 引入，
Java 20 中尚处于第二次预览的虚拟线程。于其中大致体验了在一台 36 G 物理内存，默认堆内存为 9 G 的情况下，
创建 9000 个线程没问题，但要创建 10000 个线程就 OutOfMemoryError 了。而相同的环境下创建一百万个虚拟线程都没问题，没在继续往下试探了。

其实这种比较是没有意义的, Java 线程对应到平台线程的, 每个线程要至少实实在在的 2M 栈空间, 而一百万个虚拟线程相当于是创建了一百万个 Java 
对象而已, 更像是相应数量的 Task, 实际运行时才由载体线程去调度执行 - (注: 后面所提到的载体线程和平台线程是同一个概念).

重新回顾一下何谓虚拟线程，Java 的虚拟线程实现是来自于 Project Loom 项目。与此相关的概念有线程，协程，以及纤程(Fiber)，而虚拟线程对应的应该是纤程。

1. 线程是操作系统最小的调度单位，每个线程有独立较大的栈空间(比如 2M)，内核调度，切换开销大，可有效使用 CPU 多核
2. 协程在单个线程内执行，共享线程栈空间或独立小空间，用户态调度，切换开销极小，但无法使用多核
3. 纤程，介于线程与协程之间，很小的独立栈，用户态调度，切换开销较小。结合线程池，纤程可在线程间转移，这时岂不是要经内核态调度吗？

<!--more-->

不管我们使用何种方式处理任务，一定要清楚任务是 CPU 还是 IO 密集型的，如果是 CPU 密集型基本上任务数超过 CPU 内核数，性能可能反而下降，
甚至要控制比 CPU 内核数还要小的并发规模。而 IO 密集型的任务就必须有效使用线程了，或者多一份心思考虑是否要使用虚拟线程。

从网上一些关于线程与虚拟线程的对比测试，仿佛性能改进不大，大概是测试程序中并没有让虚拟线程做它擅长的事情，或者线程独立栈空间，
或线程切换对测试程序的性能影响不大。我们在此不先对那些测试数据进行任何评判，而是先从另外一些角度来感受一下 Java 21 中虚拟线程的内在表现.

### 虚拟线程的使用

线程在现代编程中每天都要接触，既然线程是由系统调度的，那么虚拟线程肯定要依附于某个线程而存在的。比如新建一个线程，线程有自己的名称

```java
new Thread(() -> {
    System.out.println("Current thread: " + Thread.currentThread());
}).start();
```

打印出的线程名称大约是

>current thread: Thread[#21,Thread-0,5,main]

如果是一个虚拟线程呢？

```java
Thread.ofVirtual().start(() -> {
    System.out.println("Current thread: " + Thread.currentThread());
}).join();
```

得到的输出为

```text
Current thread: VirtualThread[#21]/runnable@ForkJoinPool-1-worker-1
```
可以看到该虚拟线程的编号为 VirtualThread[#21], 以及它的载体是一个 ForkJoinPool 线程池的一个线程。直接的理解就是该虚拟线程名为这个整体.

以上的代码也可写成

```java
Thread.startVirtualThread(() -> {
    System.out.println("Current thread: " + Thread.currentThread());
}).join();
```

首先有一个问题，为什么一定要马上连接一个  `join()` 调用，如果没有该调用我们将看不到任何输出，因为载体线程 ForkJoinPool 
中的线程 isDaemon() 是 true, 所以随着主线程的退出，子线程立即退出，其中的代码来不及执行，除非我们让主线程在外头等候， join() 是一种等候方式。

从这里我们也学到了启动虚拟线程的两种方式

1. Thread.ofVirtual().start(Runnable)
2. Thread.startVirtualThread(Runnable)

Thread 的这两个方法都要求创建虚拟线程后立即启动，要像普通线程

```java
Thread thread = new Thread(runnable);
thread.start()
```

创建再启动的话，可以这样调用 `unstarted(runnable)` 方法

```java
Thread virtualThread = Thread.ofVirtual().unstarted(() -> {
    System.out.println(Thread.currentThread().isVirtual());
    System.out.println("Current thread: " + Thread.currentThread());
});

virtualThread.start();
virtualThread.join();
```

注意到 Thread 新加了 isVirtual() 判断是否是虚拟线程

Thread.ofVirtual() 和 Thread.startVirtualThread() 都会用到 ThreadBuilders 来创建虚拟线程。与 Thread.ofVirtual() 
相对应的方法是 Thread.ofPlatform(), 即普通的与操作系统对应的线程。'

而 ThreadBuilders 最后创建虚拟线程时都会调用到

```java
new VirtualThread(scheduler, name, characteristics, task)
```
再查看该构造函数

```java
VirtualThread(Executor scheduler, String name, int characteristics, Runnable task) {
    super(name, characteristics, /*bound*/ false);
    Objects.requireNonNull(task);

    // choose scheduler if not specified
    if (scheduler == null) {
        Thread parent = Thread.currentThread();
        if (parent instanceof VirtualThread vparent) {
            scheduler = vparent.scheduler;
        } else {
            scheduler = DEFAULT_SCHEDULER;
        }
    }

    this.scheduler = scheduler;
    this.cont = new VThreadContinuation(this, task);
    this.runContinuation = this::runContinuation;
}
```

比如前面的虚拟线程的例子，来到这个构造函数时相应的的参数分别为， scheduler: null, name: null, characteristics: 0, task: runnable

其中代码由主线程执行，所以 Thread parent = Thread.currentThread() 是主线程，而不是一个 VirtualThread, 因此会默认使用

```java
scheduler = DEFAULT_SCHEDULER; 
```

这是一个 ForkJoinPool


```java
private static final ForkJoinPool DEFAULT_SCHEDULER = createDefaultScheduler();
```

在 VirtualThread.createDefaultScheduler() 创建了一个默认大小为 CPU 内核数的线程池，可以通过以下系统属性改变相应的参数

```text
jdk.virtualThreadScheduler.parallelism
jdk.virtualThreadScheduler.maxPoolSize
jdk.virtualThreadScheduler.minRunnable
```

从该方法中也可看到 CarrierThread 的概念，即载体线程。上面 scheduler: Executor 的概念和载体线程也是一个意思, 即虚拟线程任务会被 Schedule
到哪个平台线程池执行. 如果要使用自定义的平台线程池, 突破口就是要用自定义的 ThreadFactory.

我当前电脑的 CPU 内核数为 12，最终由 VirtualThread.createDefaultScheduler() 创建的 ForkJoinPool 是虚拟线程的载体线程池初始大小为
12，最大线程数为 256, 

```java
new ForkJoinPool(parallelism=12, factory=..., handler=(t, e) -> {}, asyncMode=true,
       corePoolSize=0, maximumPoolSize=256, minimumRunnable=6, saturate=pool -> true, keepAliveTime=30, unit=SECONDS)
```

(上面并不是 Java 语法，用  parallelism=12 的方法旨在表示每一个传入的参数值). 再深入到这个构造函数 `ForkJoinPool` 中, 最终的几个
ForkJoinPool 关键参数是

>parallelism=corePoolSize=12, maximumPoolSize=256, minimumRunnable=6,  
>minAvail=6, maxSpares=244, queues=new WorkQueue[32];

### 虚拟线程与载体(平台)线程的关系

虚拟线程更应该想像为一种可以载体

面就要进一步探索虚拟线程与线程池间的关系，譬如下面的测试代码

```java
public static void main(String[] args) {
    for (int i = 0; i < 15; i++) {
        int taskId = i;
        Thread.ofVirtual().start(() -> {
            System.out.printf("Task #%s running, current thread: %s%n", taskId, Thread.currentThread());
            sleepInSeconds(60);
        });
    }

    sleepInSeconds(3600); // wait for an hour to let virtual threads complete
}

private static void sleepInSeconds(int seconds) {
    try {
        Thread.sleep(seconds * 1000);
    } catch (InterruptedException e) {
        Thread.currentThread().interrupt();
    }
}
```

每个任务  sleep 60 秒，以便于我们有充足的时间进行观察

启动后的输出为

```text
Task #0 running, current thread: VirtualThread[#25]/runnable@ForkJoinPool-1-worker-1
Task #1 running, current thread: VirtualThread[#27]/runnable@ForkJoinPool-1-worker-7
Task #2 running, current thread: VirtualThread[#28]/runnable@ForkJoinPool-1-worker-6
Task #3 running, current thread: VirtualThread[#29]/runnable@ForkJoinPool-1-worker-4
Task #4 running, current thread: VirtualThread[#30]/runnable@ForkJoinPool-1-worker-3
Task #5 running, current thread: VirtualThread[#31]/runnable@ForkJoinPool-1-worker-7
Task #6 running, current thread: VirtualThread[#32]/runnable@ForkJoinPool-1-worker-6
Task #8 running, current thread: VirtualThread[#34]/runnable@ForkJoinPool-1-worker-4
Task #10 running, current thread: VirtualThread[#36]/runnable@ForkJoinPool-1-worker-6
Task #11 running, current thread: VirtualThread[#37]/runnable@ForkJoinPool-1-worker-4
Task #12 running, current thread: VirtualThread[#39]/runnable@ForkJoinPool-1-worker-7
Task #13 running, current thread: VirtualThread[#40]/runnable@ForkJoinPool-1-worker-6
Task #14 running, current thread: VirtualThread[#41]/runnable@ForkJoinPool-1-worker-7
Task #7 running, current thread: VirtualThread[#33]/runnable@ForkJoinPool-1-worker-7
Task #9 running, current thread: VirtualThread[#35]/runnable@ForkJoinPool-1-worker-4
```

这就有些颠覆我们目前对线程的理解的，如果是线程池，已知线程池大小为 12, 那么同时最多只有 12 个任务得到执行。然而用了虚拟线程之后，
15 个任务全部同时在执行，而且用到的载体线程只需要 5 个，分别是 ForkJoinPool-1-worker-[1,3,4,7,6], 从 JConsole 观察到的 
ForkJoinPool-1 线程池的大小也是 8

{{< bundle-image java21-virtual-thread-3-800x308.png 812 >}}

这就虚拟线程的超能力，是怎么实现了只要 5 个平台线程就能同时执行 15 任务呢？

注：要让 JConsole 能连接 Java 程序，在启动时设置了以下系统属性

```text
-Djava.rmi.server.hostname=localhost
-Dcom.sun.management.jmxremote=true
-Dcom.sun.management.jmxremote.port=1099
-Dcom.sun.management.jmxremote.authenticate=false
-Dcom.sun.management.jmxremote.ssl=false
```

如果我们改变 for (int i = 0; i < 15; i++) 中的参数 15，把程序稍微加上统计功能

```java
public static void main(String[] args) {
    AtomicInteger tasksAreRunning = new AtomicInteger();
    Set<String> threadNames = ConcurrentHashMap.newKeySet();

    int numberOfTasks = 300;
    for (int i = 0; i < numberOfTasks; i++) {
        int taskId = i;
        Thread.ofVirtual().start(() -> {
            tasksAreRunning.incrementAndGet();
            System.out.printf("Task #%s running, current thread: %s%n", taskId, currentThread());
            var carrierThreadName = currentThread().toString().split("@")[1];
            threadNames.add(carrierThreadName);
            sleepInSeconds(30);
        });
    }

    sleepInSeconds(5);
    System.out.printf("Running tasks: %s, Platform threads used: %s\n", tasksAreRunning.get(), threadNames.size());

    sleepInSeconds(3600); // wait for an hour to let virtual threads complete
}
```

用不同的 numberOfTasks 进行测试，执行任务时首先累加 count, 然后不退出，保证所有任务并发执行，主线程上等待 5 
秒后查看正在执行的任务数和用到的载体线程，据此可估计 ForkJoinPool 线程池的大小。经过几轮测试分别得到下面的组合值

1. Running tasks: 10, Platform threads used: 4
2. Running tasks: 20, Platform threads used: 6
3. Running tasks: 30, Platform threads used: 5
4. Running tasks: 50, Platform threads used: 5
5. Running tasks: 100, Platform threads used: 8
6. Running tasks: 200, Platform threads used: 12
7. Running tasks: 500, Platform threads used: 12
8. Running tasks: 5000, Platform threads used: 12
9. Running tasks: 50000, Platform threads used: 12


为什么能用极小的 ForkJoinPool 执行如此多的虚拟线程任务，因为那些虚拟线程会在 Thread.sleep() 时立即让出当前线程去承接别的虚拟线程任务。
如果把上面的第 13 行换成能一定程序占用 CPU 资源无法立即让出 CPU 的操作，修改后的 main 函数是

```java
public static void main(String[] args) {
    AtomicInteger tasksAreRunning = new AtomicInteger();
    Set<String> threadNames = ConcurrentHashMap.newKeySet();

    int numberOfTasks = 20;
    for (int i = 0; i < numberOfTasks; i++) {
        int taskId = i;
        Thread.ofVirtual().start(() -> {
            tasksAreRunning.incrementAndGet();
            System.out.printf("Task #%s running, current thread: %s%n", taskId, currentThread());
            var carrierThreadName = currentThread().toString().split("@")[1];
            threadNames.add(carrierThreadName);
            while (true) {
                try {
                    Files.writeString(Path.of("temp/abc-" + taskId + ".txt"), "hello world");
                } catch (IOException e) {
                    throw new RuntimeException(e);
                }
            }
        });
    }

    sleepInSeconds(5);
    System.out.printf("Running tasks: %s, Platform threads used: %s\n", tasksAreRunning.get(), threadNames.size());

    sleepInSeconds(3600); // wait for an hour to let virtual threads complete
}
```

再来跑一组数据

1. Running tasks: 10, Platform threads used: 10
2. Running tasks: 20, Platform threads used: 20
3. Running tasks: 30, Platform threads used: 30
4. Running tasks: 50, Platform threads used: 50
5. Running tasks: 100, Platform threads used: 100
6. Running tasks: 200, Platform threads used: 200
7. Running tasks: 500, Platform threads used: 256
8. Running tasks: 5000, Platform threads used: 256

从这轮测试我们的观感是

1. 虚拟线程任务仍然会被立即安排执行，由 ForkJoinPool 中的线程来调度
2. 载体线程池的大小初始为 CPU 内核数，最大为  256
3. 虚拟线程任务在碰到无法让出 CPU 的代码时则会长时间占用该载体线程，从而激发 ForkJoinPool 达到最大的线程池大小

到目前为止对虚拟线程的强大有了感性的认识，现在要开始阅读 <a href="https://openjdk.org/jeps/444">
JEP 444: Virtual Threads</a> 以更准确捕捉到 Java 引入虚拟线程的初衷及要达到的目标。

测试真正 CPU 密集型的操作, 执行一段计算 PI 的操作

```java
public static void main(String[] args) {
    AtomicInteger tasksAreRunning = new AtomicInteger();
    Set<String> threadNames = ConcurrentHashMap.newKeySet();

    int numberOfTasks = Integer.parseInt(args[0]);
    var countDown = new CountDownLatch(numberOfTasks);

    for (int i = 0; i < numberOfTasks; i++) {
        int taskId = i;
        Thread.ofVirtual().start(() -> {
            tasksAreRunning.incrementAndGet();
            System.out.printf("Task #%s running, current thread: %s%n", taskId, currentThread());
            var carrierThreadName = currentThread().toString().split("@")[1];
            threadNames.add(carrierThreadName);

            calculatePi(1_000_000_000L);
            countDown.countDown();
            System.out.printf("Task # %s completed, current thread: %s\n", taskId, Thread.currentThread());
        });
    }

    try {
        countDown.await();
    } catch (InterruptedException e) {
        throw new RuntimeException(e);
    }

    System.out.printf("Running tasks: %s, Platform threads used: %s\n", tasksAreRunning.get(), threadNames.size());
}

public static double calculatePi(long iterations) {
    double pi = 0.0;
    for (int i = 0; i < iterations; i++) {
        pi += Math.pow(-1, i) / (2 * i + 1);
    }
    return pi * 4;
}
```

测试参数分别为不数值时的输出

```text
Running tasks: 10, Platform threads used: 10
Running tasks: 20, Platform threads used: 12
Running tasks: 50, Platform threads used: 12
Running tasks: 100, Platform threads used: 12
Running tasks: 500, Platform threads used: 12
```

Java 判定出是 CPU 密集操作, 平台线程直接锁定在 CPU 内核数上, 它很聪明的知道增加更多的线程去执行虚拟线程的任务无益.

现在还想设计的一个测试是, 在载体线程执行某个虚拟线程任务 #1 时, 碰到 IO 等待, 让出 CPU 切换到另一个虚拟线程任务 #2, 最后虚拟线程任务 #1 
和 #2 是不是必须由同一个载体线程来完成, 不得始乱终弃?

```java
public static void main(String[] args) {
    int numberOfTasks = Integer.parseInt(args[0]);
    var countDown = new CountDownLatch(numberOfTasks);

    for (int i = 0; i < numberOfTasks; i++) {
        int taskId = i;
        Thread.ofVirtual().start(() -> {
            List<String> objects = Collections.synchronizedList(new ArrayList<>());
            objects.add("#" + taskId);
            objects.add("before: " + currentThread());

            sleepInSeconds(5);

            objects.add("after: " + currentThread());
            countDown.countDown();

            System.out.println(String.join(", ", objects));
        });
    }

    try {
        countDown.await();
    } catch (InterruptedException e) {
        throw new RuntimeException(e);
    }
}
```

参数为 10 时测试的输出结果是

```text {hl_lines=[1, 2]}
#4, before: VirtualThread[#26]/runnable@ForkJoinPool-1-worker-5, after: VirtualThread[#26]/runnable@ForkJoinPool-1-worker-5
#0, before: VirtualThread[#21]/runnable@ForkJoinPool-1-worker-1, after: VirtualThread[#21]/runnable@ForkJoinPool-1-worker-9
#8, before: VirtualThread[#30]/runnable@ForkJoinPool-1-worker-9, after: VirtualThread[#30]/runnable@ForkJoinPool-1-worker-2
#5, before: VirtualThread[#27]/runnable@ForkJoinPool-1-worker-6, after: VirtualThread[#27]/runnable@ForkJoinPool-1-worker-8
#2, before: VirtualThread[#24]/runnable@ForkJoinPool-1-worker-3, after: VirtualThread[#24]/runnable@ForkJoinPool-1-worker-1
#6, before: VirtualThread[#28]/runnable@ForkJoinPool-1-worker-7, after: VirtualThread[#28]/runnable@ForkJoinPool-1-worker-10
#7, before: VirtualThread[#29]/runnable@ForkJoinPool-1-worker-8, after: VirtualThread[#29]/runnable@ForkJoinPool-1-worker-3
#3, before: VirtualThread[#25]/runnable@ForkJoinPool-1-worker-4, after: VirtualThread[#25]/runnable@ForkJoinPool-1-worker-4
#1, before: VirtualThread[#23]/runnable@ForkJoinPool-1-worker-2, after: VirtualThread[#23]/runnable@ForkJoinPool-1-worker-6
#9, before: VirtualThread[#32]/runnable@ForkJoinPool-1-worker-10, after: VirtualThread[#32]/runnable@ForkJoinPool-1-worker-7
```

说明在 CPU 让出前后可由不同的线程执行. 比如上面第一行显示在执行虚拟线程任务 #4 时, 在 sleep 让出 CPU 前后都是由同一个平台线程 ForkJoinPool-1-worker-5
执行的, 而第二行显示虚拟线程任务 #0 在 sleep 让出 CPU 前后分别由不同的平台线程 ForkJoinPool-1-worker-1 和 ForkJoinPool-1-worker-9 执行的

### 虚拟线程与 ThreadLocal

TheadLocal 以前都是与平台线程相关联在的, 那么在 Java 21 引入虚拟线程之后, ThreadLocal 是否还能像预期那样与虚拟线程良好配合呢? 即希望
TheadLocal 只关联虚拟线程而不会污染相对应的平台线程.

比如分析前面第二行的日志

> #0, before: VirtualThread[#21]/runnable@ForkJoinPool-1-worker-1, after: VirtualThread[#21]/runnable@ForkJoinPool-1-worker-9

第一个虚拟线程任务 #0, 在 Java 21 中的 ThreadLocal 是关联到虚拟线程 `VirtualThread[#21]` 还是平台载线程 `ForkJoinPool-worker-1`
或 `ForkJoinPool-work-9` 呢?

查看 Java 21 的 ThreadLocal 源代码, 其中加入了有关 VirtualThread 的代码, 如果还如预期的工作应该要关联到 VirtualThread[#21], 与平台线程无关.

为了验证设计了如下的测试, 只在第一轮进入虚拟线程任务时设置 ThreadLocal 中的值, 然后在 sleep 让出 CPU 前后读取其中的值, 第一轮任务完后, 
再跑第二轮来重用平台线程看之前绑定在对应虚拟线程上的 ThreadLocal 值还在不在?

```java
    public static ThreadLocal<String> threadLocal = new ThreadLocal<>();

    public static void main(String[] args) {
        int numberOfTasks = Integer.parseInt(args[0]);

        runTasks(numberOfTasks, true);
        System.out.println("\n=====================================\n");
        runTasks(numberOfTasks, false);

    }

    public static void runTasks(int numberOfTasks, final boolean shouldSetThreadLocal) {
        var countDown = new CountDownLatch(numberOfTasks);

        for (int i = 0; i < numberOfTasks; i++) {
            int taskId = i;
            Thread.ofVirtual().start(() -> {
                if (shouldSetThreadLocal) {
                    threadLocal.set("$threadLocal: %s$".formatted(taskId));
                }
                List<String> objects = Collections.synchronizedList(new ArrayList<>());
                objects.add("#" + taskId);
                objects.add(threadLocal.get());
                objects.add("before: " + currentThread());

                sleepInSeconds(10);

                objects.add("\n    after: " + currentThread());
                objects.add(threadLocal.get());
                countDown.countDown();

                System.out.println(String.join(", ", objects));
            });
        }

        try {
            countDown.await();
        } catch (InterruptedException e) {
            throw new RuntimeException(e);
        }
    }
```

输入参数为 10, 执行结果如下

```text {linenos=table,hl_lines=[3, 4, 24, 25]}
#0, $threadLocal: 0$, before: VirtualThread[#21]/runnable@ForkJoinPool-1-worker-1, 
    after: VirtualThread[#21]/runnable@ForkJoinPool-1-worker-1, $threadLocal: 0$
#3, $threadLocal: 3$, before: VirtualThread[#25]/runnable@ForkJoinPool-1-worker-4, 
    after: VirtualThread[#25]/runnable@ForkJoinPool-1-worker-5, $threadLocal: 3$
#4, $threadLocal: 4$, before: VirtualThread[#26]/runnable@ForkJoinPool-1-worker-5, 
    after: VirtualThread[#26]/runnable@ForkJoinPool-1-worker-2, $threadLocal: 4$
#2, $threadLocal: 2$, before: VirtualThread[#24]/runnable@ForkJoinPool-1-worker-3, 
    after: VirtualThread[#24]/runnable@ForkJoinPool-1-worker-7, $threadLocal: 2$
#7, $threadLocal: 7$, before: VirtualThread[#29]/runnable@ForkJoinPool-1-worker-8, 
    after: VirtualThread[#29]/runnable@ForkJoinPool-1-worker-3, $threadLocal: 7$
#9, $threadLocal: 9$, before: VirtualThread[#31]/runnable@ForkJoinPool-1-worker-10, 
    after: VirtualThread[#31]/runnable@ForkJoinPool-1-worker-6, $threadLocal: 9$
#5, $threadLocal: 5$, before: VirtualThread[#27]/runnable@ForkJoinPool-1-worker-6, 
    after: VirtualThread[#27]/runnable@ForkJoinPool-1-worker-8, $threadLocal: 5$
#1, $threadLocal: 1$, before: VirtualThread[#23]/runnable@ForkJoinPool-1-worker-2, 
    after: VirtualThread[#23]/runnable@ForkJoinPool-1-worker-10, $threadLocal: 1$
#6, $threadLocal: 6$, before: VirtualThread[#28]/runnable@ForkJoinPool-1-worker-7, 
    after: VirtualThread[#28]/runnable@ForkJoinPool-1-worker-4, $threadLocal: 6$
#8, $threadLocal: 8$, before: VirtualThread[#30]/runnable@ForkJoinPool-1-worker-9, 
    after: VirtualThread[#30]/runnable@ForkJoinPool-1-worker-9, $threadLocal: 8$
    
=====================================

#2, null, before: VirtualThread[#46]/runnable@ForkJoinPool-1-worker-12, 
    after: VirtualThread[#46]/runnable@ForkJoinPool-1-worker-5, null
#7, null, before: VirtualThread[#51]/runnable@ForkJoinPool-1-worker-5, 
    after: VirtualThread[#51]/runnable@ForkJoinPool-1-worker-9, null
#0, null, before: VirtualThread[#44]/runnable@ForkJoinPool-1-worker-1, 
    after: VirtualThread[#44]/runnable@ForkJoinPool-1-worker-8, null
#9, null, before: VirtualThread[#53]/runnable@ForkJoinPool-1-worker-5, 
    after: VirtualThread[#53]/runnable@ForkJoinPool-1-worker-8, null
#1, null, before: VirtualThread[#45]/runnable@ForkJoinPool-1-worker-8, 
    after: VirtualThread[#45]/runnable@ForkJoinPool-1-worker-9, null
#3, null, before: VirtualThread[#47]/runnable@ForkJoinPool-1-worker-5, 
    after: VirtualThread[#47]/runnable@ForkJoinPool-1-worker-8, null
#6, null, before: VirtualThread[#50]/runnable@ForkJoinPool-1-worker-1, 
    after: VirtualThread[#50]/runnable@ForkJoinPool-1-worker-8, null
#5, null, before: VirtualThread[#49]/runnable@ForkJoinPool-1-worker-4, 
    after: VirtualThread[#49]/runnable@ForkJoinPool-1-worker-8, null
#8, null, before: VirtualThread[#52]/runnable@ForkJoinPool-1-worker-12, 
    after: VirtualThread[#52]/runnable@ForkJoinPool-1-worker-9, null
#4, null, before: VirtualThread[#48]/runnable@ForkJoinPool-1-worker-9, 
    after: VirtualThread[#48]/runnable@ForkJoinPool-1-worker-9, null
```

分析上面的结果

第 3, 4 行显示任务 #3(虚拟线程[#25]), sleep 前由平台线程 `ForkJoinPool-1-worker-4` 执行时绑定的 ThreadLocal 值, 在 sleep
后切换到平台线程 `ForkJoinPool-1-worker-5` 执行时仍然可见.

在 25, 26 行时再次重用平台线程 `ForkJoinPool-1-worker-5` 时, 第一轮中绑定给虚拟线程的 ThreadLocal 值是不存在的.

所以结论是 ThreadLocal 能够与虚拟线程完美的协作. 即当使用虚拟线程时, ThreadLocal 相当于是一个 VirtualThreadLocal(当然不存在这个类的)

### Thread.ofVirtual() 与 Executors.newVirtualThreadPerTaskExecutor()

`Thread.start()` 每次都会启动一个平台线程, 而 `Thread.ofVirtual().start()` 从字面上也可以理解每次都会启动一个虚拟线程, 
然而在底层则却是由一个默认 CPU 个数 - 256 大小的 ForkJoinPool 平台线程池来调度, 该线程池知道在 IO 等待(任务执行其间)时切换到别的任务, 
并在应对 CPU 密集型任务时平台线程池保持在 CPU 个数大小. 由此我想到的 ForkJoinPool 会根据挂起(而非提交)的任务数来控制平台线程池的大小.

Thread.ofVirutal() 默认用的是 ForkJoinPool 平台线程池, Java 21 还提供了新方法 `Executors.newVirtualThreadPerTaskExecutor()`
来创建自己的平台线程池, 

该 API 的 Java Doc 是

>Creates an Executor that starts a new virtual Thread for each task. The number of threads created by the Executor is unbounded.  
>This method is equivalent to invoking newThreadPerTaskExecutor(ThreadFactory) with a thread factory that creates virtual threads.  
>Returns:  
>a new executor that creates a new virtual Thread for each task

即为每个任务启动一个虚拟线程, 这与 `Thread.ofVirtual().start()` 不一个意思吗? 阅读它的源代码也确实是, 它在创建 ThreadFactory 时

ThreadBuilders.factory()
```java
public ThreadFactory factory() {
    return new VirtualThreadFactory(scheduler, name(), counter(), characteristics(),
            uncaughtExceptionHandler());
}
```

`scheduler` 是 null 值, 所以在调用 VirtualThread 构造函数时如最前面的代码所示, 还是落入到了

```java
scheduler = DEFAULT_SCHEDULER;
```

所以结论是 Thread.ofVirtual() 与 Executors.newVirtualThreadPerTaskExecutor() 会共享同一个平台线程池, 我们用代码来进一步验证

```java
    public static void main(String[] args) {

        try (ExecutorService executor = Executors.newVirtualThreadPerTaskExecutor()) {
            executor.submit(() -> {
                System.out.println("executor thread: " + currentThread());
            });
        }

        Thread.ofVirtual().start(() -> {
            System.out.println("virtual thread: " + currentThread());
        });

        sleepInSeconds(2);
    }
```

输出为

> executor thread: VirtualThread[#21]/runnable@ForkJoinPool-1-worker-1  
> virtual thread: VirtualThread[#24]/runnable@ForkJoinPool-1-worker-1

Thread.ofVirtual() 和 Executors.newVirtualThreadPerTaskExecutor() 所使用的平台线程池是同一个 `ForkJoinPool-1`

#### Spring 的 VirtualThreadTaskExecutor 

Spring 6.1 新加了一个新类 `VirtualThreadTaskExecutor` 用了支持虚拟线程, 在 Spring 6.2 的 `ExecutorConfigurationSupport` 中加入了
`setVirtualThreads(boolean virtualThreads)` 方法, 它有以下几个实现类

1. ThreadPoolTaskExecutor
2. ThreadPoolExecutorFactoryBean
3. ThreadPoolTaskScheduler
4. ScheduledExecutorFactoryBean

就是说在声明 `ThreadPoolTaskExecutor` 或 `ThreadPoolTaskScheduler` 时可选择使用虚拟线程. 

下面分别来体验 `VirtualThreadTaskExecutor` 和 `ThreadPoolTaskExecutor` 的用法, `ThreadPoolTaskScheduler` 应与后者类似.

##### VirtualThreadTaskExecutor

`VirtualThreadTaskExecutor` 提供了两个构造函数, 默认的无参和带一个 `String threadNamePrefix` 的, 我们用后者来进行测试, 同样与
`Thread.ofVirtual()` 进行对比

```java
    public static void main(String[] args) {

        var springVtExecutor = new VirtualThreadTaskExecutor("my-vt-");

        springVtExecutor.submit(() -> {
            System.out.println("executor thread: " + currentThread());
        });

        Thread.ofVirtual().start(() -> {
            System.out.println("virtual thread: " + currentThread());
        });

        sleepInSeconds(2);
    }
```

输出结果没什么意外

>executor thread: VirtualThread[#21,my-vt-0]/runnable@ForkJoinPool-1-worker-1
>virtual thread: VirtualThread[#23]/runnable@ForkJoinPool-1-worker-2

底层平台线程池仍然是那个 `ForkJoinPool-1`, 前缀只是给虚拟线程命名用的.

##### ThreadPoolTaskExecutor

它会不会也是用 `VirtualThread.createDefaultScheduler()`? 代码测试

```java
public static void main(String[] args) {

    ThreadPoolTaskExecutor springTaskExecutor = new ThreadPoolTaskExecutor();
    springTaskExecutor.setCorePoolSize(2);
    springTaskExecutor.setMaxPoolSize(5);
    springTaskExecutor.setVirtualThreads(true);
    springTaskExecutor.initialize();

    springTaskExecutor.submit(() -> {
        System.out.println("executor thread: " + currentThread());
    });

    Thread.ofVirtual().start(() -> {
        System.out.println("virtual thread: " + currentThread());
    });

    sleepInSeconds(2);
}
```

这次输出也没什么意外, 依然用了与 `Thread.ofVirtual()` 相同的线程池

>executor thread: VirtualThread[#21,ThreadPoolTaskExecutor-0]/runnable@ForkJoinPool-1-worker-1  
>virtual thread: VirtualThread[#23]/runnable@ForkJoinPool-1-worker-2

只是有了自己的虚拟线程名称前缀. 在 `ThreadPoolTaskExecutor.initialize()` 方法中如果在配置了使用虚拟线程(setVirtualThreas(true)) 的话,
会最终使用到

```java
ThreadFactory factory = (this.virtualThreads ?
		new VirtualThreadTaskExecutor(getThreadNamePrefix()).getVirtualThreadFactory() : this.threadFactory);
this.executor = initializeExecutor(factory, this.rejectedExecutionHandler);
```

最后的 ThreadFactory 实例是

```java
Thread.ofVirtual().name("ThreadPoolTaskExecutor", 0).factory();

//Thread.ofVirtual().factory(); // 对应没有前缀的 ThreadFactory 是这个
```

平台线程与虚拟线程创建过程还不一样, 创建虚拟线程时一定要关联一个 scheduler. 如果使用 `ThreadPoolTaskExecutor` 时指定了
`setVirtualThreads(true)` 的话, 那么其他的参数如 `CorePoolSize`, `MaxPoolSize` 就限制在虚拟线程任务数目上了. 观察下面的代码输出

```java
    public static void main(String[] args) {
        ThreadPoolTaskExecutor springTaskExecutor = new ThreadPoolTaskExecutor();
        springTaskExecutor.setCorePoolSize(2);
        springTaskExecutor.setMaxPoolSize(5);
        springTaskExecutor.setVirtualThreads(true);
        springTaskExecutor.initialize();

        for (int i = 0; i < 20; i++) {
            int taskId = i;
            springTaskExecutor.submit(() -> {
                System.out.println("task " + taskId + " running in thread: " + currentThread());
                sleepInSeconds(30);
            });
        }

        sleepInSeconds(200);
    }
```

上任务等待 30 秒, 执行后 30 秒内一直停留在

>task 1 running in thread: VirtualThread[#23,ThreadPoolTaskExecutor-1]/runnable@ForkJoinPool-1-worker-2  
>task 0 running in thread: VirtualThread[#21,ThreadPoolTaskExecutor-0]/runnable@ForkJoinPool-1-worker-1

30 秒后看到的输出是

>task 0 running in thread: VirtualThread[#21,ThreadPoolTaskExecutor-0]/runnable@ForkJoinPool-1-worker-1  
>task 1 running in thread: VirtualThread[#23,ThreadPoolTaskExecutor-1]/runnable@ForkJoinPool-1-worker-2  
>task 3 running in thread: VirtualThread[#23,ThreadPoolTaskExecutor-1]/runnable@ForkJoinPool-1-worker-3  
>task 2 running in thread: VirtualThread[#21,ThreadPoolTaskExecutor-0]/runnable@ForkJoinPool-1-worker-1

看到虚拟线程也是可以重用的.

当我们设定 `CorePoolSize=2` 时只能有两个虚拟线程任务同时执行, 尽管任务只是 sleep 若干秒, 能让出 CPU 来, 这样一来与我们之前使用虚拟线程的初衷产生了冲突. 
使用虚拟线程的目的就是要利用它能在 IO 等待时主动让出 CPU 让当前平台线程去执行另一个虚拟线程任务. 因此, 如果使用 `ThreadPoolTaskExecutor`
的虚拟线程, 就应该考虑把它的 `CorePoolSize`, `MaxPoolSize`, `QueueCapacity` 参数设置比较大的数目, 如 5000 这种级别的数字, 
更应把它当作 `WorkingQueue` 来看待.

相应的 `Thread.ofVirtual()`, `Executors.newVirtualThreadPerTaskExecutor()`, `VirtualThreadTaskExecutor` 都是为每一个任务创建一个虚拟线程.
相当是虚拟线程的 CorePoolSize 不受限.

注意, `ThreadPoolTaskExecutor` 为虚拟线程时, `CorePoolSize`, `MaxPoolSize` 不再是平台线程的相应大小, 这些必须要搞清楚. 
要是依然认为它们是平台线程的大小, 仅在原来的 `ThreadPoolTaskExecutor`配置(如 CorePoolSize = 50)基础, 加上 `setVirtualThreads(true)`,
性能可能变得更差. 创建虚拟线程的代价相比于平台线程来说极低, 所以可直接考虑用不限制同时执行虚拟线程任务数的方式, 除非可能积压数十万虚拟线程任务时
造成内存不足的情况下考虑约束同时执行虚拟线程任务数.

### 关于虚拟线程的总结{#conclusion}

本文前面的主要侧重测试代码的行为, 文字表达上十分混乱, 最后寄希望能简单的几点把对虚拟线程的要领和精髓梳理如下

1. 在操作系统层面的任务调度的最小单位仍然为平台线程, 使用虚拟线程后可理解为在 Java 虚拟机中调度任务的单位变成了虚拟线程
2. Java 线程与平台线程一一对应, 每个纯种都要有独产的线程栈和本地存储空间, 比在 macOS 26 的 Java 25 下用命令 
 `java -XX:+PrintFlagsFinal -version | grep ThreadStackSize` 看到的线程栈是 2M, 这就限制了不能创建过多的平台线程. 创建 10000 
 个平台线程, 线程栈空间将耗费 20G, 这需要特别配置 JVM 的 Xmx 参数
3. 虚拟线程不与平台线程一一对应, 但在创建虚拟线程的时候需要关联一个 `scheduler: Executor`, 当执行虚拟线程任务时才从 `scheduler` 
 线程池中找一个线程作为载体线程. 所以即使是创建大量的虚拟线程, 它们都只是与同一个线程池相关联. 平台线程池大小是确定的, 那么所需的总的线程栈空间也是确定的
4. 进一步说明 #3, 多数时候上面的 `scheduler` 是一个 `CorePoolSize=<cpu内核数>`, `MaxPoolSize=256` 的 `ForkJoinPool`,  
  所以无论创建多少虚拟线程, 最在线程栈空间只要 `256 * 2M = 512M`. 虚拟线程相对于是普通的 Java 对象, 每个虚拟线程对象占用空间远远小于 2M, 
  所以才能创建百万, 千万级的虚拟线程数.
5. 虚拟线程与平台线程池的关系类似于 Task 与平台线程一样关系
6. 然而虚拟线程与普通 Task 最大的区别就是平台线程在执行虚拟线程任务期间, 碰到类似 IO 等不占有 CPU 资源的操作(比如 sleep)时会挂起当前虚拟线程任务, 
  让出 CPU, 转而执行另一个虚拟线程任务, 等到挂起的虚拟线程任务恢复时继续由平台线程池执行 如此能更有效的利用 CPU 资源, 而不是傻傻的空转 CPU 等待
7. 虚拟线程任务在挂起前后可能由平台线程池中的不同线程执行
8. 不管是用 `Thread.ofVirtual()`, `Executors.newVirtualThreadPerTaskExecutor()`, `VirtualThreadTaskExecutor`, 甚至是 Spring 
 `ThreadPoolTaskExecutor`, 其底层的平台线程池都是在 `VirtualThread` 类中定义的那个 `ForkJoinPool DEFAULT_SCHEDULER = createDefaultScheduler()` 
 它的 `CorePoolSize` 是 CPU 内核数, `MaxPoolSize` 是 256.
9. 以上 `DEFAULT_SCHEDULER` 平台线程池大小可由系统属性 `jdk.virtualThreadScheduler.parallelism`, `jdk.virtualThreadScheduler.maxPoolSize`, 
   `jdk.virtualThreadScheduler.minRunnable` 进行定制
10. `Thread.ofVirtual()`, `Executors.newVirtualThreadPerTaskExecutor()`, `VirtualThreadTaskExecutor` 都是为每一个虚拟线程任务创建一个新的虚拟线程, 
   相当于能同时执行的虚拟线程任务是不受限的. 类似于在使用一个 `WorkingQueue` 不限大小的线程池, 在虚拟线程任务数十分巨大时要考虑内存压力
11. Spring 的 `ThreadPoolTaskExecutor` 有了一个新的方法 `setVirtualThreads(boolean)` 来指定是否使用虚拟线程, 默认为 false. 
   当它为 `false` 时, `CorePoolSize`, `MaxPoolSize`, 和 `QueueCapacity` 作用于要创建的平台线程池. 而当它为 `true` 的时候, 平台线程池会是默认的 
  `DEFAULT_SCHEDULER`, 而相应的 `CorePoolSize`, `MaxPoolSize`, 和 `QueueCapacity` 只作用在虚拟线程池(这是我目前想到的一个抽象概念). 
  比如说传统使用线程池情况下我们设定了 `CorePoolSize` 为 10 的话, 用 `setVirtualThreads(true)` 切换到虚拟线程后, 因为有让出 CPU 的行为, 
  `CorePoolSize` 的大小就应该大很多, 譬如 100, 甚至更大
12. 我们知道虚拟线程任务用的平台线程池是 `ForkJoinPool`, 当 CPU 内核数为 12 时, 它的 `CorePoolSize` 是 12, `MaxPoolSize`  256,
  `queue` 大小为 32. 它在有大量有 IO 等待虚拟线程任务时, `ForkJoinPool` 可由 12 增至 256, 而在全是 CPU 密集任务时, 平台线程池大小固守在 
  `CorePoolSize` 12 的数目上呢? 这和平台线程池类似, 当提交的任务占满了 `WorkingQueue` 后线程数目就会由 `CorePoolSize` 增长至 `MaxPoolSize` 大小. 
  使用了虚拟线程时也可以这样理解, 当因 IO 等待挂起的虚拟线程任务占满了 `WorkingQueue` 后也会增加适当的平台线程数, 如果是执行 CPU 密集型的任务, 
  没什么机会因为 IO 等待而挂起任务, 所以保持在了 `CorePoolSize` 大小的线程数目. 虚拟线程结合 `ForkJoinPool` 的这一特性完美的同时适配了
  IO 和 CPU 密集型任务. 虚拟线程简直就是多任务的全栈时解决方案
13. 虽然一个虚拟线程任务可能会被多个平台线程切换执行, 但 `ThreadLocal` 依然正常工作, 所以也就不用担心像日志的 `MDC`, Spring 的数据库事物. 
  可能以后 `ScopedValue` 是一个比 `ThreadLocal` 更好的选择.