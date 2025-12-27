---
title: Java 21 虚拟线程外其他新特性
url: /java-21-new-features-other-than-virtual-thread/
date: 2025-12-27T01:04:00-05:00
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

- 430: [String Templates (Preview)](https://openjdk.org/jeps/430) *
- 431: [Sequenced Collections](https://openjdk.org/jeps/431) *
- 439: [Generational ZGC](https://openjdk.org/jeps/439) *
- 440: [Record Patterns](https://openjdk.org/jeps/440) *
- 441: [Pattern Matching for switch](https://openjdk.org/jeps/441) *
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

在 IntelliJ IDEA 中标识 Java 21 的 Language level: `Record patterns, pattern matching for watching`, Java 21 Prview:
`String templates, unnamed classes and instance main methods, etc.`. 奇怪的是为什么不提 `Virtual Threads`

本文主要学习的是正式的新特性 431 Sequenced Collections, 439 Generational ZGC, 440 Record Patterns, 441 Pattern Matching for switch,
并且提前了解一下 430 String Templates.

### 430: String Templates

这是一个预备学习, 该特性直到 JDK 26 也没见到定稿, 也有可能终将放弃. 大概来看一下它要解决什么问题.简单来讲就是它想要实现其他编程语言中的字符串插值功能,
即 `String interpolation`. Java 不能满足于用加号, StringBuilder, String.format(), str.formatted(), 或 MessageFormat() 等方式来构造字符串,
希望能有像 Python F-String 功能, 如 `f"{x} plus {y} equals {x + y}"`.

`String interpolaton` 在不少流行语言中都支持, 如 C#, Python, Scala, Groovy, Kotlin, JavaScript, Ruby, Swift 等.

`String Teamplates` 有三个重要 API, 它们分别是

1. java.lang.StringTemplate.STR;
2. java.lang.StringTemplate.RAW;
3. java.util.FormatProcessor.FMT;

其中第一个是可以直接在源码中用 `STR`, 像 `Integer` 一样会被自动引入. 其他两个要显示用 `import` 引入. 看它们的定义

```java
Processor<String, RuntimeException> STR = StringTemplate::interpolate;
Processor<StringTemplate, RuntimeException> RAW = st -> st;
public static final FormatProcessor FMT = FormatProcessor.create(Locale.ROOT);
```

综合看它们的各种用法

```java
String name = "Joan";
String info = STR."My name is \{name}, age \{20 + 5}, fullname: \{"DOE, " + name.toUpperCase()}";
System.out.println("1\n" + info);

StringTemplate st = RAW."My name is \{name}, age \{20 + 5}, fullname: \{"DOE, " + name.toUpperCase()}";
System.out.println("2\n" + STR.process(st));
System.out.println("3\n" + st.toString());

Integer a = 10;

String msg = STR."file exists: \{
        new File("test.").exists() ?
                "yes" : "no"
        }";
System.out.println("4\n" + msg);

String[] names = {"John", "Jane", "Doe"};
String html = STR."""
        <html>
            <body>
                <h1>Hello, \{names[1]}!</h1>
                <p>Welcome to the Java 21 world.</p>
            </body>
        """;
System.out.println("5\n" + html);

Object[] data = {"Alice", 30, "Engineer"};
String formated = FMT."""
        Name     age   profession
        %-8s\{data[0]} %-5d\{data[1]} %-12s\{data[2]}
        """;
System.out.println("6\n" + formated);
```

执行后输出为

```text
1
My name is Joan, age 25, fullname: DOE, JOAN
2
My name is Joan, age 25, fullname: DOE, JOAN
3
StringTemplate{ fragments = [ "My name is ", ", age ", ", fullname: ", "" ], values = [Joan, 25, DOE, JOAN] }
4
file exists: no
5
<html>
    <body>
        <h1>Hello, Jane!</h1>
        <p>Welcome to the Java 21 world.</p>
    </body>

6
Name     age   profession
Alice    30    Engineer   
```
很容易对照理解 `STR`, `RAW`, 和 `FMT` 的用法, 用 `\{}` 在单行或多行字符串中插入各种行式表达式的值. `\{}` 即使在单行的字符串中也能使用多行形式的表达式.
`STR` 好比 Python 的 `f-string`, 只是 Python 的 `f-string` 兼具 Java 的 `FMT` 的功能, 而 `RAW` 就像是 Python `t-string`.

像 `STR.""`, `RAW.""`, 和 `FMT.""` 的形式似是一种特殊的语法数糖, 如果反编译如下的代码

```java {linenos=table}
String name = "Joan";
var x = STR."Hello, \{name}!";  // 对应如下字节码的第 4 行
var y = RAW."Hello, \{name}!";  // 对应如下字节码的第 7, 9 行
var z = FMT."Hello, \{name}!";  // 对应如下字节码的第 11 行
```
字节码为

```java {linenos=table}
 0: ldc           #7       // String Joan
 2: astore_1
 3: aload_1
 4: invokedynamic #9,  0   // InvokeDynamic #0:makeConcatWithConstants:(Ljava/lang/String;)Ljava/lang/String;
 9: astore_2
10: aload_1
11: invokedynamic #13,  0  // InvokeDynamic #1:process:(Ljava/lang/String;)Ljava/lang/StringTemplate;
16: astore_3
17: getstatic     #17      // Field java/util/FormatProcessor.FMT:Ljava/util/FormatProcessor;
20: aload_1
21: invokedynamic #23,  0  // InvokeDynamic #2:process:(Ljava/util/FormatProcessor;Ljava/lang/String;)Ljava/lang/String;
26: astore        4
```

`StringTemplate` 是一个接口, 我们看它定义的静态方法

{{< bundle-image java-21-new-features-string-template.png 600 >}}

我们能够手动通过指定 fragments 和 values 来构造一个 StringTemplate, `StringTemplate.Processor` 是一个函数式接口, 它定义的唯一抽象方法是

```java
R process(StringTemplate stringTemplate) throws E;
```
它是一个泛型方法, 返回值可为任意类型, 所以这里就可以像 Python 的 `t-string` 可定制了, 比如类似下面的方式

```java
   StringTemplate st = null;  // create a StringTemplate instance
   List<Object> ret = st.process(s -> {
       List<Object> result = new ArrayList<>();
       result.add(s.fragments()); // process fragments
       result.add(s.values());  // process values
       return result;
   });
```

从 StringTemplate 中可得到任意的数据类型.

最后使用 String Template 时要注意安全, 小心像 SQL Injection 那样被注入了恶意代码.

### 431: Sequenced Collections

为了某些内部维持有顺序的集合操作便利, 在集合层次关系中插入了几个新的接口 `SequencedCollection`, `SequencedSet`, 和 `SequencedMap`

```java
interface SequencedCollection<E> extends Collection<E> {
    // new method
    SequencedCollection<E> reversed();
    // methods promoted from Deque
    void addFirst(E);
    void addLast(E);
    E getFirst();
    E getLast();
    E removeFirst();
    E removeLast();
}
```

```java
interface SequencedSet<E> extends Set<E>, SequencedCollection<E> {
    SequencedSet<E> reversed();    // covariant override
}
```

```java
interface SequencedMap<K,V> extends Map<K,V> {
    // new methods
    SequencedMap<K,V> reversed();
    SequencedSet<K> sequencedKeySet();
    SequencedCollection<V> sequencedValues();
    SequencedSet<Entry<K,V>> sequencedEntrySet();
    V putFirst(K, V);
    V putLast(K, V);
    // methods promoted from NavigableMap
    Entry<K, V> firstEntry();
    Entry<K, V> lastEntry();
    Entry<K, V> pollFirstEntry();
    Entry<K, V> pollLastEntry();
}
```

它们在类层次中的位置是

{{< bundle-image SequencedCollectionDiagram20220216.png 800 >}}

针对相应的集合类型, 某些操作更直截了当了, 如对 List 取首尾元素不再用 `list.get(0)`(或 `list.iterator().next()`), 
和 `list.get(list.size() - 1)`, 用新的 API `list.getFirst()` 和 `list.getLast()`

### 439: Generational ZGC - 分代式 ZGC

自 G1 GC 在 Java 7 Update 4 正式可以, Java 9 才开始作为默认 GC, 而作为下一代的 ZGC 在 Java 15 可正式使用, 但在 Java 25 中仍默认使用 G1 GC.
GC 算法一直在演化, 与之相关的概念有串行, 并行, 标识清除, 分代, 分 Region, 在时间与空间中寻找平衡. 原来时常感受到 STW(Stop The World),
现代的 GC 算法更追求的是低延迟, 现在有了 G1(Garbage First), ZGC 新垃圾回收算法 

不知道为什么叫做 ZGC, 只找到了 JEP 333 的标题是 JEP 333: ZGC: A Scalable Low-Latency Garbage Collector (Experimental)

下面简单对比 G1 和 ZGC

1. G1 仍用分代收集, Young Generation 和 Old Generation, 但它划分了若干 Region, 进行可预测暂停, 渐进式回收. 支持的堆大小在 4-64GB 之间, 
   可接受暂停时间在 100-200ms 之间. 比 ZGC 拥有更高的吞量
2. ZGC 几乎所有工作都并发进行, 使用染色指针(Colored Pointers), 读屏障(Load Barriers) 技术, 支持更短的暂停时间(< 10ms),
   支持超大堆内存(16TB). 需要足够的 CPU 资源.

ZGC 在 Java 21 之前是非分代收集, 如果要启用 ZGC 和分代式 ZGC, 要使用启动参数

```java
-XX:+UseZGC -XX:+ZGenerational
```

Java 21 在选用 ZGC 时, 默认仍然是不分代的, 要到 Java 23 选择 ZGC 时默认使用分代式收集, 所以从 Java 23 开始只要 `-XX:+UseZGC`.

说到 GC, 这里有个疑问, 为什么 Java 程序会更有意识的去了解不同的 GC 算法, 而其他带 GC 的语言, 如 C#, 及各种脚本型的语言 Python, Node.js,
Ruby, 甚至 Go, 就好像 GC 不存在一般, 没人真正去关心, 或者它们的历史和如何持续改进的.

### 440: Record Patterns - 记录类与模式匹配

`Record` 自 Java 16 出现以来, 似乎普及率不高, 因为它实现的是一个不可变的数据类, 不及 `Lombok` 注解生成的数据类灵活. Java 的 `enum`, 
`Sealed class`, `Record` 在应用于模式匹配时, 三个都打不过 Rust 的 `enum`.

记录类支持模式匹配是用
`if`, `switch-case` 语句中不仅能进行类型判断, 还能提取其中的值, 如

```java
record Point(int x, int y) {}

Object obj = new Point(10, 20);
if (obj instanceof Point(int x, int y)) {
    System.out.println("Point coordinates: x=" + x + ", y=" + y);
}
```

switch 语句中

```java
switch (obj) {
    case Point(int x, int y) when x == y -> {
        System.out.println("Point coordinates: x=" + x + ", y=" + y + ", x equals y");
    }
    case Point(int x, int y) -> {
        System.out.println("Point coordinates: x=" + x + ", y=" + y);
    }
    case null, default ->  {
        System.out.println("default point coordinates");
    }
}
```

还支持嵌套拆解

```java
record Rectangle(Point upperLeft, Point lowerRight) {}

Object obj = new Rectangle(new Point(0, 0), new Point(400, 300));

if (obj instanceof Rectangle(Point(int x1, int y1), var lowerRight)) {
    System.out.println("Rectangle from (" + x1 + ", " + y1 + ") to (" + lowerRight.x() + ", " + lowerRight.y() + ")");
}
```

### 441: Pattern Matching for switch

Java 这种渐进式加进来的特性很容易被人忽略, 如果正在使用一个像 IntelliJ IDEA 那样智能的 IDE, 很多语言特性都是靠 IDE 推荐的, 不像 JDK 5
的泛型, Java 8 的 Lambda/Stream 那种革命性的变化. 其实 Java 后来新加的一些小特性, 像 record, sealed class, pattern matching,
甚至是 `var` 类型推断都是悄无声息的进来, 带不起半点涟漪.

Java 21 对 switch/case 模式匹配的增加是把原来要 `if (obj instance of Integer i)` 的写法加到了 switch/case 语法中. 下面有些地方
直接引用了 JEP 441 中的示例代码

```java
// Java 21 之前
static String formatter(Object obj) {
    String formatted = "unknown";
    if (obj instanceof Integer i) {
        formatted = String.format("int %d", i);
    } else if (obj instanceof Long l) {
        formatted = String.format("long %d", l);
    } else if (obj instanceof Double d) {
        formatted = String.format("double %f", d);
    } else if (obj instanceof String s) {
        formatted = String.format("String %s", s);
    }
    return formatted;
}
```

```java
// Java 21 开始
static String formatterPatternSwitch(Object obj) {
    return switch (obj) {
        case Integer i -> String.format("int %d", i);
        case Long l    -> String.format("long %d", l);
        case Double d  -> String.format("double %f", d);
        case String s  -> String.format("String %s", s);
        default        -> obj.toString();
    };
}
```

#### 加入了 `case null` 支持

```java
// Java 21 开始
static void testFooBarNew(String s) {
    switch (s) {
        case null         -> System.out.println("Oops");
        case "Foo", "Bar" -> System.out.println("Great");
        default           -> System.out.println("Ok");
    }
}
```

#### case 中加入了 when 条件判断

这样在 case 条件可以更细化了, 其实在介绍 Record 模式匹配时就用到了.

```java
static void testStringEnhanced(String response) {
    switch (response) {
        case null -> { }
        case "y", "Y" -> {
            System.out.println("You got it");
        }
        case "n", "N" -> {
            System.out.println("Shame");
        }
        case String s when s.equalsIgnoreCase("YES") -> {
            System.out.println("You got it");
        }
        case String s when s.equalsIgnoreCase("NO") -> {
            System.out.println("Shame");
        }
        case String s -> {
            System.out.println("Sorry?");
        }
    }
}
```

#### switch 匹配与 sealed class

```java
    sealed interface S permits A, B, C {}
    final class A implements S {}
    final class B implements S {}
    record C(int i) implements S {}    // Implicitly final

    static int testSealedExhaustive(S s) {
        return switch (s) {
            case A a -> 1;
            case B b -> 2;
            case C c -> 3;
        };
    }
```

关于模式匹配应该大胆去尝试自己想当然的写法, 试探 Java 的底线.

### 445: Unnamed Classes and Instance main Method(Preview)

提一下这个特性, 觉得也不是很重要, 这是给 Java 初学者的一个便利. 有不少新语言学习者时常抱怨写一个 Java 的 Hello World 为何要写这么多行代码,
希望象所有的其他脚本语言的 Hello World 基本就是一行, 如 Python 的

```python
print("Hello World")
```

而 Java 里要写成

```java
public class HelloWorld { 
    public static void main(String[] args) { 
        System.out.println("Hello, World!");
    }
}
```

可以写成
```java
// main.java
void main() {
   System.out.println("Hello, World!");
}
```

编译之后的类文件名是源文件对应的 `main.class`, 代码反编译后为

```java
final class main {
    void main() {
        System.out.println("Hello, World!");
    }
}
```

如果源代码文件名为 `Main.java`, 则生成的类名为 `Main.class`, 在 Maven 项目中即使文件路径是 `src/main/java/org/example/Main.java`,
生成的类仍能是 `target/classes/Main.class`, 并且在 Main.java 不能使用 `package` 声明.

不过可以声明实例变量, 如下面的 `Main.java`

```java
String greeting = "Hello, World!";

void main() {
    System.out.println(greeting);
}
```

产生的类 `Main.class` 反编译后的代码是

```java
final class Main {
    String greeting = "Hello, World!";

    void main() {
        System.out.println(this.greeting);
    }
}
```

当然也能用 `java Main.java` 直接运行. 只是这种特性在正式的项目中是不会采用的.

### 其他值得一提的杂项变化

##### `java.lang.Character` 加入了 `Emoji` 的相关方法

- isEmoji(int codePoint)
- isEmojiPresentation(int codePoint)
- isEmojiModifier(int codePoint)
- isEmojiModifierBase(int codePoint)
- isEmojiComponent(int codePoint)
- isExtendedPictographic(int codePoint)

##### String 和 java.util.regex.Pattern 中新加了 `splitWithDelimiters()` 方法

它与 `split()` 方法不同的是, 不仅返回切分后的字符串结果, 还返回分一处具体匹配到的分隔符, 如

```java
    String[] strings = "a:b::c:::d".splitWithDelimiters(":+", 5);
    System.out.println(String.join(", ", strings));
```

输出为

> a, :, b, ::, c, :::, d

##### StringBuffer 和 StringBuilder 中添加了 repeat(int codePoint, int count) 方法

```java
new StringBuilder().repeat('-', 80);
```