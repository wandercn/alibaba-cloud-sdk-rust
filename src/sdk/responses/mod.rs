#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use gostd::net::http;
use std::collections::HashMap;
pub type AcsResponse = BaseResponse;

#[derive(Default, Debug)]
pub struct BaseResponse {
    pub httpStatus: i32,
    pub httpHeaders: HashMap<String, Vec<String>>,
    pub httpContentString: String,
    pub httpContentBytes: Vec<u8>,
    pub originHttpResponse: http::Response,
}
