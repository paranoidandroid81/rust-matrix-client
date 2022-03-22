use serde::{Deserialize, Serialize};


#[derive(Deserialize, Debug)]
pub struct WellKnownResponse {
    #[serde(rename = "m.homeserver")]
    homeserver: WellKnownHomeServer,
    #[serde(rename = "m.identity_server")]
    identity_server: WellKnownIdentityServer,
}

#[derive(Deserialize, Debug)]
struct WellKnownHomeServer {
    base_url: String,
}

#[derive(Deserialize, Debug)]
struct WellKnownIdentityServer {
    base_url: String,
}

#[derive(Serialize, Debug)]
pub struct LoginRequest {
    #[serde(rename = "type")]
    type_name: String,
    identifier: UserIdentifier,
    password: String,
}

#[derive(Serialize, Debug)]
struct UserIdentifier {
    #[serde(rename = "type")]
    type_name: String,
    user: String,
}

impl WellKnownResponse {
    pub fn get_base_url(&self) -> String {
        self.homeserver.base_url.clone()
    }
}

#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    access_token: String,
    device_id: String,
    user_id: String,
}

impl LoginRequest {
    pub fn new_username_password_login(user_name: String, password: String) -> LoginRequest {
        LoginRequest {
            type_name: "m.login.password".to_owned(),
            identifier: UserIdentifier { type_name: "m.id.user".to_owned(), user: user_name, },
            password,
        }
    }
}