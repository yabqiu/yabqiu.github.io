---
title: Java 25 新特性学习
url: /java-25-new-features/
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
 
继续探索到当前最新的 Java LTS 版本 25. AI 时代知识和经验仿佛没了价值，有了 AI 还要不要学习语言的新特性呢？或者应该问的是有了 AI 还要不要学习。
用 AI 来生成 Java 代码，比如现在的 LTS 版本是 25， AI 给你生成的代码可能还是在用  Java 17， 甚至是 Java 8, 确实能稳定工作。
很显然的一个问题是越来越少的人会关心编程语言进化，更关注的是某个大语言模型有多强，还有诸如 Vibe Coding, Prompt Engineering, 
Context Engineering, Harness Engineering, Agent Skills, [Superpower](https://github.com/obra/superpowers) 之类的概念与工具。

还是老办法，下面两个链接

1. [JDK 25 Release Notes - Major New Functionality](https://www.oracle.com/java/technologies/javase/25all-relnotes.html#JSERN24)
2. [OpenJDK JDK 25 Features](https://openjdk.org/projects/jdk/25/)

IntelliJ IDEA 对 Java 25 Language level 描述是

1. 25 - Compact source files, module imports, etc
2. 25 (Preview) - Primitive typs in patterns (3rd preview)

把上面第二个链接中的特性列出来

<div style="display: flex;font-size: 15px;">
   <div style="flex: 1;">
      <ul>
        <li>470: <a href="https://openjdk.org/jeps/470">PEM Encodings of Cryptographic Objects (Preview)</a></li>
        <li>502: <a href="https://openjdk.org/jeps/502">Stable Values (Preview)</a></li>
        <li>503: <a href="https://openjdk.org/jeps/503">Remove the 32-bit x86 Port</a></li>
        <li>505: <a href="https://openjdk.org/jeps/505">Structured Concurrency (Fifth Preview)</a></li>
        <li>506: <a href="https://openjdk.org/jeps/506">Scoped Values</a> <strong style="color: red">*</strong></li>
        <li>507: <a href="https://openjdk.org/jeps/507">Primitive Types in Patterns, instanceof, and switch (Third Preview)</a></li>
        <li>508: <a href="https://openjdk.org/jeps/508">Vector API (Tenth Incubator)</a></li>
        <li>509: <a href="https://openjdk.org/jeps/509">JFR CPU-Time Profiling (Experimental)</a></li>
        <li>510: <a href="https://openjdk.org/jeps/485">Key Derivation Function API</a></li>
      </ul>
   </div>
   <div style="flex: 1;">
      <ul>
        <li>511: <a href="https://openjdk.org/jeps/511">Module Import Declarations</a></li>
        <li>512: <a href="https://openjdk.org/jeps/512">Compact Source Files and Instance Main Methods</a> <strong style="color: red">*</strong></li>
        <li>513: <a href="https://openjdk.org/jeps/513">Flexible Constructor Bodies</a> <strong style="color: red">*</strong></li>
        <li>514: <a href="https://openjdk.org/jeps/514">Ahead-of-Time Command-Line Ergonomics</a></li>
        <li>515: <a href="https://openjdk.org/jeps/515">Ahead-of-Time Method Profiling</a></li>
        <li>518: <a href="https://openjdk.org/jeps/518">JFR Cooperative Sampling</a></li>
        <li>519: <a href="https://openjdk.org/jeps/519">Compact Object Headers</a> <strong style="color: red">*</strong></li>
        <li>520: <a href="https://openjdk.org/jeps/520">JFR Method Timing & Tracing</a></li>
        <li>521: <a href="https://openjdk.org/jeps/521">Generational Shenandoah</a></li>
      </ul>
   </div>
</div>

Java 25 中值得关注的特性不多，能主要影响到编程方式可能就只有 `Scoped Values`, 它能用来替代 `ThreadLocal` 的使用，特别是在虚拟线程当中。
其他的特性，像紧缩的源文件和即时的 `main` 方法，和更灵活的构造器不定会在正式项目中用到，影响也不大。另外，真的用考虑用 `JFR` 来替代像 `JMX`
等方式来监控程序性能了，因为随着 Java 的升级，`JFR` 支持东西越来越多。 <!--more-->

下面依然拣几个重点描述，最首要的就是 `Scoped Values`, 像 Java 21 的虚拟线程一样，重要的单列一篇来学习，见
[Java 25 新特性学习 - Scoped Values](/java-25-new-features-scoped-values/)

