// This file is generated by atrium-codegen. DO NOT EDIT.
#![doc = "Definitions for the `com.atproto.server.createSession` namespace."]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    #[doc = "Handle or other identifier supported by the server for the authenticating user."]
    pub identifier: String,
    pub password: String,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub access_jwt: String,
    pub did: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did_doc: Option<crate::records::Record>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_confirmed: Option<bool>,
    pub handle: String,
    pub refresh_jwt: String,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
    AccountTakedown(Option<String>),
}
