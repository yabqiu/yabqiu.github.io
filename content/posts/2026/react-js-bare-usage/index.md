---
title: "React.js 原始用法 - 掌握核心语法"
url: /react-js-bare-usage/
date: 2026-01-01T19:40:41-05:00
featured: false
draft: false
type: post
toc: false
# menu: main
usePageBundles: true
thumbnail: "/images/logos/react-logo.png"
categories:
  - Web
tags: 
  - React.js
comment: true
codeMaxLines: 50
# additional
lastmod: 
---

前端框架目前基本还是 Vue.js 和 React.js 两大阵营为主流，尽管 Angular.js 版本升的飞快, 事实就是鲜有人问津, 想用 Angular.js 
的人何不直接用 Vue.js 呢. 关于 Vue.js 和 React.js 的数据显示, Vue.js 延续了传统模板(如 JSP, Velocity, Freemarker, Thymeleaf)的用法,
通过自定义 Tag, 许多逻辑写到 HTML 里. 而 React.js 则另辟蹊径, 通过 JSX 语法将 HTML 直接写到 JavaScript 里, 页面显示时不夹杂逻辑.

以前算是用 Vue.js 做过一些小项目, 对 React.js 未有多少了解, 现在也算是初学. 刚开始不想从一个 `npx create-react-app my-react-app`
的脚手架开始, 而是想从最原始的 HTML 中引入 React.js 的方式开始学习, 这样可以更清晰地了解 React.js 的工作原理. 也是为使用 React.js 
拥抱 Vibe Coding 做准备.

### 基本 React.js Virtual DOM 渲染

用 `<script>` 标签引入 react 和 react-dom 两个核心库的方式已经不推荐了, 官方的 [CDN Links](https://legacy.reactjs.org/docs/cdn-links.html)
已经变成 Legacy 了, 也找不到引用 react@19 的相关链接了. 但本着要明就里的原则还是希望以这种方式演练一下.

下面是使用 react@19 和  react-dom@19 的最简 HTML 示例:

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Test React</title>
    <script src="https://unpkg.com/umd-react@19/dist/react.development.js" crossorigin></script>
    <script src="https://unpkg.com/umd-react@19/dist/react-dom.development.js" crossorigin></script>
</head>
<body>
<div id="root"></div>

<script>
    const root = ReactDOM.createRoot(document.getElementById('root'));
    root.render(React.createElement('h2', null, 'Hello, React!'));
</script>
</body>
</html>
```

页面会显示

<div style="text-align: center; border: 1px solid #ccc;">
<h2>Hello, React!</h2>
</div>

可是还没有看到 `JSX` 的语法啊, 没有 `JSX` 的 React.js 不能叫做 `React.js`. 但如果直接把上面的 `<script>` 中的代码改写成

```react
const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(<h2>Hello, React!</h2>);
```

后是无法正常工作的, 浏览器 Console 中出现的错误是

>Uncaught SyntaxError: Unexpected token '<'

因为浏览器并不认识 `JSX` 语法. 这时就需要引入 `Babel` 来进行转换了, 在原来引入 `react` 和 `react-dom` 的基础上还要加上

```html
<script src="https://unpkg.com/@babel/standalone/babel.min.js"></script>
```

但这还不够, 还要把包含 `JSX` 语法的 `<script>` 标签加上 `type="text/babel"` 属性, 这样 `Babel` 才会对其进行转换. 最终的 HTML 代码如下:

```react
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Test React</title>
    <script src="https://unpkg.com/umd-react@19/dist/react.development.js" crossorigin></script>
    <script src="https://unpkg.com/umd-react@19/dist/react-dom.development.js" crossorigin></script>
    <script src="https://unpkg.com/@babel/standalone/babel.min.js"></script>

</head>
<body>
<div id="root"></div>

<script type="text/babel">
    const root = ReactDOM.createRoot(document.getElementById('root'));
    root.render(<h2>Hello, React!</h2>);
</script>
</body>
</html>
```

这样页面也能正确显示 `Hello, React!` 了, 支持了 `JSX` 语法. 在此我们可对比一下 `Vue.js` 和极简代码, 代码中省去了不影响页面显示的元素

{{< bundle-image vue2-vue3-basic.png 1048 >}}

<!-- 注释掉的 vue2 和 vue3 代码块
```vue
<script src="https://unpkg.com/vue@2/dist/vue.js"></script>

<div id="app">{{ message }}</div>

<script>
    const vm = new Vue({
        el: '#app',
        data: {
            message: 'Hello, Vue.js 2!'
        }
    });
</script>
```

```vue
<script src="https://unpkg.com/vue@3/dist/vue.global.js"></script>

<div id="app">{{ message }}</div>

<script>
    const { createApp } = Vue;
    const app = createApp({
        data() {
            return {
                message: 'Hello, Vue.js 3!'
            }
        }
    }).mount('#app');
</script>
```
-->

下面将演示 `React.js` 一些功能性扩展.

### 自定义组件

上面在使用 `root.render()` 不光可以渲染已有的 HTML 标签(相当于是内置组件), 也可以渲染自定义组件. 下面定义一个简单的组件 `Welcome` 来演示:

```react
<script type="text/babel">
    function Welcome(props) {
        return <h2>Hello, {props.name}</h2>;
    }

    const root = ReactDOM.createRoot(document.getElementById('root'));
    root.render(<Welcome name="React User" />); 
</script>
```

这里定义了一个函数组件 `Welcome`, 它接收 `props` 作为参数, 并返回一个包含 `props.name` 的 `h2` 标签. 函数组件可用 `function` 关键字显式定义,
也可定义为一个闭包函数变量, 如上面的 `Welcome` 函数可改写成

```react
    const Welcome = (props) => {
        return <h2>Hello, {props.name}</h2>;
    }
```

除渲染函数组件外, 也可以渲染类组件, 例如:

```react
<script type="text/babel">
    class Welcome extends React.Component {
        render() {
            return <h2>Hello, {this.props.name}</h2>;
        }
    }
    const root = ReactDOM.createRoot(document.getElementById('root'));
    root.render(<Welcome name="React User" />);
</script>
```

这是一个类组件, 和前面函数组件的显示效果是一样的.

无论哪种组件, 返回的内容都只能有一个根节点, 如果有多个节点, 则需要用一个父节点包裹起来, 或者用虚拟节点 `React.Fragment` 包裹起来, 例如:

```react
<script type="text/babel">
    function App() {
        return (
            <React.Fragment>
                <h2>topic</h2>
                <Welcome name="React User" />
            </React.Fragment>
        );
    }
</script>
``` 

注意到上面,  组件也可以嵌套使用别的组件, 就像是 HTML 标签一样的方式使用. `<react.Fragment>` 也可以简写成 `<>` 和 `</>` 的形式, 例如:

```react
function App() {
    return (
        <>
            <h2>topic</h2>
            <Welcome name="React User" />
        </>
    );
}
```

现在多是使用函数组件加上 `Hooks` 的方式来编写组件, 类组件已经很少使用了. 如果组件变的复杂或者要让功能清晰, 可复用, 可以把组件拆分到单独的
`js` 文件中, 可以想见 `React.js` 项目中基本就是一些 `js` 文件组成的, 而 `Vue.js` 项目含有许多 `.vue` 文件.

### 组件中插值

这就相当于在模板中插值的概念, 在 `JSX` 语法中, 可以使用 `{}` 来包裹 JavaScript 表达式, 例如:

```react {hl_lines="5-9"}
const fruits = ['apple', 'banana', 'cherry'];
function App() {
    return (
        <ul>
            {
                fruits.map((fruit, index) => (
                    <li key={index}>{fruit}</li>
                ))
            }
        </ul>
    );
}
```

也可以事先生成一个变量, 然后在 `JSX` 中插入该变量, 例如:

```react {hl_lines="2-4 7"}
function App() {
    const fruitItems = fruits.map((fruit, index) => (
        <li key={index}>{fruit}</li>
    ));
    return (
        <ul>
            {fruitItems}
        </ul>
    );
}
```

可以自己来感受哪种方式更适合自己.

### 事件处理与状态管理

在 `Vue.js` 中, `data` 中定义的数据与网页显示是双向绑定的, 而在 `React.js` 中, 需要使用 `useState` 来定义状态变量, 
并通过事件处理函数来更新状态. 例如:

```react
<script type="text/babel">
    function App() {
        const [name, setName] = React.useState('World');
        return (
            <div>
                <h2>Hello {name}</h2>
                <input value={name} onChange={(e) => setName(e.target.value)}/>
            </div>
        );
    }
    const root = ReactDOM.createRoot(document.querySelector('#root'));
    root.render(<App/>);
</script>
```

我们在函数组件中新加了一个状态, 并拆解为属性和更新方法, `React.useState('World')` 中的 `World` 是初始值.
这样打开网页时会显示 `Hello World`, 在输入框中输入内容时, `h2` 标签中的内容也会随之变化.

注间: React.js 的 JSX 代码中只要是 `{}` 中的内容被视为 JavaScript 代码, 而且不要像写 HTML 那样两边加双引号, 用了双引号就变成纯字符串了.
比如写成 `<input value="{name}" ... />` 就不对了, 页面上会原样显示 `{name}`, `onChange={}` 也是同样的规则.

如果 `onChange` 事件处理函数比较复杂, 可以在函数组件中单独定义一个函数, 然后在 `JSX` 中引用该函数, 例如:

```react
function App() {
    const [name, setName] = React.useState('World');
    const updateName = (e) => {
        setName(e.target.value)
    };
    return (
        <div>
            <h2>Hello {name}</h2>
            <input value={name} onChange={updateName}/>
        </div>
    );
}
```

这与 `Vue.js` 的双向绑定相比, 我们只需关注要变化的状态, 以及清楚变化的方向, 可避免内部过多的事件通知.

### 组件中使用 CSS 样式

如果还像以前那样在组件中用 `style` 属性定指定字符串形式的样式, 会有问题. 像下面的代码

```react
function App() {
    return (
        <h2 style="color: red">Hello World!</h2>
    );
}
```

浏览时会报错

> Uncaught Error: The `style` prop expects a mapping from style properties to values, not a string. For example,
> style={{marginRight: spacing + 'em'}} when using JSX.

原因是 `style` 属性的值应该是一个对象, 而不是字符串. 正确的写法是:

```react
return (
    <h2 style={{"color": "red"}}>Hello World!</h2>
)
```

不要把它想成什么双大括号语法, 而要分开来理解, 外层的 `{}` 是 `JSX` 语法中表示插值的意思, 内层的 `{}` 才是 JavaScript 对象的定义方式.
比如, 该样式有个属性的话, 写成下面的风格就来理解了.

```react
    function App() {
        return (
            <h2 style={
                {
                    "color": "red",
                    "fontSize": "24px"
                }
            }>Hello World!</h2>
        );
    }
```

或者把整个 style 属性提取到一个变量中, 这样更容易使用状态来交互, 例如:

```react
    function App() {
        const style = {
            color: 'red',
            fontSize: '24px'
        };
        return (
            <h2 style={style}>Hello World!</h2>
        );
    }
```

并且注意, `React.js` 中的 CSS 属性名是它们在 JS 中名称, 而且在 `css` 中的名名, 例如 `font-size` 应该写成 `fontSize`, 
`background-color` 应该写成 `backgroundColor`.

### 函数组件与 Hooks(钩子)

函数组件如果是不纯的(会带来副作用), 那么就需要使用 `Hooks`(钩子) 来处理状态和副作用等问题. 例如之前的 `useState` 就是一个常用的 `Hook`.

下面来看什么时候需要用到 `useEffect` 这个 `Hook`. 假设我们有一个组件, 希望它在第次从网络上加载数据, 然后改变某一状态. 进而反应到页面上,
例如, 我们很可能会写出类似下面的代码

```react { hl_lines="8-10" }
    function App() {
        const [name, setName] = React.useState('World');

        const fetchData = () => {
            setName('React User');
        };

        console.log("before fetching data")
        fetchData();
        console.log("after fetching data")

        return (
            <h2>Hello {name}</h2>
        );
    }
```

那么恭喜你, 你自然而然的掉了 `React` 给下设下的陷阱. 打开浏览器的 `Console`, 你会发现疯狂的一段输出

```text
before fetching data
after fetching data
before fetching data
after fetching data
before fetching data
........
```

最后看到

```text
Uncaught Error: Too many re-renders. React limits the number of renders to prevent an infinite loop.
    at renderWithHooksAgain (react-dom-client.development.js:5614:17)
    at renderWithHooks (react-dom-client.development.js:5532:21)
    at updateFunctionComponent (react-dom-client.development.js:8897:19)
    at beginWork (react-dom-client.development.js:10522:18)
    at runWithFiberInDEV (react-dom-client.development.js:1520:30)
    at performUnitOfWork (react-dom-client.development.js:15132:22)
    at workLoopSync (react-dom-client.development.js:14956:41)
    at renderRootSync (react-dom-client.development.js:14936:11)
    at performWorkOnRoot (react-dom-client.development.js:14462:44)
    at performWorkOnRootViaSchedulerTask (react-dom-client.development.js:16216:7)
    at MessagePort.performWorkUntilDeadline (scheduler.development.js:45:48)
```

这还算不错, 浏览器聪明的制止了死循环.

出现这个死循环的原因是当第一次执行该 `<App/>` 组件时, `fetchData()` 函数被调用, 它调用了 `setName('React User')`, 这会触发组件重新渲染,
重新渲染时又调用了 `fetchData()` 函数, 又调用了 `setName('React User')`, 这样就子子孙孙无穷尽也.

为解决这个问题, `useEffect` 这个 `Hook` 就该派上用场. `useEffect` 允许我们在函数组件中执行副作用操作, 并且可以指定依赖项数组. 
只有其中的某个依赖项发生变化时, 副作用才会重新执行. 如果我们希望副作用只在组件挂载时执行一次, 可以传入一个空数组作为依赖项.
把前面高亮的代码改写成下面这样:

```react
   React.useEffect(() => {
       console.log("before fetching data")
       fetchData();
       console.log("after fetching data")
   }, []);
```

再刷新页面, 你会发现 `Console` 中只输出了一次

>before fetching data
>after fetching data

`React.js` 16.8 版本引入了 `Hooks` 的概念, 除了上面介绍的 `useState` 和 `useEffect` 外,: 还有许多其他常用的 `Hooks`, 如下:

- `useRef`: DOM 引用、存放不触发渲染的可变值.
- `useMemo`: 缓存计算结果，避免重复重算.
- `useCallback`: 缓存函数引用，常配合 React.memo.
- `useContext`: 跨层共享数据：主题/用户/配置等.
- `useReducer`: 复杂状态逻辑：表单、状态机、聚合更新.
- `useId`: 生成稳定 id，表单/ARIA，React 18 后更常见.
- `useImperativeHandle`: 封装组件库时暴露方法给父组件.

### 结束语

掌握了这些核心语法之后, 在 `JS` 采了 ES6 的模块化 `import` 语法后, 也能理解 `React.js` 项目是如何组织和运作起来的. 现代的 `React.js`
项目自然是不会用上面这种原始的 HTML 引入方式了, 而是通过 `npm` 包管理工具来安装 `react` 和 `react-dom` 包. 对于自定义组件也是用
`import` 来引入的. 像 `create-react-app` 脚手架创建的项目中 `index.js` 长得是下面这个样子

```react
import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import App from './App';

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
```

此外, 在使用 `IntelliJ IDEA` 以 Markdown 格式写作此篇日志时, 发现编辑器的 AI 功能太强大了, 每次都能预判我下面一至两行要写什么, 
真是太恐怖了. 而且在准备演示代码时也能帮我一行行推断出来, 当然还是要亲自执行验证相关的代码.

从 WordPress 的在线程写作切换来本地写作, AI 确实能能发挥很大的作用, 目前 AI 提示的文字还是仅供参考, 如果总是 Tab 键接收 AI 的建议, 
恐将失去自己思考的训练了.