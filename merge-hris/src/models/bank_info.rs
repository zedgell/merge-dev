use crate::configuration::HRISConfig;
use serde::{Deserialize, Serialize};

use merge_proc_macros::{generate_url_params, send_request};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[builder(setter(into))]
pub struct BankInfoModel {
    id: String,
    #[builder(setter(into, strip_option), default)]
    remote_id: Option<String>,
    #[builder(setter(into, strip_option), default)]
    employee: Option<String>,
    #[builder(setter(into, strip_option), default)]
    account_number: Option<String>,
    #[builder(setter(into, strip_option), default)]
    routing_number: Option<String>,
    #[builder(setter(into, strip_option), default)]
    bank_name: Option<String>,
    #[builder(setter(into, strip_option), default)]
    account_type: Option<String>,
    #[builder(setter(into, strip_option), default)]
    remote_created_at: Option<String>,
    #[builder(setter(into, strip_option), default)]
    remote_data: Option<Vec<RemoteDaum>>,
    remote_was_deleted: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[builder(setter(into))]
pub struct RemoteDaum {
    #[builder(setter(into, strip_option), default)]
    pub path: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub data: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Builder)]
#[builder(setter(into))]
#[send_request(service="hris", model="bank-info", return_type=GetRequestResponse)]
pub struct GetRequest {
    pub config: HRISConfig,
    #[builder(setter(into, strip_option), default)]
    pub params: Option<GetRequestParams>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Builder)]
#[builder(setter(into))]
#[generate_url_params]
pub struct GetRequestParams {
    #[builder(setter(into, strip_option), default)]
    pub account_type: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub bank_name: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub created_after: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub created_before: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub cursor: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub employee_id: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub expand: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub include_deleted_data: Option<bool>,
    #[builder(setter(into, strip_option), default)]
    pub include_remote_data: Option<bool>,
    #[builder(setter(into, strip_option), default)]
    pub modified_after: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub modified_before: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub order_by: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub page_size: Option<i32>,
    #[builder(setter(into, strip_option), default)]
    pub remote_fields: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub remote_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetRequestResponse {
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<BankInfoModel>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Builder)]
#[builder(setter(into))]
#[send_request(service="hris", model="bank-info", return_type=BankInfoModel)]
pub struct GetRequestById {
    pub config: HRISConfig,
    pub id: String,
    #[builder(setter(into, strip_option), default)]
    pub params: Option<GetRequestByIdParams>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Builder)]
#[builder(setter(into))]
#[generate_url_params]
pub struct GetRequestByIdParams {
    #[builder(setter(into, strip_option), default)]
    pub expand: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub include_remote_data: Option<bool>,
    #[builder(setter(into, strip_option), default)]
    pub remote_fields: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_builds_the_model() {
        let remote_data = RemoteDaumBuilder::default()
            .path("/bank/1234")
            .data(vec!["1".to_string(), "2".to_string()])
            .build()
            .unwrap();

        let model: BankInfoModel = BankModelBuilder::default()
            .id("1234")
            .remote_id("4321")
            .account_number("7890")
            .remote_data(vec![remote_data.clone()])
            .remote_was_deleted(false)
            .build()
            .unwrap();

        assert_eq!(model.id, "1234".to_string());
        assert_eq!(model.remote_id, Some("4321".to_string()));
        assert_eq!(model.account_number, Some("7890".to_string()));
        assert_eq!(model.remote_data, Some(vec![remote_data]));
        assert_eq!(model.remote_was_deleted, false);
    }

    #[tokio::test]
    async fn it_build_the_request() {
        let config = HRISConfig::new("1234", "4321");

        let request_params: GetRequestParams = GetRequestParamsBuilder::default()
            .account_type("checking")
            .bank_name("test")
            .include_remote_data(true)
            .build()
            .unwrap();

        let request: GetRequest = GetRequestBuilder::default()
            .params(request_params)
            .config(config)
            .build()
            .unwrap();

        println!("{:#?}", request.send_request().await)
    }

    #[tokio::test]
    async fn it_build_the_request_by_id() {
        let config = HRISConfig::new("1234", "4321");

        let request_params = GetRequestByIdParamsBuilder::default()
            .include_remote_data(true)
            .build()
            .unwrap();

        let request: GetRequestById = GetRequestByIdBuilder::default()
            .config(config.clone())
            .id("test")
            .build()
            .unwrap();

        assert_eq!(request.config, config);
        assert_eq!(request.id, "test".to_string());
        println!("{:#?}", request.send_request().await)
    }
}