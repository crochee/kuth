use sqlx::MySqlPool;

use crate::{model::cache::Policy, Result};

pub struct Abac;

#[async_trait::async_trait]
impl super::Matchers for Abac {
    async fn authorize(
        &self,
        pool: &MySqlPool,
        att: &super::Attribute,
    ) -> Result<(super::Decision, String)> {
        let policys = super::get_policys(pool, &att.user_id).await?;
        tracing::debug!("{:?}", policys);
        for policy in policys.iter() {
            if self.matches(policy, att) {
                match policy.statement.effect.as_str() {
                    "Allow" => return Ok((super::Decision::Allow, "".to_string())),
                    "Deny" => return Ok((super::Decision::Deny, "".to_string())),
                    _ => return Ok((super::Decision::Nop, "".to_string())),
                }
            }
        }
        Ok((super::Decision::Nop, "not matched".to_string()))
    }
}

impl Abac {
    fn matches(&self, p: &Policy, a: &super::Attribute) -> bool {
        self.subject_matches(p, a) && self.verb_matches(p, a) && self.object_matches(p, a)
    }
    fn subject_matches(&self, p: &Policy, a: &super::Attribute) -> bool {
        let mut user_matched = false;
        let mut account_matched = false;
        for subject in p.statement.subjects.iter() {
            if let Some((object, value)) = subject.split_once(':') {
                match object {
                    "user" => {
                        if value.eq("*") || value.eq(&a.name) || value.eq(&a.user_id) {
                            user_matched = true;
                        }
                    }
                    "account" => {
                        if value.eq("*") || value.eq(&a.account_id) {
                            account_matched = true;
                        }
                    }
                    _ => {}
                }
            }
        }
        user_matched && account_matched
    }
    fn verb_matches(&self, p: &Policy, a: &super::Attribute) -> bool {
        if a.is_only_read() {
            return true;
        }
        for action in p.statement.action.iter() {
            if action.eq("*") {
                return true;
            }
            if action.eq(&a.action) {
                return true;
            }
        }
        false
    }
    fn object_matches(&self, p: &Policy, a: &super::Attribute) -> bool {
        if a.resource.is_none() {
            return self.non_resource_matches(p, a);
        }
        self.resource_matches(p, a)
    }
    fn resource_matches(&self, p: &Policy, a: &super::Attribute) -> bool {
        let (temp_resource, _) = a.resource.clone().unwrap_or_default();
        for resource in p.statement.resources.iter() {
            if resource.eq("*") {
                return true;
            }
            if resource.eq(&temp_resource) {
                return true;
            }
        }
        false
    }
    fn non_resource_matches(&self, p: &Policy, a: &super::Attribute) -> bool {
        for resource in p.statement.resources.iter() {
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
