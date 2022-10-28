use std::str::FromStr;

use axum::{
    body::{self, Full},
    response::{IntoResponse, Response},
};
use http::{header, HeaderValue, StatusCode};
use serde::Serialize;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Any(#[from] anyhow::Error),
    #[error("{0} isn't found")]
    NotFound(String),
    #[error("{0}")]
    Validates(#[source] validator::ValidationErrors),
    #[error("{0}")]
    Forbidden(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("{0}")]
    NotImpl(String),
    #[error("{0}")]
    BadRequest(String),
}

impl Error {
    #[inline]
    pub fn any<E>(err: E) -> Self
    where
        E: std::error::Error,
    {
        Self::Any(anyhow::format_err!("{}", err))
    }
}

#[derive(Serialize, Debug)]
pub struct Message {
    pub code: String,
    pub message: String,
    pub result: String,
}

impl Message {
    pub fn status_code(&self) -> Result<StatusCode, Error> {
        let codes: Vec<&str> = self.code.split('.').collect();
        if codes.len() != 3 {
            return Err(Error::Any(anyhow::anyhow! {"code's lenght isn't 3"}));
        }
        StatusCode::from_str(codes[1]).map_err(Error::any)
    }
}

impl From<Error> for Message {
    fn from(val: Error) -> Self {
        match val {
            Error::Any(err) => Message {
                code: "kuth.500.1010001".to_string(),
                message: "服务器内部错误".to_string(),
                result: err.to_string(),
            },
            Error::NotFound(err) => Message {
                code: "kuth.404.1010002".to_string(),
                message: "资源不存在".to_string(),
                result: err,
            },
            Error::Forbidden(err) => Message {
                code: "kuth.403.1010003".to_string(),
                message: "非法操作".to_string(),
                result: err,
            },
            Error::Unauthorized(err) => Message {
                code: "kuth.401.1010004".to_string(),
                message: "未登录".to_string(),
                result: err,
            },
            Error::Validates(err) => Message {
                code: "kuth.400.1010005".to_string(),
                message: "请求参数不正确".to_string(),
                result: err.to_string(),
            },
            Error::NotImpl(err) => Message {
                code: "kuth.500.1010006".to_string(),
                message: "未实现该功能".to_string(),
                result: err,
            },
            Error::BadRequest(err) => Message {
                code: "kuth.400.1010007".to_string(),
                message: "请求参数不正确".to_string(),
                result: err,
            },
        }
    }
}

impl From<&Error> for Message {
    fn from(val: &Error) -> Self {
        match val {
            Error::Any(err) => Message {
                code: "kuth.500.1010001".to_string(),
                message: "服务器内部错误".to_string(),
                result: err.to_string(),
            },
            Error::NotFound(err) => Message {
                code: "kuth.404.1010002".to_string(),
                message: "资源不存在".to_string(),
                result: err.to_owned(),
            },
            Error::Forbidden(err) => Message {
                code: "kuth.403.1010003".to_string(),
                message: "非法操作".to_string(),
                result: err.to_owned(),
            },
            Error::Unauthorized(err) => Message {
                code: "kuth.401.1010004".to_string(),
                message: "未登录".to_string(),
                result: err.to_owned(),
            },
            Error::Validates(err) => Message {
                code: "kuth.400.1010005".to_string(),
                message: "请求参数不正确".to_string(),
                result: err.to_string(),
            },
            Error::NotImpl(err) => Message {
                code: "kuth.500.1010006".to_string(),
                message: "未实现该功能".to_string(),
                result: err.to_owned(),
            },
            Error::BadRequest(err) => Message {
                code: "kuth.400.1010007".to_string(),
                message: "请求参数不正确".to_string(),
                result: err.to_string(),
            },
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let content: Message = self.into();
        tracing::error!("{}", content.result);
        let code = match content.status_code() {
            Ok(v) => v,
            Err(err) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                    )
                    .body(body::boxed(Full::from(err.to_string())))
                    .unwrap();
            }
        };
        let bytes = match serde_json::to_vec(&content) {
            Ok(res) => res,
            Err(err) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                    )
                    .body(body::boxed(Full::from(err.to_string())))
                    .unwrap();
            }
        };

        let mut res = Response::new(body::boxed(Full::from(bytes)));
        *res.status_mut() = code;
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
        );
        res
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        let content: Message = self.into();
        let other_content: Message = other.into();
        content.code == other_content.code
    }
}
