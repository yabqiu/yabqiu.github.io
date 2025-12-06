Rust 项目一旦增大，用 `cargo new demo` 创建的单一包，单个 src/main.rs 的项目组织方式不能满足需求了

> $ cargo new demo  
>  $ tree demo  
>  demo  
>  ├── Cargo.toml  
>  └── src  
>            └── main.rs

比如至少要一个 src/lib.rs 文件吧，复杂些还需在  src 目录中创建模块层次的目录; 更大型项目还要在 Package 上边创建 Workspace。 这里就引出了 Rust 项目的几个概念，即 Package, Crate, 模块，以及 Workspace，再就是如何在代码中引用不同 Package, Crate, 模块中的资源要用到路径。 比如这个最基本的 demo 项目中

1.  Package: 一个可以构建，测试和分享 Crate 的单元，它可包含可选的 lib crate 和多个二进制 crate 项。  demo 就是一个 Package，src/main.rs 就是一个二进制 crate, 如果有 src/lib.rs 就是一 个 lib crate, 它只能有一个。其他的放在 src/bin/\* 中的多个 \*.rs 文件是一个个独立的二进制 crate，它们会被编译成多个执行文件。下面将会演示。
2.  Crates: Rust 编译器编译的最小单位。每个  crate 会输出一二进制文件或 lib
3.  Module: crate 内部的代码层次组织结构，由 mod xxx; 或  mod xxx { ... } 定义
4.  Path: 访问 module, 函数，类型等和路径，分绝对路径(crate::foo::bar::baz, lib::something::func)与相对路径(self::foo::bar, super::baz)

而 Workspace 是用来组织多个 Package 的，像下面的一个  my-workspace 例子

```rust
my-workspace                        # 这是一个 Workspace, 在这里运行 cargo build 将编译所有 Package，相当于类型为 pom 的 Maven 项目
├── Cargo.toml                      # Workspace 有自己的 Cargo.toml, 其中列举了气管理的 Package(account, billing)
├── account                         # 这是一个 Package
│   ├── Cargo.toml                  # Package 也有自己的 Cargo.toml 文件，其下有一个或多个 Crate
│   └── src
│       ├── address                 # 从此开始的三行为 Crate 中模块的组织
│       │   └── mail.rs
│       ├── address.rs              # address.rs 必须与模块目录 address 同名
│       ├── bin                     # bin 目录下的每一个 *.rs 文件对应为一个二进制 crate, 它们都需要有 main 函数
│       │   ├── delete_account.rs   # 编译生成  delete_account 执行文件，Windows 下为 delete_account.exe
│       │   └── show_account.rs     # 编译生成 show_account 执行文件，Windows 下为 show_account.exe
│       ├── lib.rs                  # 可选的 lib crate, 编译后生成 libaccount.rlib
│       └── main.rs                 # 也是可选的，crate 的主二进制 crate, 文件名必须为 main.rs, 编译后生成执行文件 account (Windows 下为 account.exe)
└── billing                         # 另一个 Crate，遵循与 account 的组织形式
    ├── Cargo.toml
    └── src
    └── main.rs
```

上方的注释应该对我们理解 Rust 项目的组织结构有所帮助。 my-workspace  中的 Cargo.toml 内容为

```
[workspace]
resolvers = "3"
members = [
    "account",
    "billing"
]
```

在 my-workspace 目录中运行 `cargo build` 之后，在 my-workspace/target/debug 中生成的主要文件有可执行文件 account, billing, delete\_account, show\_account, 以及库 libaccount.rlib. 进到 my-workspace/account 目录中运行  `cargo build` 构建生成的产物也是在 my-workspace/target 目录中，而不是在 account/target 中，但在此时只会构建  account 包。 在 Rust 中，通常 Crate 指的就是库(Library)。Cargo 包中有以几个 Crate 的约定(不需要在 Cargo.toml 中特别配置)

1.  src/main.rs: 与包同名的二进制 crate 的 crate  根，将生成执行文件 account
2.  src/lib.rs:  与包同名的库 crate 的 crate 根，将生成库文件 libaccount.rlib
3.  其余的二进制 crate 只要把 \*.rs 文件放在 src/bin 目录下即可，将生成与 \*.rs 文件同名的可执行文件。这很方便我们在一个包中创建多个可执行文件。

src/main.rs 和 src/lib.rs  组成根模块，其余的每一个 \*.rs 都可认为是一个模块，有点像 Python 的文件即模块，但 Rust 的每一个模块都需要显式的声明(从根开始)。

### 如何使用 src/lib.rs 中模块和函数

一个只有  src/main.rs 文件的 Rust 项目见的太多，接下来我们把某些内容移到  src/lib.rs  中，看如何使用。直接贴出 src/lib.rs 和 src/main.rs 的代码 src/lib.rs

```
pub fn get_account_info(_id: &str) -> String {
    "account package, source: lib.rs".to_string()
}

pub mod utils {
    pub fn helper_function() -> String {
        "This is a helper function from account::utils".to_string()
    }
}
```

src/main.rs

```
use account::get_account_info;
use account::utils::helper_function;

fn main() {
    get_account_info("xyz");
    helper_function();
}
```

注意两点：

1.  写在 src/lib.rs 中的代码属于和包(account) 同名的库，所以用 use account::\* 的方式引用
2.  src/lib.rs 中只有 pub 的模块或函数才允许被  src/main.rs 引用

或全部内联的写在  src/lib.rs 中

```
pub mod address {
    pub mod mail {
        pub fn get_mail(user: &str) {
            println!("Getting mail for user: {}", user);
        }
    }
}
```

在 src/main.rs 中使用

```
fn main() {
    account::address::mail::get_mail("xyz")
}
```

但项目就膨胀，必定是不能全写在 src/lib.rs 中了，需要用更多的文件来拆分实现。这就是后面将要学习到的模块与子模块。

### 不使用包为库名，及库文件 src/lib.rs 的约定

如果不想遵循库文件为 src/lib.rs 以及包(account) 为库名的约定，该如何配置呢？在 account/Cargo.toml 中配置 \[lib\] 区块的内容为

```
 class="lang:default decode:true">[lib]
name = "mylib"            # 命名自己的库名，而非与包同名的库
path = "src/mylib.rs"     # 定义自己的库文件，而非 src/lib.rs
```

现在只要把 src/lib.rs 更名为 src/mylib.rs, 其中的内容保持不变，最后在 src/main.rs 中使用 mylib 库时 use 语句变换为如下

```
 class="lang:default decode:true ">use mylib::get_account_info;
use mylib::utils::helper_function;
```

当然，编译后在 target/debug 目录中看到就库文件就是 libmylib.rlib, 不再是 libaccount.rlib。

### 使用自定义模块

src/lib.rs 是一个库 crate, 用起来也像是一个模块，与库 crate 同一级别的，我们可创建自定义的模块，就是前面的 address 模块。 当 rustc(或 cargo build) 编译时首先从 crate 根文件(如 src/lib.rs 和 src/main.rs) 中寻找要编译的代码。在 crate 根文件中还可以声明自定义模块，Rust 编译寻找模块要从 crate 根文件开始。 之所以把  src/main.rs 和  src/lib.rs 称之为 crate 根，是因为这两个文件内容在 crate  模块结构的根组成了一个名为  `crate` 的模块。从后面的路径引用也会发现模块可从 `crate::` 开始。 我们将要创建一个 address 模块，首先需要在 crate 根文件(如 src/lib.rs 或 src/main.rs 中) 声明模块 address, 我们以  src/main.rs 为例(为 src/lib.rs  所用的模块就声明在 src/lib.rs 中)，有以下三种方式自定义模块

#### 1）内联方式

直接在  src/main.rs 中声明并定义

```
 class="lang:default decode:true">mod address {
    pub fn get_address(_id: &str) -> String {
        todo!()
    }
}
```

虽然写在 src/main.rs 文件中，但在它的 main() 想要调用的话，address::get\_address() 函数也必须声明为 pub

#### 2）同名文件 src/address.rs 中

做法是同样需要在根 crate src/main.rs 中声明 address 模块

```
 class="lang:default decode:true ">mod address;
```

只是实现部分移入到 src/address.rs 中, 内容为

```
pub fn get_address(_id: &str) -> String {
    todo!()
}
```

#### 3）或实现写在 src/address/mod.rs 中

此种方式与前一种方式唯一的不同之处就是把 src/address.rs 的内容放到了  src/address/mod.rs.  在 src/main.rs 中使用方式为

```
fn main() {
    address::get_address("xyz");
}
```

\*\*/mod.rs 是老旧的风格，不过仍然支持，在新项目中不推荐使用该风格。

### 子模块的声明

子模块为模块的模块，当我们一旦确定了从某一个根 crate(src/main.rs 或  src/lib.rs) 文件中声明的 `mod address` 引导到了 `src/address.rs` 后，就可从这里开始声明 address 的子模块，同样的有三种方式(address.rs 中内联模块 mail, 子模块文件 src/address/mail, 或 src/address/mail/mod.rs)。 我个人觉得可以摒弃 \<module>/mod.rs 的方式 从声明子模块的方式也能帮助我们理解如何声明模块的方式，或者要声明更深层次的子级模块。 下面是一个有子模块 address/mail 的项目结构

```
account
├── Cargo.toml
└── src
    ├── address
    │   └── mail.rs
    ├── address.rs
    └── main.rs
```

main.rs 的内容

```
pub mod address;

fn main() {
    address::mail::get_mail("xyz");  # 使用子模块
}
```

在该根 crate 中用  `pub mod address`  声明一个模块，它有两个目的

1.  Rust 编译器由根 crate 由此找到需编译 address 模块
2.  由声明的 address 定位到 src/address.rs 或 src/address/mod.rs 文件

address.rs 内容

```
pub mod mail;
```

编译器追踪到了这里，在 address 中又声明了一个子模块 mail, 那就会要求存在文件 src/address/mail.rs 或  src/address/mail/mod.rs 文件。这就是为什么有子模块时 src/address.rs 与目录 src/address 要同名。当我们在 IntelliJ IDEA 中修改 src/address.rs 文件名是，src/address 目录名也跟着变化。 src/address/mail.rs 的内容

```
pub fn get_mail(_id: &str) -> String {
    "mail package, source: address::mail".to_string()
}
```

注意，Rust  默认时模块，函数等的可见性为私有，只有 pub 时才能在其他模块中访问到，这不会是问题，编译器会清楚的提示。 关于模块或子模块，关键的地方就是要理解 Rust 如何从 crate  根节点(src/main.rs 或 src/lib.rs) 开始通过 mod 声明一路定位到模块实现文件的。 回顾一下根模块 crate 及整个模块树的结构现在就是

```
crate            # 由 src/main.rs 和  src/lib.rs 组成，在根文件中声明的 mod address 就会定位到 address 模块 src/address.rs
└── address      # 由 src/address.rs 中声明的 mod mail 进一步定位到 mail 子模块 src/address/mail.rs 文件
    └── mail     # 子模块 mail 的实现
```

在每一级模块都有自己放置实现代码或声明子一级模块的文件，如 src/address.rs 和 src/address/mail.rs。习惯用 mod.rs 的就是 src/address/mod.rs 和 src/address/mail/mod.rs，这会在项目中产生大量的无自描述能力的 mod.rs 文件。

### 关于引用模块树中荐的路径

记住根模块名为 crate，所以有相应的绝对和相对引用路径， 下面的各种方式多试试就明白了，此路不通必有路。

```
 class="lang:default decode:true">use address::mail;
use crate::address::mail;

super::mail::get_mail(user);
self::get_mail("xyz");
```

### 其他一些 use 相关用法

use as 别名

> use std::io::Result as IoResult;

pub use 重导出，私有的模块或函数，并改变外部访问路径 比如 src/lib.rs 中

```
 class="lang:default decode:true">mod address {
    pub fn get_address(_id: &str) -> String {
        todo!()
    }
}

pub use address::get_address;  # 没有这行，foo() 中无法直接用 get_address("xyz"), 必须用 address::get_address("xyz")

pub fn foo(){
    get_address("xyz");
}
```

在 src/main.rs 中

```
fn main() {
    account::get_address("xyz");
}
```

没有前面的 `pub use` 语句，这里用全路径 `account::address::get_address("xyz")` 也访问不了该方法，因为它是私有的。有了 `pub use` 语句，还能直接由根模块 account::get\_address("xyz")  引用，不同中间的 `address`。

### 关于 Cargo 工作空间

前面提到过  Cargo 的 Workspace 就类似于 Maven 中类型为 pom 的项目。同一 Workspace 中的所有 Package 共享同一个 Cargo.lock 文件。 同一个 Workspace 中并不假定 package 之间是互相依赖的，所以需要显式的声明依赖。例如我们想在 billing 中调用 account/src/lib.rs 中定义的 address::get\_address(\_id: &str) 函数，首先须在 billing/Cargo.toml 中配置

```
 class="lang:default decode:true ">[dependencies]
account = {path = "../account"}
```

然后就能在 billing/src/main.rs 使用了

```
 class="lang:default decode:true ">use account;

fn main() {
    account::address::get_address("xyz");
}
```

在 Workspace 目录上运行 `cargo test --workspace` 会执行所有 Package 的测试，要测试特定 Package 中的测试用

> cargo test -p billing

与 mvn 命令一样的. 只构建某一个 Package 用

> cargo build -p billing

### 用 cargo install 安装二进制文件

这与本文的内容不相关，只借此地记录一下，一个 Rust  项目有  src/main.rs 会生成与包同名的二进制文件，还有放在 src/bin/ 目录中的 \*.rs  会生成对应的二进制文件，如果想把这些二进制文件安装到本地可直接使用的话，以前的  `cargo install`  会安装到 `~/.cargo/bin/` 目录中，现在不支持了

> cargo install  
>  error: Using \`cargo install\` to install the binaries from the package in current working directory is no longer supported, use \`cargo install --path .\` instead. Use \`cargo build\` if you want to simply build the package.

需显式指定 `--path`, `cargo install --path .`

> cargo install --path .  
>  Installing account v0.1.0 (/Users/yanbin.qiu/Desktop/my-workspace/account)  
>  Finished \`release\` profile \[optimized\] target(s) in 0.04s  
>  Replacing /Users/yanbin/.cargo/bin/account  
>  Replacing /Users/yanbin/.cargo/bin/delete\_account  
>  Replacing /Users/yanbin/.cargo/bin/show\_account  
>  Replaced package \`account v0.1.0 (/Users/yanbin/my-workspace/account)\` with \`account v0.1.0 (/Users/yanbin/my-workspace/account)\` (executables \`account\`, \`delete\_account\`, \`show\_account\`)

在 ~/.cargo/bin 下生成了三个执行文件 account, delete\_account  和  show\_account, 查看了下环境变量 $PATH, 其中包含了 /Users/yanbin/.cargo/bin，因此在任何地方都能执行它们。 自定义 Cargo 扩展命令只要求在  $PATH 下有 `cargo-something` 的二进制文件，就能用 `cargo something` 的方式执行，像 AWS Lambda 扩展用的 `cargo lambda build`, 和 git 要求的命令 `git-something` 相似。 其余更灵活的用途就是在 Cargo.toml 中自定义使用 lib, 模块等，非特别需求尽量遵循约定就是了。