use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Password {
    pub user_id: String,
    pub account_id: String,
    pub name: String,
    pub admin: u8,
    pub extra: HashMap<String, Vec<String>>,
    pub password: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Policy {
    pub id: String,
    pub version: String,
    pub statement: Statement,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Statement {
    /// 描述
    pub description: String,
    /// 访问实体
    pub subjects: Vec<String>,
    /// 策略结果，一般为空，只有两种结果（allow、deny（默认））
    // Effect（效果）作用包含两种：Allow（允许）和Deny（拒绝），
    // 系统预置策略仅包含允许的授权语句，自定义策略中可以同时包含允许和拒绝的授权语句，
    // 当策略中既有允许又有拒绝的授权语句时，遵循Deny优先的原则。
    pub effect: String,
    /// 访问的方式
    // Action（动作）对资源的具体操作权限，格式为：服务名:资源类型:操作，
    // 支持单个或多个操作权限，支持通配符号*，通配符号表示所有。例如 s3:GetObject ，表示获取对象
    pub action: Vec<String>,
    /// 访问的资源
    // Resource（资源）策略所作用的资源，支持通配符号*，通配符号表示所有。在JSON视图中，不带Resource表示对所有资源生效。Resource支持以下字符：-_0-9a-zA-Z*./\，如果Resource中包含不支持的字符，请采用通配符号*。
    // 例如：arn:aws:s3:::my-bucketname/myobject*\，表示minio中my-bucketname/myobject目录下所有对象文件
    pub resources: Vec<String>,
    // Condition（条件）您可以在创建自定义策略时，通过Condition元素来控制策略何时生效。
    // Condition包括条件键和运算符，条件键表示策略语句的Condition元素，分为全局级条件键和服务级条件键。
    // 全局级条件键（前缀为g:）适用于所有操作，服务级条件键（前缀为服务缩写，如obs:）仅适用于对应服务的操作。
    // 运算符与条件键一起使用，构成完整的条件判断语句
    pub collections: Vec<String>,
}
