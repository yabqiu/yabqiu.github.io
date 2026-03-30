---
title: "使用 Spring 的 gRPC 服务"
url: /spring-grpc-service-demo/
date: 2026-03-29T13:52:41-05:00
featured: false
draft: false
type: post
toc: false
# menu: main
usePageBundles: true
thumbnail: "/images/logos/spring-logo.png"
categories:
  - Java
  - Spring
tags: 
  - gRPC
  - SpringBoot
comment: true
codeMaxLines: 30
# additional
lastmod: 
---

人们在跨语言, 跨进程通信方面采用过不少的方案, 交换文件, CORBA, SOAP, 最后得到最广泛应用的是 RESTful API, 交换格式通常用文本格式的 JSON 和
XML. 但作为更高效的通信还是二进制格式, 在 Java 方面, 有过 Java 内置的 RMI, Spring 的 Hessian, Dubbo. 而发展到今天 gRPC 受到了更多的关注,
gRPC 的通信协议是 HTTP/2, 编码格式是 Google 高效的 Protobuf.

[gRPC](https://grpc.io) 是由 Google 发起的一个远程调用框架, 是 **g**RPC **R**emote **P**rocedure **C**alls 的缩写, 各处的解释是
**g** 最初是代表 Google, 现在只是像 **Y**AML **A**in't **M**arkup **L**anguage 类似的命名, 说 **g** 不再代表 Google, 怎么听起来有点
既要又要的感觉, 怎么都会认为 gRPC 为 Google 的 RPC.

总之, gRPC 是一个高性能的开源统一的 RPC 框架, 能叫做统一是因为它支持多种语方, 如 Go, C++, Java, Python, Node, C#, PHP 等, 多语言支持是
由 Protobuf 格式决定的, 总之它是 Protobuf over HTTP/2, 实现上是用 Protobuf 定义数据结构与服务方法, 再映射成不同语言的代码实现. gRPC 
已然成为了 RPC 的标准, 真正意识到它的不一般是在 Postman 中发现了它, 可见其在业界受不到了应用的重视.<!--more-->

{{< bundle-image postman-grpc-1.png 600 >}}

本文来体验在 Java Spring 中如何应用 gRPC, 用 Spring 实现的服务端, 客户端, 并用 Postman 和 grpcurl 进行测试.

在 [spring initializr](https://start.spring.io/) 要选择 SpringBoot 4 才能选择 gRPC 依赖, Spring gRPC Server 或 Spring gRPC Client.
如果用 `Maven` 的, 选择这两个依赖后, 在 `pom.xml` 中对应的依赖是

```xml
    <dependency>
      <groupId>org.springframework.grpc</groupId>
      <artifactId>spring-grpc-client-spring-boot-starter</artifactId>
    </dependency>
    <dependency>
      <groupId>org.springframework.grpc</groupId>
      <artifactId>spring-grpc-server-spring-boot-starter</artifactId>
    </dependency>
```

在 SpringBoot 4 之前要用 gRPC 的话, 要使用如下依赖(无法用 Spring initializr 为 SpringBoot 3 选择 gRPC)

```xml
<dependency>
    <groupId>net.devh</groupId>
    <artifactId>grpc-server-spring-boot-starter</artifactId>
    <version>3.1.0.RELEASE</version>
    <scope>compile</scope>
</dependency>
<dependency>
<groupId>net.devh</groupId>
    <artifactId>grpc-client-spring-boot-starter</artifactId>
    <version>3.1.0.RELEASE</version>
    <scope>compile</scope>
</dependency>
```

Spring gRPC 的官方文档 [Getting Started](https://docs.spring.io/spring-grpc/reference/getting-started.html) 是这个, 当前版本
是 1.1.0-M1, 却没有正式版本 1.0.2 的文档, 这个 M1 版本的 Maven 依赖的 group 又变了

```xml
<dependency>
	<groupId>org.springframework.boot</groupId>
	<artifactId>spring-boot-starter-grpc-server</artifactId>
</dependency>
```

本文实例基于 [spring initializr](https://start.spring.io/), 选择 `Maven`, `Java 25`, `Spring Boot 4.0.5`, 再选择依赖
`Spring gRPC Server` 和 `Spring gRPC Client` 生成的 `pom.xml`, 然后创建相应的 proto, gRPC 的服务与客户端代码.

整个代码目录结构是

```text
test-grpc/
├── pom.xml
└── src
    └── main
        ├── java
        │   └── com
        │       └── example
        │           └── testgrpc
        │               ├── client
        │               │   ├── GrpcClientConfig.java
        │               │   ├── HelloGrpcClient.java
        │               │   └── RunGrpcClient.java
        │               └── server
        │                   ├── HelloGrpcService.java
        │                   └── StartGrpcServer.java
        ├── proto
        │   └── hello.proto
        └── resources
            └── application.properties
```

下面列出完整的代码

`pom.xml`

```xml
<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 https://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <parent>
        <groupId>org.springframework.boot</groupId>
        <artifactId>spring-boot-starter-parent</artifactId>
        <version>4.0.5</version>
        <relativePath/> <!-- lookup parent from repository -->
    </parent>
    <groupId>com.example</groupId>
    <artifactId>test-grpc</artifactId>
    <version>0.0.1-SNAPSHOT</version>
    <name>test-grpc</name>
    <description>test-grpc</description>
    <properties>
        <java.version>25</java.version>
        <grpc.version>1.77.1</grpc.version>
        <protobuf-java.version>4.33.4</protobuf-java.version>
        <spring-grpc.version>1.0.2</spring-grpc.version>
    </properties>
    <dependencyManagement>
        <dependencies>
            <dependency>
                <groupId>org.springframework.grpc</groupId>
                <artifactId>spring-grpc-dependencies</artifactId>
                <version>${spring-grpc.version}</version>
                <type>pom</type>
                <scope>import</scope>
            </dependency>
        </dependencies>
    </dependencyManagement>
    <dependencies>
        <dependency>
            <groupId>io.grpc</groupId>
            <artifactId>grpc-services</artifactId>
        </dependency>
        <dependency>
            <groupId>com.google.protobuf</groupId>
            <artifactId>protobuf-java</artifactId>
        </dependency>
        <dependency>
            <groupId>org.springframework.grpc</groupId>
            <artifactId>spring-grpc-client-spring-boot-starter</artifactId>
        </dependency>
        <dependency>
            <groupId>org.springframework.grpc</groupId>
            <artifactId>spring-grpc-server-spring-boot-starter</artifactId>
        </dependency>

        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-test</artifactId>
            <scope>test</scope>
        </dependency>
        <dependency>
            <groupId>org.springframework.grpc</groupId>
            <artifactId>spring-grpc-test</artifactId>
            <scope>test</scope>
        </dependency>
    </dependencies>
    <build>
        <plugins>
            <plugin>
                <groupId>io.github.ascopes</groupId>
                <artifactId>protobuf-maven-plugin</artifactId>
                <version>4.0.3</version>
                <configuration>
                    <protoc>${protobuf-java.version}</protoc>
                    <binaryMavenPlugins>
                        <binaryMavenPlugin>
                            <groupId>io.grpc</groupId>
                            <artifactId>protoc-gen-grpc-java</artifactId>
                            <version>${grpc.version}</version>
                            <options>@generated=omit</options>
                        </binaryMavenPlugin>
                    </binaryMavenPlugins>
                </configuration>
                <executions>
                    <execution>
                        <id>generate</id>
                        <goals>
                            <goal>generate</goal>
                        </goals>
                    </execution>
                </executions>
            </plugin>
            <plugin>
                <groupId>org.springframework.boot</groupId>
                <artifactId>spring-boot-maven-plugin</artifactId>
            </plugin>
        </plugins>
    </build>

</project>
```

其中除了 gRPC server/client 依赖外, 在 build/plugins 中还配置了由 `proto` 服务定义文件生成 Java 代码的插件.

`hello.proto`

```protobuf
syntax = "proto3";

package hello;

option java_package = "com.example.testgrpc.proto";
option java_outer_classname = "HelloProto";
option java_multiple_files = true;

message HelloRequest {
  string name = 1;
}

message HelloResponse {
  string message = 1;
  int64 timestamp = 2;
}

service HelloService {
  rpc SayHello (HelloRequest) returns (HelloResponse);

  rpc SayHelloStream (HelloRequest) returns (stream HelloResponse);
}
```

在 `hello.proto` 文件中不仅定义了数据结果, 还有两个服务方法. 分别是 `SayHello` 和 `SayHelloStream`, 前者是普通的请求响应, 后者是服务端流式响应.

如果我们此时执行`mvn generate-sources`, 或者 `mvn compile` 的话, 就会在 `target/generated-sources` 目录中生成相应的 Java 代码文件

```text
target/generated-sources
└── protobuf
    └── com
        └── example
            └── testgrpc
                └── proto
                    ├── HelloProto.java
                    ├── HelloRequest.java
                    ├── HelloRequestOrBuilder.java
                    ├── HelloResponse.java
                    ├── HelloResponseOrBuilder.java
                    └── HelloServiceGrpc.java
```

`HelloGrpcService.java`

```java
package com.example.testgrpc.server;

import com.example.testgrpc.proto.HelloRequest;
import com.example.testgrpc.proto.HelloResponse;
import com.example.testgrpc.proto.HelloServiceGrpc;
import io.grpc.stub.StreamObserver;
import org.springframework.stereotype.Service;

@Service
public class HelloGrpcService extends HelloServiceGrpc.HelloServiceImplBase {

    @Override
    public void sayHello(HelloRequest request, StreamObserver<HelloResponse> responseObserver) {
        HelloResponse response = HelloResponse.newBuilder()
                .setMessage("Hello, " + request.getName() + "!")
                .setTimestamp(System.currentTimeMillis())
                .build();

        responseObserver.onNext(response);
        responseObserver.onCompleted();
    }

    @Override
    public void sayHelloStream(HelloRequest request, StreamObserver<HelloResponse> responseObserver) {
        for (int i = 1; i <= 5; i++) {
            HelloResponse response = HelloResponse.newBuilder()
                    .setMessage("Hello " + request.getName() + " - #" + i + " message")
                    .setTimestamp(System.currentTimeMillis())
                    .build();
            responseObserver.onNext(response);

            try {
                Thread.sleep(500);
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
            }
        }
        responseObserver.onCompleted();
    }
}
```

实现了两个服务方法.

`StartGrpcServer.java`

```java
package com.example.testgrpc.server;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
public class StartGrpcServer {

    public static void main(String[] args) {
        SpringApplication.run(StartGrpcServer.class, args);
    }
}
```

这是一个 SpringBoot 的启动类, 运行后会启动 gRPC 服务.

### 启动 gRPC 服务

可在 IDE 中运行 `StartGrpcServer`, 下面用 `mvn` 命令的方式来启动它

```bash
mvn spring-boot:run -Dspring-boot.run.main-class=com.example.testgrpc.server.StartGrpcServer
```

因为后面还会有一个 gRPC 客户端的 main class, 所以这里用 `-Dspring-boot.run.main-class` 来指定要运行的 main class.

启动后控制台输出如下

```text
2026-03-29 12:20:01.578 | INFO  | StartGrpcServer | No active profile set, falling back to 1 default profile: "default"
2026-03-29 12:20:01.778 | INFO  | NettyGrpcServerFactory | Registered gRPC service: hello.HelloService
2026-03-29 12:20:01.778 | INFO  | NettyGrpcServerFactory | Registered gRPC service: grpc.reflection.v1.ServerReflection
2026-03-29 12:20:01.778 | INFO  | NettyGrpcServerFactory | Registered gRPC service: grpc.health.v1.Health
2026-03-29 12:20:01.815 | INFO  | GrpcServerLifecycle | gRPC Server started, listening on address: [/[0:0:0:0:0:0:0:0]:9090]
2026-03-29 12:20:01.816 | INFO  | StartGrpcServer | Started StartGrpcServer in 0.342 seconds (process running for 0.452)
```

从这里看到 gRPC 除了启动在 `hello.proto` 中定义的 `HelloService` 服务外, 还注册了 `grpc.reflection.v1.ServerReflection` 和
`grpc.health.v1.Health` 两个服务, 前者是为了让客户端能够通过反射来查询服务信息, 后者是为了提供健康检查的接口. gRPC 服务默认监听在 `9090` 端口上.
该端口号可在 `application.properties` 中通过 `spring.grpc.server.port=9090` 来配置, 如

```properties
spring.grpc.server.port=9091
```

### 测试 gRPC 服务

现在 gRPC 服务已经启动了, 可以用 Postman 来测试它. 在 Postman 中创建一个新的请求, 选择 `gRPC` 类型, 没输入 URL 之前, 右侧下拉可以看到
两个发现 gRPC 服务的方式, 1) Import .proto file, 2) Use Server Reflection

{{< bundle-image postman-grpc-2.png 672 >}}

可以选择第一种方式, 直接导入 `hello.proto` 文件, 就会列出在其中定义的两个服务

{{< bundle-image postman-grpc-3.png 710 >}}

再输入 URL `localhost:9090` 就能开始测试了.

或者选择第二种方式, 在前面启动 gRPC 服务时我们看到控制台输出说启动了 ServerReflection 服务, 需要事先输入 URL `localhost:9090`, 然后在
`Select a method` 中点击下拉框就会列出 gRPC 服务中注册的所有方法了, 不仅包括 `hello.proto` 中定义的 `hello.HelloService.SayHello`
和 `hello.HelloService.SayHelloStream` 服务方法, 还包括 `grpc.health.v1.Health.Check` 和 `grpc.health.v1.Health.Watch` 两个服务的方法.

{{< bundle-image postman-grpc-4.png 710 >}}

不管是哪一种发现服务的方式, 我们现在还测试一下 `hello.HelloService.SayHello` 方法, 选择它后, 在请求体中输入如下 JSON 数据

```json
{
  "name": "World"
}
```

{{< bundle-image postman-grpc-7.png 890 >}}

还有一个 `grpcurl` 命令行工具可用来测试 gRPC 服务, 正如从上图的右侧 Postman 显示出来的 `gRPCurl` 命令, 下面用 `grpcurl` 来测试

```bash
grpcurl -plaintext -d '{"name":"World"}' localhost:9090 hello.HelloService/SayHello
```

输出结果如下

```json
{
  "message": "Hello, World!",
  "timestamp": "1774806359051"
}
```

### 观察通信网络包

用 Wireshark 来观察 gRPC 的通信网络包, 过滤条件是 `tcp.port == 9090`, 可以看到 gRPC 的通信协议是 HTTP/2, 编码格式是 Protobuf.

{{< bundle-image wireshark-grpc.png 787 >}}

Protobuf 的编码格式是二进制的, 直接看网络包的内容是无法理解的, 需要用 Protobuf 的工具来解析它.

### 测试 SayHelloStream 方法

```bash
grpcurl -plaintext -d '{"name":"World"}' localhost:9090 hello.HelloService/SayHelloStream
{
  "message": "Hello World - #1 message",
  "timestamp": "1774807151281"
}
{
  "message": "Hello World - #2 message",
  "timestamp": "1774807151786"
}
{
  "message": "Hello World - #3 message",
  "timestamp": "1774807152286"
}
{
  "message": "Hello World - #4 message",
  "timestamp": "1774807152787"
}
{
  "message": "Hello World - #5 message",
  "timestamp": "1774807153289"
}
```

在控制台看到每隔 500ms 就会有一个新的响应, 直到最后服务端调用 `responseObserver.onCompleted()` 来结束流式响应.

在 Postman 中测试 `hello.HelloService.SayHelloStream` 方法

{{< bundle-image postman-grpc-6.png 750 >}}

### Spring gRPC 客户端

下面用 Spring gRPC 客户端代码来调用 `hello.HelloService` 服务, 代码分别如下

`GrpcClientConfig.java`

```java
package com.example.testgrpc.client;

import com.example.testgrpc.proto.HelloServiceGrpc;
import io.grpc.ManagedChannel;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.grpc.client.GrpcChannelFactory;

@Configuration
public class GrpcClientConfig {

    @Bean
    public HelloServiceGrpc.HelloServiceBlockingStub helloBlockingStub(GrpcChannelFactory channelFactory) {
        // 任意写的 channel 名称则使用默认的配置 spring.grpc.client.default-channel.*
        // 会使用 spring.grpc.client.channels.hello-service 下的配置
        ManagedChannel channel = channelFactory.createChannel("hello-service");
        return HelloServiceGrpc.newBlockingStub(channel);
    }

    @Bean
    public HelloServiceGrpc.HelloServiceStub helloAsyncStub(GrpcChannelFactory channelFactory) {
        // ManagedChannel channel = channelFactory.createChannel("hello-service");
        ManagedChannel channel = channelFactory.createChannel("192.168.1.100:9090");
        return HelloServiceGrpc.newStub(channel);
    }
}
```

定位 gRPC 服务时要创建  `ManagedChannel` 对象, 有以下几种方式

1. 任意写服务名称则使用默认的配置 `spring.grpc.client.default-channel.*`, 会访问 `localhost:9090`, 下的服务, 例如 
    `channelFactory.createChannel("any-service-name")`
2. 也可以直接指定远端服务地址, 如 `192.168.1.100:9090`
3. 或者使用在 `application.properties` 中配置的服务名称, 如 `channelFactory.createChannel("hello-service")`, 如果在
  `application.properties` 中有对应的配置, 则会使用相应的参数, 如
    ```properties
    spring.grpc.client.channels.hello-service.address=192.168.1.101:9090
    spring.grpc.client.channels.hello-service.negotiation-type=plaintext
    ```
   
有了相应的 `HelloServiceBlockingStub` 和 `HelloServiceStub` 对象后, 就可以调用 gRPC 服务了, 继续在 `RunGrpcClient.java` 中

```java
package com.example.testgrpc.client;

import com.example.testgrpc.proto.HelloRequest;
import com.example.testgrpc.proto.HelloResponse;
import com.example.testgrpc.proto.HelloServiceGrpc;
import io.grpc.stub.StreamObserver;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;

@Service
public class HelloGrpcClient {

    @Autowired
    private HelloServiceGrpc.HelloServiceBlockingStub blockingStub;

    @Autowired
    private HelloServiceGrpc.HelloServiceStub asyncStub;

    public String sayHello(String name) {
        HelloRequest request = HelloRequest.newBuilder()
                .setName(name)
                .build();

        HelloResponse response = blockingStub.sayHello(request);
        System.out.printf("Received response: %s\n", response.getMessage());
        return response.getMessage();
    }

    public void sayHelloStream(String name) throws InterruptedException {
        HelloRequest request = HelloRequest.newBuilder()
                .setName(name)
                .build();

        CountDownLatch latch = new CountDownLatch(1);

        asyncStub.sayHelloStream(request, new StreamObserver<HelloResponse>() {
            @Override
            public void onNext(HelloResponse response) {
                System.out.printf("Stream response: %s\n", response.getMessage());
            }

            @Override
            public void onError(Throwable t) {
                System.err.printf("Stream call error: %s", t.getMessage());
                latch.countDown();
            }

            @Override
            public void onCompleted() {
                System.out.println("Stream call complete");
                latch.countDown();
            }
        });

        latch.await(10, TimeUnit.SECONDS);
    }
}
```

创建客户端启动类 `RunGrpcClient.java`

```java
package com.example.testgrpc.client;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.CommandLineRunner;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
public class RunGrpcClient implements CommandLineRunner {

    @Autowired
    private HelloGrpcClient helloGrpcClient;

    public static void main() {
        SpringApplication.run(RunGrpcClient.class);
    }

    @Override
    public void run(String... args) throws Exception {
        String response = helloGrpcClient.sayHello("gRPC Client");
        System.out.println("Received response: " + response);

        System.out.println("Starting stream call...");
        helloGrpcClient.sayHelloStream("gRPC Stream Client");
    }
}
```

用 `mvn` 命令来运行客户端

```bash
mvn spring-boot:run -Dspring-boot.run.main-class=com.example.testgrpc.client.RunGrpcClient
```

执行后控制台相关的输出如下

```text
Received response: Hello, gRPC Client!
Received response: Hello, gRPC Client!
Starting stream call...
Stream response: Hello gRPC Stream Client - #1 message
Stream response: Hello gRPC Stream Client - #2 message
Stream response: Hello gRPC Stream Client - #3 message
Stream response: Hello gRPC Stream Client - #4 message
Stream response: Hello gRPC Stream Client - #5 message
Stream call complete
```

### gRPC 的优势与应用场景

在微服务架构中, 很多时候都会选择 RESTful API 来实现服务间的通信, 但服务不需要暴露给外部用户, 那么选择 gRPC 协议可获得更高的性能. 因为它使用
HTTP/2 协议和 Protobuf 编码格式, 相比于传统的 RESTful API 和 JSON/XML 格式, gRPC 的通信效率更高, 延迟更低. gRPC 充分利用到了 HTTP/2
的多路复用, 头部压缩, 流式传输的特性, 以及 Protobuf 的高效二进制编码格式(体积小, 序列化快), 使得它在服务间通信中表现出色. 另外 gRPC 还支持多语言, 
可以在不同的编程语言之间进行通信. 比如对于带宽受限的移动端或 IoT 设备, gRPC 的高效通信协议和编码格式可以显著降低网络开销, 提升性能. 
另外 gRPC 还支持双向流式通信, 适合于实时数据传输的场景.

一个现实的项目考虑, SpringBoot Web 项目中内部使用 C++ 的动态库, 这会造成 C++ 代码一崩溃整个 SpringBoot 服务就崩溃了, 这时可以把 C++ 
动态库调用放在一个独立的进程中, SpringBoot 与该外部进程就需要一种高效的通信方式, 如用 `.sock` 文件的 `Unix Domain Socket` 通信, 内存映射,
或自定义 TCP/UDP Socket 通信, 或往高层次的 RESTful API. 思考到这一步的话, `gRPC` 自然就成了比 `RESTful API` 更优的选择了.

也就是由 `gRPC` 来调用 C++ 的动态库, 这样就把 C++ 代码的崩溃风险隔离在了独立的进程中, 即使 C++ 代码崩溃了, 也不会影响到 SpringBoot 服务的稳定性.
但是需要在 `SpringBoot` 端实现 `gRPC` 进程的管理功能, 包括启动, 停止, 重启等操作, 以及监控 `gRPC` 进程的状态, 以确保它能够正常运行. 
在 `gRPC` 崩溃时能够重新启动一个新的 `gRPC` 进程来继续提供服务, 先前失败的调用需要重试, 还有为防止内存泄漏, 可设置在 `gRPC` 处理了若干请求
后由 SpringBoot 来重启它, 以释放 `gRPC` 进程占用的内存资源.