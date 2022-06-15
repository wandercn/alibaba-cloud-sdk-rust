#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use base64;
use ring::hmac;
pub fn ShaHmac1(source: &str, secret: &str) -> String {
    let key = hmac::Key::new(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, secret.as_bytes());
    let signedBytes = hmac::sign(&key, source.as_bytes());
    base64::encode(signedBytes)
}
