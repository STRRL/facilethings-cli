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
pub struct OAuthFlowPasswordResponseBody {
    #[serde(rename = "access_token", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "token_type", skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
}

impl OAuthFlowPasswordResponseBody {
    pub fn new() -> OAuthFlowPasswordResponseBody {
        OAuthFlowPasswordResponseBody {
            access_token: None,
            token_type: None,
            created_at: None,
        }
    }
}

