use facilethings_openapi::apis::configuration::Configuration;
use facilethings_openapi::apis::default_api::oauth_token;
use facilethings_openapi::models::OAuthFlowPasswordRequestBody;

#[tokio::main]
async fn main() {
    let configuration = Configuration::default();
    let oauth_request = OAuthFlowPasswordRequestBody {
        grant_type: "password".to_string(),
        client_id: "c84a81df8b636b154679f3e5e0b579cf270303bc4c85dbadf34561fd0ea52c64".to_string(),
        client_secret: "0903337d77e5a3c6eefe0c1209f2b5ce5295b02713ed020842d8fc5a65216f5c".to_string(),
        username: "".to_string(),
        password: "".to_string(),
    };
    let response = oauth_token(&configuration, oauth_request).await.unwrap();
    println!("{}", serde_json::to_string(&response).unwrap());
    println!("Hello, world!");
}