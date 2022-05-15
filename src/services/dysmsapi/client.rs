#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::{io::{Error,ErrorKind}, collections::HashMap, hash::Hash};
use regex::Regex;

use crate::sdk::auth::credentials::AccessKeyCredential;
type Client = crate::sdk::client::Client;
use crate::sdk::client::Config;
const  EndpointType:&str = "central";
impl Client {
    fn NewClientWithAccessKey(
        regionId: &str,
        accessKeyId: &str,
        accessKeySecret: &str,
    ) -> Result<Client, Error> {
        let mut  client = Client::default();
        client.InitWithAccessKey(regionId, accessKeyId, accessKeySecret)?;
        SetEndpointDataToClient(&mut client);
        Ok(client)
    }

    fn InitWithAccessKey(&mut self,
        regionId: &str,
        accessKeyId: &str,
        accessKeySecret: &str,
    ) -> Result<(), Error> {
        let config = self.InitClientConfig();
        let credential = AccessKeyCredential{
            AccessKeyId:     accessKeyId.to_string(),
            AccessKeySecret: accessKeySecret.to_string(),
        };
        self.InitWithOptions(regionId, &config, &credential)?;
        Ok(())
    }

    fn InitClientConfig(&mut self)->Config{
       if self.config.is_some() {
           return self.config.to_owned().unwrap();
       }else {
          NewConfig() 
       }
    }

    fn InitWithOptions(&mut self,regionId:&str, config :&Config, credential:&AccessKeyCredential)->Result<(),Error>{
       let matched =Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap().is_match(regionId);
        if !matched{
            return Err(Error::new(ErrorKind::Other,"regionId contains invalid characters"));
        }
        self.regionId=regionId.to_string();
        self.config=Some(config.to_owned());
        
        todo!()
    }
}

pub fn NewConfig()->Config{
let mut config = Config::default();
config.AutoRetry=false;
config.MaxRetryTime=3;
config.UserAgent="".to_string();
config.Debug=false;
config.EnableAsync=false;
config.MaxTaskQueueSize=1000;
config.GoRoutinePoolSize=5;
config.Scheme="HTTP".to_string();
config
}

fn SetEndpointDataToClient(client: &mut Client) {
    client.EndpointMap=GetEndpointMap();
    client.EndpointType=GetEndpointType();
}

fn GetEndpointMap() ->HashMap<String,String>{
		let mut EndpointMap = HashMap::new();
		EndpointMap.insert(	"cn-beijing".to_string(),    "dysmsapi-proxy.cn-beijing.aliyuncs.com".to_string());
			EndpointMap.insert("cn-hongkong".to_string(), "dysmsapi-xman.cn-hongkong.aliyuncs.com".to_string());
			EndpointMap.insert("ap-southeast-1".to_string(), "dysmsapi.ap-southeast-1.aliyuncs.com".to_string());
			EndpointMap.insert("ap-southeast-5".to_string(), "dysmsapi-xman.ap-southeast-5.aliyuncs.com".to_string());
	 EndpointMap
}

fn GetEndpointType()->String{
EndpointType.to_string()
}