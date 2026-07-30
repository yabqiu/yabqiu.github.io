---
title: Java 25 新特性学习 - Scoped Values
url: /java-25-new-features-scoped-values/
date: 2026-04-24T23:34:00-05:00
featured: false
type: post
draft: false
toc: false
# menu: main
usePageBundles: true
thumbnail: "/images/logos/java-logo.png"
categories:
  - java
tags:
  - java 25
series: Java New Features
comment: true
codeMaxLines: 50
showLastmod: true
---
 
Java 25 是一个 LTS 版本，它的众多新特性中就数 [`JEP 506: Scoped Values`](https://openjdk.org/jeps/506) 能改变我们从前使用
`ThreadLocal` 时的编程范式。因其重要性，所以单列一篇来专门学习它。`Scoped Values`, 可用来替代 `ThreadLocal` 的使用，特别是在虚拟线程当中。

`Scoped Values` 用于在当前线程或子线程中共享不可变的数据，说到不可变的数据就会想到 `JEP 502: Stable Values (Preview)`.
`ThreadLocal`， 甚至 `InheritableThreadLocal` 用于子线程中共享数据还是有些挑战性，特别碰到线程池的情况, 而且 `ThreadLocal`
不在乎数据可变还是不可变的，执行当中谁改了数据，不知道。`Scoped Values` 的 JEP 说其目的不是用来完全替代 `ThreadLocal`。

下面来看看 `Scoped Values` 如何在子线程中共享数据，它还能与虚拟线程的结构化并发配合使用。

`ThreadLocal`/`InteritableThreadLocal` 仍然是很多框架用来在线程中(间)共享数据的办法，在 `Spring` 框架中有大量的使用， 如
`XxxContextHolder` 之类的。目前的 `ThreadLocal` 存在一些问题

- ThreadLocal 中的数据是可变的，多数应用场景只要不可变的数据
- 难以管理共享数据的生命周期，特别是在线程池中，线程被重用时，ThreadLocal 中的数据可能会被意外共享或泄露。由使用方主动清除数据，更是会造成潜在的 Bug
- 成本高昂，创建子线程，或线程切换时要对共享数据进行复制，`VirtualThread` 也是继承自 `Thread`, 所以虚拟线程也能用 `ThreadLocal`。
  一旦虚拟线程的数量达到成千上万，十百万的级别时，`ThreadLocal` 数据不停复制的代价就很高了
<!--more-->

`Scroped Values` 就是设计来解决以上问题的，减少像 `ThreadLocal` 的复杂性; 数据为不可变的话，就能更高效的共享，特别是在有巨量的虚拟线程时;
数据在超过共享期后自动清除。

除去成本来说，个人觉得 `ThreadLocal` 用起来还是很方便的，可以让两个不怎么相关的代码共享数据。比如用 `ThreadLocal` 时，共享代码的方式为

```java
public class Test {
    public static void main(String[] args) {
        A.foo();
    }
}

class A {
    public static void foo() {
        Holder.set("value a");
        B.bar();
    }
}

class B {
    public static void bar() {
        System.out.println(Holder.get()); // value a
    }
}

class Holder {
    private static final ThreadLocal<String> threadLocal = ThreadLocal.withInitial(() -> "default");

    public static String get() {
        return threadLocal.get();
    }

    public static void set(String value) {
        threadLocal.set(value);
    }
}
```

就是使用线程池的时候，经常要把线程绑定的变量手动复制并绑定到任务子线程，任务执行完后还得从子线程上清除掉。这就是 `Spring` 的 `TaskDecorator`
经常做的，比如把 `slf4j` 的 `MDC(Mapped Diagnostic Context)` 数据复制到子线程中的做法

```java
ThreadPoolTaskExecutor executor = new ThreadPoolTaskExecutor();
executor.setTaskDecorator(runnable -> {
    Map<String, String> mdcContextMap = MDC.getCopyOfContextMap();
    if (mdcContextMap == null) {
        return runnable;
    }
    return () -> {
        try {
            MDC.setContextMap(mdcContextMap);
            runnable.run();
        } finally {
            MDC.clear();
        }
    };
});
```

倒是想看看 `Scoped Values` 如何在线程中共享数据。用 `Claude` 告诉它用 `Scoped Value` 来重构上面的代码的话，会被改成

```java
class A {
    public static void foo() {
        ScopedValue.where(Holder.VALUE, "value a").run(B::bar);
    }
}

class B {
    public static void bar() {
        System.out.println(Holder.VALUE.orElse("default")); // value a
    }
}

class Holder {
    static final ScopedValue<String> VALUE = ScopedValue.newInstance();
}
```

`Test` 类不变，用 `Scoped Values` 关键就在 `ScopedValue.where()` 和 `ScopedValue.run()` 上。这两方法的原型为

```java
    public static <T> Carrier where(ScopedValue<T> key, T value) {
        return Carrier.of(key, value);
    }
    
    Carrier.run(Runnable op);
    Carrier.call(CallableOp<? extends R, X> op) throws X;
```

这种简单问题用 `Scoped Value` 好像解决的不错，可是很多现实问题，比如在方法 `a` 与  `b` 经常许多的方法调用，代码跨越了很多类，就不是一个

```java
ScopedValue.where(Holder.VALUE, "value a").run(() -> {
    // method a
    // method b
});
```

能简单解决的。再如果要共享更多的数据，可能会写出 `ScropedValue.where()` 嵌套出来

```java
ScopedValue.where(Holder.VALUE1, "value 1").run(()-> {
    ScopedValue.where(Holder.VALUE2, "value 2").run(() -> {
        // method a
        // method b
        ...
    });
});
```

想要跨线程如何共享数据呢？尝试下面的代码

```java
public class Test {
    static final ScopedValue<Object> VALUE = ScopedValue.newInstance();

    public static void main(String[] args) {
        var obj = new Object();
        System.out.println(obj);
        ScopedValue.where(VALUE, obj).run(()->{
            new Thread(()-> {
                System.out.println(VALUE.get());
            }).start();
        });
    }
}
```

执行出错

```text
Exception in thread "Thread-0" java.util.NoSuchElementException: ScopedValue not bound
	at java.base/java.lang.ScopedValue.slowGet(ScopedValue.java:571)
	at java.base/java.lang.ScopedValue.get(ScopedValue.java:564)
	at Test.lambda$main$1(Test.java:9)
	at java.base/java.lang.Thread.run(Thread.java:1474)
```

如果用 `InteritableThreadLocal` 是可解决这个问题的。

继续阅读 [JEP 506](https://openjdk.org/jeps/506).

对于一个应用框架，如 `SpringBoot Web` 框架，每个请求都必经一些关口，如 `org.springframework.web.servlet.service(request, response)`
方法，或各级 `RequestFilter` 的 `doFilter()` 都可以开始一个 Scope, 然后自己写的 `Controller` 方法自然而然的涵盖进了这个 `Scope`, 
也就能访问到其中的共享数据，等 `servlet.service()` 或 `doFilter()` 结束后共享数据自动被清除，这样就不想考虑往一个 HTTP 线程放进去的
`ThreadLocal` 数据会不会污染到另一个 HTTP 请求。

{{< bundle-image java25-scoped-value-1.png 550 >}}

对于我们最容易的使用方式应该是在自定义的 `RequestFilter` 中, 下面就试着用 `Scoped Value` 来重构一个 `SpringBoot` Web 程序的就用场景，
请求进入后，首后获得或生成 requestId 和 `userId`, 然后在该 HTTP 线程上随时可用, 三个代码来演示

#### RequestContext
```java
package yanbin.blog;

public record RequestContext(String requestId, String userId) {
    static ScopedValue<RequestContext> holder = ScopedValue.newInstance();

    public static String getRequestId() {
        return holder.get().requestId;
    }

    public static String getUserId() {
        return holder.get().userId;
    }
}
```

让 `CustomRequestFilter` 和 `RequestContext` 同处一个包中，这样在 `CustomRequestFilter` 中可以访问到 `holder`, 其他地方只能调用
`RequestContext.getRequestId()` 和 `RequestContext.getUserId()` 来访问共享数据了。

#### CustomRequestFilter
```java
package yanbin.blog;

import jakarta.servlet.FilterChain;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import org.jspecify.annotations.NonNull;
import org.springframework.core.Ordered;
import org.springframework.core.annotation.Order;
import org.springframework.stereotype.Component;
import org.springframework.web.filter.OncePerRequestFilter;

import java.util.UUID;

@Component
@Order(Ordered.HIGHEST_PRECEDENCE)
public class CustomRequestFilter extends OncePerRequestFilter {

  @Override
  protected void doFilterInternal(@NonNull HttpServletRequest request, @NonNull HttpServletResponse response,
                                  @NonNull FilterChain filterChain) {

    ScopedValue.where(RequestContext.holder,
                    new RequestContext(UUID.randomUUID().toString(), "user-1234"))
            .run(() -> {
              try {
                filterChain.doFilter(request, response);
              } catch (Exception e) {
                throw new RuntimeException(e);
              }
            });
  }
}
```

#### TestController

```java
package yanbin.blog;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class TestController {

    @GetMapping("/test")
    public String test() {
        return "requestId: %s, userId: %s".formatted(
            RequestContext.getRequestId(), RequestContext.getUserId());
    }
}
```

测试

```bash
curl http://localhost:8080/test
requestId: bf713edb-1a84-488f-9481-6be54ef9e8f3, userId: user-1234
```

不用 `ThreadLocal` 的方式，比如任何时候要在 HTTP 线程上找 `Request` 的相关数据就不必从 `RequestContextHolder.getRequestAttributes()`
中翻找。

### Scoped Values 像 Rust 那样的绑定

`Scopped Values` 的值绑定就点像 `Rust` 的规则一样

```rust
    let name = "value1";
    println!("{}", name); // value1
    {
        let name = "value2"; // shadowing, 遮盖了外部的 name
        println!("{}", name); // value2
    }
    println!("{}", name); // value1
```

在 `Rust` 中 `let` 被称作值的绑定，块内部的绑定临时遮盖外部绑定的值，Java 的 `Scoped Values` 也有类似的效果

```java
    ScopedValue.where(VALUE, "value1").run(() -> {
        System.out.println(VALUE.get());     // value1
        ScopedValue.where(VALUE, "value2").run(() -> {
            System.out.println(VALUE.get()); // value2
        });
         System.out.println(VALUE.get());    //value1
    });
```

`Scoped Values` 在实现上也确实如其名所表达的那样，在每个 `where(X, v).run(()->...)` 时就会开启一个 `Scope` 并在该 `Scope` 上绑定值，
再来 `where(Y, w).run(()->...)` 又会往下层创建一个 `Scope`, 在 `Scope` 中要使用某个值时则从内往外找，找到即止。 

### 子线程上获取 `Scoped Values`

`InteritableThreadLocal` 会让子线程自动继承父线程上绑定的值，但必须父线程上有绑定值后用 `new Thread()` 创建的线程(包括线程池中新创建的线程)
才能继承父线程上绑定的值。

前面提过这个疑问，那么用 `Scoped Values` 如何让子线程共享父线程上绑定的数据呢？最好的实现方式是在创建虚拟线程时用结构化并发(Structured
Concurrency API(JEP 505)), 使用类 `StructuredTaskScope`, 父线程上绑定的值会通过 `StructuredTaskScope` 自动被子虚拟线程继承，
而且这其中还不存在 `Scoped Values` 的拷贝过程。

代码演示

```java
    ScopedValue.where(VALUE, "123").run(() -> {
        try (var scope = StructuredTaskScope.open()) {
            scope.fork(() -> {
                System.out.printf("%s: %s: %s\n", "task1", Thread.currentThread(), VALUE.get());
            });
            scope.fork(() -> {
                System.out.printf("%s: %s: %s\n", "task2", Thread.currentThread(), VALUE.get());
            });
            scope.join();
        } catch (InterruptedException e) {
            throw new RuntimeException(e);
        }
    });
```

`StructuredTaskScope` 在 Java 25 和 26 中仍处于预览阶段，`scope.join()` 后绑定的值会被自动清除。`StructuredTaskScope` 还不能与传统
的线程池(像 FokJoinPool)一起使用，这是一个缺憾，只能用于虚拟子线程。

上面代码的输出结果为

```text
task1: VirtualThread[#28]/runnable@ForkJoinPool-1-worker-1: 123
task2: VirtualThread[#30]/runnable@ForkJoinPool-1-worker-2: 123
```

值是传递到了子虚拟线程去了。

### Scoped Values 真的是不可变的吗？

`Scoped Values` 的 `Immutable` 只是说一旦向线程绑定了值后，在相同的范围内不能像 `ThreadLocal.set(value)` 那样可以重新绑定值，只能通过
`ScopedValue.where(key, value)` 来绑定并生成一个 `Scope`, 如果绑定的值的内部状态是可变的，执行下方的代码

```java
public class Test {
    static final ScopedValue<User> VALUE = ScopedValue.newInstance();

    public static void main(String[] args) {
        User user = new User("name1");
        ScopedValue.where(VALUE, user).run(() -> {
            User u = VALUE.get();
            System.out.println(u.name);     // name1
            u.name = "name2";
            System.out.println(VALUE.get().name);     // name2
        });
        System.out.println(user.name);     // name2
    }

    static class User {
        public String name;

        public User(String name) {
            this.name = name;
        }
    }
}
```

输出值分别为

> name1 <br/>
> name2 <br/>
> name2

### 迁移到 `Scoped Values`

在我们使用 `ThreadLocal` 时应首先考虑一下是否可以用 `Scoped Values` 实现。`Scoped Values` 能自动检测递归，可用于嵌套事物当中，
比如当前已存在一个事物则当前操作自动加入到该事物当中，Spring 的事务就是靠 `Aspect` 和 `ThreadLocal` 实现的。

`Scoped Values` 在一个范围中绑定多个值可以把多个 `where()` 串联起来，如

```java
    ScopedValue<String> VALUE1 = ScopedValue.newInstance();
    ScopedValue<String> VALUE2 = ScopedValue.newInstance();
    ScopedValue.where(VALUE1, "111").where(VALUE2, "222").run(() -> {
        System.out.println(VALUE1.get());     // 111
        System.out.println(VALUE2.get());     // 222
    });
```

以后还要继续关注 `structured concurrency` 的发展，不知到后面 `Scoped Values` 能不能支持平台线程，以及传统的线程池。

由于 `Scoped Values` 尚不支持平台线程和传统的线程池，在现有代码使用了 `ThreadLocal` 的情况下(如 SLF4J MDC), 有时候必须手动的在
`Scoped Values` 和 `ThreadLocal` 之间拷贝数据。