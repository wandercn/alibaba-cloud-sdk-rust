#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use crate::sdk::auth::credentials;
use crate::sdk::auth::singers;
use crate::sdk::requests;
use std::collections::HashMap;
use std::io::Error;
pub trait Signer {
    fn GetName(&self) -> String;
    fn GetType(&self) -> String;
    fn GetVersion(&self) -> String;
    fn GetAccessKeyId(&self) -> Result<String, Error>;
    fn GetExtraParam(&self) -> Option<HashMap<String, String>>;
    fn Sign(&self, stringToSign: &str, secretSuffix: &str) -> String;
}

pub fn NewSignerWithCredential(
    credential: credentials::AccessKeyCredential,
) -> Result<singers::AccessKeySigner, Error> {
    Ok(singers::AccessKeySigner::NewAccessKeySigner(credential))
}

pub fn Sign(
    request: &mut requests::AcsRequest,
    signer: Option<Box<dyn Signer>>,
    regionId: &str,
) -> Result<(), Error> {
    super::signRpcRequest(request, signer, regionId)
}
