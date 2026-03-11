---
title: "AWS ECS 使用 EC2 Capacity Provider (Managed Instances)" 
url: /aws-ecs-capacity-provider-managed-instances/
date: 2026-03-10T22:33:20-05:00
featured: false
draft: false
type: post
toc: false
# menu: main
usePageBundles: true
thumbnail: "../images/logos/aws-logo.png"
categories:
  - AWS
tags: 
  - AWS
  - Linux
comment: true
codeMaxLines: 20
---

上篇 [AWS ECS 使用 EC2 Capacity Provider (EC2 Auto Scaling)](/aws-ecs-capacity-provider-ec2-auto-scaling/) 学习了如何在 ECS
中使用 Capacity Provider + EC2 Auto Scaling 来部署一个简单的 Web 应用，以及了解 ECS 如何管理 EC2 的 Auto Scaling Group.

ECS Capacity Providers 是于 2019 年 12 月发布的，随同的功能支持了 Fargate, Fargate Spot, 和 EC2 AutoScaling. 
而 Managed Instances 在 2025-09-30 才加入的新特性，见 [Announcing Amazon ECS Managed Instances](https://aws.amazon.com/about-aws/whats-new/2025/09/amazon-ecs-managed-instances/).

在近六年之间, AWS 大概也理解到了 Capacity Provider + EC2 Auto Scaling 的复杂性，虽说可由 ECS 来管理 EC2 的 ASG, 但毕竟有个 ASG 在那里。
ECS 与 EC2 ASG 之间联接由 CloudWatch Metrics, Alarms, EC2 ASG 的 Dynamic scaling policies 和 Lifecycle hooks 的一整套机制协作。
经常还不得不在 EC2 ASG 与 ECS 两个界面之间来回找问题。而 Managed Instances 的出现则将 EC2 实例的管理完全透明化了，在 EC2 端压根就不存在
一个相应的 ASG, 更不需要 CloudWatch Alarms 之类的关联组件。

Managed Instances 与 EC2 Auto Scaling 之间就好比 Serverless 与 非 Serverless 的区别，用 Managed Instances 之后你只管控制好 ECS 
Desired Count(ECS AutoScaling), 其余的都由 Managed Instances 来管理，在界面上只需要关注 ECS Infrastructure 中的 Container Instances.
由 Managed Instances 管理的 EC2 实例，你即使用有管理员权限都无法关闭它，只能全权由 Managed Instances 来控制。试图关闭这样的 EC2 实例时报错
<!--more-->

{{< bundle-image ecs-capacity-provider-managed-1.png 900 >}}

相比于 ECS 直接用 EC2(ASG)，通过 Capacity Provider 使用 EC2 的方式，还有一个好处就是可以实现 Min and max running tasks:
100% min and 200% max 的部署方式，即启动与当前等量的任务来替换全部旧任务，即使是高峰期也能较安心的部署。而 ECS 直接用 EC2 的时只能实现
Min and max running tasks: max 最大 100%, 所以不得不在 `Rolling update`, `Blue/green`, `Canary` 和 `Linear` 之间选择。

Managed Instances 可以配置 vCPU, Memory, GPU 等约束条件，在运行时由 Capacity Provider 自动为你选择 EC2 实例类型. 而对于计算型的任务
还是较倾向于直接锁定 EC2 的实例类型, 如 AMD CPU 的 c8a.4xlarge 等。

下面是一个 Web 应用部署实例，使用 `t3.medium` 实例类型，它有 2 vCPU, 4 GiB 内存, Web 任务需要 1 vCPU, 1 GiB 内存，这样实际在一个
`t3.medium` 实例上可以运行 2 个 Web 任务。

下面是相关的 Terraform 脚本如下

*capacity-provider.tf*

```terraform
resource "aws_ecs_capacity_provider" "ec2" {
  name = "ec2-capacity-provider"
  cluster = aws_ecs_cluster.main.name

  managed_instances_provider {
    infrastructure_role_arn = aws_iam_role.ecs_infrastructure.arn
    propagate_tags          = "CAPACITY_PROVIDER"

    infrastructure_optimization {
      scale_in_after = 300
    }
    instance_launch_template {
      ec2_instance_profile_arn = var.ecs_instance_profile_arn
      storage_configuration {
        storage_size_gib = 30
      }

      network_configuration {
        subnets         = var.subnet_ids
        security_groups = var.ec2_security_groups
      }

      instance_requirements {
        vcpu_count {
          min = 2
        }

        memory_mib {
          min = 4096
        }
        allowed_instance_types = ["t3.medium"]
        burstable_performance = "included"
      }
    }
  }
  tags = {
    Name = "test-cp"
  }
}

data "aws_iam_policy_document" "ecs_infra_trust" {
  statement {
    actions = ["sts:AssumeRole"]
    effect  = "Allow"
    principals {
      type        = "Service"
      identifiers = ["ecs.amazonaws.com"]
    }
  }
}

resource "aws_iam_role" "ecs_infrastructure" {
  name               = "ECSInfraRole"
  assume_role_policy = data.aws_iam_policy_document.ecs_infra_trust.json
}

resource "aws_iam_role_policy_attachment" "ecs_infrastructure" {
  role       = aws_iam_role.ecs_infrastructure.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonECSInfrastructureRolePolicyForManagedInstances"
}
```

通过 `allowed_instance_types = ["t3.medium"]` 来锁定使用 `t3.medium` 类型，注意 cpu 和 memory 的配置不能与此有冲突，并且 `t3.medium`
是 `burstable` 的类型, 所以必须加上 `burstable_performance = "included"` 才能选择到. 

`infrastructure_role_arn` 需要指定为一个有 `arn:aws:iam::aws:policy/AmazonECSInfrastructureRolePolicyForManagedInstances`
policy, 能被 `ecs.amazonaws.com` assume 的 IAM role 即可。

使用 `Managed Instances` 时 EC2 除了可以指定 Security group, subnets, storage 外，其他都几乎都不能自定义。比如 AMI 是由
`Managed Instances` 指定的，像 `ecs-managed-instances-standard-x86_64-20260220222634`, 它是专门优化的，并且 EC2 会每 14 
天被自动更新一次，来保证安全和性能的优化。

*ecs.tf*

```terraform
resource "aws_ecs_cluster" "main" {
  name = "test-cp"
  setting {
    name  = "containerInsights"
    value = "enabled"
  }
}

resource "aws_ecs_service" "main" {
  name                 = "my-service"
  cluster              = aws_ecs_cluster.main.id
  task_definition      = aws_ecs_task_definition.main.arn
  desired_count        = 1
  network_configuration {
    subnets         = var.subnet_ids
    security_groups = var.ec2_security_groups
  }

  capacity_provider_strategy {
    capacity_provider = aws_ecs_capacity_provider.ec2.name
    base              = 1
    weight            = 1
  }

  load_balancer {
    target_group_arn = aws_lb_target_group.main.arn
    container_name   = "my-container"
    container_port   = 80
  }
}

resource "aws_ecs_task_definition" "main" {
  family                   = "my-task"
  requires_compatibilities = ["MANAGED_INSTANCES","EC2"]
  network_mode             = "awsvpc"

  container_definitions = jsonencode([
    {
      name        = "my-container"
      image       = "strm/helloworld-http"
      cpu         = 1024
      memory      = 1024
      stopTimeout = 5
      portMappings = [
        {
          containerPort = 80
          hostPort      = 80
          protocol      = "tcp"
        }
      ]
    }
  ])
}

resource "aws_appautoscaling_target" "ecs_target" {
  max_capacity       = 10
  min_capacity       = 1
  resource_id        = "service/${aws_ecs_cluster.main.name}/${aws_ecs_service.main.name}"
  scalable_dimension = "ecs:service:DesiredCount"
  service_namespace  = "ecs"
}
```

Network 采用了 `vpc`, 每个容器将会从 VPC 获得自己的 IP 地址，Security Group 更易配置，初始启动一个任务。实际项目中应创建 ECS 的 AutoScaling
规则来控制 Desired Count, 后面测试将手工来调节。

*elb.tf*

```terraform
resource "aws_lb" "test-cp" {
  name            = "test-cp-lb"
  internal        = true
  subnets         = var.subnet_ids
  security_groups = var.elb_security_groups

}

resource "aws_lb_listener" "main" {
  port              = 80
  protocol          = "HTTP"
  load_balancer_arn = aws_lb.test-cp.arn
  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.main.arn
  }
}

resource "aws_lb_target_group" "main" {
  name        = "test-cp-tg"
  port        = 80
  protocol = "HTTP"
  vpc_id      = var.vpc_id
  target_type = "ip"
}
```

单纯测试 `Managed Instances` 类型的 `Capacity Provider` 可以不用建立 ELB, 作为一个较完备的应用还是加上这一层。

在运行以上的 Terraform 脚本时，请填入对应环境的以下变量

```terraform
  subnet_ids
  ecs_instance_profile_arn
  elb_security_groups
  ec2_security_groups
  vpc_id
```

准备好 AWS Credentials, 再补上 AWS Provider 后, 运行

```shell
terraform init
terraform apply-auto-approve
```

成功后会产生整个架构。在 ECS cluster `test-cp` 的 `Infrastructure` 将会看到

{{< bundle-image ecs-capacity-provider-managed-2.png 1000 >}}

使用了 `Managed Instances` 的 `Capacity Provider` 的管理界面就只要这个就行了。对于相应 EC2 的管理十分有限，不能指定 `key_name`, 
也没有 `Session Manager` 可以连，不能自定义 `userdata`, 可以说 `Managed Instances` 产生的 EC2 是无法管理的。

由 `Managed Instances` 生成的 EC2 的 userdata 内容为

```text
[settings.ecs] 
cluster = 'arn:aws:ecs:us-east-1:123456789000:cluster/test-cp' 
awsvpc-block-imds = true 
[settings.two] 
platform-revision = 'Linux-X86_64-1.0.0-55' 
available-memory = 3530 
```

下面是 `Capacity Provider` 的界面

{{< bundle-image ecs-capacity-provider-managed-3.png 1000 >}}

下面直接改变 ECS Service `my-service` 的 Desired Count

1. Desired Count 为 2： 因现有 `t3.medium` 实例上还有充足的 CPU/Memory 资源，所以直接在其上运行一个新任务
  {{< bundle-image ecs-capacity-provider-managed-4.png 1000 >}} 
2. Desired Count 为 3：由于现有实例 CPU 资源不足，所以必须新启一个 EC2 实例来运行新的任务
  {{< bundle-image ecs-capacity-provider-managed-5.png 1000 >}} 
3. Desired Count 为 4 时又会在新的 EC2 实例上运行一个新任务, 这里就不再往测试了
4. 现在把 Desired Count 改为 2：发现从运行两个 Task 的 EC2 上停掉了一个任务，而不是从运行一个任务的 EC2 实例上停掉任务
  {{< bundle-image ecs-capacity-provider-managed-6.png 1000 >}} 

再等了很久，也是用两个 EC2 实例各自运行一个 Task, 从资源来讲有些浪费, 看来还是优先考虑性能，也不会把一个任务从一个 EC2 实例上挪到另一个去。

5. 把 Desired Count 改为 1， 

{{< bundle-image ecs-capacity-provider-managed-8.png 1000 >}} 

那么什么时候会把空闲的 EC2 实例关闭呢？大概等了七八分钟，没有运行任务的 EC2 上出现 `Deregistrating`, 并且一两秒间从该列表中消失了。

{{< bundle-image ecs-capacity-provider-managed-9.png 1000 >}}

剩下的事情就是 `Capacity Provider` 把该 EC2 实例也结束掉。

最后只省下一个 EC2 实例

{{< bundle-image ecs-capacity-provider-managed-10.png 1000 >}}

而且这个是更老的 EC2 实例，Capacity Provider` 优先关闭新的 Task.

如果 Container 实例卡在了 `ACTIVE` 或 `DRAINING` 状态，但无法关闭或运行任务，需要用命令强行注销，命令是

```shell
aws ecs deregister-container-instance --cluster <cluster-name>  --container-instance <id> --force
```