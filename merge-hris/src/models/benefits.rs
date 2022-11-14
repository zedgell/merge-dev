use crate::configuration::HRISConfig;
use serde::{Deserialize, Serialize};

use merge_proc_macros::{generate_url_params, send_request};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(setter(into))]
pub struct BenefitModel {
    pub id: String,
    #[builder(setter(into, strip_option), default)]
    pub remote_id: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub employee: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub provider_name: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub benefit_plan_type: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub employee_contribution: Option<f64>,
    #[builder(setter(into, strip_option), default)]
    pub company_contribution: Option<f64>,
    #[builder(setter(into, strip_option), default)]
    pub remote_data: Option<Vec<RemoteData>>,
    pub remote_was_deleted: bool,
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
#[send_request(service="hris", model="benefits", return_type=GetRequestResponse)]
pub struct GetRequest {
    pub config: HRISConfig,
    #[builder(setter(into, strip_option), default)]
    pub params: Option<GetRequestParams>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetRequestResponse {
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<BenefitModel>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Builder)]
#[builder(setter(into))]
#[generate_url_params]
pub struct GetRequestParams {
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
    pub include_deleted_data: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub include_remote_data: Option<bool>,
    #[builder(setter(into, strip_option), default)]
    pub modified_after: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub modified_before: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub page_size: Option<i32>,
    #[builder(setter(into, strip_option), default)]
    pub remote_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Builder)]
#[builder(setter(into))]
#[send_request(service="hris", model="benefits", return_type=BenefitModel)]
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

    #[tokio::test]
    async fn it_should_make_request() {
        let m = mock("GET", "/api/hris/v1/benefits?include_remote_data=true")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("
                            {
                              \"next\": \"cD0yMDIxLTAxLTA2KzAzJTNBMjQlM0E1My40MzQzMjYlMkIwMCUzQTAw\",
                              \"previous\": \"cj1sZXdwd2VycWVtY29zZnNkc2NzUWxNMEUxTXk0ME16UXpNallsTWtJ\",
                              \"results\": [
                                {
                                  \"id\": \"3fe5ae7a-f1ba-4529-b7af-84e86dc6d232\",
                                  \"remote_id\": \"19202938\",
                                  \"employee\": \"d2f972d0-2526-434b-9409-4c3b468e08f0\",
                                  \"provider_name\": \"Blue Shield of California\",
                                  \"benefit_plan_type\": \"MEDICAL\",
                                  \"employee_contribution\": 23.65,
                                  \"company_contribution\": 150,
                                  \"remote_data\": [
                                    {
                                      \"path\": \"/benefits\",
                                      \"data\": [
                                        \"Varies by platform\"
                                      ]
                                    }
                                  ],
                                  \"remote_was_deleted\": true
                                }
                              ]
                            }")
            .expect(1)
            .create();

        let expected_remote_data: RemoteData = RemoteDataBuilder::default()
            .path("/benefits")
            .data(vec!["Varies by platform".to_string()])
            .build()
            .unwrap();

        let expected_model = BenefitModelBuilder::default()
            .id("3fe5ae7a-f1ba-4529-b7af-84e86dc6d232")
            .remote_id("19202938")
            .employee("d2f972d0-2526-434b-9409-4c3b468e08f0")
            .provider_name("Blue Shield of California")
            .benefit_plan_type("MEDICAL")
            .employee_contribution(23.65)
            .company_contribution(150)
            .remote_data(vec![expected_remote_data])
            .remote_was_deleted(true)
            .build()
            .unwrap();

        let config = HRISConfig::new("test", "test");

        let params: GetRequestParams = GetRequestParamsBuilder::default()
            .include_remote_data(true)
            .build()
            .unwrap();

        let request = GetRequestBuilder::default()
            .config(config)
            .params(params)
            .build()
            .unwrap();

        let response: GetRequestResponse = request.send_request().await.unwrap();

        assert_eq!(
            response.next,
            Some("cD0yMDIxLTAxLTA2KzAzJTNBMjQlM0E1My40MzQzMjYlMkIwMCUzQTAw".to_string())
        );
        assert_eq!(
            response.previous,
            Some("cj1sZXdwd2VycWVtY29zZnNkc2NzUWxNMEUxTXk0ME16UXpNallsTWtJ".to_string())
        );
        assert_eq!(response.results.is_empty(), false);
        assert_eq!(response.results.get(0).unwrap().clone(), expected_model);
        m.assert()
    }

    #[tokio::test]
    async fn it_should_make_request_by_id() {
        let m = mock(
            "GET",
            "/api/hris/v1/benefits/3fe5ae7a-f1ba-4529-b7af-84e86dc6d232?include_remote_data=true",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            "
            {
              \"id\": \"3fe5ae7a-f1ba-4529-b7af-84e86dc6d232\",
              \"remote_id\": \"19202938\",
              \"employee\": \"d2f972d0-2526-434b-9409-4c3b468e08f0\",
              \"provider_name\": \"Blue Shield of California\",
              \"benefit_plan_type\": \"MEDICAL\",
              \"employee_contribution\": 23.65,
              \"company_contribution\": 150,
              \"remote_data\": [
                {
                  \"path\": \"/benefits\",
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

        let expected_remote_data: RemoteData = RemoteDataBuilder::default()
            .path("/benefits")
            .data(vec!["Varies by platform".to_string()])
            .build()
            .unwrap();

        let expected_model = BenefitModelBuilder::default()
            .id("3fe5ae7a-f1ba-4529-b7af-84e86dc6d232")
            .remote_id("19202938")
            .employee("d2f972d0-2526-434b-9409-4c3b468e08f0")
            .provider_name("Blue Shield of California")
            .benefit_plan_type("MEDICAL")
            .employee_contribution(23.65)
            .company_contribution(150)
            .remote_data(vec![expected_remote_data])
            .remote_was_deleted(true)
            .build()
            .unwrap();

        let config = HRISConfig::new("test", "test");

        let params: GetRequestByIdParams = GetRequestByIdParamsBuilder::default()
            .include_remote_data(true)
            .build()
            .unwrap();

        let request = GetRequestByIdBuilder::default()
            .config(config)
            .params(params)
            .id("3fe5ae7a-f1ba-4529-b7af-84e86dc6d232")
            .build()
            .unwrap();

        let response: BenefitModel = request.send_request().await.unwrap();

        assert_eq!(response, expected_model);
        m.assert()
    }
}
