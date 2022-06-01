/*
 * FacileThings API
 *
 * This is the OpenAPI for `app.facilethings.com`.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: im@strrl.dev
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StuffResponse {
    #[serde(rename = "stuff", skip_serializing_if = "Option::is_none")]
    pub stuff: Option<Box<crate::models::StuffResponseStuff>>,
}

impl StuffResponse {
    pub fn new() -> StuffResponse {
        StuffResponse {
            stuff: None,
        }
    }
}


