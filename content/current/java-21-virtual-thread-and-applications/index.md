---
title: Java 21 之虚拟线程深入学习及应用场景
url: /java-21-virtual-thread-and-applications/
date: 2025-12-23T10:13:00-05:00
featured: false
type: post
draft: true
toc: false
# menu: main
usePageBundles: true
thumbnail: "../images/logos/java-logo.png"
categories:
  - Java/JEE
tags:
  - virtual threads
comment: true
codeMaxLines: 50
# additional
wpPostId: 14464 
wpStatus: draft
views: 
lastmod: 2025-11-02T14:59:01-06:00
---

Java 21 于两年前 2023 年 9 月份放出，它是一个 LTS(long term support) 版本，个人基本就是把 LTS 当作能在正式项目中使用的版本。
Java 21 有几个增进编程体验的特性，像 Sequenced Collections, Record Patterns, 和 Pattern Matching for switch, 而对于性能改进的，
也是 Java 21 最具代表的特性无疑就是 Virtual Threads -- 虚拟线程。本文单列出它来，着重感受一下虚拟线程是什么，以及我们应该如何使用它。


其实在之前的 [Java 19, 20 新特性学习](/java-19-20-new-features/) 就有一定的笔墨介绍了于 Java 19 引入，
Java 20 中尚处于第二次预览的虚拟线程。于其中大致体验了在一台 36 G 物理内存，默认堆内存为 9 G 的情况下，
创建 9000 个线程没问题，但要创建 10000 个线程就 OutOfMemoryError 了。而相同的环境下创建一百万个虚拟线程都没问题，没在继续往下试探了。

其实这种比较是没有意义的, Java 线程对应到平台线程的, 每个线程要至少实实在在的 2M 栈空间, 而一百万个虚拟线程相当于是创建了一百万个 Java 
对象而已, 更像是相应数量的 Task, 实际运行时才由载体线程去调度执行.

重新回顾一下何谓虚拟线程，Java 的虚拟线程实现是来自于 Project Loom 项目。与此相关的概念有线程，协程，以及纤程(Fiber)，而虚拟线程对应的应该是纤程。

1. 线程是操作系统最小的调度单位，每个线程有独立较大的栈空间(比如 2M)，内核调度，切换开销大，可有效使用 CPU 多核
2. 协程在单个线程内执行，共享线程栈空间或独立小空间，用户态调度，切换开销极小，但无法使用多核
3. 纤程，介于线程与协程之间，很小的独立栈，用户态调度，切换开销较小。结合线程池，纤程可在线程间转移，这时岂不是要经内核态调度吗？

<!--more-->

不管我们使用何种方式处理任务，一定要清楚任务是 CPU 还是 IO 密集型的，如果是 CPU 密集型基本上任务数超过 CPU 内核数，性能可能反而下降，
甚至要控制比 CPU 内核数还要小的并发规模。而 IO 密集型的任务就必须有效使用线程了，或者多一份心思考虑是否要使用虚拟线程。

从网上一些关于线程与虚拟线程的对比测试，仿佛性能改进不大，大概是测试程序中并没有让虚拟线程做它擅长的事情，或者线程独立栈空间，
或线程切换对测试程序的性能影响不大。我们在此不先对那些测试数据进行任何评判，而是先从另外一些角度来感受一下 Java 21 中虚拟线程的内在表现.

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

从该方法中也可看到 CarrierThread 的概念，即载体线程，现在暂时没找到如何传入自定义载体线程池。


我当前电脑的 CPU 内核数为 12，最终由 VirtualThread.createDefaultScheduler() 创建的 ForkJoinPool 是虚拟线程的载体线程池初始大小为
12，最大线程数为 256, 


```java
new ForkJoinPool(parallelism=12, factory=..., handler=（t, e) -> {}, asyncMode=true,
       corePoolSize=0, maximumPoolSize=256, minimumRunnable=6, saturate=pool -> true, keepAliveTime=30, unit=SECONDS)
```

(上面并不是 Java 语法，用  parallelism=12 的方法旨在表示每一个传入的参数值)


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
15 个任务全部同时在执行，而且用到的载体线程只需要 5 个，分别是 ForkJoinPool-1-worker-[1,3,4,7,6], 
从 JConsole 观察到的 ForkJoinPool-1 线程池的大小也是 8

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

```text
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

说明在 CPU 让出前后可由不同的线程执行, 那么 ThreadLocal 会不会变得混乱? 比如说对于上面的第一个虚拟线程任务

> #0, before: VirtualThread[#21]/runnable@ForkJoinPool-1-worker-1, after: VirtualThread[#21]/runnable@ForkJoinPool-1-worker-9

Java 21 中的 ThreadLocal 是关联到虚拟线程 `VirtualThread[#21]` 还是平台载线程 `ForkJoinPool-worker-1` 或 `ForkJoinPool-work-9` 呢?

从 Java 21 中 ThreadLocal 源代码是加入了有关 VirtualThread 的代码, 如果还如预期的工作应该要关联到 VirtualThread[#21], 与平台线程无关.

设计如下的测试

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
