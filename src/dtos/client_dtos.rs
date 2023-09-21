use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateClientRequest {
    pub name: String,
    pub secret: String,
    pub redirect_uri: String,
}

#[derive(Debug, Deserialize)]
pub struct GetClientRequest {
    pub client_id: i32,
}

