#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use regex::Regex;
use std::{
    collections::HashMap,
    env::consts::ARCH,
    env::consts::OS,
    hash::Hash,
    io::{Error, ErrorKind},
};

use crate::sdk::auth::singers::{Sign, Signer};
use crate::sdk::client::Config;
use crate::sdk::requests;
use crate::sdk::responses;
use crate::sdk::{
    auth::credentials::AccessKeyCredential, requests::AcsRequest, responses::AcsResponse,
};
use crate::sdk::{auth::singers, endpoints};
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
            return self.config.to_owned().unwrap();
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
        let matched = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap().is_match(regionId);
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
        request: requests::AcsRequest,
        response: &mut responses::AcsResponse,
    ) -> Result<(), Error> {
        self.DoActionWithSigner(request, response, None)?;
        Ok(())
    }

    pub fn DoActionWithSigner(
        &self,
        request: AcsRequest,
        response: &mut AcsResponse,
        signer: Option<Box<dyn Signer>>,
    ) -> Result<(), Error> {
        if self.Network != "" {
            let matched = Regex::new(r"^[a-zA-Z0-9_-]+$")
                .unwrap()
                .is_match(self.Network.as_str());
            if !matched {
                return Err(Error::new(
                    ErrorKind::Other,
                    "netWork contains invalid characters",
                ));
            }
        }
        let mut httpRequest = self.buildRequestWithSigner(request, signer)?;
        let mut httpClient = http::Client::New();
        let httpResponse = httpClient.Do(&mut httpRequest)?;
        Ok(())
    }
    pub fn buildRequestWithSigner(
        &self,

        mut request: AcsRequest,
        signer: Option<Box<dyn Signer>>,
    ) -> Result<http::Request, Error> {
        request
            .Headers
            .insert("x-sdk-core-version".to_owned(), Version.to_owned());
        let mut regionId = self.regionId.to_owned();
        if request.RegionId.len() > 0 {
            regionId = request.RegionId.to_owned();
        }
        let mut endpoint = request.Domain;
        if endpoint == "" && self.Domain != "" {
            endpoint = self.Domain.to_owned()
        }
        if endpoint == "" {
            endpoint = endpoints::GetEndpointFromMap(regionId.as_str(), request.product.as_str());
        }
        if endpoint == ""
            && self.EndpointType != ""
            && (request.product != "Sts" || request.QueryParams.len() == 0)
        {
            if !self.EndpointMap.is_empty() && self.Network == "" || self.Network == "public" {
                endpoint = self.EndpointMap.get(&regionId).unwrap().to_string();
            }

            if endpoint == "" {
                endpoint = self.GetEndpointRules(regionId.as_str(), request.product.as_str())?;
            }
        }

        // if endpoint =="" {
        //     let resolveParam=
        // }
        request.Domain = endpoint;
        if request.Scheme == "" {
            request.Scheme = self.config.as_ref().unwrap().Scheme.to_owned();
        }
        let mut httpRequest: http::Request = buildHttpRequest(request, signer, regionId.as_str())?;
        let DefaultUserAgent: String = format!(
            "AlibabaCloud ({}; {}) Rust/{} Core/{}",
            OS, ARCH, "rustc/1.60.0", Version
        );
        let userAgent = DefaultUserAgent.to_owned();
        httpRequest.Header.Set("User-Agent", &userAgent);
        Ok(httpRequest)
    }

    pub fn GetEndpointRules(&self, regionId: &str, product: &str) -> Result<String, Error> {
        let mut endpointRaw: String;
        if self.EndpointType == "regional" {
            if regionId == "" {
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
        }
        endpointRaw = strings::Replace(endpointRaw, "<product>", strings::ToLower(product), 1);
        if self.Network == "" || self.Network == "public" {
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
    mut request: AcsRequest,
    singer: Option<Box<dyn Signer>>,
    regionId: &str,
) -> Result<http::Request, Error> {
    Sign(&mut request, singer, regionId)?;
    let requestMethod = request.GetMethod();

    let requestUrl = request.BuildUrl();
    let body = request.GetBodyReader();
    let mut httpReqeust = http::Request::New(requestMethod, &requestUrl, Some(body.Bytes()))?;
    for (key, value) in request.Headers {
        httpReqeust.Header.Set(&key, &value);
    }
    Ok(httpReqeust)
}
pub fn NewConfig() -> Config {
    let mut config = Config::default();
    config.AutoRetry = false;
    config.MaxRetryTime = 3;
    config.UserAgent = "".to_string();
    config.Debug = false;
    config.EnableAsync = false;
    config.MaxTaskQueueSize = 1000;
    config.GoRoutinePoolSize = 5;
    config.Scheme = "HTTP".to_string();
    config
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
