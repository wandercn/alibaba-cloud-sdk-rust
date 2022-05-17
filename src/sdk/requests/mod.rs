#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
mod types;
pub use types::*;
pub const RPC: &str = "RPC";
pub const ROA: &str = "ROA";

pub const HTTP: &str = "HTTP";
pub const HTTPS: &str = "HTTPS";

pub const DefaultHttpPort: &str = "80";

pub const GET: &str = "GET";
pub const PUT: &str = "PUT";
pub const POST: &str = "POST";
pub const DELETE: &str = "DELETE";
pub const PATCH: &str = "PATCH";
pub const HEAD: &str = "HEAD";
pub const OPTIONS: &str = "OPTIONS";

pub const Json: &str = "application/json";
pub const Xml: &str = "application/xml";
pub const Raw: &str = "application/octet-stream";
pub const Form: &str = "application/x-www-form-urlencoded";

pub const Header: &str = "Header";
pub const Query: &str = "Query";
pub const Body: &str = "Body";
pub const Path: &str = "Path";

pub const HeaderSeparator: &str = "\n";

use std::collections::HashMap;
pub struct AcsRequest {}

pub struct CommonRequest {
    pub base: BaseRequest,
    pub Version: String,
    pub ApiName: String,
    pub Product: String,
    pub ServiceCode: String,
    pub EndpointType: String,

    // roa params
    pub PathPattern: String,
    pub PathParams: HashMap<String, String>,

    pub Ontology: AcsRequest,
}
#[derive(Default)]
pub struct BaseRequest {
    pub Scheme: String,
    pub Method: String,
    pub Domain: String,
    pub Port: String,
    pub RegionId: String,
    // ReadTimeout    time.Duration
    // ConnectTimeout time.Duration
    pub isInsecure: bool,

    pub userAgent: HashMap<String, String>,
    pub product: String,
    pub version: String,

    pub actionName: String,

    pub AcceptFormat: String,

    pub QueryParams: HashMap<String, String>,
    pub Headers: HashMap<String, String>,
    pub FormParams: HashMap<String, String>,
    pub Content: Vec<u8>,

    pub locationServiceCode: String,
    pub locationEndpointType: String,

    pub queries: String,

    pub stringToSign: String,
}

impl BaseRequest {
    pub fn defaultBaseRequest() -> Self {
        let mut base = BaseRequest::default();

        base.Scheme = "".to_owned();
        base.AcceptFormat = "JSON".to_owned();
        base.Method = GET.to_owned();
        base.Headers = HashMap::from([
            ("x-sdk-client".to_owned(), "golang/1.0.0".to_owned()),
            ("x-sdk-invoke-type".to_owned(), "normal".to_owned()),
            ("Accept-Encoding".to_owned(), "identity".to_owned()),
        ]);

        base
    }
}

pub type RpcRequest = BaseRequest;

impl RpcRequest {
    pub fn init(&mut self) {
        let mut base_reqeust = BaseRequest::defaultBaseRequest();
        self.Scheme = base_reqeust.Scheme;
        self.AcceptFormat = base_reqeust.AcceptFormat;
        self.Headers = base_reqeust.Headers;
        self.Method = POST.to_string();
    }
    pub fn InitWithApiInfo(
        &mut self,
        product: &str,
        version: &str,
        action: &str,
        serviceCode: &str,
        endpointType: &str,
    ) {
        self.init();
        self.product = product.to_owned();
        self.version = version.to_owned();
        self.actionName = action.to_owned();
        self.locationServiceCode = serviceCode.to_owned();
        self.locationEndpointType = endpointType.to_owned();
        self.Headers
            .insert("x-acs-version".to_owned(), version.to_owned());
        self.Headers
            .insert("x-acs-action".to_owned(), action.to_owned());
    }
}
