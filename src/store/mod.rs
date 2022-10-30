pub mod account;
pub mod cache;
pub mod policy;
pub mod pool;
pub mod user;

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::{
    model::{List, ID},
    Result,
};

#[async_trait::async_trait]
pub trait Store<'a> {
    type Output: Debug + Send + Sync + Deserialize<'a> + Serialize;
    type IDs: Send + Sync + ToString;
    type Content: Send + Sync;
    type Opts: Send + Sync;
    type Query: Send + Sync;

    async fn create(&self, content: &Self::Content) -> Result<ID>;
    async fn get(&self, id: &Self::IDs) -> Result<Self::Output>;
    async fn delete(&self, id: &Self::IDs) -> Result<()>;
    async fn update(&self, id: &Self::IDs, content: &Self::Opts) -> Result<()>;
    async fn list(&self, opts: &Self::Query) -> Result<List<Self::Output>>;
    async fn exist(&self, id: &Self::IDs) -> Result<()>;
}
