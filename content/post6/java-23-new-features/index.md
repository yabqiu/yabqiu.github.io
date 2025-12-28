---
title: Java 23 新特性学习
url: /java-23-new-features/
date: 2025-12-27T23:21:00-05:00
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
  - java 23
series: Java New Features
comment: true
codeMaxLines: 50
showLastmod: true
---
 
Java 24 也是一个过渡版本, 还是到下面两个链接中找相应的更新

1. [JDK 23 Release Notes - Major New Functionality](https://www.oracle.com/java/technologies/javase/23all-relnotes.html#JSERN23)
2. [OpenJDK JDK 23 Features](https://openjdk.org/projects/jdk/23/)

IntelliJ IDEA 对 Java 22 Language level 描述是

1. 23 - Markdown document comments
2. 22(Preview) - Primitive types in patterns, implicitly declared classes, etc.

把上面第二个链接中的特性列出来

<div style="display: flex;">
   <div style="flex: 1;">
      <ul>
         <li>455: <a href="https://openjdk.org/jeps/455">Primitive Types in Patterns, instanceof, and switch (Preview)</a> <strong style="color: red">*</strong></li>
         <li>466: <a href="https://openjdk.org/jeps/466">Class-File API (Second Preview)</a></li>
         <li>467: <a href="https://openjdk.org/jeps/467">Markdown Documentation Comments</a> <strong style="color: red">*</strong></li>
         <li>469: <a href="https://openjdk.org/jeps/469">Vector API (Eighth Incubator)</a></li>
         <li>473: <a href="https://openjdk.org/jeps/473">Stream Gatherers (Second Preview)</a></li>
         <li>471: <a href="https://openjdk.org/jeps/471">Deprecate the Memory-Access Methods in sun.misc.Unsafe for Removal</a> <strong style="color: red">*</strong></li>
      </ul>
   </div>
   <div style="flex: 1;">
      <ul>
         <li>474: <a href="https://openjdk.org/jeps/474">ZGC: Generational Mode by Default</a> <strong style="color: red">*</strong></li>
         <li>476: <a href="https://openjdk.org/jeps/476">Module Import Declarations (Preview)</a></li>
         <li>477: <a href="https://openjdk.org/jeps/477">Implicitly Declared Classes and Instance Main Methods (Third Preview)</a></li>
         <li>480: <a href="https://openjdk.org/jeps/463">Structured Concurrency (Third Preview)</a></li>
         <li>481: <a href="https://openjdk.org/jeps/481">Scoped Values (Third Preview)</a></li>
         <li>482: <a href="https://openjdk.org/jeps/482">Flexible Constructor Bodies (Second Preview)</a></li>
      </ul>
   </div>
</div>

本文对上面用红点标记的特性重点关注  <!--more-->

Java 23 中真正意义上的新特性也就是一个  `Markdown Documentation Comments`, 但对于源码即文档的习惯又很少写文档注释. 基本上 Java 23
就是其余特性的 X Preview, 或 X Incubator, 那些 Preview/Incubator 的特性以前也有研究过, 所以本文的篇幅预计不会太长.

首先两个不需要太多解释的小特性

### JEP 471: Deprecate the Memory-Access Methods in sun.misc.Unsafe for Removal

就是 `sun.misc.Unsafe` 下的一些方法不要再去用了, 就是看到这个 `Unsafe` 就要立即警觉起来.

### JEP 474: ZGC: Generational Mode by Default

如果要使用 `ZGC`, 用 JVM 参数  `-XX:+UseZGC`, 那么这时候默认就是用分代的 `ZGC`, 不需加 `-XX:+ZGenerational`, 除非你还想用非分代的
`ZGC`, 那就是加上 `-XX:-ZGenerational`, 但到了下一个 Java 24 版本, 不再有非分代的 `ZGC` 了, 加 `-XX:+ZGenerational` 或
`-XX:-ZGenerational` 都会收到警告信息.

### JEP 467: Markdown Documentation Comments 

踏入最具代表性的 Java 23 更新, 可用 Markdown 写文档注释了. 这是我近来最熟悉的了, 忙了几周从 WordPress 到 Hugo 的搬迁, 终于可以舒舒服服的用
Markdown 写日志了.

原来的文档注释写在 `/** */` 中, 现在新的 Markdown 文档注释采用全新的和 Rust 语言一样的 `///` 文档注释. 怀旧的依然可以用老的 `/** */` 方式.
采用 Markdown 的 `///` 格式后, 一般都不用 HTML 标签了, `<p>`, `<ul>`, `<li>` 在 Markdown 中都有自己的写法, 如在 Markdown 中空一行就是一个新段落,
列表有好几种写法, 如 `1. 2. ...`, 如 `-`, 代码片断不用写成 `<pre>{@code ... }</pre>`, 或者较新的 `{@snippet: ... }`, 而简单用
<code>```</code> 就行了. 

反正就着 Markdown 的习惯试就行, 如果预览中不如预期再调整. 其他的如 `@implSec`, `@return`, `@param`, `@see` 和原来一样. 
`{@link java.util.HashMap}` 在 Markdown 中写成 `[java.util.HashMap]`

Java Markdown 文档注释是依据这个 [CommonMark](https://spec.commonmark.org/0.30) 版本. 与 Hugo 的 Markdown 有些差异. 比如 Hugo
文本与链接写法是 `[text content](link)`, 而 Java Markdown 文本链接是用两个中括号的形式, `[text_content][link]`

#### 尝试文档注释中的语法高亮

如下文档注释
```markdown
///
/// ```java
/// int sum = widgets.stream()
///                  .filter(w -> w.getColor() == RED)
///                  .mapToInt(w -> w.getWeight())
///                  .sum();
/// ```
///
```

用命令 `javadoc *.java` 生成的 HTML 中形成代码块, 但还不能真正称作语法高亮

{{< bundle-image javadoc-syntax-highlight1.png 460 >}}

看它生成的 HTML 代码是

```html
<pre><code class="language-java">...</code></pre>
```

而真正能语法亮起来还得做些额外的步骤, 有了上面生成的 HTML, 可用 `Prism` 或 `highlight.js` 等来进行语法高亮.

选择 Prism 的 [1.30.0](https://cdnjs.com/libraries/prism/1.30.0) 版本, 分别下载

```shell
wget https://cdnjs.cloudflare.com/ajax/libs/prism/1.30.0/prism.min.js
wget https://cdnjs.cloudflare.com/ajax/libs/prism/1.30.0/components/prism-java.min.js
wget https://cdnjs.cloudflare.com/ajax/libs/prism/1.30.0/themes/prism.min.css
```

生成 `javadoc` 文档中用如下命令

```shell
javadoc --add-script prism.min.js  --add-script prism-java.min.js --add-stylesheet prism.min.css  *.java
```

现在查看 JavaDoc 页面显示就漂亮了

{{< bundle-image javadoc-syntax-highlight2.png 468 >}}

或者用 `prism-autoloder` 的方式, 那么需要用 `wget` 下载

```shell
wget https://cdnjs.cloudflare.com/ajax/libs/prism/1.30.0/components/prism-core.min.js
wget https://cdnjs.cloudflare.com/ajax/libs/prism/1.30.0/plugins/autoloader/prism-autoloader.min.js
wget https://cdnjs.cloudflare.com/ajax/libs/prism/1.30.0/components/prism-clike.min.js
wget https://cdnjs.cloudflare.com/ajax/libs/prism/1.30.0/components/prism-java.min.js
wget https://cdnjs.cloudflare.com/ajax/libs/prism/1.30.0/themes/prism.min.css
```

```shell
javadoc --add-script prism-core.min.js --add-script prism-autoloader.min.js --add-script prism-clike.min.js \ 
   --add-script prism-java.min.js --add-stylesheet prism.min.css  *.java
```

当你在代码中使用了别的语法, 如 <code>```css</code>, 从浏览器的 Inspect 中可以看到它试图加载哪个语法的 JS 文件, 这时候可去
[https://cdnjs.com/libraries/prism/1.30.0](https://cdnjs.com/libraries/prism/1.30.0) 找到需要的 JS 文件, 下载并加到 `javadoc`
命令中.

还有一种更简单的语法高亮方式只要运行 `javadoc` 命令时加上 `--syntax-highlight` 参数

```shell
javadoc --syntax-highlight *.java
```

{{< bundle-image javadoc-syntax-highlight3.png 468 >}}

这是最简单的方式, 这是使用的 `Highlight.js`.

而且在 IntelliJ IDEA 已对 Markdown 的代码块很友好

{{< bundle-image javadoc-syntax-highlight3.png 480 >}}

另外, Java 还提供了相应的 JavaDoc API, 在 [com.sun.source.doctree](https://docs.oracle.com/en/java/javase/22/docs/api/jdk.compiler/com/sun/source/doctree/package-summary.html)
包的 [Compiler Tree API](https://docs.oracle.com/en/java/javase/22/docs/api/jdk.compiler/module-summary.html#CompilerTreeAPI).

`com.sun.source.doctree` 是在 Java 1.8 加入, 那时候 Java 已经卖身给了 Oracle, 为何还沿用了 `com.sun` 的包名.

通过 `com.sun.source.doctree` API 可以编程方式读取源代码中的 Markdown 注释, 比如原始注释, 渲染后的内容等.

### JEP 455: Primitive Types in Patterns, instanceof, and switch (Preview)

Java 终于在考虑在某些时候对原始类型的支持了, 可惜泛型还不支持原始类型.

在 switch 可判断 `int` 类型并获得值

```java
switch (x.getYearlyFlights()) {
    case 0 -> ...;
    case 1 -> ...;
    case 2 -> issueDiscount();
    case int i when i >= 100 -> issueGoldCard();
    case int i -> ... appropriate action when i > 2 && i < 100 ...
}
```

在 `if instanceof` 中

```java
    int n = 10;
    if (n instanceof int && n > 5) {
        System.out.println("n is an integer greater than 5");
    } else {
        System.out.println("n is not an integer greater than 5");
    }
```

输出

> n is an integer greater than 5

但仔细一想, 原始类型作 `case int i` 或 `if(n instanceof int)` 有什么意义呢? 因为它们的类型总是确定的, 比如说前面定义的变量是
`int n = 10`, 或者是方法传入的参数类型 `foo(long n)`, 不可能有

```java
    switch (n) {
        case int x -> System.out.println("x is an integer: " + x);
        case long x -> System.out.println("x is an integer: " + x); // 错误: duplicate unconditional pattern
    }
```

不过下面的语句却能正常执行

```java
    Integer n = 10;

    switch (n) {
        case int x -> System.out.println("x is an integer: " + x);
        case long x -> System.out.println("x is an integer: " + x);
    }
```

语法上没问题, 只是光标放在 `int x` 上会显示

> Switch label 'int x' is the only reachable in the whole switch  
> Remove unreachable branches

执行后的输出是

> x is an integer: 10

加个  `Integer` 的类型判断会如何呢?

```java
   Integer n = 10;

   switch (n) {
       case int x -> System.out.println("x is an integer: " + x);
       case Integer x -> System.out.println("x is an Integer: " + x);
       case long x -> System.out.println("x is an integer: " + x);
   }
```

输出依旧为

> x is an integer: 10

这就有问题了, n 显示是一个 `Integer`, 但在执行 `case int x` 进行了拆箱, 所以被认为是 `int` 而进到第一个分支了.

但要是交换 `case int x` 和 `case Integer x` 这两行, `case int x` 就会认识到自己错了

```java
   Integer n = 10;

   switch (n) {
       case Integer x -> System.out.println("x is an Integer: " + x);
       case int x -> System.out.println("x is an integer: " + x); // 错误: this case label is dominated by a preceding case label
       case long x -> System.out.println("x is an integer: " + x);
   }
```

可是去掉 `case int x` 这一行语法又是合法的

```java
    Integer n = 10;

    switch (n) {
        case Integer x -> System.out.println("x is an Integer: " + x);
        case long x -> System.out.println("x is an integer: " + x);
    }
```

输出为

> x is an Integer: 10

看来, Java 在某些地方支持原始类型还真不容易啊.

### 其他一些更新

#### javac 的注释处理默认被禁用

像 `Lombok` 就是使用的 `Annotation processing`, 升级的时候要留意下. 如果要保持原来的行为就加上 `-proc:full` 编译参数. 最新的 Maven
Complier 插件也支持该属性配置, 或者也可用 `-Dmaven.compiler.proc=full` 参数.

#### java.io.Console` 添加了几个带 `Locale` 参数的重载方法

- public Console format(Locale locale, String format, Object ... args)
- public Console printf(Locale locale, String format, Object ... args)
- public String readLine(Locale locale, String format, Object ... args)
- public char[] readPassword(Locale locale, String format, Object ... args)

#### Duration 新加了两个 `until` 方法

- Duration until(Instant endExclusive)
- long until(Temporal endExclusive, TemporalUnit unit)

它可得到与 `Duration.between(Temporal, Temporal)` 相同的结果, 比如下面的应用

```java
   Instant start = Instant.now();
   Thread.sleep(500);
   Duration diff = start.until(Instant.now());
   System.out.println("Elapsed time: " + diff.toMillis() + " ms");
```

#### 新的 Parallel GC Full GC 算法

Parallel GC 再处理 Full GC 时采用了与 Serial GC, G1 GC 一样的算法, 再说一样, 也不会像 G1 那样划分许多 Region, 况且如今都用上了 G1,
谁还在乎 Parallel GC 怎么进步呢.

#### Thread, ThreadGroup 中从来不建议使用的方法终于移除了

移除的方法有

- Thread.suspend()
- Thread.resume()
- ThreadGroup.stop()

`Thread.stop()` 方法目前还在, 在 JDK 20 时直接在方法中抛出 `UnsupportedOperationException()`, 不让使用, 预计在 JDK 26 中移除.