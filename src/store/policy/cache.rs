use redis::AsyncCommands;

use crate::{
    model::{List, ID},
    store::cache::{NilString, CLIENT, DEFAULT_EXPIRE_SEC},
    store::Store,
    Error, Result,
};

pub struct Cache<T>(pub T);

#[async_trait::async_trait]
impl<'a, T: Store<'a> + Send + Sync> Store<'a> for Cache<T> {
    type Output = T::Output;
    type IDs = T::IDs;
    type Content = T::Content;
    type Opts = T::Opts;
    type Query = T::Query;

    async fn create(&self, content: &Self::Content) -> Result<ID> {
        self.0.create(content).await
    }
    async fn get(&self, id: &Self::IDs) -> Result<Self::Output> {
        let mut conn = CLIENT.get_async_connection().await.map_err(Error::any)?;
        let value: NilString = conn
            .get(self.key(&id.to_string()))
            .await
            .map_err(Error::any)?;
        match value.value {
            Some(data) => {
                let content: Self::Output = serde_json::from_str(&data).map_err(Error::any)?;
                return Ok(content);
            }
            None => {
                let content = self.0.get(id).await?;
                let data = serde_json::to_string(&content).map_err(Error::any)?;
                conn.set_ex(self.key(&id.to_string()), data, DEFAULT_EXPIRE_SEC)
                    .await
                    .map_err(Error::any)?;
                return Ok(content);
            }
        }
    }
    async fn delete(&self, id: &Self::IDs) -> Result<()> {
        let mut conn = CLIENT.get_async_connection().await.map_err(Error::any)?;
        conn.del(self.key(&id.to_string()))
            .await
            .map_err(Error::any)?;
        self.0.delete(id).await
    }
    async fn update(&self, id: &Self::IDs, content: &Self::Opts) -> Result<()> {
        self.0.update(id, content).await?;
        let mut conn = CLIENT.get_async_connection().await.map_err(Error::any)?;
        conn.del(self.key(&id.to_string()))
            .await
            .map_err(Error::any)
    }
    async fn list(&self, opts: &Self::Query) -> Result<List<Self::Output>> {
        self.0.list(opts).await
    }
    async fn exist(&self, id: &Self::IDs) -> Result<()> {
        self.0.exist(id).await
    }
}

impl<T> Cache<T> {
    fn key(&self, id: &str) -> String {
        format!("kuth:policy:{}", id)
    }
}
