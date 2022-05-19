#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use super::Client;
use std::io::Error;

impl Client {
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

#[derive(Default, Debug)]
pub struct SendSmsResponse {
    pub baseResponse: responses::BaseResponse,
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
