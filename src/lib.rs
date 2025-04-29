//! 阿里云短信sdk
//!  # Example
//! ```not run
//! use alibaba_cloud_sdk_rust::services::dysmsapi;
//! use gostd::strings;
//! const AliyunSmsServerRegion: &str = "cn-hangzhou";
//! const AliyunSmsAccessKeyID: &str = "LTAI4FwqPxiA111111111";
//! const AliyunSmsAccessKeySecret: &str = "ESX1wX11111FJqHTTLwDU2222cP1";
//! const AliyunSmsReportTempleateCode: &str = "SMS_900699011"; // 短信通知模版
//! const AliyunSmsSignName: &str = "阿里云"; // 短信署名
//! fn main()-> Result<(), std::io::Error> {
//!     let phoneNumber="139xxxxxxxx" ;//手机号
//!     let mut client = dysmsapi::Client::NewClientWithAccessKey(
//!         AliyunSmsServerRegion,
//!         AliyunSmsAccessKeyID,
//!         AliyunSmsAccessKeySecret,
//!     )?;
//!     let mut request = dysmsapi::CreateSendSmsRequest();
//!     request.PhoneNumbers = strings::Replace(phoneNumber, "+86", "", -1);
//!     request.SignName = AliyunSmsSignName.to_owned();
//!     request.TemplateCode = AliyunSmsReportTempleateCode.to_owned();
//!     let response = client.SendSms(&mut request)?;
//!     println!("{:?}", &response);
//!     Ok(())
//! }
//! ```
//! # Output
//! ```text
//! {"RequestId":"A25164FD-44BD-5D99-85A3-FA51F0C86164","Message":"OK","BizId":"158003852956472458^0","Code":"OK"}
//! ```
pub mod error;
pub mod sdk;
pub mod services;
