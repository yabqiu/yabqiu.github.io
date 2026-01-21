---
title: 创建基于 Amazon Linux 2023 的允许 SSH 登陆的 Docker 镜像
url: /amazonlinux-2023-root-ssh-enabled-image/
date: 2026-01-20T18:40:00-06:00
featured: false
type: post
draft: false
toc: false
# menu: main
usePageBundles: true
thumbnail: "/images/logos/docker-logo.png"
categories:
  - docker
tags:
  - ssh
comment: true
codeMaxLines: 50
---

前年记录过一篇 [创建可直接用 root 用户 ssh 登陆的 Docker 镜像](/create-root-ssh-docker-image/), 采用了基础镜像是 Ubuntu:20.04.
因为在 AWS 使用 AmazonLinux 2023 更为频繁，为贴近生产环境，本地开发也使用基本 AmazonLinux 2023 为基础镜像的容器，与 IDE 连接以 SSH 协议，
容器的操作由自己的控制，而非直接使用 devcontainer 的方式。比如可以在 Windows 或 macOS 进行 Linux 相关的开发。

当今 Linux 主要还是两个发行版，一个是 RedHat 的家族，一个是 Debian 的家族，之前验证过 Debian 族的 Ubuntu, 这次要验证 RedHat 族的
AmazonLinux 2023, 也作为将来不时之需。

### 创建允许 root + 密码登陆的镜像

Dockerfile 内容为<!--more-->

```dockerfile
FROM public.ecr.aws/amazonlinux/amazonlinux:2023

RUN dnf -y update \
    && dnf -y install openssh-server passwd \
    && dnf clean all \
    && rm -rf /var/cache/dnf

RUN mkdir /var/run/sshd

RUN ssh-keygen -A

RUN echo 'root:changeme' | chpasswd

RUN sed -i 's/^#\?PasswordAuthentication.*/PasswordAuthentication yes/' /etc/ssh/sshd_config \
 && sed -i 's/^#\?PermitRootLogin.*/PermitRootLogin yes/' /etc/ssh/sshd_config \
 && sed -i 's/^#\?UsePAM.*/UsePAM no/' /etc/ssh/sshd_config


CMD ["/usr/sbin/sshd", "-D"]
```

这里不对上面的命令作具体的解释了，因为是 RedHat 系列，所以 `dnf` 命令也可以换成 `yum` 命令, 好像如今天列建议用 `dnf` 命令来管理软件包。

#### 构建镜像
```shell
docker build -t demo-ssh .
```

创建了一个 tag 为 demo-ssh:latest 的镜像。

#### 运行容器

```shell
docker run -d -p 2222:22 --name dev-container demo-ssh
```

映射了宿主机的 2222 端口到容器的 22 端口，这样就可以通过 SSH 连接到容器了。

#### ssh 登陆

```shell
~ ssh root@localhost -p 2222
The authenticity of host '[localhost]:2222 ([::1]:2222)' can't be established.
ED25519 key fingerprint is SHA256:m6vcc3uV+ZsEho1Ljf74IHA+EVUL+KmTmMWo/uhnIx4.
This key is not known by any other names.
Are you sure you want to continue connecting (yes/no/[fingerprint])? yes
Warning: Permanently added '[localhost]:2222' (ED25519) to the list of known hosts.
root@localhost's password:
   ,     #_
   ~\_  ####_        Amazon Linux 2023
  ~~  \_#####\
  ~~     \###|
  ~~       \#/ ___   https://aws.amazon.com/linux/amazon-linux-2023
   ~~       V~' '->
    ~~~         /
      ~~._.   _/
         _/ _/
       _/m/'
[root@c56f3ed9d155 ~]#
```

输入密码 `changeme`, 这样用 ssh 就登陆了 Docker 容器，再就是根据自己的开发需求安装相应的软件。

本文的内容也就差不多了，由于只在本地作为开发测试使用，安全性不用太过于考虑，顶多把密码设置稍为复杂一些。在 IDE 连接本地的 localhost:2222 SSH
服务设置时也只需要填入一次密码。

### 其他相关话题

想要免密码登陆的话，那就直接参考 [创建可直接用 root 用户 ssh 登陆的 Docker 镜像](/create-root-ssh-docker-image/) 中 
[创建免密登陆的镜像](/create-root-ssh-docker-image/#no-password-ssh)
一节，主动就是在构建时把本地的用 `keygen` 命令产生的 `~/.ssh/id_rsa.pub` 内容拷入到 Docker 镜像的 `/root/.ssh/authorized_keys` 文件中。

像用 JetBrains 之类的 IDE 连接 Docker 容器后要能远程调试，它们会在容器中安装必要的代理软件，我们可以在容器不使用时 `docker stop`, 需要用的时候再
`docker start`。 `docker kill` 也没多大关系，还是可用 `docker start` 启动的。如果用了 `docker rm <容器id>`, 那么下次启动时就是一个全新的容器了，
之前由 IDE 安装的代理软件又要重新安装了。

由 IDE 自动安装的软件可能难以配置到 `Dockerfile` 中，所以最好的办法是对容器当前状态进行备份，如 `docker commit` 保存当前状态。例如

```shell
docker commit dev-container demo-ssh:backup

# 下次启动就用
docker run -d -p 2222:22 --name dev-container demo-ssh:backup
```

commit 了之后，电脑重启也不怕。如果镜像要与别人共享，可用 `docker save`, `docker load` 命令导出和导入镜像文件。

当 IDE 用 ssh 连接容器后会进行文件同步，项目文件不大的话，每次同步到容器中也行，或者在启动容器时用 `-v <本地目录>:/work` 映射到容器的一个卷，
这样下次启动容器时不须再将时行完整文件的同步。

### 本文碰到的两个问题

1. 如果在 `Dockerfile` 中没有加 `RUN ssh-keygen -A`， 则在启动容器时报错 `sshd: no hostkeys available -- exiting.`
2. 如果在 `Dockerfile` 中缺少 `sed -i 's/^#\?UsePAM.*/UsePAM no/' /etc/ssh/sshd_config` 的话容器可以启动，但用 `ssh` 
登陆时客户端报错 `root@localhost: Permission denied (publickey,gssapi-keyex,gssapi-with-mic).`