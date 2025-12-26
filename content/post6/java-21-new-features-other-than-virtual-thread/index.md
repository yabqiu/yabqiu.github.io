---
title: Java 21 虚拟线程外其他新特性
url: /java-21-new-features-other-than-virtual-thread/
date: 2025-12-25T15:00:00-05:00
featured: false
type: post
draft: true
toc: false
# menu: main
usePageBundles: true
thumbnail: "../images/logos/java-logo.png"
categories:
  - java
  - new features
tags:
  - java 21
series: Java New Features
comment: true
codeMaxLines: 50
---

迁移完所有的 WordPress 日志到 Hugo 之后, 终于有时间真正继承学习相关的新技术. Java 21 是于 2023 年 9 月份释放出来的 LTS 版本, 目前主要在用该版.
Java 25 LTS 版本已发布, 按正常节奏应该要切换到该版本. 

随着 AI 在编程界的花式表演, 所宣传的似乎就是要扑灭他人的学习热情, 编程方面越小白越好, 只要能写好小作文就行了. AI 当然还是要用, 
但我对以往多少年传统的学习方式并不感到白花了心血. 告诉 AI 的一个课题 AI 确实能写出一篇漂亮, 规整的博客文章, 但其中有没有胡说八道, 
只有试了才知道, 即使生成的文章无误, 也必须实践一遍才有更多更深的斩获. 

如果没有相关的技术储备, 每次与 AI 互动的时候都要告诉它尽量用 Java 21 新特性, 因为新引入的特性基本能实现得更简洁, 高效,
估计 AI 才不那么在乎这些, 写出适于人阅读的代码恐怕不是 AI 的首要关注.

还是老办法, 关于 Java 某一版本新特性从两个链接出发 <!--more-->

1. [JDK 21 Release Notes - Major New Functionality](https://www.oracle.com/java/technologies/javase/21all-relnotes.html#JSERN21)
2. [OpenJDK JDK 21 Features](https://openjdk.org/projects/jdk/21/)

把那些关键的新特性列出来就是

- 430: [String Templates (Preview)](https://openjdk.org/jeps/430)
- 431: [Sequenced Collections](https://openjdk.org/jeps/431)
- 439: [Generational ZGC](https://openjdk.org/jeps/439)
- 440: [Record Patterns](https://openjdk.org/jeps/440)
- 441: [Pattern Matching for switch](https://openjdk.org/jeps/441)
- 442: [Foreign Function & Memory API (Third Preview)](https://openjdk.org/jeps/442)
- 443: [Unnamed Patterns and Variables (Preview)](https://openjdk.org/jeps/443)
- 444: [Virtual Threads](https://openjdk.org/jeps/444)
- 445: [Unnamed Classes and Instance Main Methods (Preview)](https://openjdk.org/jeps/445)
- 446: [Scoped Values (Preview)](https://openjdk.org/jeps/446)
- 448: [Vector API (Sixth Incubator)](https://openjdk.org/jeps/448)
- 449: [Deprecate the Windows 32-bit x86 Port for Removal](https://openjdk.org/jeps/449)
- 451: [Prepare to Disallow the Dynamic Loading of Agents](https://openjdk.org/jeps/451)
- 452: [Key Encapsulation Mechanism API](https://openjdk.org/jeps/452)
- 453: [Structured Concurrency (Preview)](https://openjdk.org/jeps/453)

其中 444 虚拟线程最 Java 21 最受关注的新特性, 所以单独写过一篇日志 [Java 21 虚拟线程外其他新特性](/java-21-new-features-other-than-virtual-thread/), 
在另一篇 [Java 19, 20 新特性学习](/java-19-20-new-features/) 也简单介绍了非正式的 440 Record Patterns, 446 Scoped Values,
和 453 Structured Concurrency.

本文主要学习的是正式的新特性 431 Sequenced Collections, 439 Generational ZGC, 440 Record Patterns, 441 Pattern Matching for switch,
并且提前了解一下 430 String Templates.

### 430 String Templates

这是一个预备学习, 该特性直到 JDK 26 也没见到定稿, 也有可能终将放弃. 大概来看一下它要解决什么问题.简单来讲就是它想要实现其他编程语言中的字符串插值功能,
即 String interpolation, 