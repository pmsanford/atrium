// This file is generated by atrium-codegen. DO NOT EDIT.
#![doc = "Definitions for the `com.atproto.sync.getCommitPath` namespace."]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    #[doc = "The DID of the repo."]
    pub did: String,
    #[doc = "The earliest commit to start from"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earliest: Option<String>,
    #[doc = "The most recent commit"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest: Option<String>,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub commits: Vec<String>,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
