#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use super::Client;
use std::io::Error;

impl Client {
    pub fn SendSms(&self, request: &SendSmsRequest) -> Result<SendSmsRequest, Error> {
        let response = CreateSendSmsResponse();

        todo!()
    }
}

use crate::sdk::requests;
use crate::sdk::responses;
#[derive(Default)]
pub struct SendSmsRequest {
    rpcRequest: requests::RpcRequest,
    ResourceOwnerId: requests::Integer, //`position:"Query" name:"ResourceOwnerId"`
    SmsUpExtendCode: String,            //`position:"Query" name:"SmsUpExtendCode"`
    SignName: String,                   //`position:"Query" name:"SignName"`
    ResourceOwnerAccount: String,       //`position:"Query" name:"ResourceOwnerAccount"`
    PhoneNumbers: String,               //`position:"Query" name:"PhoneNumbers"`
    OwnerId: requests::Integer,         //`position:"Query" name:"OwnerId"`
    OutId: String,                      //`position:"Query" name:"OutId"`
    TemplateCode: String,               //`position:"Query" name:"TemplateCode"`
    TemplateParam: String,              //`position:"Query" name:"TemplateParam"`
}

#[derive(Default)]
pub struct SendSmsResponse {
    baseResponse: responses::BaseResponse,
    RequestId: String, //`json:"RequestId" xml:"RequestId"`
    BizId: String,     //`json:"BizId" xml:"BizId"`
    Code: String,      //`json:"Code" xml:"Code"`
    Message: String,   //`json:"Message" xml:"Message"`
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
    let response = SendSmsResponse::default();
    response
}
