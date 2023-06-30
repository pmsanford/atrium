// This file is generated by atrium-codegen. DO NOT EDIT.
#![doc = "Definitions for the `com.atproto.moderation.createReport` namespace."]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub reason_type: crate::com::atproto::moderation::defs::ReasonType,
    pub subject: InputSubjectEnum,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub created_at: String,
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub reason_type: crate::com::atproto::moderation::defs::ReasonType,
    pub reported_by: String,
    pub subject: OutputSubjectEnum,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum InputSubjectEnum {
    #[serde(rename = "com.atproto.admin.defs#repoRef")]
    ComAtprotoAdminDefsRepoRef(Box<crate::com::atproto::admin::defs::RepoRef>),
    #[serde(rename = "com.atproto.repo.strongRef")]
    ComAtprotoRepoStrongRefMain(Box<crate::com::atproto::repo::strong_ref::Main>),
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum OutputSubjectEnum {
    #[serde(rename = "com.atproto.admin.defs#repoRef")]
    ComAtprotoAdminDefsRepoRef(Box<crate::com::atproto::admin::defs::RepoRef>),
    #[serde(rename = "com.atproto.repo.strongRef")]
    ComAtprotoRepoStrongRefMain(Box<crate::com::atproto::repo::strong_ref::Main>),
}
