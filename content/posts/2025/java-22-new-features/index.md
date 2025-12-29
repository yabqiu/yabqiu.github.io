---
title: Java 22 新特性学习
url: /java-22-new-features/
date: 2025-12-27T13:13:00-05:00
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
  - java 22
series: Java New Features
comment: true
codeMaxLines: 50
---

Java 22 是一个过渡版本, 还是到下面两个链接中找相应的更新

1. [JDK 22 Release Notes - Major New Functionality](https://www.oracle.com/java/technologies/javase/22all-relnotes.html#JSERN22)
2. [OpenJDK JDK 22 Features](https://openjdk.org/projects/jdk/22/)

IntelliJ IDEA 对 Java 22 Language level 描述是

1. 22 - Unnamed variables and patterns
2. 22(Preview) - Statements before super(), string templates (2nd preview), etc.

把上面第二个链接中的特性列出来

<div style="display: flex;">
   <div style="flex: 1;">
      <ul>
         <li>423: <a href="https://openjdk.org/jeps/423">Region Pinning for G1</a> <strong style="color: red">*</strong></li>
         <li>447: <a href="https://openjdk.org/jeps/447">Statements before super(...) (Preview)</a> <strong style="color: red">*</strong></li>
         <li>454: <a href="https://openjdk.org/jeps/454">Foreign Function & Memory API</a></li>
         <li>456: <a href="https://openjdk.org/jeps/456">Unnamed Variables & Patterns</a> <strong style="color: red">*</strong></li>
         <li>457: <a href="https://openjdk.org/jeps/457">Class-File API (Preview)</a> <strong style="color: red">*</strong></li>
         <li>458: <a href="https://openjdk.org/jeps/458">Launch Multi-File Source-Code Programs</a> <strong style="color: red">*</strong></li>
      </ul>
   </div>
   <div style="flex: 1;">
      <ul>
         <li>459: <a href="https://openjdk.org/jeps/459">String Templates (Second Preview)</a></li>
         <li>460: <a href="https://openjdk.org/jeps/460">Vector API (Seventh Incubator)</a></li>
         <li>461: <a href="https://openjdk.org/jeps/461">Stream Gatherers (Preview)</a> <strong style="color: red">*</strong></li>
         <li>462: <a href="https://openjdk.org/jeps/462">Structured Concurrency (Second Preview)</a></li>
         <li>463: <a href="https://openjdk.org/jeps/463">Implicitly Declared Classes and Instance Main Methods (Second Preview)</a></li>
         <li>464: <a href="https://openjdk.org/jeps/464">Scoped Values (Second Preview)</a></li>
      </ul>
   </div>
</div>

本文对上面用红点标记的特性重点关注  <!--more-->

### JEP 423: Region Pinning for G1

自 Java 9 开始默认垃圾收集算法由 `ParallelGC` 变为 `G1(Garbage First)` 后, 到目前 Java 25 一直默默的扮演了一个十分重要的角色. G1 
采用分代多 Region 的方式, 实现低延迟(100-200 毫米级), 且低 CPU 消耗. 现在发展出的 ZGC, 号称更低延迟(<10ms), 但完全并行式更消耗 CPU, 
在超大内存和强 CPU 下可以考虑用 ZGC.

好了, 回到 G1 的 `Region pinning for G1` 这一改进, JNI 有一类 API（例如 GetPrimitiveArrayCritical / ReleasePrimitiveArrayCritical
允许 native 代码拿到 Java 对象（常见是数组/字符串数据）的“直接指针”，并要求在这段 critical region 里 JVM 不能移动这个对象, 因为该象关联到 Off heap 的数据.
这个被操作的对象被称作 `cretical object`. 当 G1 运行时发现还有线程处于这个 `critical region` 中, 就会禁用 GC, 线程退出了该 `critical region`,
这会造成 GC 等待数分种, 进而可能造成 OutOfMemory 异常.

注意: G1 把新老代分成若干个内存区域 - region, 比如 2048 个 region, 这里的 region 与上面的 `critical region` 是两个不一样的概念, 
`critical region` 更类似于 `try/catch` 那样的监视区域.

G1 新的办法是在每个 G1 region 中维护一个 `critical object` 的引用计数, 进入 `critical region` 时计数 +1, 离开时计数 -1, 当计数 > 0
则认为 region pinned.

在 Major/Full GC 时不处理该 pinned region; 在 Minor/Young GC 时, 如果 pinned region 是年轻代, 就把该 region 提升为老年代.

这时候 G1 不用等待 `critical object` 离开 `critical region`, 只维护引用计数, 同时能处理其他的 region. 对于使用了 JNI 技术的项目值得升级到 Java 22.

### JEP 447: Statements before super(...) (Preview)

自我学 Java 开始就明白, 当继承有默认构造函数的父类, 在子类构造函数中不写的 `super()`, `super(values)` 的话会隐式调用 `super()`, 
而父类不存在默认构造函数时, 在子类构造函数的第一行就必须显示的调用用 `super(values)` 形式指定调用相应的父类构造函数. 
这是因为在创建子实例之前必须能成功创建出一个类似于不可见的父实例.

```java
class A {
   public A(int x) {
   }
}

class B extends A {
   public B() {
       super(10);   // super(10) 必须放在最前面
       int y = 5;
   }
}
```

如果换一下

```java
   public B() {
       int y = 5;
       super(10);
   }
```

就会提示

> java: call to super must be first statement in constructor

如果想要计数获得传入 A(int x) 构造函数的 x 值, 则必须调用一个静态方法

```java
   public B() {
       super(buildX());
   }

   public static int buildX() {  // buildX() 方法必须为静态
         return 10;
   }
```

因为假如 `buildX()` 是一个实例方法, 在调用构造函数 `B()` 时相应的实例都还不存在.

而这一特性是允许下面的语法

```java
class B extends A {
    public B() {
        int y = 5;
        super(y);
    }
}
```

但想要在构造函数中调用自身的实例方法仍然不可行

```java
    public B() {
        super(buildX());
    }

    public int buildX() {
        return 10;
    }
```

> java: cannot reference buildX() before supertype constructor has been called

道理还是一样的, 在调用 `B()` 时, 它的实例还不存在.

子构造函数在调用父构造时 `super(values)` 不能放在条件语句, 必须明确是谁.

其实不光调用 `super(values)` 方法, `this(values)` 也适用该新规则

```java
// Java 22 之前
class B {
    public B() {
        int x = 10;
        this("" + x);  // 报错: java: call to this must be first statement in constructor
    }

    public B(String s) {
    }
}
```

```java
// Java 22 开始
class B {
    public B() {
        int x = 10;
        this("" + x);   // 这是合法的
    }

    public B(String s) {
    }
}
```

### JEP 456: Unnamed Variables & Patterns

`_` 一直以来只是一个普通的变量名, 如

```java
int _ = 10;
```

但是 `_` 在 Java 22 开始某些时候赋予了新的隐式约定, 当然它依然可以作为普通变量来使用. 比如有时候必须有命名变量, 但无需使用的时候

{{< bundle-image unused-variable.png 600 >}}

IntelliJ IDEA 会提示命名为 `ignored` 来避免该提示

在 Java 8 的 `for` 循环中还能用 `_`  作为变量名

{{< bundle-image unnamed_variable_in_java8.png 600 >}}

但到了 `_` 变成了一定含义的保留字, `for(String _: orders)` 在 Java 9 中就不合法了, 错误信息是

> java: as of release 9, '_' is a keyword, and may not be used as an identifier

在该 JEP 456 特性之下, 也不用写成 `ignore` 了, 只要改成

```java
   for (String _ : orders) {
       total++;
       //System.out.println(_); // underscore not allowed here
   }
```

这里的 `_` 不仅仅是有 `ignore` 的含义, 而且并确禁止对 `_` 的访问, 所谓是彻底 Ignore.

所以还要回过头来看 `int _ = 10`, 其实它已不是 Java 8 及之前的 `_` 了, 因为无法通过 `_` 来访问了, 所以有下面的错误

```java
 int _ = 10;
 System.out.println(_); // 错误: underscore not allowed here
```

还有其他很多时候的 `unused` 都可以使用 `_`, 比如

```java
List<String> list = new ArrayList<>();
boolean _ = list.add("Hello");
String _ = list.get(0);   // _ 已经不是一个普通变量了, 这里没有 _ 重复定义错误

try (var _ = driverManager.getConnection("url")) {
    System.out.println("database is available");
} catch (Exception _) {
}

if(obj instanceof Point(int _, int y)) {
    System.out.println("It's a point with y = " + y);
}

switch (obj) {
    case Point(int x, int _):
        System.out.println("It's a point with x = " + x);
    case null:
        System.out.println("It's null");
}
```

### JEP 457: Class-File API (Preview)

这一特性让我们更容易解析生成的 class 文件, 给 Java 反编译工具, 和 ASM 相关库创造了便利, 像 [ASM](https://asm.ow2.io/), 
[BCEL](https://commons.apache.org/proper/commons-bcel/), 和 [Javassist](https://www.javassist.org/).

由于 Java 每 6 个月的发布周期, 类文件格式可能也变化太快, 第三方的 ASM 工具怕是难以跟上步伐, 所以 Java 自己发布一套 Class-File API.
这也是像 JVM TI(Tool Interface) 一样是给第三方工具用的, 这里不展开研究.

### JEP 458: Launch Multi-File Source-Code Programs

在正式的使用了 `Maven` 或其他构建/依赖管理的项目, 这一特性不会用到. 这也是让 Java 本身进一步脚本化执行更简单. 它的作用是让 Java
在执行源文件时自动找到相关的源文件进行编译, 如下两个文件

```java
// Prog.java
class Prog {
    public static void main(String[] args) {
        Helper.run();
    }
}
```

```java
// Helper.java
class Helper {
    static void run() {
        System.out.println("Hello!");
    }
}
```

不需 `Prod` 和 `Helper` 写在同一个文件中, 只用

```sh
java Prog.java
```

Java 就能分析 `Prog.java` 中的引用, 找到文件 `Helper.java`, 在内存中编译 `Prog.java` 和 `Helper.java` 并执行.

还有一种更聪明的 `---class-path` 写法, 假如当前目录下的结构是

```text
.
├── Helper.java
├── Prog.java
└── libs
    ├── library1.jar
    └── library2.jar
```

执行命令

```sh
java --class-path 'libs/*' Prog.java
```

不仅能在当前目录中找到 `Helper.java`, `--class-path 'libs/*'` 会把 `libs/` 中所有的 JAR 文件加入到 classpath 下.

搜索源文件也遵循相同的 `package` 规则, 假如 `Helper.java` 的代码是

```java
package pkg;
class Helper {
    static void run() {
        System.out.println("Hello!");
    }
}
```

则应把 `Helper.java` 源文件放到 `pkg` 目录下.

### JEP 461: Stream Gatherers (Preview)

最后初步学习一下 `Stream Gatherers`, 它将在 JDK 24 中转正.

Java 觉得目前 `Stream` 存在的那些中间操作方法(Intermediate Operations) 还不足以满足某些需求, 也不够灵活. 例如 Stream 提供了基于 Object
整体的去重 `distinct()` 方法, 但无法基于某一属性去重, 所以先前研究过曲线解决方式 [Java 8 根据属性值对列表去重](/java-8-list-deduplication-with-lambda/).

在该 JEP 中举的例子是

```java
var result = Stream.of("foo", "bar", "baz", "quux")
                   .distinctBy(String::length)      // 现在没有这个方法
                   .toList();
```

Java 对些的解决办法是, 类似于所有的 Stream 的 `terminal operation` 都可以用 `collect/reduce` 操作来实现; 同等的, 对于所有的
`intermediate operation`, 也应该有一个终极方法, 那就是 `Stream::gather(Gatherer)`.

`collect`, `reduce` 在某些编程语言中叫做 `fold` 方法, 不明白为什么 Java 的 `Stream` 要同时有 `collect` 和 `reduce`, 相比
`reduce` 比 `collect` 更抽象一些.

`java.util.stream.Gatherer` 接口中定义了一些方法, 同时在 `java.util.stream.Gatherers` 中提供几个实现

{{< bundle-image stream-gatherers.png 600 >}}

这下可好, 把类似于 Flink 中的固定, 滑动窗口等概念引进来了. 因为 `gather` 是万能的 `intermediate` 方法, 而 `terminal` 方法只能有一个,
所以任何 Stream 操作都可以用

```java
stream.gather(...).gather(...).gather(...).collect(...);
```

这里不详细解释 `Stream Gatherers`, 现在只看下如何用 `gather` 方法解决老问题 [Java 8 根据属性值对列表去重](/java-8-list-deduplication-with-lambda/).

```java
List<Book> uniqueBooks =  books.stream()
        .gather(Gatherers.fold(
                () -> new HashMap<Integer, Book>(),
                (map, book) -> {
                    map.putIfAbsent(book.id, book);
                    return map;
                }
        ))
        .flatMap(map -> map.values().stream())
        .toList();
```

也没感觉到更优雅, 还不如只用 filter, 借助一个外部 Set<Integer> uniqueIds 集合.

```java
Set<Integer> uniqueIds = new HashSet<>();
List<Book> uniqueBooks = books.stream().filter(book -> uniqueIds.add(book.id)).toList();
```

随着 `filter` 的操作, `uniqueIds` 中的内容也是动态, 这比当初 [Java 8 根据属性值对列表去重](/java-8-list-deduplication-with-lambda/)
还要好的多.

这个具体问题, 或者从基础创建自己的 Gather 看起来很复杂, 但 `Gather` 接口和 `Stream.gather()` 势将第三方实现功能强大的 Gather 很大的便利,
例如将可能只需

```java
List<Book> uniqueBooks = book.stream.gather(gathers.distinctBy(Book::id)).toList();
```

其他的特性如 `454: Foreign Function & Memory API` 等着象 `JNA`, `JNR-FFI` 等第三方组件使用这一特性实现出更高效的 JNI 调用代码.

至于 `Vector API`, 不知道它将来如何与 Python 的 SciPy, NumPy 比, 灵活性肯定不好, 效果上 Python 的 Vector API 可是 C/C++ 实现的.