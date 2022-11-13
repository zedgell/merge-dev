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
    remote_data: Option<Vec<RemoteData>>,
    remote_was_deleted: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[builder(setter(into))]
pub struct RemoteData {
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
    use mockito::mock;

    #[test]
    fn it_builds_the_model() {
        let remote_data = RemoteDataBuilder::default()
            .path("/bank/1234")
            .data(vec!["1".to_string(), "2".to_string()])
            .build()
            .unwrap();

        let model: BankInfoModel = BankInfoModelBuilder::default()
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
    async fn it_successfully_sends_request() {
        let m = mock("GET", "/api/hris/v1/bank-info?include_remote_data=true")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                "{
                  \"next\": \"cD0yMDIxLTAxLTA2KzAzJTNBMjQlM0E1My40MzQzMjYlMkIwMCUzQTAw\",
                  \"previous\": \"cj1sZXdwd2VycWVtY29zZnNkc2NzUWxNMEUxTXk0ME16UXpNallsTWtJ\",
                  \"results\": [
                    {
                      \"id\": \"fd1e0fb5-8f92-4ec9-9f32-179cf732867d\",
                      \"remote_id\": \"123234\",
                      \"employee\": \"a3617eb4-dfe3-426f-921e-a65fc1661e10\",
                      \"account_number\": \"439291590\",
                      \"routing_number\": \"089690059\",
                      \"bank_name\": \"Chase\",
                      \"account_type\": \"CHECKING\",
                      \"remote_created_at\": \"2021-12-06T10:11:26Z\",
                      \"remote_data\": [
                        {
                          \"path\": \"/bank-info\",
                          \"data\": [
                            \"Varies by platform\"
                          ]
                        }
                      ],
                      \"remote_was_deleted\": true
                    }
                  ]
                }",
            )
            .expect(1)
            .create();
        let config = HRISConfig::new("test", "test");

        let expected_remote_data: RemoteData = RemoteDataBuilder::default()
            .path("/bank-info")
            .data(vec!["Varies by platform".to_string()])
            .build()
            .unwrap();

        let expected_model: BankInfoModel = BankInfoModelBuilder::default()
            .id("fd1e0fb5-8f92-4ec9-9f32-179cf732867d")
            .remote_id("123234")
            .employee("a3617eb4-dfe3-426f-921e-a65fc1661e10")
            .account_number("439291590")
            .routing_number("089690059")
            .bank_name("Chase")
            .account_type("CHECKING")
            .remote_created_at("2021-12-06T10:11:26Z")
            .remote_was_deleted(true)
            .remote_data(vec![expected_remote_data])
            .build()
            .unwrap();

        let request_params: GetRequestParams = GetRequestParamsBuilder::default()
            .include_remote_data(true)
            .build()
            .unwrap();

        let request: GetRequest = GetRequestBuilder::default()
            .params(request_params)
            .config(config)
            .build()
            .unwrap();

        let result: GetRequestResponse = request.send_request().await.unwrap();
        assert_eq!(
            result.next,
            Some("cD0yMDIxLTAxLTA2KzAzJTNBMjQlM0E1My40MzQzMjYlMkIwMCUzQTAw".to_string())
        );
        assert_eq!(
            result.previous,
            Some("cj1sZXdwd2VycWVtY29zZnNkc2NzUWxNMEUxTXk0ME16UXpNallsTWtJ".to_string())
        );
        assert_eq!(result.results.is_empty(), false);
        assert_eq!(result.results.len(), 1);
        assert_eq!(result.results.get(0).unwrap().clone(), expected_model);
        m.assert()
    }

    #[tokio::test]
    async fn it_sends_request_and_get_by_id() {
        let m = mock(
            "GET",
            "/api/hris/v1/bank-info/fd1e0fb5-8f92-4ec9-9f32-179cf732867d?include_remote_data=true",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            "
                {
                  \"id\": \"fd1e0fb5-8f92-4ec9-9f32-179cf732867d\",
                  \"remote_id\": \"123234\",
                  \"employee\": \"a3617eb4-dfe3-426f-921e-a65fc1661e10\",
                  \"account_number\": \"439291590\",
                  \"routing_number\": \"089690059\",
                  \"bank_name\": \"Chase\",
                  \"account_type\": \"CHECKING\",
                  \"remote_created_at\": \"2021-12-06T10:11:26Z\",
                  \"remote_data\": [
                    {
                      \"path\": \"/bank-info\",
                      \"data\": [
                        \"Varies by platform\"
                      ]
                    }
                  ],
                  \"remote_was_deleted\": true
                }",
        )
        .expect(1)
        .create();

        let config = HRISConfig::new("test", "test");

        let request_params = GetRequestByIdParamsBuilder::default()
            .include_remote_data(true)
            .build()
            .unwrap();

        let request: GetRequestById = GetRequestByIdBuilder::default()
            .config(config.clone())
            .params(request_params.clone())
            .id("fd1e0fb5-8f92-4ec9-9f32-179cf732867d")
            .build()
            .unwrap();

        let result: BankInfoModel = request.send_request().await.unwrap();

        let expected_remote_data: RemoteData = RemoteDataBuilder::default()
            .path("/bank-info")
            .data(vec!["Varies by platform".to_string()])
            .build()
            .unwrap();

        let expected_model: BankInfoModel = BankInfoModelBuilder::default()
            .id("fd1e0fb5-8f92-4ec9-9f32-179cf732867d")
            .remote_id("123234")
            .employee("a3617eb4-dfe3-426f-921e-a65fc1661e10")
            .account_number("439291590")
            .routing_number("089690059")
            .bank_name("Chase")
            .account_type("CHECKING")
            .remote_created_at("2021-12-06T10:11:26Z")
            .remote_was_deleted(true)
            .remote_data(vec![expected_remote_data])
            .build()
            .unwrap();

        assert_eq!(result, expected_model);
        m.assert()
    }

    #[tokio::test]
    async fn test_it_return_error_on_failed_status() {
        let config = HRISConfig::new("test", "test");

        let m = mock("GET", "/api/hris/v1/bank-info/not-found")
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body("Some Body")
            .expect(1)
            .create();

        let request: GetRequestById = GetRequestByIdBuilder::default()
            .config(config.clone())
            .id("not-found")
            .build()
            .unwrap();

        let result: Result<BankInfoModel, String> = request.send_request().await;

        assert_eq!(result.is_err(), true);
        assert_eq!(
            result.unwrap_err(),
            "Request was not successfully status: 404 body: Some Body"
        );

        m.assert()
    }
}
