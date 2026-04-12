---
title: Java 24 新特性学习
url: /java-24-new-features/
date: 2026-04-12T13:30:00-05:00
featured: false
type: post
draft: false
toc: false
# menu: main
usePageBundles: true
thumbnail: "/images/logos/java-logo.png"
categories:
  - java
  - new features
tags:
  - java 24
series: Java New Features
comment: true
codeMaxLines: 50
showLastmod: true
---
 
Java 24 也是一个过渡版本, 还是到下面两个链接中找相应的更新

1. [JDK 24 Release Notes - Major New Functionality](https://www.oracle.com/java/technologies/javase/24all-relnotes.html#JSERN24)
2. [OpenJDK JDK 24 Features](https://openjdk.org/projects/jdk/24/)

IntelliJ IDEA 对 Java 24 Language level 描述是

1. 24 - No new language features
2. 24 (Preview) - Flexible constructor bodies, simple source files, etc.

把上面第二个链接中的特性列出来

<div style="display: flex;font-size: 14px;">
   <div style="flex: 1;">
      <ul>
        <li>404: <a href="https://openjdk.org/jeps/404">Generational Shenandoah (Experimental)</a></li>
        <li>450: <a href="https://openjdk.org/jeps/450">Compact Object Headers (Experimental)</a></li>
        <li>472: <a href="https://openjdk.org/jeps/472">Prepare to Restrict the Use of JNI</a></li>
        <li>475: <a href="https://openjdk.org/jeps/475">Late Barrier Expansion for G1</a></li>
        <li>478: <a href="https://openjdk.org/jeps/478">Key Derivation Function API (Preview)</a></li>
        <li>479: <a href="https://openjdk.org/jeps/479">Remove the Windows 32-bit x86 Port</a></li>
        <li>483: <a href="https://openjdk.org/jeps/483">Ahead-of-Time Class Loading & Linking</a></li>
        <li>484: <a href="https://openjdk.org/jeps/484">Class-File API</a> <strong style="color: red">*</strong></li>
        <li>485: <a href="https://openjdk.org/jeps/485">Stream Gatherers</a> <strong style="color: red">*</strong></li>
        <li>486: <a href="https://openjdk.org/jeps/486">Permanently Disable the Security Manager</a></li>
        <li>487: <a href="https://openjdk.org/jeps/487">Scoped Values (Fourth Preview)</a></li>
        <li>488: <a href="https://openjdk.org/jeps/488">Primitive Types in Patterns, instanceof, and switch (Second Preview)</a></li>
      </ul>
   </div>
   <div style="flex: 1;">
      <ul>
        <li>489: <a href="https://openjdk.org/jeps/489">Vector API (Ninth Incubator)</a></li>
        <li>490: <a href="https://openjdk.org/jeps/490">ZGC: Remove the Non-Generational Mode</a></li>
        <li>491: <a href="https://openjdk.org/jeps/491">Synchronize Virtual Threads without Pinning</a></li>
        <li>492: <a href="https://openjdk.org/jeps/492">Flexible Constructor Bodies (Third Preview)</a></li>
        <li>493: <a href="https://openjdk.org/jeps/493">Linking Run-Time Images without JMODs</a></li>
        <li>494: <a href="https://openjdk.org/jeps/494">Module Import Declarations (Second Preview)</a></li>
        <li>495: <a href="https://openjdk.org/jeps/495">Simple Source Files and Instance Main Methods (Fourth Preview)</a></li>
        <li>496: <a href="https://openjdk.org/jeps/496">Quantum-Resistant Module-Lattice-Based Key Encapsulation Mechanism</a></li>
        <li>497: <a href="https://openjdk.org/jeps/497">Quantum-Resistant Module-Lattice-Based Digital Signature Algorithm</a></li>
        <li>498: <a href="https://openjdk.org/jeps/498">Warn upon Use of Memory-Access Methods in sun.misc.Unsafe</a></li>
        <li>499: <a href="https://openjdk.org/jeps/499">Structured Concurrency (Fourth Preview)</a></li>
        <li>501: <a href="https://openjdk.org/jeps/501">Deprecate the 32-bit x86 Port for Removal</a></li>
      </ul>
   </div>
</div>

Java 24 中列出的特性恰如其版本一样，有 24 条，因其是一个过渡版本，多数为非正式特性，也就为什么 IntelliJ IDEA 从语言特性上把 Java 标记为
No new language features. 其实也不然，个人觉得有两个正式特性值得关注，即 Class-File API 和 Stream Gathers。 <!--more-->

下面拣几个重点描述，有些在前面的版本尚处于 Incubator 或 Preview 状态时提到，会给出相应的链接，还有些不甚重要的只一笔带过.

首先两个不需要太多解释的小特性

### JEP 485: Stream Gatherers

关于 Stream Gatherers 的介绍，可参考 [Java 22 新特性学习 - JEP 461: Stream Gatherers (Preview)](/java-22-new-features/#jep-461-stream-gatherers-preview).
这里仍然是要理解引入 `Stream Gatherers` 的意图，Stream 通过 reduce/collect 方法进行终结操作(terminal operation), 由接口 `java.util.stream.Collector`
可自定义 `Collector`. 而相应的，`java.util.stream.Gatherer` 是为 Stream 的中间方法(`intermediate operation`) 定义的接口，这样对 Stream
的中间操作就不局限于 `map`, `filter` 等，可以是统一的

```java
stream.gather(Gatherer 实现).gather(Gatherer 实现).collect(Collector);
```

有了 `Gatherer` 接口，Java 也就不需要像 Java 9 那样给 `Stream` 特别的添加 `takeWhile()`, `dropWhile()` 等方法，以后只需 `Gatherer`
接口来扩展 `Stream` 的中间方法了。

`java.util.stream.Gatherer` 的接口定义，或理解其内部工作过程并不容易，即使用 `Gatherer.of()` 和 `Gatherer.ofSequential()` 工厂方法
创建一个 Gatherer 实例也有着使用 `Stream.reduce()` 一样的复杂性，所以我主要关心的是基于该接口有哪些第三方的库可用。

找到两个第三方 Gatherer 实现库

1. [io.github.jpspetersson:packrat](https://github.com/jhspetersson/packrat)
2. [com.gisberg:gatherers4j](https://github.com/tginsberg/gatherers4j)

这两个库提供的 `Gatherer` 实现已经够丰富了，像 `distinctBy`, `minBy`, `shuffle`, `mapUntil`, `nCopies`, `repeat`, `zip`,
`mapWithIndex`, `windowSlidinWithIndex`, `windowFixedWithIndex`; `foldIndexed`, `throttle`, `takeEveryNth`, `groupBy`,
`window`, `movingMedian(window)`, `movingProduct(window)`.

下面是一个使用了 `Packrat` 的 distinctBy 根据某个字段值去重的例子

```java
record Person(String name, String address);

Person p1, p2, p3;
List<Person> persons = Stream.of(p1, p2, p3).gather(Packrat.distinctBy(Person::name)).toList();
```

估计后面两大通用组件 `Guava` 和 `Apache Common` 也会染指 `Gatherer`, 会提供更多的 `Gatherer` 实现。

### JEP 484: Class-File API

Class-File API 经过 Java 22, 23 两次 Preview 便在 Java 24 中转为正式特性。一般来说在我们的代码中也不会直接用 Class-File API，
但它会影响一些 AOP, 或 Mock 测试框架的实现，Class-File API 为解析，生成，和转换(修改) Java class(字节码) 提供了标准 API, 它的目的不是为
了替代 ASM 库，如 [ASM](https://asm.ow2.io/), [BCEL](https://commons.apache.org/proper/commons-bcel/), [Javassist](https://www.javassist.org/)
, 但足以改变第三方 ASM 库的生态。前面一直未真正了解过 Class-File API, 这里简单学习一下。

由于 Java Class 文件格式不断在演进，如支持 `sealed classes`, `dynamic constants` 和 `nestmates` 等，所以会对第三方库造成不兼容性，
所以要提供一个标准的 API. 与其阅读 [JEP 484: Class-File API](https://openjdk.org/jeps/484), 还不如参考
[Class-File API](https://docs.oracle.com/en/java/javase/24/docs/api/java.base/java/lang/classfile/package-summary.html)。

#### 解析 Class 文件

假设有下面的 `Cat` 类生成的  `Cat.class` 文件

```java
public class Cat implements Animal {
    private String name = "Rogue";

    public void makeSound() {
        System.out.printf("%s meow\n", name);
    }
}
```

它实现了 `Animal` 接口, 我们用 Class-File API 来解析

```java
import java.io.IOException;
import java.lang.classfile.*;
import java.nio.file.Files;
import java.nio.file.Path;

public class ParseClass {
    public static void main(String[] args) throws IOException {
        ClassModel cm = ClassFile.of().parse(Files.readAllBytes(Path.of("target/classes/Cat.class")));
        System.out.printf("Version: %d.%d\n", cm.majorVersion(), cm.minorVersion());
        System.out.printf("Interfaces: %s\n", cm.interfaces());
        System.out.println("------fields------");
        for (FieldModel field : cm.fields()) {
            System.out.println(field);
        }
        System.out.println("------methods------");
        cm.methods().forEach(System.out::println);
        System.out.println("------class elements------");
        for(ClassElement ce: cm) {
            System.out.println(ce);
        }
        System.out.println("------makeSound code------");
        CodeModel meowCodeModel = cm.methods().stream().filter(m -> m.methodName().equalsString("makeSound"))
            .map(MethodModel::code).findFirst().get().get();
        for (CodeElement e : meowCodeModel) {
            System.out.println(e);
        }
    }
}
```

看起来像是在反射，然而核心 API 都在 `java.lang.classfile` 包下，如 `ClassModel`, `FieldModel`, `MethodModel`, `CodeModel`,
`CodeElement` 等等.

上面代码的输出为

```text
Version: 68.0
Interfaces: [7 Animal]
------fields------
FieldModel[fieldName=name, fieldType=Ljava/lang/String;, flags=2]
------methods------
MethodModel[methodName=<init>, methodType=()V, flags=1]
MethodModel[methodName=makeSound, methodType=()V, flags=1]
------class elements------
AccessFlags[flags=33]
ClassFileVersion[majorVersion=68, minorVersion=0]
Superclass[superclassEntry=java/lang/Object]
Interfaces[interfaces=Animal]
FieldModel[fieldName=name, fieldType=Ljava/lang/String;, flags=2]
MethodModel[methodName=<init>, methodType=()V, flags=1]
MethodModel[methodName=makeSound, methodType=()V, flags=1]
Attribute[name=SourceFile]
------makeSound code------
LocalVariable[name=this, slot=0, type=LCat;]
Label[context=CodeModel[id=445051633], bci=0]
LineNumber[line=5]
Field[OP=GETSTATIC, field=java/lang/System.out:Ljava/io/PrintStream;]
LoadConstant[OP=LDC, val=%s meow
]
UnboundIntrinsicConstantInstruction[op=ICONST_1]
NewRefArray[OP=ANEWARRAY, type=java/lang/Object]
UnboundStackInstruction[op=DUP]
UnboundIntrinsicConstantInstruction[op=ICONST_0]
Load[OP=ALOAD_0, slot=0]
Field[OP=GETFIELD, field=Cat.name:Ljava/lang/String;]
UnboundArrayStoreInstruction[op=AASTORE]
Invoke[OP=INVOKEVIRTUAL, m=java/io/PrintStream.printf(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;]
UnboundStackInstruction[op=POP]
LineNumber[line=6]
Return[OP=RETURN]
Label[context=CodeModel[id=445051633], bci=21]
```

#### 生成新的 Class

比如基于一个接口 `Animal` 在内存中生成 `Dog` 类，并加载执行

```java
import java.lang.classfile.ClassFile;
import java.lang.constant.ClassDesc;
import java.lang.constant.ConstantDescs;
import java.lang.constant.MethodTypeDesc;
import java.lang.invoke.MethodHandles;

public class GenerateClass {
    public static void main(String[] args) throws Exception {
        ClassFile cf = ClassFile.of();
        byte[] bytes = cf.build(
            ClassDesc.of("Dog"),
            clb -> clb
                .withFlags(ClassFile.ACC_PUBLIC)
                .withInterfaceSymbols(ClassDesc.of("Animal"))

                // Add a default constructor: public Dog() { super(); }
                .withMethod(ConstantDescs.INIT_NAME, MethodTypeDesc.of(ConstantDescs.CD_void),
                    ClassFile.ACC_PUBLIC,
                    mb -> mb.withCode(cb -> cb
                        .aload(0)
                        .invokespecial(ConstantDescs.CD_Object, ConstantDescs.INIT_NAME, MethodTypeDesc.of(ConstantDescs.CD_void))
                        .return_()
                    ))

                // Implement the makeSound method: public void makeSound() { System.out.println("Woof!"); }
                .withMethod(
                    "makeSound",
                    MethodTypeDesc.of(ConstantDescs.CD_void),
                    ClassFile.ACC_PUBLIC,
                    mb -> mb.withCode(cb -> cb
                        .getstatic(ClassDesc.of("java.lang.System"), "out", ClassDesc.of("java.io.PrintStream"))
                        .ldc("Woof!")
                        .invokevirtual(ClassDesc.of("java.io.PrintStream"), "println",
                            MethodTypeDesc.of(ConstantDescs.CD_void, ConstantDescs.CD_String))
                        .return_()
                    )
                )
        );

        Class<?> dogClass = MethodHandles.privateLookupIn(Animal.class, MethodHandles.lookup()).defineClass(bytes);

        Animal dog = (Animal) dogClass.getDeclaredConstructors()[0].newInstance();
        dog.makeSound();
    }
}
```

执行

> Woof!

符合预期，也可以用下面自定义类加载器的方式来加载 `Dog` 类

```java
class CustomClassLoder extends ClassLoader {
    public Class<?> load(String name, byte[] b) {
        return super.defineClass(name, b, 0, b.length);
    }
}

Class<?> dogClass =  new CustomClassLoder().load("Dog", bytes);
```

#### 修改已有类

对于当前 ClassLoader 可见的类文件在类加载之前可以对其进行修改

```java
import java.lang.classfile.ClassFile;
import java.lang.classfile.ClassTransform;
import java.lang.classfile.MethodTransform;
import java.lang.classfile.instruction.ReturnInstruction;
import java.lang.constant.ClassDesc;
import java.lang.constant.ConstantDescs;
import java.lang.constant.MethodTypeDesc;
import java.nio.file.Files;
import java.nio.file.Path;

public class TransferClass {
    public static void main(String[] args) throws Exception {
        var cf = ClassFile.of();
        Path classPath = Path.of("target/classes/Cat.class");
        byte[] original = Files.readAllBytes(classPath);

        ClassTransform ct = ClassTransform.transformingMethods(
            mm -> !mm.methodName().stringValue().equals("<init>"),
            MethodTransform.transformingCode(
                (codeBuilder, elem) -> {
                    if (elem instanceof ReturnInstruction ret) {
                        // 在 return 前注入：System.out.println("method exit")
                        codeBuilder
                            .getstatic(ClassDesc.of("java.lang.System"), "out", ClassDesc.of("java.io.PrintStream"))
                            .ldc("method exit")
                            .invokevirtual(ClassDesc.of("java.io.PrintStream"), "println",
                                MethodTypeDesc.of(ConstantDescs.CD_void, ConstantDescs.CD_String))
                            .with(ret);
                    } else {
                        codeBuilder.with(elem);
                    }
                }
            )
        );

        byte[] instrumented = cf.transformClass(cf.parse(original), ct);
        Files.write(classPath, instrumented);

        new Cat().makeSound();
    }
}
```

执行上面的代码会对 Class 文件进行重复修改，为避免多个 `method exit`, 执行前用 `mvn clean compile` 生成新鲜的 `Cat.class` 文件。`Cat.makeSound()`
原本的输出为 

> Rogue meow

执行 `TransferClass` 后的输出为

> method exit<br/>
> Rogue meow

这是一个简单的演示代码，所以要确保它能得到预期的结果，必须在 `Cat` 类加载之前执行，并且为避免 `Cat.class` 被修重复修改，可在修改完做上标记。
如果 `Cat.class` 对当前类加载器不可见，可用定制类加载器从内存加载修改后的字节。

Class-File API 的内容在本文占主要篇幅，下面对其他特性大概说一下。

### JEP 472: 准备有限的使用 JNI

自 Java 22 起， [Foreign Function & Memory (FFM) API](https://openjdk.org/jeps/454) 可替代 JNI 的使用，但性能上由于有 JIT, 不
一定比纯 JNI, 或 JNA 效率高。

该 JEP 的进一步解释是保留 JNI 为本地代码的标准交互方式，无论是用 JNI 还是 FFM API 在启动时都有警告信息，开发者将要显式的声明要使用 JNI 或
FFM API. 它并非是不建议使用 JNI 或将从 Java 中移除 JNI, 并不会限制通过 JNI 调用本地代码的行为。好像是说不会对 JNI 有任何限制，那标题怎么说的。

该 JEP 的一个要义是 JNI 很危险，使用需谨慎，它可能会破坏 Java 的诚信约定([integrity by default](https://openjdk.org/jeps/8305968).
例如 `direct byte buffers` 让垃圾收集器特别对待。`FFM API` 作为 `JNI` 的更优先选择，比 JNI 相对安全.

比如在用 JNA 加载动态库时，会收到如下警告信息

>WARNING: A restricted method in java.lang.System has been called<br/>
>WARNING: java.lang.System::load has been called by com.sun.jna.Native in an unnamed module (file:/Users/yanbin/.m2/repository/net/java/dev/jna/jna/5.18.1/jna-5.18.1.jar)<br/>
>WARNING: Use --enable-native-access=ALL-UNNAMED to avoid a warning for callers in this module<br/>
>WARNING: Restricted methods will be blocked in a future release unless native access is enabled

用纯 JNI 或 FFM API 都会有一样的警告信息，要加上 JVM 参数 `--enable-native-access=ALL-UNNAMED` 来避免警告信息，或者更细粒度的 
`--enable-native-access=模块1,模块2` 来允许特定模块使用 JNI 或 FFM API.

### JEP 404: 分代式 Shenandoah 垃圾收集(实验性的)

Java 中有 G1GC, 和新的 ZGC，还有 Shenandoah GC (Java 15 正式引入该 GC)还在发展，传统 GC 包括 G1 在压缩堆时都必须停止所有应用线程，而 Shenandoah GC 
通过使用读屏障和写屏障来实现并发压缩堆，减少停顿时间。Java 24 开始实验性对 Shenandoah GC 加入象 G1, ZGC 普遍应用的将堆划分为年轻代和老年代
的特性，以优化垃圾收集的性能。在 Java 24 要使用 `Shenadoah GC` 并开启堆分代的话，命令为

```bash
java -XX:+UseShenandoahGC \
       -XX:+UnlockExperimentalVMOptions \
       -XX:ShenandoahGCMode=generational ...
```

到了 Java 25 后， 分代式 Shenandoah GC 是默认的特性了，所以在 Java 25 中只要启用 Shenandoah GC 就是分代式的了

```bash
java -XX:+UseShenandoahGC ...
```

### JEP 479: Remove the Windows 32-bit x86 Port
### Deprecate the 32-bit x86 Port for Removal

不再支持 Windows 32 位 x86 平台的, 对其他操作系统 32 位 X86 的也将不再支持

### JEP 486: 永久性的禁用 SecurityManager

SecurityManager 是 Java 1.0 就引入的安全机制，允许应用程序定义一个安全策略来限制代码的行为，如访问文件系统，网络，系统属性等。
随着 Java 平台的发展，SecurityManager 已经过时了，Java 17 开始被标记为过时，Java 24 被永久禁用.

在一些老旧的代码中才可能见到  `SecurityManager` 的使用，新生代的程序也见不到。

### JEP 490: ZGC: 移除了非分代的模式

分代收集都是主流 GC 的标本，如 G1 GC, ZGC, 和 Shenandoah GC. 以后只要启动时 `java -XX:+UseZGC` 就是分代的 ZGC 了。

### JEP 491: 虚拟线程在 synchronized 中不再固定平台线程

Java 24 之前，Java 21 引入的虚拟线程碰到 `synchronized` 的方法的代码块时无法让出所对应的平台线程，也就是说虚拟线程对 `synchronized` 无效。
Java 24 解决了这一问题，在 `synchronized` 方法或函数中的 IO 等待也能让出其所在的平台线程。它的解决办法是把原来对 `synchronized` 监控平台
线程转换为监控虚拟线程。

### JEP 495: 简化的源文件和 Main 方法

在 IntelliJ IDEA 中选择了 `24 (Preview) - Flexiable constructor bodies, simple source files, etc.` 语方级别后，写下面的代码

```java
public class Demo {
    public static void main(String[] args) {
        System.out.println("Hello, World!");
    }
}
```

就会建议写成

```java
void main() {
    System.out.println("Hello, World!");
}
```

对生成的 `Demo.class` 反编译后，是下面的样子

```java
final class Demo {
    Demo() {
    }

    void main() {
        System.out.println("Hello, World!");
    }
}
```

该特性将在 Java 25 中转为正式。即使是

```java
class Demo {
    void main() {
        System.out.println("Hello, World!");
    }
}
```

也才能在 Java 25 中被识别为一个可执行的 `main` 方法。这种特性其实没什么用，放在哪里也不知道会有多少人会去使用，或不被正式项目接纳，只为了迎合
脚本语言的开发人员。比如上面的代码放在任何一个版本的 Java 中都是合法的，只是在 Java 25 之前不认作为可执行的 `main` 方法，反而会让代码混乱。

### JEP 498: 使用 sun.misc.Unsafe 中内存访问方法将会有警告

`sun.misc.Unsafe` 与 `SecurityManager` 同样古老，现在也将被警告

>WARNING: A terminally deprecated method in sun.misc.Unsafe has been called<br/>
>WARNING: sun.misc.Unsafe::setMemory has been called by com.foo.bar.Server (file:/tmp/foobarserver/thing.jar)<br/>
>WARNING: Please consider reporting this to the maintainers of com.foo.bar.Server<br/>
>WARNING: sun.misc.Unsafe::setMemory will be removed in a future release

JDK 26 开始将会抛出异常，加上 JVM 参数 `--sun-misc-unsafe-memory-access=allow` 才能屏蔽掉以上警告信息。