#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use crate::error::AliyunResult;
use crate::error::AliyunSDKError::AliyunSMSError;
use gostd::net::http;
use std::collections::HashMap;
pub type AcsResponse = BaseResponse;

#[derive(Default, Debug)]
pub struct BaseResponse {
    pub httpStatus: i32,
    pub httpHeaders: HashMap<String, Vec<String>>,
    pub httpContentString: String,
    pub httpContentBytes: Vec<u8>,
    originHttpResponse: http::Response,
}

impl BaseResponse {
    pub fn parseFromHttpResponse(&mut self, httpResponse: &http::Response) -> AliyunResult<()> {
        if let Some(bytesBody) = &httpResponse.Body {
            self.httpStatus = httpResponse.StatusCode as i32;
            self.httpContentBytes = bytesBody.to_vec();
            self.httpContentString = String::from_utf8(bytesBody.to_vec())?;
            self.originHttpResponse = httpResponse.to_owned();
            Ok(())
        } else {
            Err(AliyunSMSError("http response body is NONE".to_string()))
        }
    }
}
