// This file is generated by atrium-codegen. DO NOT EDIT.
#![doc = "Definitions for the `com.atproto.admin.defs` namespace."]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AccountView {
    pub did: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub handle: String,
    pub indexed_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_by: Option<crate::com::atproto::server::defs::InviteCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invites: Option<Vec<crate::com::atproto::server::defs::InviteCode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invites_disabled: Option<bool>,
}
#[doc = "Moderation action type: Acknowledge. Indicates that the content was reviewed and not considered to violate PDS rules."]
pub struct Acknowledge;
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ActionReversal {
    pub created_at: String,
    pub created_by: String,
    pub reason: String,
}
pub type ActionType = String;
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ActionView {
    pub action: ActionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_label_vals: Option<Vec<String>>,
    pub created_at: String,
    pub created_by: String,
    #[doc = "Indicates how long this action was meant to be in effect before automatically expiring."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_hours: Option<i32>,
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negate_label_vals: Option<Vec<String>>,
    pub reason: String,
    pub resolved_report_ids: Vec<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reversal: Option<ActionReversal>,
    pub subject: ActionViewSubjectEnum,
    pub subject_blob_cids: Vec<String>,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ActionViewCurrent {
    pub action: ActionType,
    #[doc = "Indicates how long this action was meant to be in effect before automatically expiring."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_hours: Option<i32>,
    pub id: i32,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ActionViewDetail {
    pub action: ActionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_label_vals: Option<Vec<String>>,
    pub created_at: String,
    pub created_by: String,
    #[doc = "Indicates how long this action was meant to be in effect before automatically expiring."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_hours: Option<i32>,
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negate_label_vals: Option<Vec<String>>,
    pub reason: String,
    pub resolved_reports: Vec<ReportView>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reversal: Option<ActionReversal>,
    pub subject: ActionViewDetailSubjectEnum,
    pub subject_blobs: Vec<BlobView>,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BlobView {
    pub cid: String,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<BlobViewDetailsEnum>,
    pub mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation: Option<Moderation>,
    pub size: i32,
}
#[doc = "Moderation action type: Escalate. Indicates that the content has been flagged for additional review."]
pub struct Escalate;
#[doc = "Moderation action type: Flag. Indicates that the content was reviewed and considered to violate PDS rules, but may still be served."]
pub struct Flag;
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ImageDetails {
    pub height: i32,
    pub width: i32,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Moderation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_action: Option<ActionViewCurrent>,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ModerationDetail {
    pub actions: Vec<ActionView>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_action: Option<ActionViewCurrent>,
    pub reports: Vec<ReportView>,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RecordView {
    pub blob_cids: Vec<String>,
    pub cid: String,
    pub indexed_at: String,
    pub moderation: Moderation,
    pub repo: RepoView,
    pub uri: String,
    pub value: crate::records::Record,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RecordViewDetail {
    pub blobs: Vec<BlobView>,
    pub cid: String,
    pub indexed_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::com::atproto::label::defs::Label>>,
    pub moderation: ModerationDetail,
    pub repo: RepoView,
    pub uri: String,
    pub value: crate::records::Record,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RecordViewNotFound {
    pub uri: String,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RepoBlobRef {
    pub cid: String,
    pub did: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_uri: Option<String>,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RepoRef {
    pub did: String,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RepoView {
    pub did: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub handle: String,
    pub indexed_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_by: Option<crate::com::atproto::server::defs::InviteCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invites_disabled: Option<bool>,
    pub moderation: Moderation,
    pub related_records: Vec<crate::records::Record>,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RepoViewDetail {
    pub did: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub handle: String,
    pub indexed_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_by: Option<crate::com::atproto::server::defs::InviteCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invites: Option<Vec<crate::com::atproto::server::defs::InviteCode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invites_disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::com::atproto::label::defs::Label>>,
    pub moderation: ModerationDetail,
    pub related_records: Vec<crate::records::Record>,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RepoViewNotFound {
    pub did: String,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ReportView {
    pub created_at: String,
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub reason_type: crate::com::atproto::moderation::defs::ReasonType,
    pub reported_by: String,
    pub resolved_by_action_ids: Vec<i32>,
    pub subject: ReportViewSubjectEnum,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_repo_handle: Option<String>,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ReportViewDetail {
    pub created_at: String,
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub reason_type: crate::com::atproto::moderation::defs::ReasonType,
    pub reported_by: String,
    pub resolved_by_actions: Vec<crate::com::atproto::admin::defs::ActionView>,
    pub subject: ReportViewDetailSubjectEnum,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct StatusAttr {
    pub applied: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
}
#[doc = "Moderation action type: Takedown. Indicates that content should not be served by the PDS."]
pub struct Takedown;
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VideoDetails {
    pub height: i32,
    pub length: i32,
    pub width: i32,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum ActionViewDetailSubjectEnum {
    #[serde(rename = "com.atproto.admin.defs#repoView")]
    RepoView(Box<RepoView>),
    #[serde(rename = "com.atproto.admin.defs#repoViewNotFound")]
    RepoViewNotFound(Box<RepoViewNotFound>),
    #[serde(rename = "com.atproto.admin.defs#recordView")]
    RecordView(Box<RecordView>),
    #[serde(rename = "com.atproto.admin.defs#recordViewNotFound")]
    RecordViewNotFound(Box<RecordViewNotFound>),
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum ActionViewSubjectEnum {
    #[serde(rename = "com.atproto.admin.defs#repoRef")]
    RepoRef(Box<RepoRef>),
    #[serde(rename = "com.atproto.repo.strongRef")]
    ComAtprotoRepoStrongRefMain(Box<crate::com::atproto::repo::strong_ref::Main>),
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum BlobViewDetailsEnum {
    #[serde(rename = "com.atproto.admin.defs#imageDetails")]
    ImageDetails(Box<ImageDetails>),
    #[serde(rename = "com.atproto.admin.defs#videoDetails")]
    VideoDetails(Box<VideoDetails>),
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum ReportViewDetailSubjectEnum {
    #[serde(rename = "com.atproto.admin.defs#repoView")]
    RepoView(Box<RepoView>),
    #[serde(rename = "com.atproto.admin.defs#repoViewNotFound")]
    RepoViewNotFound(Box<RepoViewNotFound>),
    #[serde(rename = "com.atproto.admin.defs#recordView")]
    RecordView(Box<RecordView>),
    #[serde(rename = "com.atproto.admin.defs#recordViewNotFound")]
    RecordViewNotFound(Box<RecordViewNotFound>),
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum ReportViewSubjectEnum {
    #[serde(rename = "com.atproto.admin.defs#repoRef")]
    RepoRef(Box<RepoRef>),
    #[serde(rename = "com.atproto.repo.strongRef")]
    ComAtprotoRepoStrongRefMain(Box<crate::com::atproto::repo::strong_ref::Main>),
}
