use std::ops::Deref;

use axum::{
    body::HttpBody,
    extract::{FromRequest, Query, RequestParts},
    Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::Error;

pub struct Valid<T>(pub T);

#[async_trait::async_trait]
impl<T, B> FromRequest<B> for Valid<Query<T>>
where
    T: DeserializeOwned + Validate,
    B: Send,
{
    type Rejection = Error;
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let value = Query::<T>::from_request(req)
            .await
            .map_err(|err| Error::BadRequest(err.to_string()))?;
        value.deref().validate().map_err(Error::Validates)?;
        Ok(Self(value))
    }
}

#[async_trait::async_trait]
impl<T, B> FromRequest<B> for Valid<Json<T>>
where
    T: DeserializeOwned + Validate,
    B: HttpBody + Send,
    B::Data: Send,
    B::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    type Rejection = Error;
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let value = Json::<T>::from_request(req)
            .await
            .map_err(|err| Error::BadRequest(err.to_string()))?;
        value.deref().validate().map_err(Error::Validates)?;
        Ok(Self(value))
    }
}

#[derive(Debug, Default)]
pub struct Header {
    pub account_id: String,
    pub user_id: String,
    pub source: String,
}

#[async_trait::async_trait]
impl<B> FromRequest<B> for Header
where
    B: Send,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let mut result = Header::default();
        let header = req.headers();
        result.account_id = header
            .get("X-Account-ID")
            .ok_or_else(|| Error::Forbidden("miss request header X-Account-ID".to_string()))?
            .to_str()
            .unwrap_or_default()
            .to_string();
        result.user_id = header
            .get("X-User-ID")
            .ok_or_else(|| Error::Forbidden("miss request header X-User-ID".to_string()))?
            .to_str()
            .unwrap_or_default()
            .to_string();

        if let Some(v) = header.get("X-Source") {
            result.source = v.to_str().unwrap_or_default().to_owned();
        };
        Ok(result)
    }
}
