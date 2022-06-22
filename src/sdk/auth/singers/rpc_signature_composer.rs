#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use super::Signer;
use crate::sdk::requests;
use crate::sdk::requests::BaseRequestExt;
use gostd::net::url;
use gostd::net::url::Values;
use gostd::strings;
use gostd::time;
use log::debug;
use std::{
    borrow::{Borrow, BorrowMut},
    collections::HashMap,
    io::Error,
};
pub fn signRpcRequest(
    request: &mut requests::AcsRequest,
    signer: Option<Box<dyn Signer>>,
    regionId: &str,
) -> Result<(), std::io::Error> {
    completeRpcSignParams(request, &signer, regionId)?;

    if request.GetQueryParams().contains_key("Signature") {
        request
            .QueryParams_as_mut()
            .remove("Signature")
            .expect("remove Signature failed ");
    }
    let stringToSign = buildRpcStringToSign(request);
    request.SetStringToSign(stringToSign.as_str());
    let signature = signer.expect("signer is NONE").Sign(&stringToSign, "&");
    request.addQueryParam("Signature", signature.as_str());
    Ok(())
}

fn buildRpcStringToSign(request: &mut requests::RpcRequest) -> String {
    let mut signParams = HashMap::new();

    for (key, value) in request.GetQueryParams() {
        signParams.insert(key.to_owned(), value.to_owned());
    }

    for (key, value) in request.GetFormParams() {
        signParams.insert(key.to_owned(), value.to_owned());
    }

    let mut stringToSign = GetUrlFormedMap(&signParams);
    stringToSign = strings::Replace(stringToSign, "+", "%20", -1);
    stringToSign = strings::Replace(stringToSign, "*", "%2A", -1);
    stringToSign = strings::Replace(stringToSign, "%7E", "~", -1);
    stringToSign = url::QueryEscape(&stringToSign);
    stringToSign = request.GetMethod().String().to_owned() + "&%2F&" + &stringToSign;
    stringToSign
}

fn completeRpcSignParams(
    request: &mut requests::AcsRequest,
    signer: &Option<Box<dyn Signer>>,
    regionId: &str,
) -> Result<(), Error> {
    let signer = signer.as_ref().expect("signer is None");
    let version = request.GetVersion().to_string();
    let action = request.GetActionName().to_string();
    let formart = request.GetAcceptFormat().to_string();
    request.addQueryParam("Version", &version);
    request.addQueryParam("Action", &action);
    request.addQueryParam("Format", &formart);
    request.addQueryParam("Timestamp", &GetTimeInFormatISO8601());
    request.addQueryParam("SignatureMethod", &signer.GetName());
    request.addQueryParam("SignatureType", &signer.GetType());
    request.addQueryParam("SignatureVersion", &signer.GetVersion());
    request.addQueryParam("SignatureNonce", &GetUUID());
    request.addQueryParam("AccessKeyId", &signer.GetAccessKeyId()?);

    if request.GetQueryParams().contains_key("RegionId") {
        request.addQueryParam("RegionId", &regionId);
    }

    if let Some(param) = signer.GetExtraParam() {
        param.iter().for_each(|(k, v)| request.addQueryParam(k, v));
    }
    request.addHeaderParam("Content-Type", requests::Form);

    let formString = GetUrlFormedMap(request.GetFormParams());
    request.SetContent(formString.as_bytes());
    debug!("queryParams: {:?}", request.GetQueryParams());
    Ok(())
}

pub fn GetUUID() -> String {
    uuid::Uuid::new_v4().to_string()
}
pub fn GetTimeInFormatISO8601() -> String {
    let gmt = time::FixedZone("GMT", 0);
    time::Now().In(gmt).Format("2006-01-02T15:04:05Z")
}

pub fn GetUrlFormedMap(source: &HashMap<String, String>) -> String {
    let mut map = HashMap::<String, Vec<String>>::new();
    for (k, v) in source.iter() {
        let mut vals: Vec<String> = vec![v.to_owned()];
        map.insert(k.to_owned(), vals.to_owned());
    }
    let urlencoder = Values::new(map);
    debug!("urlencoder: {}\n", urlencoder.Encode());
    urlencoder.Encode()
}
