use std::collections::HashMap;

use validator::Validate;

use crate::{Error, Result};

#[derive(Debug, Validate)]
pub struct Attributes {
    // subject item
    #[validate(length(min = 1))]
    pub account_id: String,
    #[validate(length(min = 1))]
    pub user_id: String,
    #[validate(length(min = 1))]
    pub name: String,
    pub role_id: String,
    #[validate(length(min = 1))]
    pub action: String,
    // object item
    #[validate(length(min = 1))]
    pub object_id: String,
    #[validate(length(min = 1))]
    pub object_name: String,
    pub env: HashMap<String, HashMap<String, Vec<String>>>,
}

pub struct Abac;

#[async_trait::async_trait]
impl super::Matchers for Abac {
    /// input attributes given by Matchers
    type Attributes = Attributes;
    /// authorize return  ok or error
    async fn authorize(&self, input: &Self::Attributes) -> Result<()> {
        input.validate().map_err(Error::Validates)?;
        Ok(())
    }
}
