---
title: "一个简单的 C++ 实现的 gRPC 服务"
url: /c++-grpc-server-example/
date: 2026-03-29T20:52:41-05:00
featured: false
draft: false
type: post
toc: false
# menu: main
usePageBundles: true
thumbnail: "/images/logos/cpp-logo.png"
categories:
  - Web
tags: 
  - React.js
comment: true
codeMaxLines: 30
# additional
lastmod: 
---

上一篇 [使用 Spring 的 gRPC 服务](/spring-grpc-service-demo/) 学习了如何用 Java SpringBoot 4 + Spring gRPC 实现服务端与客户端，
并且用 Postman 和 grpcurl 命令测试了如何访问 Java 实现的 gRPC 服务，其中还用 Wireshark 抓包工具进一步验证了 gRPC 的通信方式是 Protobuf
Over HTTP/2。

完后又突然对 C++ 实现一个 gRPC 服务产生了兴趣，因为现实项目是想要把 C++ 动态库的调用代码独立为一个进程，这时候直接用 C++ 代码调用 C++ 动态库
就是最简单直接的方式，不存在任何数据结构的转换。把这部分实现为一个 C++ 的 gRPC 服务，然后由 Java 的 gRPC 客户端来调用。

下面来看这样一个简单的例子，在 C++ 只实现一个只包含一个同步的 gRPC 的服务，测试将用 grpcurl 命令和 Postman，并且也用 Wireshark 抓包工具验证 gRPC 
的通信方式是否依然是 HTTP/2, Protobuf 数据编码肯定没得说了。 <!--more-->

项目构建将使用 `cmake`, 编译器将要 `g++`, 编译平台为 macOS，系统版本为 Tahoe 26.3, 芯片为 Apple 的 M3 Pro(ARM64)

### 系统准备

编译器 `g++` 随同 `CommandLineTool` 安装的，版本为

```bash
g++ --version
Apple clang version 17.0.0 (clang-1700.6.4.2)
Target: arm64-apple-darwin25.3.0
Thread model: posix
InstalledDir: /Library/Developer/CommandLineTools/usr/bin
```
用于从 proto 生成 C++ 源码的 `protoc`，`protobuf` 库, `grpc_cpp_plugin`, 和 `cmake` 分别由以下命令安装

```bash
brew install protobuf
brew install grpc
brew install cmake
```

如果是 Linux 平台，请找到相应的命令进行以上组件的安装。

### 创建 `hello.proto` 文件

```protobuf
syntax = "proto3";

package hello;

message HelloRequest {
  string name = 1;
}

message HelloResponse {
  string message = 1;
}

service HelloService {
  rpc SayHello (HelloRequest) returns (HelloResponse);
}
```

### 由 `proto` 生成 C++ 源文件

```bash
mkdir grpc-generated
protoc --grpc_out=grpc-generated --cpp_out=grpc-generated \
  --plugin=protoc-gen-grpc=$(which grpc_cpp_plugin) \
  hello.proto
```

上面命令会在 grpc-generated 目录中生成如下源文件

```text
grpc-generated
├── hello.grpc.pb.cc
├── hello.grpc.pb.h
├── hello.pb.cc
└── hello.pb.h
```

### 创建 `server.cpp` 文件

```cpp
#include <iostream>
#include <string>
#include <grpcpp/grpcpp.h>
#include <grpcpp/ext/proto_server_reflection_plugin.h>
#include "hello.grpc.pb.h"

using grpc::Server;
using grpc::ServerBuilder;
using grpc::ServerContext;
using grpc::Status;
using hello::HelloService;
using hello::HelloRequest;
using hello::HelloResponse;

class HelloServiceImpl final : public HelloService::Service {
    Status SayHello(ServerContext* context,
                    const HelloRequest* request,
                    HelloResponse* response) override {
        std::string msg = "Hello, " + request->name() + "!";
        response->set_message(msg);
        std::cout << "[Server] " << msg << std::endl;
        return Status::OK;
    }
};

int main() {
    std::string address = "0.0.0.0:50051";
    HelloServiceImpl service;

    grpc::reflection::InitProtoReflectionServerBuilderPlugin(); // 启用服务反射发现功能
    ServerBuilder builder;
    builder.AddListeningPort(address, grpc::InsecureServerCredentials());
    builder.RegisterService(&service);

    std::unique_ptr<Server> server(builder.BuildAndStart());
    std::cout << "Server listening on " << address << std::endl;
    server->Wait();

    return 0;
}
```

加上 `#include <grpcpp/ext/proto_server_reflection_plugin.h>` 和 `grpc::reflection::InitProtoReflectionServerBuilderPlugin();`
是为了启用服务反射发现的功能，不然在用 `grpcurl` 命令访问时会出现如下错误

> Error invoking method "hello.HelloService/SayHello": failed to query for service descriptor "hello.HelloService": 
> server does not support the reflection API

也就是说 `grpcurl` 依赖于服务发现功能。同样的没有启用它，在 Postman 中也必须导入 `hello.proto` 文件才能正常访问。

### 创建 `CMakeLists.txt` 文件

```cmake
cmake_minimum_required(VERSION 3.15)
project(grpc_server)

set(CMAKE_CXX_STANDARD 17)

find_package(gRPC REQUIRED)

file(GLOB SOURCES "*.cpp")
file(GLOB_RECURSE GRPC_SOURCES "grpc-generated/*.cc")

add_executable(server ${SOURCES} ${GRPC_SOURCES})
target_include_directories(server PRIVATE grpc-generated)

#target_link_libraries(server gRPC::grpc++ protobuf::libprotobuf)
target_link_libraries(server gRPC::grpc++ protobuf::libprotobuf gRPC::grpc++_reflection)
```

代码中若需启用服务发现功能，在 `target_link_libraries` 必须把 `gRPC::grpc++_reflection` 加上，否则无法编译。

### 编译并执行

```bash
cmake -B build
cmake --build build
```

编译成功后， 在 `build` 目录下生成了可执行文件 `server`, 执行它

```bash
build/server
Server listening on 0.0.0.0:50051
```

### grpcurl 测试

用 `grpcurl` 命令

```bash
grpcurl -plaintext -d '{"name":"World"}' localhost:50051 hello.HelloService/SayHello
{
  "message": "Hello, World!"
}
```

服务端当然也有相应的输出 `[Server] Hello, World!`。

### Postman 测试

在测试之前就有点好奇 Postman 能发现些什么服务，看图

{{< bundle-image "c++-grpc-postman.png" 500 >}}

只有 `SayHello` 一个服务，C++ gRPC 并没有像 Spring gRPC 那样自动加入 `Watch` 和 `Check` 那样的检测服务，有这种需求的话还得自行添加。
在 Postman 中的请求测试就不演示了，参考 `grpcurl` 的结果就清楚了。

### Wireshark 抓包验证

执行 `grpcurl -plaintext -d '{"name":"World"}' localhost:50051 hello.HelloService/SayHello` 的同时，开启 Wireshark 抓下网络包

{{< bundle-image "c++-grpc-wireshark.png" 900 >}}

gRPC Protobuf Over HTTP/2, 没问题，通信效率和性能上应该可以得到保障。

### 为何选择 C++ 编写 gRPC 服务

对于独立受管理的 gRPC 服务端进程，用 C++ 编写比 Java 的实现优势是明显的，内存消耗低，启动快, 无 GC, 手动管理内存，所以性能更优。
对于极致的性能追求，毫无疑问是要选择 C++ 编写 gRPC 服务。

那么是否可用 Rust 语言来编写呢？在 [gRPC - Supported languages](https://grpc.io/docs/languages/) 中没看到官方目前有对 Rust 语言的支持.
不过 `Protocol Buffers` 已支持 `Rust` 语言，用 `protoc` 编译可看到它可用 `--rust_out=OUT_DIR` 参数，也就是说 `protoc` 能编译生成
`Rust` 代码。

我们来 Vibe Coding 试一下，在 `grpc-rust` 目录中放置前面的 `hello.proto` 文件，然后用 `claude` 试下

> based on current hello.proto file, gernate gRPC service by using Rust lanaguage, project managed by cargo, 
> and enable service relection feature, start server to listen to ipv4 address 0.0.0.0:50051

AI 全部写完，构建，然后执行 `target/debug/server`, 一切正常，测试服务也没问题，用 Wireshark 看到的也是工作在 HTTP/2 协议之上。

此处就没必要列出相关 Rust 代码，在 `Cargo.toml` 中所用到的依赖列表是

```toml
[dependencies]
tonic = "0.12"
tonic-reflection = "0.12"
prost = "0.13"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```
从版本号来看，`tonic` 和 `prost` 还是零点几，但从网上来看 `tonic` 已经是一个比较成熟的 gRPC 实现了，可用于生产环境中。