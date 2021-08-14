# Summer2021-No.151 用Rust实现Serverless无感知存储层

#### 介绍
https://gitee.com/openeuler-competition/summer-2021/issues/I3S1DY

#### 软件架构
本项目主要分为三部分：   
1. 前端语言库，将存储层操作接口以语言库/包的形式提供给用户，例如Python下的wheel。语言库将用户指令通过grpc发送到存储代理服务器并返回请求结果。   
2. 存储代理服务器，本项目的核心，通过grpc接收请求，在确认其操作权限后从后端数据平台中获取相关数据并返回给用户。抹平不同后端数据存储平台的抽象及接口差异，提供统一的命名空间及数据操作接口。
3. 授权及鉴权服务器，为用户提供管理网页，用以授权不同Serverless应用对数据操作的权限范围，为应用发放token。为存储代理服务器提供鉴权服务，通过token确认应用的数据操作权限及其生存时间。   
   
其中前端语言库将用于openfaas(Serverless平台)的函数中，存储代理服务将与openfaas结合，作为per-node application为node上的函数提供存储服务。认证服务由于并非本项目核心因此采用Django框架配合OAuth库实现，后期可能改为Rust实现。当前实现的进度为基本完成了单机版本的编写，基于openfaas的分布式版本正在实现中。
#### 使用说明
1. 启动认证服务器
    1. 安装Django
        ```
        pip3 install django
        ```
    2. 安装Django OAuth Toolkit
        ```
        pip3 install django-oauth-toolkit
        ```
    3. 创建管理员用户

        ```
        cd auth_server
        python manage.py createsuperuser

        Username: wiliam
        Email address: me@wiliam.dev
        Password:
        Password (again):
        Superuser created successfully.
        ```
    4. 启动认证服务器
        ```
        python3 manage.py runserver localhost:10087
        ```
    5. 创建应用   
        1. 浏览器中打开 http://localhost:10087/o/applications/register/。
        2. 将Client type设置为Confidential，Authorization grant type设置为Client credentials。
        3. 记录下Client id 和 Client secret。
        4. 重复以上过程，分别为用户函数以及代理服务器创建应用，并记录Client id 和 Client secret。
2. 启动存储代理服务器
    1. 初始化环境变量
    将上文步骤中获得的id及secret存入环境变量。
        ```
        export agent_url="localhost:10086"
        export auth_url="http://127.0.0.1:10087/o/token/"
        export sa_client_id="LkrnLMAoKhfbcUDxqdvysUj3DGvWymzk8vPAPRgQ"
        export sa_client_secret="grPyHYtMUh1Lv9pdn07MMbTXMiVCEdKdEGz4X6SAHBjw2G7VEVmVot9gURFUfgytvMb9DE0T2ahaz4QJk80MUhvZT0Ib7Bacxkb9BgoyIJEMAc4Iusj1jdi95aSYlyHJ"
        export sas_client_id="IUkCQnAo8Un4pTVfWNf1a0LlKbD7neBdVwmMeqLy"
        export sas_client_secret="Hp8IH6BL7iRkKdbwTvVs17A7pUIkIMhc0TU9sq40cHoxpkPPFqJwe865HG1IZhtXDRekIdWuOp3UmwPBKWq6L0TBYgXQmTFhW5UG7FPfp23Otff4gtsBCAmXzmtYRwh7"
        ```
    2. 编译运行agent_server
        ```
        cd agent_server
        cargo build
        cargo run
        ```
3. 运行测试函数(模拟Serverless函数)   

    项目的最终要求是将存储api包装成为语言库使用，当前项目开发过程中为测试方便还未打包，而是将测试程序放在库函数文件中运行。    

        ```
        cd language_libs/python/faas-storage-agent
        python3 api.py
        ```
#### 参与贡献

1.  Fork 本仓库
2.  新建 Feat_xxx 分支
3.  提交代码
4.  新建 Pull Request

