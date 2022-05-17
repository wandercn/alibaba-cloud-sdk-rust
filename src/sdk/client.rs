#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::sdk::auth::singers::AccessKeySigner;
use crate::sdk::auth::singers::Signer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Error;
const Version: &str = "0.0.1";

#[derive(Default)]
pub struct Client {
    pub SourceIp: String,
    pub SecureTransport: String,
    pub isInsecure: bool,
    pub regionId: String,
    pub config: Option<Config>,
    pub httpProxy: String,
    pub httpsProxy: String,
    pub noProxy: String,
    // logger          *Logger
    pub userAgent: HashMap<String, String>,
    pub signer: AccessKeySigner,

    pub httpClient: gostd::net::http::Client,
    // asyncTaskQueue  chan func()
    // readTimeout     time.Duration
    // connectTimeout  time.Duration
    pub EndpointMap: HashMap<String, String>,
    pub EndpointType: String,
    pub Network: String,
    pub Domain: String,
    pub isOpenAsync: bool,
}

#[derive(Debug, Default, Clone)]
pub struct Config {
    pub AutoRetry: bool,        // `default:"false"`
    pub MaxRetryTime: i32,      //`default:"3"`
    pub UserAgent: String,      //`default:""`
    pub Debug: bool,            //`default:"false"`
    pub EnableAsync: bool,      //`default:"false"`
    pub MaxTaskQueueSize: i32,  //`default:"1000"`
    pub GoRoutinePoolSize: i32, //`default:"5"`
    pub Scheme: String,         //`default:"HTTP"`
                                // HttpTransport     *http.Transport   //`default:""`
                                // Transport         http.RoundTripper //`default:""`
                                // Timeout           :time.Duration
}
