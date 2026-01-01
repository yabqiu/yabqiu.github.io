---
title: "React.js 原始用法学习"
url: /react-js-bare-usage/
date: 2026-01-01T14:14:41-05:00
featured: false
draft: true
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

这样页面也能正确显示 `Hello, React!` 了, 支持了 `JSX` 语法. 下面的演示将基于这个最简 HTML 进行扩展.

### 渲染组件

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

这里定义了一个函数组件 `Welcome`, 它接收 `props` 作为参数, 并返回一个包含 `props.name` 的 `h2` 标签. 除渲染函数组件外, 也可以渲染类组件,
例如:

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

现在多是使用函数组件加上 `Hooks` 的方式来编写组件, 类组件已经很少使用了.

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

