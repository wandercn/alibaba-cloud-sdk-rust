# [alibaba-cloud-sdk-rust](https://github.com/wandercn/alibaba-cloud-sdk-rust)

[![crates.io](https://img.shields.io/crates/v/alibaba-cloud-sdk-rust.svg?color=yellow)](https://crates.io/crates/alibaba-cloud-sdk-rust)
[![Released API docs](https://docs.rs/alibaba-cloud-sdk-rust/badge.svg)](https://docs.rs/alibaba-cloud-sdk-rust)
[![GPL3 licensed](https://img.shields.io/github/license/wandercn/alibaba-cloud-sdk-rust.svg)](./LICENSE)
[![Downloads of Crates.io](https://img.shields.io/crates/d/alibaba-cloud-sdk-rust.svg)](https://crates.io/crates/alibaba-cloud-sdk-rust)
[![Lines of code](https://img.shields.io/tokei/lines/github/wandercn/alibaba-cloud-sdk-rust.svg)](#)
[![Build](https://img.shields.io/github/actions/workflow/status/wandercn/alibaba-cloud-sdk-rust/.github/workflows/rust.yml?branch=master)](#)
[![Languages](https://img.shields.io/github/languages/top/wandercn/alibaba-cloud-sdk-rust.svg)](#)

阿里云短信rust sdk


## Example
```rust
use alibaba_cloud_sdk_rust::services::dysmsapi;
use gostd::strings;
use anyhow::Result;

const AliyunSmsServerRegion: &str = "cn-hangzhou";
const AliyunSmsAccessKeyID: &str = "LTAI4FwqPxiAxxxxxx";
const AliyunSmsAccessKeySecret: &str = "xxxxx0FJqHTTLwDUuhxxxxx";
const AliyunSmsReportTempleateCode: &str = "SMS_226xxxx"; // 通知模版
const AliyunSmsSignName: &str = "阿里云"; // 短信署名

fn main()->(){
    let phoneNumber="1391212xxxx";
    match  SendSMS(phoneNumber) {
        Ok(response)=> println!("{:?}",response),
        Err(err)=>println!("{}",err),
    }
}

fn SendSMS(phoneNumber: &str) -> Result<()> {
    let mut client = dysmsapi::Client::NewClientWithAccessKey(
        AliyunSmsServerRegion,
        AliyunSmsAccessKeyID,
        AliyunSmsAccessKeySecret,
    )?;
    let mut request = dysmsapi::CreateSendSmsRequest();
    request.PhoneNumbers = strings::Replace(phoneNumber, "+86", "", -1);
    request.SignName = AliyunSmsSignName.to_owned();
    request.TemplateCode = AliyunSmsReportTempleateCode.to_owned();
    let response = client.SendSms(&mut request)?;
    println!("{:?}", &response);

    Ok(())
}
```
## 捐赠方式

If you like my open source project and would like to support me, you can donate through the following methods:
- **Dogecoin address** `DHbDfZWTYjpiGctAyYy1F9YzViTfVRW6aY`
- **AliPay:** `limiao2008@gmail.com`
- **ETH address:** `0x74682cbE11De154E38D8B220ba177c28481F41a8`
- **PayPal:** `paypal.me/wandercn`

Thank you for your support!