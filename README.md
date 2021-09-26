# Summer2021-No.151 用Rust实现Serverless无感知存储层

## 介绍
### 背景
背景请见
https://gitee.com/openeuler-competition/summer-2021/issues/I3S1DY

### 目标
本项目为面向Serverless场景的存储代理，致力于简化Serverless应用使用存储功能的方式，同时方便存储平台的变更和迁移。使得用户可以通过一套API以及简单的配置就可以方便地使用多种数据平台，并在其间进行迁移。同时在存储代理中还可以依据不同场景对存储服务做出优化，例如完成数据缓存、建立连接池，或是提供函数间通信等功能。
![](https://whypics.oss-cn-shenzhen.aliyuncs.com/pics/20210926134338.png)

### 项目架构
内容详见
docs/architecture.md。

### 使用说明
内容详见
docs/getting_start.md。

