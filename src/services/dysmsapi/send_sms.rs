#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use super::Client;
use serde::{Deserialize, Serialize};
use std::io::Error;
impl Client {
    /// # Example
    /// ```
    /// use alibaba_cloud_sdk_rust::services::dysmsapi;
    /// const AliyunSmsServerRegion: &str = "cn-hangzhou";
    /// const AliyunSmsAccessKeyID: &str = "LTAI4FwqPxiA111111111";
    /// const AliyunSmsAccessKeySecret: &str = "ESX1wX11111FJqHTTLwDU2222cP1";
    /// const AliyunSmsReportTempleateCode: &str = "SMS_900699011"; // 短信通知模版
    /// const AliyunSmsSignName: &str = "阿里云"; // 短信署名
    /// fn main()-> Result<(), std::io::Error> {
    ///     let phoneNumber="139xxxxxxxx" //手机号
    ///     let mut client = dysmsapi::Client::NewClientWithAccessKey(
    ///         AliyunSmsServerRegion,
    ///         AliyunSmsAccessKeyID,
    ///         AliyunSmsAccessKeySecret,
    ///     )?;
    ///     let mut request = dysmsapi::CreateSendSmsRequest();
    ///     request.rpcRequest.Scheme = "https".to_owned();
    ///     request.PhoneNumbers = strings::Replace(phoneNumber, "+86", "", -1);
    ///     request.SignName = AliyunSmsSignName.to_owned();
    ///     request.TemplateCode = AliyunSmsReportTempleateCode.to_owned();
    ///     let response = client.SendSms(&mut request)?;
    ///     println!("{:?}", &response);
    ///     Ok(())
    /// }
    /// ```
    pub fn SendSms(&mut self, request: &mut SendSmsRequest) -> Result<SendSmsResponse, Error> {
        let mut response = CreateSendSmsResponse();
        request
            .rpcRequest
            .QueryParams
            .insert("SignName".to_owned(), request.SignName.to_owned());
        request
            .rpcRequest
            .QueryParams
            .insert("PhoneNumbers".to_owned(), request.PhoneNumbers.to_owned());
        request
            .rpcRequest
            .QueryParams
            .insert("TemplateCode".to_owned(), request.TemplateCode.to_owned());
        self.DoAction(&mut request.rpcRequest, &mut response.baseResponse)?;
        response = serde_json::from_slice(&response.baseResponse.httpContentBytes)?;
        Ok(response)
    }
}

use crate::sdk::requests;
use crate::sdk::responses;
#[derive(Default, Debug)]
pub struct SendSmsRequest {
    pub rpcRequest: requests::RpcRequest,
    pub ResourceOwnerId: requests::Integer, //`position:"Query" name:"ResourceOwnerId"`
    pub SmsUpExtendCode: String,            //`position:"Query" name:"SmsUpExtendCode"`
    pub SignName: String,                   //`position:"Query" name:"SignName"`
    pub ResourceOwnerAccount: String,       //`position:"Query" name:"ResourceOwnerAccount"`
    pub PhoneNumbers: String,               //`position:"Query" name:"PhoneNumbers"`
    pub OwnerId: requests::Integer,         //`position:"Query" name:"OwnerId"`
    pub OutId: String,                      //`position:"Query" name:"OutId"`
    pub TemplateCode: String,               //`position:"Query" name:"TemplateCode"`
    pub TemplateParam: String,              //`position:"Query" name:"TemplateParam"`
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SendSmsResponse {
    #[serde(skip)]
    baseResponse: responses::BaseResponse,
    pub RequestId: String, //`json:"RequestId" xml:"RequestId"`
    pub BizId: String,     //`json:"BizId" xml:"BizId"`
    pub Code: String,      //`json:"Code" xml:"Code"`
    pub Message: String,   //`json:"Message" xml:"Message"`
}

pub fn CreateSendSmsRequest() -> SendSmsRequest {
    let mut request = SendSmsRequest::default();
    request
        .rpcRequest
        .InitWithApiInfo("Dysmsapi", "2017-05-25", "SendSms", "", "");
    request.rpcRequest.Method = requests::POST.to_string();
    request
}

pub fn CreateSendSmsResponse() -> SendSmsResponse {
    let mut response = SendSmsResponse::default();
    response
}
