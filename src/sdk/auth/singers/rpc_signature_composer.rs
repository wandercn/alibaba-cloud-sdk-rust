#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use super::Signer;
use crate::sdk::requests;
use gostd::net::url::Values;
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
    completeRpcSignParams(request, signer, regionId)?;

    if request.QueryParams.contains_key("Signature") {
        request
            .borrow_mut()
            .QueryParams
            .remove("Signature")
            .unwrap();
    }
    todo!()
}

fn completeRpcSignParams(
    request: &mut requests::AcsRequest,
    signer: Option<Box<dyn Signer>>,
    regionId: &str,
) -> Result<(), Error> {
    let signer = signer.as_ref().expect("signer is None");
    let mut queryParams = request.QueryParams.to_owned();
    queryParams.insert("Version".to_owned(), request.version.to_owned());
    queryParams.insert("Action".to_owned(), request.actionName.to_owned());
    queryParams.insert("Format".to_owned(), request.AcceptFormat.to_owned());
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
    if extraParam.is_some() {
        for (k, v) in extraParam.unwrap().iter() {
            queryParams.insert(k.to_owned(), v.to_owned());
        }
    }
    request
        .Headers
        .insert("Content-Type".to_owned(), requests::Form.to_owned());

    let formString = GetUrlFormedMap(&mut request.FormParams);
    request.Content = formString.as_bytes().to_vec();
    Ok(())
}

pub fn GetUUID() -> String {
    uuid::Uuid::new_v4().to_string()
}
pub fn GetTimeInFormatISO8601() -> String {
    let gmt = time::FixedZone("GMT", 0);
    time::Now().In(gmt).Format("2006-01-02T15:04:05Z")
}

pub fn GetUrlFormedMap(source: &mut HashMap<String, String>) -> String {
    let mut map = HashMap::<String, Vec<String>>::new();
    for (k, v) in source.iter() {
        let mut vals: Vec<String> = Vec::new();
        vals.push(v.to_owned());
        map.insert(k.to_owned(), vals.to_owned());
    }
    let urlencoder = Values::new(map);
    urlencoder.Encode()
}
