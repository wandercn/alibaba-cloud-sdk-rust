#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use gostd::net::http;
use std::collections::HashMap;
pub struct AcsResponse {}

#[derive(Default)]
pub struct BaseResponse {
    httpStatus: i32,
    httpHeaders: HashMap<String, String>,
    httpContentString: String,
    httpContentBytes: Vec<u8>,
    originHttpResponse: http::Response,
}
