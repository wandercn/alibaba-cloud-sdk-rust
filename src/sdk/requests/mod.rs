#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
mod types;
use gostd::io::StringWriter;
use gostd::strings::Builder;
use regex::Replacer;
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

use gostd::io;
use gostd::net::http::Method;
use gostd::strings;
use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;

use super::auth::singers::GetUrlFormedMap;
pub type AcsRequest = RpcRequest;

pub trait BaseRequestExt {
    fn base(&self) -> &BaseRequest;

    fn base_as_mut(&mut self) -> &mut BaseRequest;

    fn GetQueryParams(&self) -> &HashMap<String, String> {
        self.base().QueryParams.borrow()
    }

    fn GetFormParams(&self) -> &HashMap<String, String> {
        self.base().FormParams.borrow()
    }

    fn GetHTTPSInsecure(&self) -> bool {
        self.base().isInsecure
    }

    fn SetHTTPSInsecure(&mut self, isInsecure: bool) {
        self.base_as_mut().isInsecure = isInsecure
    }

    fn GetContent(&self) -> &[u8] {
        self.base().Content.borrow()
    }

    fn SetContent(&mut self, content: &[u8]) {
        self.base_as_mut().Content = content.to_owned()
    }

    fn SetVersion(&mut self, version: &str) {
        self.base_as_mut().version = version.to_string();
    }

    fn GetVersion(&self) -> &str {
        self.base().version.borrow()
    }

    fn GetActionName(&self) -> &str {
        self.base().actionName.borrow()
    }

    fn SetActionName(&mut self, actionName: &str) {
        self.base_as_mut().actionName = actionName.to_string();
    }

    fn GetUserAgent(&self) -> &HashMap<String, String> {
        self.base().userAgent.borrow()
    }

    fn AppendUserAgent(&mut self, key: &str, value: &str) {
        let mut newKey = true;
        if self.base_as_mut().userAgent.is_empty() {
            self.base_as_mut().userAgent = HashMap::new();
        }
        if strings::ToLower(key).as_str() != "core" && strings::ToLower(key) != "rust" {
            for (tag, mut v) in self.base_as_mut().userAgent.iter_mut() {
                if tag == key {
                    *v = value.to_string();
                    newKey = false;
                }
            }
            if newKey {
                self.base_as_mut()
                    .userAgent
                    .insert(key.to_string(), value.to_string());
            }
        }
    }

    fn addHeaderParam(&mut self, key: &str, value: &str) {
        self.base_as_mut()
            .Headers
            .insert(key.to_string(), value.to_string());
    }

    fn addQueryParam(&mut self, key: &str, value: &str) {
        self.base_as_mut()
            .QueryParams
            .insert(key.to_string(), value.to_string());
    }

    fn addFormParam(&mut self, key: &str, value: &str) {
        self.base_as_mut()
            .FormParams
            .insert(key.to_string(), value.to_string());
    }

    fn GetAcceptFormat(&self) -> &str {
        self.base().AcceptFormat.borrow()
    }

    fn GetLocationServiceCode(&self) -> &str {
        self.base().locationServiceCode.borrow()
    }

    fn SetLocationServiceCode(&mut self, locationServiceCode: &str) {
        self.base_as_mut().locationServiceCode = locationServiceCode.to_string();
    }

    fn GetLocationEndpointType(&self) -> &str {
        self.base().locationEndpointType.borrow()
    }

    fn SetLocationEndpointType(&mut self, locationEndpointType: &str) {
        self.base_as_mut().locationEndpointType = locationEndpointType.to_string();
    }

    fn GetProduct(&self) -> &str {
        self.base().product.borrow()
    }

    fn SetProduct(&mut self, product: &str) {
        self.base_as_mut().product = product.to_string();
    }

    fn GetScheme(&self) -> &str {
        self.base().Scheme.borrow()
    }

    fn SetScheme(&mut self, scheme: &str) {
        self.base_as_mut().Scheme = scheme.to_string()
    }

    fn GetMethod(&self) -> &str {
        self.base().Method.borrow()
    }

    fn SetMethod(&mut self, method: &str) {
        self.base_as_mut().Method = method.to_string()
    }

    fn GetDomain(&self) -> &str {
        self.base().Domain.borrow()
    }

    fn SetDomain(&mut self, host: &str) {
        self.base_as_mut().Domain = host.to_string()
    }

    fn GetPort(&self) -> &str {
        self.base().Port.borrow()
    }

    fn GetRegionId(&self) -> &str {
        self.base().RegionId.borrow()
    }

    fn GetHeaders(&self) -> &HashMap<String, String> {
        self.base().Headers.borrow()
    }

    fn SetContentType(&mut self, contentType: &str) {
        self.addHeaderParam("Content-Type", contentType)
    }

    fn GetContentType(&self) -> Option<&str> {
        self.base().Headers.get("Content-Type").map(|s| s.as_str())
    }

    fn SetStringToSign(&mut self, stringToSign: &str) {
        self.base_as_mut().stringToSign = stringToSign.to_string()
    }

    fn GetStringToSign(&self) -> &str {
        self.base().stringToSign.borrow()
    }
}
pub struct CommonRequest {
    base: BaseRequest,
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

impl BaseRequestExt for CommonRequest {
    fn base(&self) -> &BaseRequest {
        self.base.borrow()
    }

    fn base_as_mut(&mut self) -> &mut BaseRequest {
        self.base.borrow_mut()
    }
}

#[derive(Default, Debug)]
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
        Self {
            Scheme: "".to_owned(),
            AcceptFormat: "JSON".to_owned(),
            Method: GET.to_owned(),
            Headers: HashMap::from([
                ("x-sdk-client".to_owned(), "rust-lang/1.0.0".to_owned()),
                ("x-sdk-invoke-type".to_owned(), "normal".to_owned()),
                ("Accept-Encoding".to_owned(), "identity".to_owned()),
            ]),
            ..Default::default()
        }
    }
}

impl BaseRequestExt for BaseRequest {
    fn base(&self) -> &BaseRequest {
        self.borrow()
    }

    fn base_as_mut(&mut self) -> &mut BaseRequest {
        self.borrow_mut()
    }
}
#[derive(Default, Debug)]
pub struct RpcRequest {
    base: BaseRequest,
}

impl BaseRequestExt for RpcRequest {
    fn base(&self) -> &BaseRequest {
        self.base.borrow()
    }

    fn base_as_mut(&mut self) -> &mut BaseRequest {
        self.base.borrow_mut()
    }
}

impl RpcRequest {
    pub fn init(&mut self) {
        let mut base_reqeust = BaseRequest::defaultBaseRequest();

        base_reqeust.SetMethod(POST);
        self.base = base_reqeust;
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
        self.SetProduct(product);
        self.SetVersion(version);
        self.SetActionName(action);
        self.SetLocationServiceCode(serviceCode);
        self.SetLocationEndpointType(endpointType);
        self.addHeaderParam("x-acs-version", version);
        self.addHeaderParam("x-acs-action", action);
    }

    pub fn GetStyle(&self) -> String {
        RPC.to_string()
    }
    pub fn GetMethod(&self) -> Method {
        match self.base.Method.as_str() {
            GET => Method::Get,
            PUT => Method::Put,
            POST => Method::Post,
            DELETE => Method::Delete,
            PATCH => Method::Patch,
            HEAD => Method::Head,
            OPTIONS => Method::Options,
            _ => Method::Get,
        }
    }

    pub fn BuildUrl(&mut self) -> String {
        let mut url = format!(
            "{}://{}",
            strings::ToLower(&self.base.Scheme),
            self.base.Domain
        );
        if !self.base.Port.is_empty() {
            url = format!("{}:{}", url, self.base.Port);
        }
        url.push_str(self.BuildQueries().as_str());
        url
    }
    pub fn BuildQueries(&mut self) -> String {
        self.base.queries = "/?".to_owned() + GetUrlFormedMap(&self.base.QueryParams).as_str();
        self.base.queries.to_owned()
    }
    pub fn GetBodyReader(&self) -> Builder {
        let mut buf = strings::Builder::new();
        if self.base.FormParams.is_empty() && !self.base.FormParams.is_empty() {
            let formString = GetUrlFormedMap(&self.base.FormParams);

            buf.WriteString(&formString);
            buf
        } else {
            buf.WriteString("");
            buf
        }
    }
}
