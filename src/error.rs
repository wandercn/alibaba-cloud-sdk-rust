use std::{str::Utf8Error, string::FromUtf8Error};

use anyhow::Result;
use gostd::net::http::HTTPConnectError;
use thiserror::Error;
// 自定义错误类型 AlipaySDKError
#[derive(Debug, Error)]
pub enum AliyunSDKError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP connection error: {0}")]
    HttpConnection(#[from] HTTPConnectError),

    #[error("AliyunSMSError error: {0}")]
    AliyunSMSError(String),
    #[error("Utf8Error{0}")]
    Utf8Error(#[from] Utf8Error),

    #[error("FromUtf8Error: {0}")]
    FromUtf8Error(#[from] FromUtf8Error),
    #[error("serde json error{0}")]
    JsonError(#[from] serde_json::Error),
}

impl From<String> for AliyunSDKError {
    fn from(err: String) -> Self {
        AliyunSDKError::AliyunSMSError(err)
    }
}

pub type AliyunResult<T> = Result<T, AliyunSDKError>;
