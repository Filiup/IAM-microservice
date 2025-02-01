use poem_openapi::Enum;
use poem_openapi::Object;
use serde::Deserialize;
use serde::Serialize;

#[derive(Object)]
pub struct AclRequest {
    pub rights: Vec<AclMessage>,
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct AclMessage {
    pub operation: AclAction,
    pub permission: String,

    #[oai(rename = "clientAliasIds")]
    pub client_alias_ids: Vec<i32>,

    pub allowed: Option<bool>,
}

#[derive(PartialEq, Enum, Debug, Clone, Copy, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum AclAction {
    READ,
    EDIT,
    DELETE,
    ACCESS,
}

pub enum AclOwnerShip {
    Feature,
    Owner,
    Colleague,
    Suspended,
    Deleted,
}

#[derive(Clone, Copy, Deserialize, Serialize, Default)]
pub enum ColleageStatus {
    Deleted,
    Suspended,
    Colleague,

    #[default]
    None,
}

impl From<Option<i32>> for ColleageStatus {
    fn from(value: Option<i32>) -> Self {
        match value {
            Some(0) => Self::Deleted,
            Some(1) => Self::Suspended,
            Some(2) => Self::Colleague,
            _ => Self::None,
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccessRightEntity {
    pub permission: Option<String>,
    pub feature: Option<i32>,
    pub owner: Option<i32>,
    pub colleague: Option<i32>,
    pub suspended: Option<i32>,
    pub deleted: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct ColleagueEntity {
    pub client_alias_id: i32,
    pub client_id: Option<i32>,
    pub group_id: i32,
    pub status: ColleageStatus,
}

impl AccessRightEntity {
    fn merge_right(original_permission: Option<i32>, new_permission: Option<i32>) -> Option<i32> {
        if original_permission.is_none() && new_permission.is_none() {
            None
        } else if original_permission.is_none() {
            new_permission
        } else if new_permission.is_none() {
            original_permission
        } else {
            original_permission.min(new_permission)
        }
    }

    pub fn get_right(&self, acl_ownership: AclOwnerShip) -> Option<i32> {
        match acl_ownership {
            AclOwnerShip::Owner => self.owner,
            AclOwnerShip::Colleague => self.colleague,
            AclOwnerShip::Suspended => self.suspended,
            AclOwnerShip::Deleted => self.deleted,
            AclOwnerShip::Feature => self.feature,
        }
    }

    pub fn merge(&mut self, right: &AccessRightEntity) {
        self.feature = Self::merge_right(self.feature, right.feature);
        self.owner = Self::merge_right(self.owner, right.owner);
        self.colleague = Self::merge_right(self.colleague, right.colleague);
        self.suspended = Self::merge_right(self.suspended, right.suspended);
        self.deleted = Self::merge_right(self.deleted, right.deleted);
    }
}
