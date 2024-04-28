use reqwest::{blocking::Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::collections::HashMap;

use crate::panel_types::{Perms, Users};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct OAuth2TokenRequestPayload {
    grant_type: String,
    client_id: String,
    client_secret: String,
    domain: String,
}

impl OAuth2TokenRequestPayload {
    pub fn new(client_id: String, client_secret: String, domain: String) -> Self {
        Self {
            grant_type: String::from("client_credentials"),
            client_id: client_id,
            client_secret: client_secret,
            domain: domain,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
    scope: String,
}

#[derive(Debug)]
pub struct PanelAPI(pub Client, pub OAuth2TokenRequestPayload);

pub enum Method {
    Get = 1,
    Post = 2,
    Put = 3,
    Delete = 4,
}

impl PanelAPI {
    pub fn new(requst_payload: OAuth2TokenRequestPayload) -> Self {
        Self(Client::new(), requst_payload)
    }

    fn request_token(&self) -> Result<TokenResponse, Error> {
        let mut params = HashMap::new();
        params.insert("grant_type", "client_credentials");
        params.insert("client_id", self.1.client_id.as_str());
        params.insert("client_secret", &self.1.client_secret);

        let response = match self
            .0
            .post(format!("{}/{}", self.1.domain, "oauth2/token"))
            .form(&params)
            .send()
        {
            Ok(response) => response,
            Err(err) => return Err(err),
        };

        match response.text() {
            Ok(text) => Ok(from_str::<TokenResponse>(&text).unwrap()),
            Err(err) => Err(err),
        }
    }
    fn request_api<'a, T>(&self, api_path: String, method: Method, body: T) -> Result<String, Error>
    where
        T: Serialize,
    {
        let token_response = match self.request_token() {
            Ok(token_response) => token_response,
            Err(err) => return Err(err),
        };

        let endpoint = format!("{}/api/{}", self.1.domain, api_path);

        let method_response = match method {
            Method::Get => self
                .0
                .get(endpoint)
                .bearer_auth(token_response.access_token)
                .body(to_string::<T>(&body).unwrap())
                .send(),
            Method::Post => self
                .0
                .post(endpoint)
                .bearer_auth(token_response.access_token)
                .body(to_string::<T>(&body).unwrap())
                .send(),
            Method::Put => self
                .0
                .put(endpoint)
                .bearer_auth(token_response.access_token)
                .body(to_string::<T>(&body).unwrap())
                .send(),
            Method::Delete => self
                .0
                .delete(endpoint)
                .bearer_auth(token_response.access_token)
                .body(to_string::<T>(&body).unwrap())
                .send(),
        };
        let response = match method_response {
            Ok(response) => response,
            Err(err) => return Err(err),
        };

        match response.text() {
            Ok(text) => Ok(text),
            Err(err) => Err(err),
        }
    }
}

#[derive(Debug)]
pub struct PanelWrapper(pub PanelAPI);

impl PanelWrapper {
    pub fn users(&self) -> Result<Users, Error> {
        match self.0.request_api(format!("users"), Method::Get, {}) {
            Ok(text) => Ok(from_str::<Users>(&text).unwrap()),
            Err(err) => Err(err),
        }
    }
    pub fn perms(&self, id: u16) -> Result<Perms, Error> {
        match self
            .0
            .request_api(format!("users/{}/perms", id), Method::Get, {})
        {
            Ok(text) => Ok(from_str::<Perms>(&text).unwrap()),
            Err(err) => Err(err),
        }
    }
    pub fn set_perms(&self, id: u16, perms: Perms) -> Result<Perms, Error> {
        match self
            .0
            .request_api(format!("users/{}/perms", id), Method::Put, perms)
        {
            Ok(text) => {
                println!("{}", text);
                Ok(match from_str::<Perms>(&text) {
                    Ok(pers) => pers,
                    Err(_) => Perms::default(),
                })
            }
            Err(err) => Err(err),
        }
    }

    pub fn new(request_payload: OAuth2TokenRequestPayload) -> Self {
        Self(PanelAPI::new(request_payload))
    }
}
