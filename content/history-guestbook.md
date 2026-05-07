---
title: "历史留言"
date: 2026-04-21T19:31:40-05:00 # Date of post creation.
showShare: false
noCopyright: true
comments: false
---
{{% notice info "next 62" %}}
{{% /notice %}}

{{% notice info "Orion - 2013-07-18 18:39:25" %}}
Unmi,你在http://unmi.cc/struts2-xsltresult-string-to-document对AdapterFactory进行分析，但最后你采用的实现方式，我觉得应该可以更优些。

见AdapterFactory.adaptNode，第三行代码是这样的：

```java
Class adapterClass = getAdapterForValue(value);
if (adapterClass != null)
    return constructAdapterInstance(adapterClass, parent, propertyName, value);
```

也就是说getAdapterForValue获得不了指定的类型才往后面执行，如你在上一篇所说只要能成功registerAdapterType，
就不会执行strut提供的StringAdapter。那是否我们能用这样 一个小技巧：

```java
public class StringXSLTResult extends XSLTResult {
    
    private AdapterFactory adapterFactory;
    
    @Override
    protected AdapterFactory getAdapterFactory() {
        if (adapterFactory == null){
            adapterFactory = new AdapterFactory();
            adapterFactory.registerAdapterType(String.class,MyAdapter.class);//MyAdapter在其他地方自己定义
        }
        return adapterFactory;
    }
    @Override
    protected void setAdapterFactory(AdapterFactory adapterFactory) {
        this.adapterFactory = adapterFactory;
        this.adapterFactory.registerAdapterType(String.class,MyAdapter.class);//MyAdapter在其他地方自己定义
    }
}
```
{{% /notice %}}

{{% notice info "Komori - 2013-06-20 15:52:49" %}}
非常感谢你的博文 第一次处理httpclient验证看了你的博文问题解决了,不过你的代码好像不怎么好复制 建议换一个代码插件可以支持复制的.
{{% /notice %}}

{{% notice info "xiaofengfeng - 2013-05-28 17:58:14" %}}
你好，我在使用Quartz，但是在故障恢复上遇到问题。我已采用持久化方式，并设置JobDetail的RequestRecovery为true 然后，现象却是，
停机后错过了3次某任务的触发，但重启后，却从重新执行了一次，希望是错过几次，便重新执行几次 请问，能实现吗？有合适的解决方案吗？谢谢
{{% /notice %}}

{{% notice info "郑华伟 - 2013-05-14 09:44:59" %}}
您好，我是一名大四学生，看了您写的WordNet C#版同义词，很感兴趣，非常欣赏您的开源精神。我最近正在做毕业设计，其中涉及到一部分同义词检索的相关知识，
我自己对这个方面知识一无所知，但是从您的文章中了解到，需要自己编写分析器和过滤器等。想问下，几个东西有可以直接使用的么？
{{% /notice %}}

{{% notice info "ming - 2013-05-07 15:01:21" %}}
您的文章：《JavaDoc 编程，书写自定义的 Doclet，定制输出》http://unmi.cc/javadoc-customize-doclet

有提到：

可以为我们生成 HTML 的 JavaDoc API 文档，这就是默认的 com.sun.tools.doclets.standard.Standard 为我们做的事，
还可以像以前那样从源文件中抽取信息生成各种 XML 文件，或是 PDF, Excel，UML 图等等任何可能的内容，或做任何有作为的事情。
总之在 doclet 中可以感知道对任何包，类，方法，字段等的遍历。这里 Doclet.com 有大量的第三方的 doclet 供你选择

doclet for excel??

没找到哎，您有什么线索嘛。
{{% /notice %}}

{{% notice info "Unmi - 2013-04-25 13:35:05" %}}
@ck 版本是不是一样的，CPU 是否支持。
{{% /notice %}}

{{% notice info "ck - 2013-04-21 11:26:19" %}}
VMware 9 安装 Mac OS X 10.8 Mountain Lion

我装MAC OS的時候停住了,只有蘋果的標誌,應該怎做??求救

一直就停住,没有反應
{{% /notice %}}

{{% notice info "CK - 2013-04-21 11:22:20" %}}
我装MAC OS的時候停住了,只有蘋果的標誌,應該怎做??求救
{{% /notice %}}

{{% notice info "Sero - 2013-03-21 11:49:34" %}}
博主研究的东西好多啊 果然关注中。。
{{% /notice %}}

{{% notice info "史只平 - 2013-02-27 11:28:35" %}}
朋友能发一份，把lucene中索引存到数源库的源码给我看看么，我的邮箱shizhiping2006@163.com，非常感谢
{{% /notice %}}

{{% notice info "史志平 - 2013-02-26 19:03:18" %}}
请问《把 Lucene 索引数据存到数据库表中》这篇文章的源代码你有吗，我运行不起来，有的话能发我一分吗，万分感谢！
{{% /notice %}}

{{% notice info "Unmi - 2012-09-20 01:24:48" %}}
@汪佰想

您是指修改什么项目？
{{% /notice %}}

{{% notice info "汪佰想 - 2012-09-18 23:20:16" %}}
你好我想知道留言板可以手工修改项目么
{{% /notice %}}

{{% notice info "Unmi - 2012-09-04 16:34:42" %}}
@kyfxbl 在一定程度上可以说 tomcat 中的每个 app 组成的是分布式的，但是它们之间还是可以通过内存映射来直接访问的。而且，如果把类放置在每个 app 的上层的 Bootstrap

|

System

|

Common

的 classload 能见到的 classpath 下，那么各个 app 就能直接共享那些类了。
{{% /notice %}}

{{% notice info "kyfxbl - 2012-08-31 11:22:07" %}}
楼主好，非常感谢您的回答~~~~

那么是不是可以这么认为：

一个系统由多个APP组成，这些APP部署在同一个tomcat里，虽然处一个jvm，但是由于实际上无法直接相互调用，
只能通过其他方式集成（比如RMI WebService JMS），所以可以认为这几个app组成的系统，是一个分布式系统？
{{% /notice %}}

{{% notice info "Unmi - 2012-08-30 19:08:42" %}}
@小五 您好，是登陆 WebSphere 控制台的用户名和密码。
{{% /notice %}}

{{% notice info "小五 - 2012-08-30 17:19:42" %}}
您好，请教一下，在利用ant自动部署websphere应用时，连接的用户名和密码是连接什么的？是登录websphere控制台的，还是操作系统的？

谢谢，O(∩_∩)O~
{{% /notice %}}

{{% notice info "Unmi - 2012-08-29 14:22:41" %}}
@kyfxbl

一个 tomcat 只启动一个 JVM，也就是说 3 个应用都是跑在一个 JVM 里，之所以它们不能互相调用是因为被类加载器隔离开的。

Tomcat 的类加载器层次是：

Bootstrap

|

System

|

Common

/

Webapp1 Webapp2 ...

每个应用中的类分别是由 Webapp1, Webapp2 ... 类加载器加载的，所以是相互不可见的。

关于类加载器可以看看 http://unmi.cc/tag/classloader

类加载器的规则有三：

1. 一致性规则：类加载器不能多次加载同一个类
2. 委托规则：在加载一个类之前，类加载器总参考父类加载器
3. 可见性规则：类只能看到由其类加载器的委托加载的其他类，委托是类的加载器及其所有父类加载器的递归集。
{{% /notice %}}

{{% notice info "kyfxbl - 2012-08-29 09:50:17" %}}
博主您好，请教一个问题，麻烦您抽空解答，非常感谢

我想问的是，在一个servlet容器（比如说tomcat）里部署了3个.war，那么启动后会有几个JVM存在呢，是一个JVM，还是3个JVM?

如果是1个jvm的话，那么这3个应用都是跑在一个jvm里，为什么又不能直接互相调用呢?
{{% /notice %}}

{{% notice info "Unmi - 2012-08-02 12:31:26" %}}
我不知道你用的是什么版本的 Excel，如果是 2010 的话，可以按钮栏里右键，Customize Ribbon，右边勾选上 Developer，就会多个 Developer 标签，
然后添加按钮控件，切换到 VBA 上写你的按钮事件。
{{% /notice %}}

{{% notice info "raphael - 2012-07-31 23:24:26 " %}}
请问，如何在xsl中添加一个按钮，然后响应按钮事件将显示的数据按需要显示
{{% /notice %}}

{{% notice info "非主流唯美图片 - 2012-06-19 10:25:13" %}}
求新云系统定时插件，定时运行采集，生成文件。。谢谢
{{% /notice %}}

{{% notice info "nk.ljk - 2012-04-13 10:30:45" %}}
@Unmi tink了，我已经找到解决的办法了。
{{% /notice %}}

{{% notice info "nk.ljk - 2012-04-12 16:09:39" %}}
@Unmi 之前测试过了，shutdown 后job还是会继续执行，启动应用时不启动 Quartz 就可以顺利 关闭，而且也排除了 是job 调用其他东西导致占用线程的问题。
{{% /notice %}}

{{% notice info "Unmi - 2012-04-11 19:13:49" %}}
@nk.ljk shutdown 后可以观察下你的 job 是否还有在执行，或者启动应用时不启动 Quartz，是否能正常关闭掉 Tomcat。
{{% /notice %}}

{{% notice info "nk.ljk - 2012-04-09 16:15:23" %}}
忘记说了，项目直接给予javaben+servlet没有其他框架。

我启动 Quartz 也是直接用一个随项目启动即启动的servlet来调用执行
{{% /notice %}}

{{% notice info "nk.ljk - 2012-04-09 16:12:27" %}}
你好，我想问一个Java使用Quartz的问题。

我在项目里整合Quartz后，tomcat执行shutdown但是还是没有结束tomcat进程，似乎quartz依然在占用线程，必须得手动结束进程才能再次启动tomcat。
{{% /notice %}}

{{% notice info "Unmi - 2012-03-14 13:01:42" %}}
@艾依然

已加上友链，我的名称就用

隔叶黄莺 Unmi's Blog

谢谢!!
{{% /notice %}}

{{% notice info "艾依然 - 2012-03-10 15:14:13 " %}}
unmi，论坛跟你交换一个友链呗！

名称：WebSphere中间或WebSphere开发与应用社区

链接：http://www.webspherechina.net

你的用什么名称?
{{% /notice %}}

{{% notice info "UNMI - 2012-02-04 14:03:00" %}}
我在电脑里恐怕也找不到那份源代码了，那个远代码是基于 1.5版本的，现在的 Quartz 都到 2.1.3 了，API 相差很大了，
所以原来的源代码也只能参考了，我还是会去找找吧。
{{% /notice %}}

{{% notice info "墨竹 - 2012-01-31 09:56:16" %}}
你好，看了你翻译的Quartz Job Scheduling Framework，你能发一份源代码到我邮箱吗？不胜感激！（在网上搜了好久都没找到）
{{% /notice %}}

{{% notice info "Unmi - 2012-01-12 00:34:26" %}}
我是在 www.bloghost.cn 上海买的域名和空间。空间是用的美国的，所以域名可以不需要备案。但是美国空间要比中国的空间要慢不少，
cc 的域名每年是90元，com 域名每年是60，空间用的是 100 一年的。你进去 www.bloghost.cn 上面有销售人员的 QQ/MSN/Gtalk 可以联系了解。
{{% /notice %}}

{{% notice info "northwind - 2012-01-07 17:32:16" %}}
不知道是否能看到联系方式，我是上个问题提问题留言的人，网上有一些这方面的资料，但是很多很杂，要是您能给个指导就好了，
例如介绍一下您购买域名和服务器建站的实际情况，我的邮箱是wallbreak@163.com,谢谢
{{% /notice %}}

{{% notice info "northwind - 2012-01-07 17:27:50" %}}
hi, 你好，偶然进入你的网站，我也有意建立一个个人的独立网站，用来记录个人仿总结和发表一些观点，类似您这样的，能传授一下经验吗，
主要是怎样购买域名和服务，流程是怎样的，要感应注意事项，若得到您的答复十分感谢！
{{% /notice %}}

{{% notice info "Unmi - 2011-08-13 12:57:42" %}}
@sayablog 没有使用什么插件，是 WordPress 默认支持的，对的，在要摘要到的位置加上 More,然后通用设置里为显示摘要即可。我看到你的博客，现在就是这么做的。
{{% /notice %}}

{{% notice info "hejinhui - 2011-08-10 11:42:51" %}}
为什么QQ群加了好几次都没有回应啊，QQ：228809598
{{% /notice %}}

{{% notice info "sayablog - 2011-07-29 16:29:06" %}}
请问博主的首页摘要使用的是什么插件?
{{% /notice %}}

{{% notice info "Unmi - 2011-06-08 20:51:19 " %}}
@小菜来，工作时间不方便上QQ，你可以加入到我们的群里来。
{{% /notice %}}

{{% notice info "小英来 - 2011-05-25 22:33:24" %}}
博主，对你实在是膜拜。想和你咨询一下Quartz开发的相关问题,QQ:282231117(子午线)
{{% /notice %}}

{{% notice info "hit9 - 2011-03-30 18:03:53" %}}
我懂了。谢谢你
{{% /notice %}}

{{% notice info "Unmi - 2011-03-30 15:11:56" %}}
头像是根据你填写的 email 对应的 gravatar 头像设置，见网站 http://www.gravatar.com，这和博客的评论中显示的头像是一样的。

你还是说头像的大小和位置的设置？
{{% /notice %}}

{{% notice info "hit9 - 2011-03-28 13:32:47" %}}
我看了你的文章，不过想知道怎么设置这个留言板的头像，能自定义吗
{{% /notice %}}

{{% notice info "Unmi - 2011-03-16 18:48:47" %}}
谢谢各位的关注，已加上了日志归档的链接，可访问以下两个网页：

http://unmi.cc/archives

http://unmi.cc/sitemap
{{% /notice %}}

{{% notice info "ddatsh - 2011-03-13 18:08:41" %}}
希望您能把日志归档时间显示出来

还是想根据年月，看到您写的所有文章
{{% /notice %}}

{{% notice info "老白 - 2011-02-09 23:30:50" %}}
感谢你的教程，已经使用上了这个模板，非常cool
{{% /notice %}}

{{% notice info "IzY - 2011-01-06 09:04:23" %}}
谢了额～
{{% /notice %}}

{{% notice info "Unmi - 2011-01-05 23:39:19" %}}
关联 digu 只要填写正确的用户名和密码就行了，然后就是检查一下运行 PageCookery 的软件环境是否完全符合：我觉得主要是主机 PHP 一定要开放 CURL
扩展，因为它是通过 curl 函数去获取 digu 上内容的。

我用的是美国的主机，在 bloghost.cn 上购买的，相对来说会慢些，我是不太愿意去备案的。

嵌入到网页只是用 iframe，设置好微博的样式，用百分比定宽度较好些。
{{% /notice %}}

{{% notice info "IzY - 2011-01-05 23:00:27" %}}
nb啊。。怎么搞到网页上来的。。

还有怎么关联其它微博的。digu?

我都关联不到digu。。设觉得可能是我免费主机的问题。考完试就要换主机了。有什么好的主机推荐么？
{{% /notice %}}

{{% notice info "IzY - 2010-12-30 14:45:43" %}}
好的
{{% /notice %}}

{{% notice info "Unmi - 2010-12-30 14:36:37" %}}
我一定要试用一下这个博客程序的，到时交流一下。
{{% /notice %}}

{{% notice info "IzY - 2010-12-30 13:24:08" %}}
我不知道怎么和digu关联。

是在后台直接填下账号密码吗

在后台说不能验证密码正确性，不知道是不是我的密码填错了。。。
{{% /notice %}}

{{% notice info "Unmi - 2010-12-30 12:39:53" %}}
我现在这个留言板我觉得还是可以的。

那个微博程序很好的，可以通过手机随时随地发博了。关键是 Digu 那边会卡一道，稍有敏感一点的东西就不比发。
{{% /notice %}}

{{% notice info "IzY - 2010-12-29 22:11:26" %}}
http://www.pagecookery.com/ 按照说明安装一下就行了。。

我说还可以通过绑定安装digu再与其它新没什么微博连在一起，同时更新。

但我受还效怎么搞好，，，

------------

我正在搞留言板这个页面。。。貌似不大好。。
{{% /notice %}}

{{% notice info "申佳明 - 2010-12-03 00:25:20" %}}
您这个啥博客哈？？怎么留言者的MAIL地址都暴露了？！早知道不灌水留言了
{{% /notice %}}

{{% notice info "rising - 2010-11-12 11:47:44" %}}
您好，近来需要使用iTextSharp 5，但找不到合适的学习资源，特别是对于页眉、页脚以及在页眉、页脚和水印中使用图片如何实现的完整找不到方法（页眉、
页脚和水印中只有文字的功能会通过事件也能来实现，但不知道如何在页眉、页脚中加上分隔用的横线）。如有好的学习资源请给一份，对于"页眉、
页脚和水印中使用图片如何实现"和"如何在页眉、页脚中加上分隔用的横线"这两种功能的实现。请帮忙指导，谢谢。

我按您BLOG中的"Asp.Net(C#)生成PDF详解（支持中文、水印、层层、页脚、表格等）"方法做了，但是出现的提供信息为：
Unbalanced save/restore state operators"错误。我的iTextSharp的版本是5.0.5,请有空时帮忙解决一下。
{{% /notice %}}

{{% notice info "Unmi - 2010-11-12 11:00:43" %}}
备录：from whedward@hotmail.com

您好，近次需要使用iTextSharp 5，但找不到合适的学习资源，特别是对于页眉、页脚以及在页眉、页脚和水印中使用图片如何实现的完整找不到方法（页眉、
页脚和水印中只有文字的功能会通过事件也能来实现，但不知道如何在页眉、页脚中加上分隔用的横线）。如有好的学习资源请给一份，对于"页眉、
页脚和水印中使用图片如何实现"和"如何在页眉、页脚中加上分隔用的横线"这两种功能的实现。请帮忙指导，谢谢。

在您的BLOG中找到您的联系方式，就冒昧的打扰了，谢谢
{{% /notice %}}

{{% notice info "roywong - 2010-09-16 09:34:49" %}}
继续支持关注， 从您这学到了不少东西
{{% /notice %}}

<style>
    div.info p {
        padding: 0.2rem 0 !important;
    }

    div.info {
        font-size: 15px;
    }
    
    div.info div.label {
        margin-bottom: 13px;
    }

    .notices .highlight_wrap, .notices .highlight_wrap .panel_box {
       background: #eff1f5 !important; 
    }
</style>