#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
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
