---
title: "Java 的 Path.endsWith(String) 方法很容易用错"
url: /java-path-endswith-string-error-prone/
date: 2026-08-25T08:35:20-05:00
featured: false
draft: false
type: post
toc: false
# menu: main
usePageBundles: true
thumbnail: "../images/logos/java-logo.png"
categories:
  - Java
tags: 
  - Java
  - JDK
comment: true
codeMaxLines: 50
lastmod:
---

用 IDE 写了一段代码，目的是遍历某一目录下的所有文件，并对所有以 `.xml` 结尾的文件进行处理，代码如下：

```java
try (Stream<Path> pathStream = Files.list(Paths.get("data"))) {
    pathStream.filter(path -> path.endsWith(".xml"))
              .forEach(System.out::println); // do something with the file 
} catch(IOException e) {
    //...
}
```

看起来很直截了当，filter 中对 path 用 `endsWith(String)` 方法过滤出所有以 `.xml` 结尾的文件，然而在目录中存在 `*.xml`
文件的情况下却没有任何输出。假如测试时未该发现问题，在运行期大有可能当成是该目录中未找到任何以 `.xml` 结尾的文件而怀疑是数据问题。

这样的代码即使是让人工进行 review 也未必能发现有什么问题，遍历 `data` 目录下的文件，如果文件以 `.xml` 结尾就进行处理，很自然，正确啊。

要是前面的代码中使用了 `var` 来修改 `pathStream` 就更不太可能在 Code Review 时发现问题，像

```java
try (var pathStream = Files.list(Paths.get("data"))) {
    pathStream.filter(path -> path.endsWith(".xml"))
    .forEach(System.out::println); // do something with the file 
} catch(IOException e) {
    //...
    }
```

后来经过仔细的调试发现 filter 中 `path.endsWith(".xml")` 永远返回 `false`，产生这个 Bug 的主要原因是

- 写代码时看到 `Path.endsWith(String)` 会想当然的主为会采用 `path.toString().endsWith(String)` 以字符串的方式来判断

如果进一步验证的话就会发现<!--more-->

```java
Path.of("a/bc/hello.txt").endsWith(".txt"); // false
Path.of("a/bc/hello.txt").toString().endsWith(".txt"); // true
Path.of("a/bc/hello.txt").endsWith("hello.txt"); // true
Path.of("hello.txt").endsWith(".txt"); // false
Path.of("a/bc/hello.txt").endsWith("c/hello.txt"); // false
Path.of("a/bc/hello.txt").endsWith("bc/hello.txt"); // true
Path.of("a/bc/hello.txt").endsWith("/bc/hello.txt"); // false
```

从以上不同情景下的输出可意识到 `Path.endsWith(String)` 是判断文件路径的最后一至多段路径是否与参数字符串相等，而不是判断文件名是否以字符串结尾。

如果跟踪该方法调用

```java
Path.endsWith(String other) {
    return endsWith(getFileSystem().getPath(other));
}
```

调用的是 Path 中另一个方法

```java
Path.endsWith(Path other);
```

JDK 在同时在 `Path` 中提供两个 `endsWith` 重载方法

```java
boolean endsWith(String other);
boolean endsWith(Path other);
```

`endsWith(String)` 极易造成误用为路径字符串的比较，如果只有 `endsWith(Path)` 方法就不会有这个问题, 因为显然不大可能会写出如下代码

```java
path.endsWith(Path.of(".xml"));
```

JDK 引入 `Path.endsWith(String)` 这个方法确实能直接使用字符串作为参数，而不用时时显式的构建 `Path` 对象而有所便利，
但同时更带来了方法误用的风险，这种方法便利就得不偿失了, 还不如只提供 `endsWith(Path)` 方法更为合理些。

最后，前面问题代码应该修正为

```java
try (Stream<Path> pathStream = Files.list(Paths.get("data"))) {
    pathStream.filter(path -> path.toString().endsWith(".xml"))
              .forEach(System.out::println); // do something with the file 
} catch(IOException e) {
    //...
}
```

真是走个捷径，结果却是误入了歧途。