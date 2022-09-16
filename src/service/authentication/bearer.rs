use jsonwebtoken::{decode, decode_header, DecodingKey, Validation};
use sqlx::MySqlPool;

use crate::{
    service::authorization::{Abac, Attribute, Matchers},
    store::cache::Cache,
    Error, Result,
};

pub async fn parse(pool: MySqlPool, token: &str, r: &super::Request) -> Result<super::Effect> {
    let header = decode_header(token).map_err(Error::any)?;
    let secret_key = match header.kid {
        Some(kid) => Cache.get_temp_secret(&kid).await?,
        None => return Err(Error::Forbidden("user invalid".to_string())),
    };
    let c = match decode::<super::Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &Validation::default(),
    ) {
        Ok(v) => v,
        Err(err_value) => {
            tracing::error!("{}", err_value);
            return Err(Error::Forbidden("invalid token".to_string()));
        }
    };

    let mut att = Attribute {
        user_id: c.claims.id.clone(),
        account_id: c.claims.account_id.clone(),
        name: c.claims.name,
        extra: c.claims.args,
        action: r.action.clone(),
        resource: None,
        path: r.path.clone().unwrap_or_default(),
    };
    if let Some(resource) = &r.resource {
        att.resource = Some((
            resource.to_owned(),
            r.sub_resource.clone().unwrap_or_default(),
        ));
    };
    tracing::debug!(
        "{:#?} {}",
        att,
        serde_json::to_string(&att).map_err(Error::any)?
    );
    let (d, reason) = Abac.authorize(&pool, &att).await?;
    Ok(super::Effect {
        decision: d.to_string(),
        reason,
        user_id: c.claims.id,
        account_id: c.claims.account_id,
    })
}
