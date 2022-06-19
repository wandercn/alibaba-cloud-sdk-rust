#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use super::Client;
use serde::{Deserialize, Serialize};
use std::io::Error;
impl Client {
    pub fn SendSms(&mut self, request: &mut SendSmsRequest) -> Result<SendSmsResponse, Error> {
        let mut response = CreateSendSmsResponse();
        request.BuildQueryParams();
        let mut baseResponse = responses::BaseResponse::default();
        self.DoAction(&mut request.rpcRequest, &mut baseResponse)?;
        response = serde_json::from_slice(&baseResponse.httpContentBytes)?;
        Ok(response)
    }
}

use crate::sdk::requests::BaseRequestExt;
use crate::sdk::requests::{self, BaseRequest};
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
    // baseResponse: responses::BaseResponse,
    pub RequestId: String, //`json:"RequestId" xml:"RequestId"`
    pub BizId: String,     //`json:"BizId" xml:"BizId"`
    pub Code: String,      //`json:"Code" xml:"Code"`
    pub Message: String,   //`json:"Message" xml:"Message"`
}

impl BaseRequestExt for SendSmsRequest {
    fn base(&self) -> &BaseRequest {
        self.rpcRequest.base()
    }

    fn base_as_mut(&mut self) -> &mut BaseRequest {
        self.rpcRequest.base_as_mut()
    }
}

impl SendSmsRequest {
    pub fn BuildQueryParams(&mut self) {
        self.addQueryParam("SignName", &self.SignName.to_owned());
        self.addQueryParam("PhoneNumbers", &self.PhoneNumbers.to_owned());
        self.addQueryParam("TemplateCode", &self.TemplateCode.to_owned());
        self.addQueryParam("ResourceOwnerId", &self.ResourceOwnerId.to_owned());
        self.addQueryParam("SmsUpExtendCode", &self.SmsUpExtendCode.to_owned());
        self.addQueryParam(
            "ResourceOwnerAccount",
            &self.ResourceOwnerAccount.to_owned(),
        );
        self.addQueryParam("OwnerId", &self.OwnerId.to_owned());
        self.addQueryParam("OutId", &self.OutId.to_owned());
        self.addQueryParam("TemplateParam", &self.TemplateParam.to_owned());
    }
}

pub fn CreateSendSmsRequest() -> SendSmsRequest {
    let mut request = SendSmsRequest::default();
    request
        .rpcRequest
        .InitWithApiInfo("Dysmsapi", "2017-05-25", "SendSms", "", "");
    request.SetMethod(requests::POST);
    request
}

pub fn CreateSendSmsResponse() -> SendSmsResponse {
    SendSmsResponse::default()
}
