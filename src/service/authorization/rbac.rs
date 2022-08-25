use sqlx::MySqlPool;

use crate::{model::cache::Policy, Result};

pub struct Rbac;

#[async_trait::async_trait]
impl super::Matchers for Rbac {
    async fn authorize(
        &self,
        pool: &MySqlPool,
        att: &super::Attribute,
    ) -> Result<(super::Decision, String)> {
        tracing::info!("{:?}", att.resource);
        tracing::info!("{}", att.action);
        let role_policys = super::get_policys(pool, &att.user_id).await?;
        tracing::debug!("{:?}", role_policys);
        for policy in role_policys.iter() {
            if self.visit(policy, att) {
                match policy.statement.effect.as_str() {
                    "Allow" => return Ok((super::Decision::Allow, "".to_string())),
                    "Deny" => return Ok((super::Decision::Deny, "".to_string())),
                    _ => return Ok((super::Decision::Nop, "".to_string())),
                }
            }
        }
        Ok((super::Decision::Nop, "not impl".to_string()))
    }
}

impl Rbac {
    fn visit(&self, role_policy: &Policy, a: &super::Attribute) -> bool {
        if a.resource.is_none() {
            return self.verb_matches(role_policy, a) && self.non_resource_matches(role_policy, a);
        }
        self.verb_matches(role_policy, a) && self.resource_matches(role_policy, a)
    }
    fn verb_matches(&self, role_policy: &Policy, a: &super::Attribute) -> bool {
        for action in role_policy.statement.action.iter() {
            if action.eq("*") {
                return true;
            }
            if action.eq(&a.action) {
                return true;
            }
        }
        false
    }

    fn resource_matches(&self, role_policy: &Policy, a: &super::Attribute) -> bool {
        let (temp_resource, sub_resource) = a.resource.clone().unwrap_or_default();

        let mut combined_resource = temp_resource;
        if !sub_resource.is_empty() {
            combined_resource.push('/');
            combined_resource.push_str(&sub_resource);
        }

        for resource in role_policy.statement.resources.iter() {
            if resource.eq("*") {
                return true;
            }
            if resource.eq(&combined_resource) {
                return true;
            }
            if sub_resource.is_empty() {
                continue;
            }
            if resource.len() == sub_resource.len() + 2
                && resource.starts_with("*/")
                && resource.ends_with(&sub_resource)
            {
                return true;
            }
        }
        false
    }

    fn non_resource_matches(&self, role_policy: &Policy, a: &super::Attribute) -> bool {
        for resource in role_policy.statement.resources.iter() {
            if resource.eq("*") {
                return true;
            }
            if resource.eq(&a.path) {
                return true;
            }
            if resource.ends_with('*') && resource.starts_with(resource.trim_end_matches('*')) {
                return true;
            }
        }
        false
    }
}
