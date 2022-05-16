#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
// use crate::sdk::auth::singers::ShaHmac1;
use super::ShaHmac1;
use super::Signer;
use crate::sdk::auth::credentials;
use std::collections::HashMap;
use std::io::Error;
#[derive(Debug, Default, Clone)]
pub struct AccessKeySigner {
    credential: credentials::AccessKeyCredential,
}

impl AccessKeySigner {
    pub fn NewAccessKeySigner(credential: credentials::AccessKeyCredential) -> Self {
        Self { credential }
    }
}
impl Signer for AccessKeySigner {
    fn GetName(&self) -> String {
        "HMAC-SHA1".to_string()
    }
    fn GetType(&self) -> String {
        "".to_string()
    }
    fn GetVersion(&self) -> String {
        "1.0".to_string()
    }
    fn GetAccessKeyId(&self) -> Result<String, Error> {
        Ok(self.credential.AccessKeyId.to_string())
    }

    fn Sign(&self, stringToSign: &str, secretSuffix: &str) -> String {
        let secret = self.credential.AccessKeySecret.to_owned() + secretSuffix;
        ShaHmac1(stringToSign, &secret)
    }
    fn GetExtraParam(&self) -> Option<HashMap<String, String>> {
        None
    }
}
