# kuth

#### 介绍
身份认证服务

#### 软件架构
软件架构说明


#### 安装教程

1.  安装最新稳定版本rust,详情查看rust官网
2.  安装有mysql系数据库,最好是有mariadb

数据库模型同步,需要复制工作空间的.env.sample,并更名为.env,修改DATABASE_URL为对应的数据库连接，下面是同步命令
```shell
cargo install sqlx-cli --no-default-features --features rustls,mysql && sqlx database create && sqlx migrate run
```
3.  xxxx

#### 使用说明

1.  xxxx
2.  xxxx
3.  xxxx

#### 参与贡献

1.  Fork 本仓库
2.  新建 Feat_xxx 分支
3.  提交代码
4.  新建 Pull Request


#### 特技

1.  使用 Readme\_XXX.md 来支持不同的语言，例如 Readme\_en.md, Readme\_zh.md
2.  Gitee 官方博客 [blog.gitee.com](https://blog.gitee.com)
3.  你可以 [https://gitee.com/explore](https://gitee.com/explore) 这个地址来了解 Gitee 上的优秀开源项目
4.  [GVP](https://gitee.com/gvp) 全称是 Gitee 最有价值开源项目，是综合评定出的优秀开源项目
5.  Gitee 官方提供的使用手册 [https://gitee.com/help](https://gitee.com/help)
6.  Gitee 封面人物是一档用来展示 Gitee 会员风采的栏目 [https://gitee.com/gitee-stars/](https://gitee.com/gitee-stars/)
