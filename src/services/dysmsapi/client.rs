#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use log::debug;
use regex::Regex;
use std::{
    collections::HashMap,
    env::consts::ARCH,
    env::consts::OS,
    hash::Hash,
    io::{Error, ErrorKind},
};

use crate::sdk::client::Config;
use crate::sdk::requests;
use crate::sdk::responses;
use crate::sdk::{
    auth::credentials::AccessKeyCredential, requests::AcsRequest, responses::AcsResponse,
};
use crate::sdk::{auth::singers, endpoints};
use crate::sdk::{
    auth::singers::{Sign, Signer},
    requests::BaseRequestExt,
};
use gostd::net::http;
use gostd::strings;

use super::endpoint;
const Version: &str = "0.0.1";
const EndpointType: &str = "central";

pub type Client = crate::sdk::client::Client;
impl Client {
    pub fn NewClientWithAccessKey(
        regionId: &str,
        accessKeyId: &str,
        accessKeySecret: &str,
    ) -> Result<Client, Error> {
        let mut client = Client::default();
        client.InitWithAccessKey(regionId, accessKeyId, accessKeySecret)?;
        SetEndpointDataToClient(&mut client);
        Ok(client)
    }

    pub fn InitWithAccessKey(
        &mut self,
        regionId: &str,
        accessKeyId: &str,
        accessKeySecret: &str,
    ) -> Result<(), Error> {
        let config = self.InitClientConfig();
        let credential = AccessKeyCredential {
            AccessKeyId: accessKeyId.to_string(),
            AccessKeySecret: accessKeySecret.to_string(),
        };
        self.InitWithOptions(regionId, &config, credential)?;
        Ok(())
    }

    pub fn InitClientConfig(&mut self) -> Config {
        if self.config.is_some() {
            self.config.to_owned().unwrap()
        } else {
            NewConfig()
        }
    }

    pub fn InitWithOptions(
        &mut self,
        regionId: &str,
        config: &Config,
        credential: AccessKeyCredential,
    ) -> Result<(), Error> {
        let matched = Regex::new(r"^[a-zA-Z0-9_-]+$")
            .expect("Regex parse failed")
            .is_match(regionId);
        if !matched {
            return Err(Error::new(
                ErrorKind::Other,
                "regionId contains invalid characters",
            ));
        }
        self.regionId = regionId.to_string();
        self.config = Some(config.to_owned());
        self.httpClient = http::Client::New();
        self.signer = singers::NewSignerWithCredential(credential)?;
        Ok(())
    }
    // smd 短信用的老接口，没使用这个函数,暂时不实现
    pub fn ProcessCommonRequestWithSigner(request: http::Request) {
        todo!()
    }
    pub fn DoAction(
        &mut self,
        request: &mut requests::AcsRequest,
        response: &mut responses::AcsResponse,
    ) -> Result<(), Error> {
        self.DoActionWithSigner(request, response, None)?;
        Ok(())
    }

    pub fn DoActionWithSigner(
        &self,
        request: &mut AcsRequest,
        response: &mut AcsResponse,
        signer: Option<Box<dyn Signer>>,
    ) -> Result<(), Error> {
        if !self.Network.is_empty() {
            let matched = Regex::new(r"^[a-zA-Z0-9_-]+$")
                .expect("newwork Regex parse failed")
                .is_match(self.Network.as_str());
            if !matched {
                return Err(Error::new(
                    ErrorKind::Other,
                    "netWork contains invalid characters",
                ));
            }
        }
        if signer.is_none() {
            let mut httpRequest =
                self.buildRequestWithSigner(request, Some(Box::new(self.signer.to_owned())))?;
            let mut httpClient = http::Client::New();
            let httpResponse = httpClient.Do(&mut httpRequest)?;
            debug!("httpResponse1: {:?}\n", httpResponse);
            response.parseFromHttpResponse(&httpResponse);
        } else {
            let mut httpRequest = self.buildRequestWithSigner(request, signer)?;
            let mut httpClient = http::Client::New();
            let httpResponse = httpClient.Do(&mut httpRequest)?;
            debug!("httpResponse2: {:?}\n", httpResponse);
            response.parseFromHttpResponse(&httpResponse);
        }
        Ok(())
    }
    pub fn buildRequestWithSigner(
        &self,

        request: &mut AcsRequest,
        signer: Option<Box<dyn Signer>>,
    ) -> Result<http::Request, Error> {
        request.addHeaderParam("x-sdk-core-version", Version);
        let mut regionId = self.regionId.to_owned();
        if !request.GetRegionId().is_empty() {
            regionId = request.GetRegionId().to_owned();
        }
        let mut endpoint = request.GetDomain().to_string();
        if endpoint.is_empty() && !self.Domain.is_empty() {
            endpoint = self.Domain.as_str().to_owned()
        }
        if endpoint.is_empty() {
            endpoint = endpoints::GetEndpointFromMap(&regionId, request.GetProduct()).to_owned();
        }
        if endpoint.is_empty()
            && !self.EndpointType.is_empty()
            && (request.GetProduct() != "Sts" || request.GetQueryParams().is_empty())
        {
            if !self.EndpointMap.is_empty() && self.Network.is_empty() || self.Network == "public" {
                endpoint = self
                    .EndpointMap
                    .get(&regionId)
                    .map_or("".to_string(), |x| x.to_string())
            }

            if endpoint.is_empty() {
                endpoint = self.GetEndpointRules(&regionId, request.GetProduct())?;
            }
        }

        // if endpoint =="" {
        //     let resolveParam=
        // }
        request.SetDomain(endpoint.as_str());
        if request.GetScheme().is_empty() {
            request.SetScheme(
                self.config
                    .as_ref()
                    .expect("config is NONE")
                    .Scheme
                    .as_str(),
            );
        }
        // init request params

        let mut httpRequest: http::Request = buildHttpRequest(request, signer, &regionId)?;
        let DefaultUserAgent: String = format!(
            "AlibabaCloud ({}; {}) Rust/{} Core/{}",
            OS, ARCH, "rustc/1.60.0", Version
        );
        let userAgent = DefaultUserAgent;
        httpRequest.Header.Set("User-Agent", &userAgent);

        Ok(httpRequest)
    }

    pub fn GetEndpointRules(&self, regionId: &str, product: &str) -> Result<String, Error> {
        let mut endpointRaw: String = String::new();
        if self.EndpointType == "regional" {
            if regionId.is_empty() {
                return Err(Error::new(
                    ErrorKind::Other,
                    "RegionId is empty, please set a valid RegionId.",
                ));
            }
            endpointRaw = strings::Replace(
                "<product><network>.<region_id>.aliyuncs.com",
                "<region_id>",
                regionId,
                1,
            );
        } else {
            endpointRaw = "<product><network>.aliyuncs.com".to_string();
        };
        endpointRaw = strings::Replace(endpointRaw, "<product>", strings::ToLower(product), 1);
        if self.Network.is_empty() || self.Network == "public" {
            endpointRaw = strings::Replace(endpointRaw, "<network>", "", 1);
        } else {
            endpointRaw = strings::Replace(
                endpointRaw,
                "<network>",
                "-".to_owned() + self.Network.as_str(),
                1,
            )
        }
        Ok(endpointRaw)
    }
}

fn buildHttpRequest(
    request: &mut AcsRequest,
    singer: Option<Box<dyn Signer>>,
    regionId: &str,
) -> Result<http::Request, Error> {
    Sign(request, singer, regionId)?;
    let requestMethod = request.GetMethod();

    let requestUrl = request.BuildUrl();
    let body = request.GetBodyReader();
    let mut httpReqeust = http::Request::New(requestMethod, &requestUrl, Some(body.Bytes()))?;
    for (key, value) in request.GetHeaders() {
        httpReqeust.Header.Set(key, value);
    }
    debug!("httpRequest: {:?}\n", httpReqeust);
    debug!("Headers: {:?}\n", httpReqeust.Header);
    Ok(httpReqeust)
}
pub fn NewConfig() -> Config {
    Config {
        AutoRetry: false,
        MaxRetryTime: 3,
        UserAgent: "".to_string(),
        Debug: false,
        EnableAsync: false,
        MaxTaskQueueSize: 1000,
        GoRoutinePoolSize: 5,
        Scheme: "HTTP".to_string(),
    }
}

pub fn SetEndpointDataToClient(client: &mut Client) {
    client.EndpointMap = GetEndpointMap();
    client.EndpointType = GetEndpointType();
}

pub fn GetEndpointMap() -> HashMap<String, String> {
    let mut EndpointMap = HashMap::new();
    EndpointMap.insert(
        "cn-beijing".to_string(),
        "dysmsapi-proxy.cn-beijing.aliyuncs.com".to_string(),
    );
    EndpointMap.insert(
        "cn-hongkong".to_string(),
        "dysmsapi-xman.cn-hongkong.aliyuncs.com".to_string(),
    );
    EndpointMap.insert(
        "ap-southeast-1".to_string(),
        "dysmsapi.ap-southeast-1.aliyuncs.com".to_string(),
    );
    EndpointMap.insert(
        "ap-southeast-5".to_string(),
        "dysmsapi-xman.ap-southeast-5.aliyuncs.com".to_string(),
    );
    EndpointMap
}

pub fn GetEndpointType() -> String {
    EndpointType.to_string()
}

// hookDo  等价于 golang 的http.client.Do 方法只是改了个名字。
pub fn hookDo(
    f: fn(req: &http::Request) -> Result<http::Response, Error>,
) -> fn(req: &http::Request) -> Result<http::Response, Error> {
    f
}
