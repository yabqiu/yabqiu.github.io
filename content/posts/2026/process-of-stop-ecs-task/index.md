---
title: "ECS 正常停止一个 Task 的过程"
url: /process-of-stop-ecs-task/
date: 2026-04-24T12:30:00-05:00
featured: false
type: post
draft: true
toc: false
# menu: main
usePageBundles: true
thumbnail: "/images/logos/java-logo.png"
categories:
  - java
  - new features
tags:
  - java 25
series: Java New Features
comment: true
codeMaxLines: 50
showLastmod: true
---

### ECS 正常停止一个 Task 的过程

sprinboot web graceful shutdown，所有超时值都默认时的时间轴。

当我们触发 ECS 的 AutoScaling 的 Scale in, 或手动停止一个任务，或者进行 `force new deployment`, 对于待停止任务的容器时间轴如下.
假设 Task Stop 和 Deregister 的默认的 Timeout 时间分别是默认的 30 秒和 300 秒。

```text
1. t=0       ECS 收到 stop 指令
2. t=0       ALB(Targate group) 开始 deregister (并行), 停止向该容器转发新的请求
3. t=0       SIGTERM 发送给 PID 1
4. t=30s     stopTimeout 到期(最大 120 秒, 通过 ), 所以容器只有在这个窗口中处理完所有的请求，否则触发 504 Gateway Timeout
5. t=30s     SIGKILL(默认) 发送给 PID 1, 立即终止容器内的主进程并关闭容器
5. t=300s    ALB deregistration delay 到期 (默认)
```

所以为了实现 Zero Downtime 的发布，有几个点必须保证

1. ECS 收到 Stop 指令后必须立即让 ALB 停止向该容器转发新的请求
2. stopTimeout 要足够大来处理来当前容器已接收到的请求。如果是 Tomcat，默认同时处理 200 的请求，Socket 队列中可有最多 8192 个请求。这个
   Timeout 值可调。
3. ALB 的 deregistration delay 时间


t=0    ALB 停止新请求 + SIGTERM
t=60   ALB 排空完成，所有请求处理完毕
t=80   Spring 优雅关闭完成，进程退出
t=90   stopTimeout 到期 (进程已退出，SIGKILL 不会触发)