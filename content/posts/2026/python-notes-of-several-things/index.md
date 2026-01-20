---
title: "Python 编程过程碰到的几个问题"
url: /python-notes-of-several-things/
date: 2026-01-10T17:18:12-06:00
featured: false
draft: false
type: post
toc: false
# menu: main
usePageBundles: true
thumbnail: "/images/logos/python-logo.png"
categories:
  - python
tags: 
  - walrus
  - enum
comment: true
codeMaxLines: 50
# additional
lastmod: 
---

最近在紧锣密鼓的用 Python 写代码，不是 Vibe Coding 那种，因为在真实的敲代码才能碰到一些与 Python 语言相关且容易踏入的坑，记录如下

### Python 3.8 引入的 Walrus 操作

即赋值表达式，`:=` 的使用形式，像海象的两个牙。`:=` 让赋值有了返回值, 该赋值语句的返回值就是右边的值。C/C++ 的 `=` 同时具有 Python 的
`=` 和 `:=` 的功能。

我们在 Python 中的 `=` 与 `:=` 对比

```python
>>> print(a=1)
Traceback (most recent call last):
  File "<python-input-0>", line 1, in <module>
    print(a=1)
    ~~~~~^^^^^
TypeError: print() got an unexpected keyword argument 'a'
>>> print(a:=1)
1
```
Python 的这种赋值又有返回值的 `:=` 功能就可以应用到 `if` 或 `while` 语句中，例如<!--more-->

```python
if (sub_node := obj.node) is not None:
    print("process sub_node")
```

使用用了 `:=` 就不需像 Python 3.8 之前分两行写

```python
sub_node = obj.node
if sub_node is not None:
    pass
```

但这里很可能埋下一个潜在的 Bug， 对于 C/C++ 风格的语言，`if`, `while` 等语句后立即会写上括号，这不会有问题. 但写 Python 时又不断提醒自己
if, while 等后面不用括号的，于是很可能写成了

```python
if sub_node := obj.node is not None:
    print("process sub_node")
```

这样写语法上没有任何问题，但语义却发生了变化，这其实是因为后加进来的 `:=` 优先级最低，上面代码相当于是

```python
if sub_node := (obj.node is not None):
    print("process sub_node")
```

`sub_node` 是一个 bool 值，而非 obj.node 的值。

如果是各种数学运算符，搞不清楚优先级的情况下都会主动加括号，而 `:=` 的优先级往往被忽略。当然不知道 Python 有 `:=` 操作符的人就是不知者无罪。

### 通过不同方式引入同一个 enum 值却不等

这是一个实际项目中遇到的问题，由于 `import` 的方式不同而造成同一个 enum 值却是不同的实例(不同 id 值). 通过最小化项目终于能够重现了出来。

用 `uv` 命令创建一个 `demo` 项目

```shell
uv init --lib demo
```

将会在当前目录中创建一个 `demo` 目录，并在其中有 `src/demo` 子目录。 将用下面的文件进行重现

```text
src/
└── demo
    ├── __init__.py
    ├── main.py
    └── pkg1
        ├── __init__.py
        ├── colors.py
        ├── module1.py
        └── module2.py
```

以上代码文件 `main.py`, `pkg1/colors.py` `pkg1/module1.py`, `pkg1/module2.py` 的内容分别如下

```python {hl_lines="11 18 24"}
# pkg1/colors.py
from enum import IntEnum

class Colors(IntEnum):
    RED = 1
    GREEN = 2
    BLUE = 3
    
# main.py
from pkg1.module1 import *
from pkg1.colors import Colors

print("Colors.RED in main: ", Colors.RED, id(Colors.RED))

    
# pkg1/module1.py
from demo.pkg1.module2 import *
from .colors import Colors

print("Colors.RED in module1: ", Colors.RED, id(Colors.RED))


# pkg1/module2.py
from demo.pkg1.colors import Colors

print("Colors.RED in module2: ", Colors.RED, id(Colors.RED))
```

执行 `uv sync`, `uv` 会自动创建虚拟环境 `.venv`, 并在 `.venv/site-packages` 目录中会加上 `demo.pth` 文件用以说明 `demo` 包会加在
`sys.path` 中，该 `demo.pth` 的内容为

```text
/Users/yanbin.qiu/tests/demo/src
```

然后我们运行 `main.py`, 命令及输出结果如下

```shell
uv run python src/demo/main.py
Colors.RED in module2:  1 4323561808
Colors.RED in module1:  1 4323562192
Colors.RED in main:  1 4323562192
```

这时候我们发现在 module2 中的 `Colors.RED` 与别处的不同，如果把 `module2` 中的 `Colors.RED` 与别处的 `Colors.RED` 进行比较， 如

```python
if <red from module2> == Colors.RED:
    print("match")

match <red from module2>:
    case Colors.RED:
        print("match")
```

那么就有惊喜了，明明得到是一个 `Colors.RED` 类型， 但是与 `Colors.RED` 比较却不相等，也不匹配。

原因是上面采用不同的 `Colors` 引入方式

```python
# main.py 的
from pkg1.colors import Colors

# pkg1/module1.py 的
from .colors import Colors

# pkg1/module2.py 的
from demo.pkg1.colors import Colors
```

为何是 `module2` 中的 `Colors.RED` 与众不同呢？

下面试几种修改方式

#### 只把 pk1/module1.py 的改成 `from pkg1.colors import Colors`

```text
Colors.RED in module2:  1 4320415952
Colors.RED in module1:  1 4320415632
Colors.RED in main:  1 4320415632
```

仍然是 `module2` 中的 `Colors.RED` 不一样

#### 只把 pk1/module1.py 的改成 `from demo.pkg1.colors import Colors`

```text
Colors.RED in module2:  1 4323607888
Colors.RED in module1:  1 4323607888
Colors.RED in main:  1 4323571856
```

现在是  `main` 中的 `Colors.RED` 值不一样，原因是 `module1` 和 `module2` 引入  `colors` 的方式是一样了

#### 只把 main.py 的改成 `from demo.pkg1.colors import Colors`

```text
Colors.RED in module2:  1 4345171536
Colors.RED in module1:  1 4345171152
Colors.RED in main:  1 4345171536
```

现在是 `module1` 的 `Colors.RED` 值不一样。

到现在修复方法应该很简单了，全部采用从 `demo` 包开始的绝对路径肯定能解决，所以需要同时修改

```python
# main.py
from demo.pkg1.colors import Colors

# pkg1/module1.py
from demo.pkg1.colors import Colors
```

这个问题还不容易重现，不易重现的问题自然也不那么容易发现，明明都赋值成了同一个枚举类型，为何一比较却不相等了呢, 好象同样是 `Colors.RED`, 
但它们是来自于两个不同的命名空间。这一问题像极了是在 Java 中用了不同的 ClassLoader 加载了同一个类文件，但各自的类常量却不相等.

以后要注意的是，以后遇到类似问题，可以尝试使用绝对路径引入，避免类似于命名空间的冲突。

### 定义 @dataclass 时不小心加了逗号

Python 的逗号与括号都是要小心使用的，一个看似不起眼的逗号无意中却会改变数据类型, 例如下面的写法

```python
>>> a=1,
>>> type(a)
<class 'tuple'>
>>> a=(1)
>>> type(a)
<class 'int'>
>>> a=(1,)
>>> type(a)
<class 'tuple'>
```

所以有时候为了一个 `tuple` 类型，不得不后挂一个逗号.

如果在定义一个 `@dataclass` 时更容易忽略逗号所起的作用

```python
@dataclass
class Person:
    name: str = "John Doe",
    age: int = 30,
    address: str = "Earth"
```

这个定义在 Python 语法上没问题，看起来也很自然，每个属性后加个逗号用以分隔, 然而实际上却让有逗号结尾的属性变成了 `tuple` 类型，前面的 `str`,
或 `int` 没有实际约束力，

如果 `print(Person())` 就能看到输出类似下面

> Person(name=('John Doe',), age=(30,), address='Earth')

除非用 `mypy` 这样的静态检查工具

>src/demo/main.py:5: error: Incompatible types in assignment (expression has type "tuple[str]", variable has type "str")  [assignment]
>src/demo/main.py:6: error: Incompatible types in assignment (expression has type "tuple[int]", variable has type "int")  [assignment]

### 学到一个 enum 的 auto 用法

像前面定义的 `Colors` 如果有许多的颜色，全部要自己 1, 2, 3... 往下列太麻烦，但是有 `auto` 方法可以用，

```python
from enum import IntEnum, auto

class Colors(IntEnum):
    RED = 1
    GREEN = auto()
    BLUE = auto()
```

由上一个枚举的 `value` 往后递增，相当于 `GREEN = 2`, `BLUE = 3`. 也可以中间插入明确的值

```python
class Colors(IntEnum):
    RED = 1
    GREEN = auto()
    YELLOW = 5
    BLUE = auto()
```

还可以自定义递增方法，假如同时还给成员值指定是否是默认的颜色，可以改成下面的定义


```python
from enum import Enum, auto

class Colors(Enum):
    def __init__(self, id, is_default=False):
        self.id = id
        self.is_default = is_default

    def _generate_next_value_(name, start, count, last_values):
        return count

    RED = 1
    GREEN = auto(), True
    YELLOW = 5
    BLUE = auto()
```

我们查看该枚举中所有的成员

```python
if __name__ == '__main__':
    for name, member in Colors.__members__.items():
        print(name, member.value)
```

输出为

>RED 1
>GREEN (1, True)
>YELLOW 5
>BLUE 3

通过判断 member.value()[1] 或 `Colors.GREEN.is_default` 就知道是否是默认值。


### Python 中的代码块真的不能写太多行

C/C++ 风格的语言还能通过匹配大括号来确定层次关系，而 Python 用退格对齐的方式，一旦方法中写了太多的行，特别是超过一屏能显示的时候，
想要找代码的层次关系就比较困难了. 有时候借且 IDE 来数竖线

{{< bundle-image python-indention.png 218 >}}

如果是同一个块中的代码行，无法显示在同一屏幕时，想要确定是否在同一层次，或有没有包含关系就得十分小心了。比如本来应该子块时，却可能写在同一级上去了

```python
        for i in range(10):
            # ....
            # 上一屏的内容
```

```python
            # 这里有无数行，想象一下这个 for 已显示在屏幕之外了，由 for 没有 c/c++ 那样开闭符号 {
        ......    
        # 下面本来是作为嵌套的循环可能写到同一层次上去了
        for j in range(12):
            # ....

    # 或者写成了
    for j in range(12):
        # ....
```

加上 Python 的变量又无需事先定义，层次放错误，可能还没有语法错，只是改变了执行行为。

因此，写 Python 代码时更应该遵循单一功能，小代码块的约定, 个人觉得一个方法最好不要超过 30 行代码。