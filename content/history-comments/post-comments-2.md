---
title: "历史日志评论 2"
url: /history-post-comments-2/
date: 2026-04-21T19:31:40-05:00 # Date of post creation.
showShare: false
noCopyright: true
comments: false
---

{{% comment "yushuai_w🧸2018-08-29 22:18:14🧸yushuai_w@fang.com🧸🧸使用 Mockito 的 @InjectMocks 创建被测试类实例🧸mockito-injectmocks-initialize-tested-instance" %}}
```java
public class IndexServiceImpl implements IndexService {
public TranslateService translateService;

    private void setTranslateService(TranslateService translateService) {
        this.translateService = translateService;
    }

    @Override
    public String output() throws Exception {
        String description = read();
        //TranslateService translateService = new TranslateServiceImpl();
        Map rules = translateService.get(description);
        ComputeService computeService = new ComputeServiceImpl(rules);
        return  computeService.get(description);
    }
}


@RunWith(MockitoJUnitRunner.class)
public class IndexServiceTest {

@InjectMocks
private IndexServiceImpl indexService;
@Mock
private TranslateService translateService;

    @Before
    public void setUp() throws Exception {
        MockitoAnnotations.initMocks(this);
    }

    @Test
    public void whenInputThenCorrectOutput() throws Exception {
        //部分模拟
        indexService =  spy(IndexServiceImpl.class);
        

        //模拟翻译服务
         when(translateService.get(anyString())).thenReturn(exchangeRule);

        //主要测试内容： 输出
        String output =  indexService.output();
        verify(indexService).read();
        verify(translateService).get(anyString());
        Assert.assertNotNull(output);
        Assert.assertFalse(output.isEmpty());
    }
}
```
{{% /comment %}}

{{% comment "yushuai_w🧸2018-08-29 22:15:59🧸yushuai_w@fang.com🧸🧸使用 Mockito 的 @InjectMocks 创建被测试类实例🧸mockito-injectmocks-initialize-tested-instance" %}}
博主大大，好，请问我这样注入 TranslateServise 为什么无法注入成功 ？

```java
@RunWith(MockitoJUnitRunner.class)
public class IndexServiceTest {
@InjectMocks
private IndexServiceImpl indexService;
@Mock
private TranslateService translateService;


    @Before
    public void setUp() throws Exception {
        MockitoAnnotations.initMocks(this);
    }

    @Test
    public void whenInputThenCorrectOutput() throws Exception {
        //部分模拟
        indexService =  spy(IndexServiceImpl.class);
        

        //模拟翻译服务
         when(translateService.get(anyString())).thenReturn(exchangeRule);

        //主要测试内容： 输出
        String output =  indexService.output();
        verify(indexService).read();
        verify(translateService).get(anyString());
        Assert.assertNotNull(output);
        Assert.assertFalse(output.isEmpty());

    }
public class IndexServiceImpl implements IndexService {
public TranslateService translateService;

    private void setTranslateService(TranslateService translateService) {
        this.translateService = translateService;
    }

    @Override
    public String output() throws Exception {
        String description = read();
        //TranslateService translateService = new TranslateServiceImpl();
        Map rules = translateService.get(description);
        ComputeService computeService = new ComputeServiceImpl(rules);
        return  computeService.get(description);
    }
}
```
{{% comment "Yanbin🧸2018-08-29 22:46:04🧸yabqiu@gmail.com" %}}
```java
@RunWith(MockitoJUnitRunner.class)
MockitoAnnotations.initMocks(this);
```
二者取其一即可。

```java
@InjectMocks
private IndexServiceImpl indexService;
```

应该会调用 setter 方法 setTranslateService(TranslateService translateService) 来注入的.

测试方法中的 indexService 有什么属性值。

用了 @InjectMocks  的话就不能在测试方法中取 indexService 再次赋值。
{{% /comment %}}
{{% comment "yushuai_w🧸2018-08-30 20:38:28🧸yushuai_w@fang.com" %}}
博主您好，您是说 indexService =  spy(IndexServiceImpl.class); 这句有问题吗？这句去掉的话，会报错。 2. IndexService 里的TranlateService一直为空，注入没成功呢。
{{% comment "Yanbin🧸2018-08-31 00:36:57🧸yabqiu@gmail.com" %}}
indexService =  spy(IndexServiceImpl.class);

你的 indexService 被重新赋值了，在 spy 之前你可以检查 indexService 中的 translateService 属性是不为空的。这里有两个问题

1.  spy 应作用于已创建的实例，可以这样 IndexService spyIndexService = spy(indexService)
2. @InjextMocks 创建好的 indexService 的值不应被赋值覆盖

spyIndexService 之后 when(spyIndexService)...., spyIndexService.out() 都应该基于 spyIndexService 进行处理了。
{{% comment "yushuai_w🧸2018-08-31 00:53:42🧸yushuai_w@fang.com" %}}
收到，感谢博主大大~！
{{% /comment %}}
{{% /comment %}}
{{% /comment %}}
{{% /comment %}}

{{% comment "laixintao🧸2018-02-02 02:58:01🧸laixintao1995@163.com🧸https://www.kawabangga.com🧸使用 Mockito 的 @InjectMocks 创建被测试类实例🧸mockito-injectmocks-initialize-tested-instance" %}}
好复杂的行为，像python这种动态的语言可以运行的时候替换掉对象的方法。比如可以在测试开启的时候把模块的某个方法替换掉，做到Mock
{{% comment "Yanbin🧸2018-02-02 10:00:45🧸yabqiu@gmail.com" %}}
Java 可没有这么自由， Mockito 是通过生成子类来替换被 Mock 对象的行为，所以限制很多，但代码更流畅。JMockit 是借助了 -javaagent, Instrumentation API 来直接替换类实现代码，也就强大的多。
{{% /comment %}}
{{% /comment %}}

{{% comment "laixintao🧸2018-01-25 00:29:55🧸laixintao1995@163.com🧸http://kawabangga.com🧸Bash/Zsh 下调用 Emacs/Vim 编辑当前命令🧸bash-zsh-call-emacs-vim-edit-current-command" %}}
编辑很长的curl命令时很有用
{{% /comment %}}

{{% comment "xiaohei*🧸2020-01-07 21:26:21🧸260781948@qq.com🧸🧸谁说 HTTP GET 就不能通过 Body 来发送数据呢？🧸why-http-get-cannot-sent-data-with-reuqest-body" %}}
通过  Dsl.asyncHttpClient()的方式你，怎样实现发送https的请求？希望能帮助解答一下
{{% comment "Yanbin🧸2020-01-24 10:13:34🧸yabqiu@gmail.com" %}}
Java 要实现发送 https, 一般有两种办法， 1）信任一切证书，2）导入相应的证书
{{% /comment %}}
{{% /comment %}}

{{% comment "xws🧸2019-01-21 21:20:40🧸5c468c078e739@example.com🧸🧸谁说 HTTP GET 就不能通过 Body 来发送数据呢？🧸why-http-get-cannot-sent-data-with-reuqest-body" %}}
关于get请求body传参，并不推荐通过body传参,可以参考stackoverflow关于该问题的回答

https://stackoverflow.com/questions/978061/http-get-with-request-body
{{% comment "Yanbin🧸2019-01-21 21:22:09🧸yabqiu@gmail.com" %}}
是的，实现上并不保证能传输，接收 get body 数据
{{% comment "xws🧸2019-01-21 21:35:10🧸5c468f6e0d190@example.com" %}}
我在CSDN上看到你关于http get body传参的帖子，跳转到这里。把这个问题在在CSDN上的说明下不推荐get body方式传参，以免误导新人.
{{% comment "Yanbin🧸2019-01-22 10:20:07🧸yabqiu@gmail.com" %}}
最前面加了一句话。像这里，一般是一篇文章写成之后，有新的想法只会附加在后面。
{{% /comment %}}
{{% /comment %}}
{{% /comment %}}
{{% /comment %}}

{{% comment "CXM🧸2018-05-22 06:58:32🧸5b0405e76dc73@example.com🧸🧸谁说 HTTP GET 就不能通过 Body 来发送数据呢？🧸why-http-get-cannot-sent-data-with-reuqest-body" %}}
他只是幫你把body串在網址列後面了
{{% comment "Yanbin🧸2018-05-22 13:17:22🧸yabqiu@gmail.com" %}}
虽然说HTTP 1 是一个文本协议，但 Get Request 的 body 它也不是网址的一部份。查看请求协议数据，Get 的 Body 仍然是请求头空一行后的数据。
{{% /comment %}}
{{% /comment %}}

{{% comment "laixintao🧸2018-02-02 03:02:51🧸laixintao1995@163.com🧸https://www.kawabangga.com🧸谁说 HTTP GET 就不能通过 Body 来发送数据呢？🧸why-http-get-cannot-sent-data-with-reuqest-body" %}}
很多人讨论GET和POST的时候很容易就从“协议”讨论到“实现”上去了。  协议里说的是，GET是从服务器取回数据，POST是发送数据，HTTP请求有header，有body。但是实现怎么样，协议就不管了。本文章的HTTPClient，curl，postman，浏览器，这些都是实现。

所以我觉得讨论GET和POST区别的前提是，弄明白协议和实现的区别。比如别人可以说对于Chrome，get的区别是不能带body，这就没问题了。
{{% comment "Yanbin🧸2018-02-02 13:58:02🧸yabqiu@gmail.com" %}}
是的，从协议与实现上来讲会很清楚。
{{% /comment %}}
{{% comment "LXL🧸2018-08-05 22:53:50🧸5b67c64d72687@example.com" %}}
然而协议里确实说了哪些方法带body是没有意义并可能会产生问题，所以你硬给get加一个body也是属于不遵循规范的，尽管可能成功但是后果自担。所以说get不能带body是正确的说法
{{% comment "Yanbin🧸2018-08-06 00:17:22🧸yabqiu@gmail.com" %}}
<blockquote>
   A message-body MUST NOT be included in
   a request if the specification of the request method (section 5.1.1)
   does not allow sending an entity-body in requests. A server SHOULD
   read and forward a message-body on any request; if the request method
   does not include defined semantics for an entity-body, then the
   message-body SHOULD be ignored when handling the request.
</blockquote>
这是 RFC 中 message-body 中的描述，但是没有说 GET 不能携带 body，看到 HEAD 和 OPTIONS 忽略 body
{{% comment "LXL🧸2018-08-06 03:41:41🧸5b6809c511252@example.com" %}}
你看的协议RFC2616已经过时了，请看最新的7230-7237
{{% comment "Yanbin🧸2018-08-06 10:28:20🧸yabqiu@gmail.com" %}}
谢谢，我看到了rfc7231 中说 Obsoletes: 2616
{{% /comment %}}
{{% comment "Yanbin🧸2018-08-06 00:26:12🧸yabqiu@gmail.com" %}}
本文并没有建议去违反语义在 GET 请求中传递 body。由于 HTTP/1.1 是基于文本的协议，所以头后空一行后的数据都是 body，所以协议本身未作限制，但是有一个语义上的建议--不应在 GET 请求中放 body

这里有个比较好的回答
https://stackoverflow.com/questions/978061/http-get-with-request-body?answertab=active#tab-top
<blockquote>
Yes. In other words, any HTTP request message is allowed to contain a message body, and thus must parse messages with that in mind. Server semantics for GET, however, are restricted such that a body, if any, has no semantic meaning to the request. The requirements on parsing are separate from the requirements on method semantics.

So, yes, you can send a body with GET, and no, it is never useful to do so.

This is part of the layered design of HTTP/1.1 that will become clear again once the spec is partitioned (work in progress).

....Roy

Yes, you can send a request body with GET but it should not have any meaning. If you give it meaning by parsing it on the server and changing your response based on its contents, then you are ignoring this recommendation in the HTTP/1.1 spec, section 4.3:

[...] if the request method does not include defined semantics for an entity-body, then the message-body SHOULD be ignored when handling the request.

And the description of the GET method in the HTTP/1.1 spec, section 9.3:

The GET method means retrieve whatever information ([...]) is identified by the Request-URI.

which states that the request-body is not part of the identification of the resource in a GET request, only the request URI.
</blockquote>
{{% comment "LXL🧸2018-08-06 03:49:31🧸5b680b9a63041@example.com" %}}
RFC 7231中这才是最新的说法：A payload within a GET request message has no defined semantics; sending a payload body on a GET request might cause some existing implementations to reject the request. 因而按照规范，GET等是不可以加body的，否则出问题了不要怪规范没有提醒你
{{% comment "Yanbin🧸2018-08-06 10:39:45🧸yabqiu@gmail.com" %}}
谢谢，单纯大文本协议来说无法阻止传递 body，但服务端可以不理会 get 的 body
{{% /comment %}}
{{% /comment %}}
{{% /comment %}}
{{% /comment %}}
{{% /comment %}}
{{% /comment %}}
{{% /comment %}}

{{% comment "蓝多qwe🧸2018-01-22 23:01:19🧸2855892566@qq.com🧸🧸谁说 HTTP GET 就不能通过 Body 来发送数据呢？🧸why-http-get-cannot-sent-data-with-reuqest-body" %}}
楼主，在浏览器里如何让get请求携带body呢？
{{% comment "Yanbin🧸2018-01-23 01:38:35🧸yabqiu@gmail.com" %}}
不行。从协议方面来说，GET 是可以带 body 的，但是不赞成这么做，所以好多工具并没有去提供支持。基本上也不要用 GET 来携带 body 数据。
{{% comment "蓝多qwe🧸2018-01-24 02:06:54🧸2855892566@qq.com" %}}
对啊，我发现XHR内部就把body给移除了。
{{% /comment %}}
{{% /comment %}}
{{% /comment %}}

{{% comment "jackly🧸2018-01-21 21:47:34🧸5a655ed64a911@example.com🧸🧸JMockit 之 Expectations🧸jmockit-with-expectations" %}}
写得不错呀。

也推荐一个学习JMockit的地方，里面文章也写得挺详细的。 JMockit中文网  jmockit.cn
{{% comment "Yanbin🧸2018-01-22 10:16:47🧸yabqiu@gmail.com🧸http://unmi.cc" %}}
我一般只有在 Mockito  无法满足需求的时候才考虑用 JMockit, 因为相比而言 Mockito 的语法更流畅。甚至 PowerMockito 搭配 Mockito 的选择也更优于纯粹的 JMockit.
{{% /comment %}}
{{% /comment %}}

{{% comment "Hanger🧸2018-09-30 21:08:08🧸5bb181883743a@example.com🧸🧸Vimer 的福音：Mac 下 Caps + hjkl 作为方向键🧸vimer-mac-caps-hjkl-as-arrow-keys" %}}
Karabiner-Elements12.1.0  无法 import 你的文件
{{% comment "Yanbin🧸2018-09-30 22:41:01🧸yabqiu@gmail.com🧸http://unmi.cc" %}}
没更新了，现在我用的键盘都是可编程的，暂时用不上 Karabiner-Elements 来重新映射按键。
{{% /comment %}}
{{% /comment %}}

{{% comment "Scc🧸2018-04-12 20:43:57🧸scong@outlook.com🧸🧸Vimer 的福音：Mac 下 Caps + hjkl 作为方向键🧸vimer-mac-caps-hjkl-as-arrow-keys" %}}
好东西 赞一个
{{% /comment %}}

{{% comment "andy🧸2017-11-22 00:01:09🧸529452103@qq.com🧸🧸Vimer 的福音：Mac 下 Caps + hjkl 作为方向键🧸vimer-mac-caps-hjkl-as-arrow-keys" %}}
请教一下怎么样才能跟我的 "capslock 修改成  left_contro 兼容" 呢, 两个功能都是刚需
{{% comment "Yanbin🧸2017-11-22 13:28:54🧸yabqiu@gmail.com🧸http://unmi.cc" %}}
caps lock 不是刚需的，不会为了输入一个，或有限的几个大写字母而按 caps lock 来回切换大小写。

但实际上有几个女人用我的键盘时，我才发现她们居然不知道 shift + 字母键可以输出大写。
{{% comment "andy🧸2017-11-22 21:21:53🧸529452103@qq.com" %}}
哈哈,同感, 我老板每次敲我键盘都要骂我, 他们只是不喜欢尝试新鲜事物. 我之前表述得有问题, 其实我一直是用的 ctrl + hjkl来控制方向, 
在系统设置里面把 Capslock 又改成ctrl 键, 这样就实现 capslock + hkjl 的快捷键了, 还能实现一些shell常用操作, capslock + a 行首,
capslock + e 行末, capslock + u 删除整行的功能, 用了几年习惯了
{{% comment "Yanbin🧸2017-11-22 21:32:04🧸yabqiu@gmail.com🧸http://unmi.cc" %}}
我办公室有两个键盘，别人过来一起结对编程的时候就用那个正常的键盘。HHKB 就是 Ctrl 放在了 Caps Lock 的位置上了。Caps Lock 基本是多余的，还常年占了一个黄金位置。
{{% /comment %}}
{{% /comment %}}
{{% comment "andy🧸2017-11-22 21:24:26🧸529452103@qq.com" %}}
我刚刚在 karabinar-elements 的 example 下面找到了 ctrl + hjlk 的快捷键修改了, 谢谢啊, 不用自己改脚本了
{{% /comment %}}
{{% /comment %}}
{{% /comment %}}

{{% comment "beyond🧸2017-11-06 10:05:41🧸1551935335@qq.com🧸🧸Vimer 的福音：Mac 下 Caps + hjkl 作为方向键🧸vimer-mac-caps-hjkl-as-arrow-keys" %}}
请教一下我想修改成 空格+jkli 替换方向键  该怎么修改呢？

还有你这个规则需要能不能做成按住不放开caps lock键，然后jkl就是方向键了，现在你这必须要同时按一下caps lock+k才是上方向键啊。
{{% comment "Yanbin🧸2017-11-07 22:48:03🧸yabqiu@gmail.com🧸http://unmi.cc" %}}
非常感谢关注这个日志。
空格+jkli 有点奇特，可以把空格键映射为 right_command, 然后空格怎么输入呢？
我的定制规则原来有些问题，已更新了，更新后的链接是 <a href="karabiner://karabiner/assets/complex_modifications/import?url=https%3A%2F%2Funmi.cc%2Fwp-content%2Fuploads%2F2017%2F07%2Fcaps_lock_hjkl_to_arrow_keys.json" rel="nofollow">karabiner://karabiner/assets/complex_modifications/import?url=https%3A%2F%2Funmi.cc%2Fwp-content%2Fuploads%2F2017%2F07%2Fcaps_lock_hjkl_to_arrow_keys.json</a>
可以通过链接 <a href="https://unmi.cc/wp-content/uploads/2017/07/caps_lock_hjkl_to_arrow_keys.json">https://unmi.cc/wp-content/uploads/2017/07/caps_lock_hjkl_to_arrow_keys.json</a> 查看规则内容。
{{% /comment %}}
{{% comment "kai🧸2018-01-07 20:13:20🧸sting1280@hotmail.com" %}}
有个SpaceLauncher的工具可以做到
{{% comment "Yanbin🧸2018-01-08 12:21:41🧸yabqiu@gmail.com🧸http://unmi.cc" %}}
新的规则配置文件已经修复了按住 caps lock 不放，hjkl 连续方向键了。
{{% comment "resab🧸2019-06-18 00:08:23🧸resab@qq.com" %}}
有这个配置文件 吗,这个 bug搞了好久
{{% comment "Yanbin🧸2019-06-18 16:09:19🧸yabqiu@gmail.com🧸http://unmi.cc" %}}
文中有链接 karabiner://karabiner/assets/complex_modifications/import?url=https%3A%2F%2Funmi.cc%2Fwp-content%2Fuploads%2F2017%2F07%2Fcaps_lock_hjkl_to_arrow_keys.jsonkarabiner://karabiner/assets/complex_modifications/import?url=https%3A%2F%2Funmi.cc%2Fwp-content%2Fuploads%2F2017%2F07%2Fcaps_lock_hjkl_to_arrow_keys.json
{{% /comment %}}
{{% comment "resab🧸2019-06-19 07:27:42🧸resab@qq.com" %}}
确实可以，但是还不是我想要的效果，我之前在 win10用的是，按住 capslock，然后 hjkl 会生效，松开就回到正常，但是 mac 里，我松开了 caps
lock还要再按一次 CAPSLOCK 才变正常  ，把 capslock 改成 tab 映射到 right _command 就不会，好像只有 大写键 这样
{{% /comment %}}
{{% comment "Yanbin🧸2019-06-19 18:34:49🧸yabqiu@gmail.com🧸http://unmi.cc" %}}
我用的是 pok3r 可编程的键盘，所以设置为 CAPSLOCK + HJKL 移动是不依赖于系统了
{{% /comment %}}
{{% comment "resab🧸2019-06-20 11:34:46🧸resab@qq.com" %}}
好吧,看来还是win适合我
{{% /comment %}}
{{% /comment %}}
{{% /comment %}}
{{% /comment %}}
{{% /comment %}}

{{% comment "绿软库🧸2017-12-29 06:24:01🧸taiker@vip.qq.com🧸http://www.lvrku.com🧸Scala 中应用 Future 并发编程🧸scala-concurrent-with-future" %}}
感谢分享
{{% /comment %}}

{{% comment "stanfen🧸2017-12-13 23:57:30🧸5a3212c9a6979@example.com🧸🧸Scala 中应用 Future 并发编程🧸scala-concurrent-with-future" %}}
学习了，scala并发还可以更深入
{{% comment "Yanbin🧸2017-12-14 02:49:15🧸yabqiu@gmail.com🧸http://unmi.cc" %}}
那就是 Akka, Actor 了
{{% /comment %}}
{{% /comment %}}

{{% comment "Yanbin🧸2017-12-22 22:20:35🧸yabqiu@gmail.com🧸http://unmi.cc🧸JMockit Mock 私有方法和私有属性🧸jmockit-mock-private-methods-fields" %}}
注：JMockit 1.36 移除了 Deencapsulation.invoke() 方法。
{{% /comment %}}

{{% comment "zzz🧸2020-11-01 06:29:29🧸yangemail9353@foxmail.com🧸🧸Spring 定时任务(Schedule) 和线程🧸spring-schedule-runner-threads" %}}
赞
{{% /comment %}}

{{% comment "UltraXiaoZi🧸2017-12-21 02:10:55🧸5a3a0dc86096d@example.com🧸🧸Spring 定时任务(Schedule) 和线程🧸spring-schedule-runner-threads" %}}
为啥我不用添加@EnableScheduling的注解，service里面被@Scheduled的方法还是会定时执行?
{{% comment "Yanbin🧸2017-12-22 22:09:20🧸yabqiu@gmail.com🧸http://unmi.cc" %}}
如果是 Spring Boot 的话设置 debug=true 显示所有自动加载的配置看看你的 @EnableScheduling 是基于什么条件开启的。
{{% /comment %}}
{{% /comment %}}

{{% comment "hahaha🧸2018-11-27 01:32:34🧸5bfcf3118d409@example.com🧸🧸用 PreparedStatement 向 SqlServer 中一次性插入多条记录🧸sqlserver-insert-multiple-rows-onetime" %}}
这个留言功能的提交有点慢。。
{{% comment "Yanbin🧸2018-11-27 01:40:38🧸yabqiu@gmail.com🧸http://unmi.cc" %}}
好像是有点儿慢，像把插件一个个禁了才知道哪里出问题。
{{% /comment %}}
{{% /comment %}}

{{% comment "hahaha🧸2018-11-27 01:31:38🧸5bfcf2d998368@example.com🧸🧸用 PreparedStatement 向 SqlServer 中一次性插入多条记录🧸sqlserver-insert-multiple-rows-onetime" %}}
留言
{{% /comment %}}

{{% comment "UltraXiaoZi🧸2017-12-20 01:14:17🧸5a3a0dc86096d@example.com🧸🧸用 PreparedStatement 向 SqlServer 中一次性插入多条记录🧸sqlserver-insert-multiple-rows-onetime" %}}
你这个批量插入是SqlServer特有的？
{{% comment "Yanbin🧸2017-12-22 21:58:47🧸yabqiu@gmail.com🧸http://unmi.cc" %}}
是的，BCP 也是 SqlServer 特有的，为了优化性能使用数据库特有的东西是很值得的，不必一味的牺牲特性而追求所谓的跨数据库，对于一个公司切换数据库的比例到例有多大？微乎其微。
{{% /comment %}}
{{% /comment %}}

{{% comment "wayne🧸2017-11-29 03:41:37🧸444707088@qq.com🧸🧸体验 Scala 2.12 支持的 Java 8 风格(SAM) Lambda🧸scala-2-12-java-8-sam-lambda" %}}
博主，有个疑问，completablefuture和callable的差别是什么？两个都有get方法，不阻塞主要体现在哪里呢？
{{% comment "Yanbin🧸2017-11-29 13:20:24🧸yabqiu@gmail.com🧸http://unmi.cc" %}}
Callable 只是定义了一个接口，代表了提交给线程池的任务，并且是有返回值的，它不关心任务完成的各种状态。 而 CompletableFuture 实现了 Future,
有着更丰富的语义，Future 有点像 Promise。 CompletableFuture 实现了任务完成，失败后怎么操作，以及如何与其他的 CompletableFuture 协调等。
{{% comment "wayne🧸2017-11-29 20:59:08🧸444707088@qq.com" %}}
我可能有点问错问题了，其实我想问的是，callable在实现类放到executor线程池中执行，可以返回feature的返回值，
completablefuture也是也是可以指定放到executor中执行，返回的是completablefuture的返回值，两者其实都是异步方式执行，那这两种那个方式更优，差别在哪里呢？谢谢
{{% comment "Yanbin🧸2017-11-29 23:57:23🧸yabqiu@gmail.com🧸http://unmi.cc" %}}
线程池中怎么去执行任务的效率没什么区别，关键在于 CompletableFuture 对执行结果的响应性处理要方便的得多。Future 接口只有有限的 5 个方法，
对比一下 CompletableFuture 提供了 50 多个方法来处理执行结果，及解决多个 CompletableFuture 间的依赖关系。
{{% comment "wayne🧸2017-12-02 06:49:33🧸444707088@qq.com" %}}
哦，这样的，是说呢，我看来着，感觉用的方式是差不多，我开始是用callable写的东西，多线程去执行，看到这个completablefuture后，是新出的api，想换这个来重写，看来我需要重新考虑下。
{{% /comment %}}
{{% /comment %}}
{{% /comment %}}
{{% /comment %}}
{{% /comment %}}

{{% comment "crid🧸2017-11-14 09:20:08🧸5a0b09a765a1f@example.com🧸🧸用 iTextSharp 修改 PDF 文件的元信息(MetaData)🧸itextsharp-edit-pdf-metadata" %}}
自己添加的属性，怎样可以支持中文，我发现加入中文之后成乱码
{{% comment "Yanbin🧸2017-11-14 17:59:35🧸yabqiu@gmail.com🧸http://unmi.cc" %}}
估计要选择字符集或字体，我现在都没有相应的开发环境了，你可以找下 API 里有没有字符集或字体的相关设置。
{{% /comment %}}
{{% /comment %}}
