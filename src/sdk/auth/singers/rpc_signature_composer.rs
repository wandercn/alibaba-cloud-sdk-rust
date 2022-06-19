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
            .base_as_mut()
            .QueryParams
            .remove("Signature")
            .unwrap();
    }
    let stringToSign = buildRpcStringToSign(request);
    request.SetStringToSign(stringToSign.as_str());
    let signature = signer.unwrap().Sign(&stringToSign, "&");
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
    let mut queryParams = request.GetQueryParams().to_owned();
    queryParams.insert("Version".to_owned(), request.GetVersion().to_owned());
    queryParams.insert("Action".to_owned(), request.GetActionName().to_owned());
    queryParams.insert("Format".to_owned(), request.GetAcceptFormat().to_owned());
    queryParams.insert("Timestamp".to_owned(), GetTimeInFormatISO8601());
    queryParams.insert("SignatureMethod".to_owned(), signer.GetName());
    queryParams.insert("SignatureType".to_owned(), signer.GetType());
    queryParams.insert("SignatureVersion".to_owned(), signer.GetVersion());
    queryParams.insert("SignatureNonce".to_owned(), GetUUID());
    queryParams.insert("AccessKeyId".to_owned(), signer.GetAccessKeyId()?);
    if !queryParams.contains_key("RegionId") {
        queryParams.insert("RegionId".to_owned(), regionId.to_owned());
    }
    let extraParam = signer.GetExtraParam();
    if let Some(param) = extraParam {
        for (k, v) in param.iter() {
            queryParams.insert(k.to_owned(), v.to_owned());
        }
    }
    request.addHeaderParam("Content-Type", requests::Form);

    let formString = GetUrlFormedMap(request.GetFormParams());
    request.SetContent(formString.as_bytes());
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
    urlencoder.Encode()
}
