#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use gostd::net::http;
use std::collections::HashMap;
use std::io::Error;
use std::io::ErrorKind;
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
    pub fn parseFromHttpResponse(&mut self, httpResponse: &http::Response) -> Result<(), Error> {
        if let Some(bytesBody) = &httpResponse.Body {
            self.httpStatus = httpResponse.StatusCode as i32;
            self.httpContentBytes = bytesBody.to_owned();
            self.httpContentString =
                String::from_utf8(bytesBody.to_owned()).unwrap_or("".to_string());
            self.originHttpResponse = httpResponse.to_owned();
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "http response body is NONE"))
        }
    }
}
