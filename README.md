# alibaba-cloud-sdk-rust
阿里云短信rust sdk


## Example
```rust
use alibaba_cloud_sdk_rust::services::dysmsapi;

const AliyunSmsServerRegion: &str = "cn-hangzhou";
const AliyunSmsAccessKeyID: &str = "LTAI4FwqPxiAxxxxxx";
const AliyunSmsAccessKeySecret: &str = "xxxxx0FJqHTTLwDUuhxxxxx";
const AliyunSmsReportTempleateCode: &str = "SMS_226xxxx"; // 通知模版
const AliyunSmsSignName: &str = "阿里云"; // 短信署名

fn main()->Result<(), std::io::Error>{
    let phoneNumber="1391212xxxx";
    match  SendSMS(phoneNumber) {
        Ok(response)=> println!("{:?}",response),
        Err(err)=>println!("{}",err),
    }
}

fn SendSMS(phoneNumber: &str) -> Result<(), std::io::Error> {
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
