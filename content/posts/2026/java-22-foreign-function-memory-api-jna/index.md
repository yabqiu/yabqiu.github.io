---
title: "Java 22 - Foreign Function Memory API 与 JNA"
url: /java-22-foreign-function-memory-api-jna/
date: 2026-03-23T14:26:35-05:00
featured: false
draft: false
type: post
toc: false
# menu: main
usePageBundles: true
thumbnail: "/images/logos/java-logo.png"
categories:
  - Java
tags: 
  - JNI
comment: true
codeMaxLines: 50
# additional
lastmod: 
---

Java 22 正式引入了 [Foreign Function & Memory API](https://openjdk.org/jeps/454), 简称 FFM API, 它源自于 [Project Panama](https://openjdk.org/projects/panama/),
旨在用来替代 JNI, 提供更安全，健壮的本地动态库调用方式。之前在一文 [ Java 22 新特性学习](/java-22-new-features/) 中跳过了对它的体验，
因为直接用 FFM API 来调用本地代码的话，写起来比 JNI 还更复杂，更别提和 JNA 对比，性能上 FFM API 大致与 JNI 相当，但总比 JNA 要好些，
原因是 JNA 使用了一个与系统相关的中间层动态库。像这种调用本地代码的场景，性能的差异基本体现在本地代码上，调用过程的性能差异一般可以忽略不计.

那时没深入 FFM API 的原因是它用起来过于复杂，这种复杂性应该由第三方库来屏蔽，可是自 Java 22 于 2024-03-19 发布以来目前仍未找到一款类似于 
JNA 屏蔽了 JNI 那样的基于 FFM 的第三方库。所以还是事先来体验一下 FFM API 的开发过程，现在可以借助 AI 来引导学习, 并在后面与 JNA 作个性能对比，
只供参考。<!--more-->

还是从一个简单的 C++ 代码开始，关于不同语言调用动态库的文章，我差不多不停的在重复着这段 C++ 代码以及编译生成动态库的命令，可是也不得不这么做。
本文中稍为引入一点复杂性，给 C++ 的函数加入了一个结构作为参数, 代码如下

### 创建动态库代码 `deme.cpp`

```cpp
#include <stdio.h>
#include <string.h>

#ifndef VERSION
#define VERSION 1
#endif

struct Contrib {
  int year;
  float ratio;
};

extern "C" char* desc(char* name, Contrib contrib)
{
  static char buffer[256];
  snprintf(buffer, sizeof(buffer), "v%d: %s contributed %.1f%% in %d", VERSION, name, contrib.ratio * 100, contrib.year);
  return buffer;
}
```

代码中加了一个 `VERSION` 宏，用于产生不同版本的动态库，这样可以验证在一个 Java 进程中如何同时加载使用一个动态库的不同版本。如果是使用 JNI
方式，用 `System.load()` 或 `System.loadLibrary()` 加载不同版本的动态库的话需要用到自定义的 ClassLoader; 而用 JNA 来实现会很容易。

### 编译生成动态库

```shell
gcc -fPIC -shared -o libdemov1.dylib demo.cpp
gcc -DVERSION=2 -fPIC -shared -o libdemo2.dylib demo.cpp
```

本文的测试平台是 macOS, 苹果芯片, Java 25，上面两条命令在当前目录下分别编译出动态库文件 `libdemov1.dylib` 和 `libdemov2.dylib`.

### 测试 FFM API 调用动态库

注意，FFM API 是在 Java 开始引入的，所以在之前版本中使用它的话必须编译和执行时加上 `--enable-preview` 参数, 如 `java --enable-preview`。

`FFMDemo.java`

```java
import java.lang.foreign.*;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.VarHandle;
import java.nio.charset.StandardCharsets;

public class FFMDemo {
    static final StructLayout CONTRIB_LAYOUT = MemoryLayout.structLayout(
        ValueLayout.JAVA_INT,
        ValueLayout.JAVA_FLOAT
    );

    static final VarHandle YEAR_HANDLE = CONTRIB_LAYOUT.varHandle(
        MemoryLayout.PathElement.groupElement(0)
    );
    static final VarHandle RATIO_HANDLE = CONTRIB_LAYOUT.varHandle(
        MemoryLayout.PathElement.groupElement(1)
    );

    static class Library {
        private MethodHandle descHandle;

        public Library(MethodHandle descHandle) {
            this.descHandle = descHandle;
        }
    }

    private static Library loadLibrary(String libPath) {
        SymbolLookup libdemo = SymbolLookup.libraryLookup(libPath, Arena.global());

        MemorySegment descAddress = libdemo.find("desc").orElseThrow(
            () -> new RuntimeException("desc function not found")
        );

        FunctionDescriptor descFuncDesc = FunctionDescriptor.of(
            ValueLayout.ADDRESS,
            ValueLayout.ADDRESS,
            CONTRIB_LAYOUT
        );

        MethodHandle descHandle = Linker.nativeLinker().downcallHandle(
            descAddress,
            descFuncDesc
        );

        return new Library(descHandle);
    }

    public static void main(String[] args) {
        Library lib1 = loadLibrary("libdemo1.dylib");
        callDescMethod(lib1);
    }

    private static void callDescMethod(Library lib) {
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment contrib = arena.allocate(CONTRIB_LAYOUT);
            YEAR_HANDLE.set(contrib, 0L, 2024);
            RATIO_HANDLE.set(contrib, 0L, 0.85f);

            String name = "Alice";
            MemorySegment nameSegment = arena.allocateFrom(name, StandardCharsets.UTF_8);

            MemorySegment result = (MemorySegment) lib.descHandle.invoke(nameSegment, contrib);

            String resultStr = result.reinterpret(256).getString(0, StandardCharsets.UTF_8);
            System.out.println(resultStr);
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }
}
```

由于使用到了 C++ 的 `struct`, 所以在 Java 代码中大大增加了复杂性，如果是使用了自定义类指针的话还会得复杂。要是你的 C++ API 参数或返回值只用
基本类型，如 int, float, void 等，最多再加个 char*, void*, Java 调用时会变得得简单些，通常 C++ 写的为跨语言使用的动态库，接口都会设计得比较简单。

编译后执行, 会看到如下多行警告信息

```shell
java FFMDemo     
WARNING: A restricted method in java.lang.foreign.SymbolLookup has been called
WARNING: java.lang.foreign.SymbolLookup::libraryLookup has been called by FFMDemo in an unnamed module (file:/Users/yanbin/test-java22/)
WARNING: Use --enable-native-access=ALL-UNNAMED to avoid a warning for callers in this module
WARNING: Restricted methods will be blocked in a future release unless native access is enabled

v1: Alice contributed 85.0% in 2024
```

加上 `--enable-native-access=ALL-UNNAMED` 参数可消除

```shell
java --enable-native-access=ALL-UNNAMED FFMDemo
v1: Alice contributed 85.0% in 2024
```

### FFM API 加载动态库的不同版本

对于直接使用 JNI 来加载一个动态库的不同版本是一个挑战，因为 `System.load()` 和 `System.loadLibrary()` 的返回值都是 `void`, 
所以只能在动态库加载后，通过方法名去内存上定位本地方法，而同一动态库的不同版本具体相同的方法签名，解决办法大概有两种

1. 不同版本中方法签名都加上版本号前缀，这要对 Java 的 native 方法和 C++ 导出函数名进行编译前预处理
2. 使用自定义的 Java ClassLoader 加载定义了 native 方法的类到不同的 Namespace 中

现在来看看 FFM API 是如何实现的, 复用上面的 `FFMDemo.java`, 只需替换 `main` 方法如下

```java
    public static void main(String[] args) {
        Library lib1 = loadLibrary("libdemo1.dylib");
        callDescMethod(lib1);

        Library lib2 = loadLibrary("libdemo2.dylib");
        callDescMethod(lib2);

        callDescMethod(lib1);
    }
```

编译后，现在执行

```shell
java --enable-native-access=ALL-UNNAMED FFMDemo
v1: Alice contributed 85.0% in 2024
v2: Alice contributed 85.0% in 2024
v1: Alice contributed 85.0% in 2024
```

我们看到在使用 FFM API 加载不同版本动态库时相当的丝滑，完全没有任何麻烦，这要得利于 FFM API 的 `SymbolLookup.libraryLookup()` 方法是有返回值的。

### 对比 JNA 调用动态库

之前使用 JNA 时的场景都比较简单，只有基本类型，看下引入了 `struct` 后 JNA 应用代码如何

```java
import com.sun.jna.Library;
import com.sun.jna.Native;
import com.sun.jna.Structure;

import java.io.File;
import java.util.Map;

import static com.sun.jna.Library.OPTION_OPEN_FLAGS;

public class JNADemo {
    public interface DemoLibrary extends Library {
        String desc(String name, Contrib.ByValue contrib);
    }

    @Structure.FieldOrder({"year", "ratio"})
    public static class Contrib extends Structure {
        public int year;
        public float ratio;

        public static class ByValue extends Contrib implements Structure.ByValue {
        }
    }

    private static DemoLibrary loadLibrary(String libPath) {
        Map<String, ?> loadOptions = Map.of(OPTION_OPEN_FLAGS, 1);
        return Native.load(new File(libPath).getAbsolutePath(), DemoLibrary.class, loadOptions);
    }

    public static void main(String[] args) {
        DemoLibrary lib = loadLibrary("libdemo1.dylib");
        callDescMethod(lib);
    }

    private static void callDescMethod(DemoLibrary lib) {
        Contrib.ByValue contrib = new Contrib.ByValue();
        contrib.year = 2024;
        contrib.ratio = 0.85f;

        String result = lib.desc("Alice", contrib);
        System.out.println(result);
    }
}
```

编译或执行时必须加上 JNA 依赖 `net.java.dev.jna:jna`, 当前版本为 `5.18.1`, 它只有一个 jar 包 `jna-5.18.1.jar`. 如果展开该 jar 包，
在 com/sun/jna 目录下可以看到一系列的平台相关动态库

```text
aix-ppc/libjnidispatch.a
aix-ppc64/libjnidispatch.a
darwin-aarch64/libjnidispatch.jnilib
darwin-x86-64/libjnidispatch.jnilib
dragonflybsd-x86-64/libjnidispatch.so
freebsd-aarch64/libjnidispatch.so
freebsd-x86/libjnidispatch.so
freebsd-x86-64/libjnidispatch.so
linux-aarch64/libjnidispatch.so
linux-arm/libjnidispatch.so
linux-armel/libjnidispatch.so
linux-loongarch64/libjnidispatch.so
linux-mips64el/libjnidispatch.so
linux-ppc/libjnidispatch.so
linux-ppc64le/libjnidispatch.so
linux-riscv64/libjnidispatch.so
linux-s390x/libjnidispatch.so
linux-x86/libjnidispatch.so
linux-x86-64/libjnidispatch.so
openbsd-x86/libjnidispatch.so
openbsd-x86-64/libjnidispatch.so
sunos-sparc/libjnidispatch.so
sunos-sparcv9/libjnidispatch.so
sunos-x86/libjnidispatch.so
sunos-x86-64/libjnidispatch.so
win32-aarch64/jnidispatch.dll
win32-x86/jnidispatch.dll
win32-x86-64/jnidispatch.dll
```

从这些动态库也能够理解 JNA 是能过中间动态库桥接到实际动态库的，以 macOS 芯片为例，使用的是 `darwin-aarch64/libjnidispatch.jnilib`，
如果我们确切知道平台，其他不用的动态库可以删除，不过整个 `jna-5.18.1.jar` 也不就 `1.9M` 大小。JNA 与 FFM API, JNI 性能劣势也是在这里，
用一点点微不足道的性能牺牲换来编程的便利性还是值得的。

回到 `JNADemo.java`, 编译后执行时同样需要加上 `--enable-native-access=ALL-UNNAMED` 参数，否则会看到同样的警告信息

```shell
java --enable-native-access=ALL-UNNAMED JNADemo
v1: Alice contributed 85.0% in 2024
```

### JNA 加载动态库的不同版本

类似的，把 `JNADemo` 中的 `main` 方法修改为如下

```java
    public static void main(String[] args) {
        DemoLibrary lib1 = loadLibrary("libdemo1.dylib");
        callDescMethod(lib1);

        DemoLibrary lib2 = loadLibrary("libdemo2.dylib");
        callDescMethod(lib2);

        callDescMethod(lib1);
    }
```

编译后执行

```shell
java -cp jna-5.18.1.jar:. --enable-native-access=ALL-UNNAMED JNADemo
v1: Alice contributed 85.0% in 2024
v2: Alice contributed 85.0% in 2024
v1: Alice contributed 85.0% in 2024
```

### FFM API 与 JNA 性能对比

本测试仅供参考，在实际应用中需更多关注 Native 代码的消耗。

把 `FFMDemo` 和 `JNADemo` 的 `main` 方法分别改为

`FFMDemo::main`

```java
    public static void main(String[] args) {
        long start = System.currentTimeMillis();
        for (int i = 0; i < Integer.parseInt(args[0]); i++) {
            Library lib1 = loadLibrary("libdemo1.dylib");
            callDescMethod(lib1);
        }
        System.out.println("Total time: " + (System.currentTimeMillis() - start) + " ms");
    }
```

`JNADemo::main`

```java
    public static void main(String[] args) {
        long start = System.currentTimeMillis();
        for (int i = 0; i < Integer.parseInt(args[0]); i++) {
            DemoLibrary lib1 = loadLibrary("libdemo1.dylib");
            callDescMethod(lib1);
        }
        System.out.println("Total time: " + (System.currentTimeMillis() - start) + " ms");
    }
```

同是把各自的 `callDescMethod()` 中的打印方法调用 `System.out.println(result);` 注释掉，以免控制台输出影响到测试结果。

编译

```shell
javac FFMDemo.java
javac -cp jna-5.18.1.jar:. JNADemo.java
```

执行时取不到的参数进行对比，如

```shell
java --enable-native-access=ALL-UNNAMED FFMDemo 1000
java cp jna-5.18.1.jar:. --enable-native-access=ALL-UNNAMED JNADemo 1000
```

以下是不同取值的结果对比

| 迭代次数 | FFM API (ms) | JNA (ms) | JNA vs FFM |
|---------|--------------|----------|-----------|
| 100     | 69           | 206      | +198.6%   |
| 500     | 96           | 202      | +110.4%   |
| 1000    | 132          | 232      | +75.8%    |
| 5000    | 325          | 292      | -10.2%    |
| 10000   | 494          | 304      | -38.5%    |
| 50000   | 1739         | 598      | -65.6%    |
| 100000  | 3398         | 855      | -74.8%    |
| 500000  | 14995        | 2130     | -85.8%    |

当少量调用时(低于 1000 次调用)，FFM API 比 JNA 要快一些，不过即使存在这种性能差异也是微乎其微，一千次调用才差 100 毫米，平均每次差 0.1 秒。
但在调用次数上来后，如上面的 1000 次调用时，JNA 有 JIT 加持，性能反而超越了 FFM API, 迭代次逐渐变大时越发明显。10000 次，JNA 的性能好于
FFM API 38.5%, 500000 次时 JNA 性能优质好于 FFM API 85.8%.

### 总结

FFM API 是 Java 官方提供的调用本地动态库的 API, 它的性能与 JNI 大致相当, 但在使用上比 JNI 更加复杂, 虽然免去了从 native 方法生成 C++
代码并编译特定用于 JNI 的动态库版本，但实际上需要更多的样板代码来处理内存布局和方法句柄等。而 JNI 的包装库 JNA 用起来比 FFMI API 和 JNI
都简单的多，性能方面，也不用担忧 JNA, 在 JIT 介入后性能上比 FFM API 还好，这个 JIT 本质上是 JNI 赋予的，所以有 JIT 时 JNA 和 JNI 的性能
要比 FFM API 好。期待 FFM API 后续有更多的性能优化，即使是当前，或者 JNA 的版本还持续在更新话，JNA 仍然是一个不错的选择。