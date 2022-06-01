/*
 * FacileThings API
 *
 * This is the OpenAPI for `app.facilethings.com`.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: im@strrl.dev
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`_stuff_get_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StuffGetListError {
    UnknownValue(serde_json::Value),
}


/// 
pub async fn _stuff_get_list(configuration: &configuration::Configuration, list: &str, page: Option<&str>, tags: Option<&str>, person: Option<&str>, area: Option<&str>, goal: Option<&str>, focus: Option<&str>, time: Option<&str>, energy: Option<&str>, urgent: Option<&str>) -> Result<crate::models::StuffResponse, Error<StuffGetListError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/stuff/get_list", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("list", &list.to_string())]);
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = tags {
        local_var_req_builder = local_var_req_builder.query(&[("tags", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = person {
        local_var_req_builder = local_var_req_builder.query(&[("person", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = area {
        local_var_req_builder = local_var_req_builder.query(&[("area", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = goal {
        local_var_req_builder = local_var_req_builder.query(&[("goal", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = focus {
        local_var_req_builder = local_var_req_builder.query(&[("focus", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = time {
        local_var_req_builder = local_var_req_builder.query(&[("time", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = energy {
        local_var_req_builder = local_var_req_builder.query(&[("energy", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = urgent {
        local_var_req_builder = local_var_req_builder.query(&[("urgent", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<StuffGetListError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

