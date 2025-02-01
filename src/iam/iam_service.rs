use std::{collections::HashMap, sync::Arc};

use super::iam_model::{
    AccessRightEntity, AclAction, AclMessage, AclOwnerShip, ColleagueEntity, ColleagueStatus,
};

#[derive(Clone)]
pub struct IamService<'a> {
    pub token_alias_id: i32,
    pub colleagues: Vec<ColleagueEntity>,
    pub rights: Arc<HashMap<i32, HashMap<String, AccessRightEntity>>>,
    pub message: &'a AclMessage,
}

impl<'a> IamService<'a> {
    fn evaluate(
        access_right_entity: &AccessRightEntity,
        operation: AclAction,
        ownership: AclOwnerShip,
    ) -> bool {
        let operation_byte = match operation {
            AclAction::READ => 4,
            AclAction::EDIT => 2,
            AclAction::DELETE => 1,
            _ => 0,
        };

        let has_access = matches!(
            access_right_entity.get_right(AclOwnerShip::Feature),
            Some(1)
        );
        let right = access_right_entity.get_right(ownership);
        match (operation, right) {
            (AclAction::ACCESS, _) | (_, None) => has_access,
            (_, Some(right)) => {
                let allowed = right & operation_byte != 0;
                allowed && has_access
            }
        }
    }

    pub fn get_access(
        &self,
        access_rights: &HashMap<String, AccessRightEntity>,
        mut permission: &str,
    ) -> AccessRightEntity {
        let mut right = AccessRightEntity::default();

        loop {
            if let Some(access_right_entity) = access_rights.get(permission) {
                right.merge(access_right_entity);

                let slice_start = 0usize;
                let slice_end = permission.rfind('.');

                if let Some(slice_end) = slice_end {
                    permission = &permission[slice_start..slice_end];
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        right
    }

    pub fn is_group_allowed(&self) -> bool {
        self.rights.iter().any(|(_, rights)| {
            let access = self.get_access(rights, &self.message.permission);
            Self::evaluate(&access, AclAction::ACCESS, AclOwnerShip::Feature)
        })
    }

    pub fn are_colleagues_allowed(&self) -> bool {
        for colleague in self.colleagues.iter() {
            let no_rights = &HashMap::new();
            let rights = self
                .rights
                .get(&colleague.client_alias_id)
                .unwrap_or(no_rights);

            let access = self.get_access(&rights, &self.message.permission);
            let colleague_allowed =
                Self::evaluate(&access, AclAction::ACCESS, AclOwnerShip::Feature);
            if !colleague_allowed {
                return false;
            }
        }

        true
    }

    pub fn is_allowed(&self) -> bool {
        let no_rights = &HashMap::new();
        let owner_rights = self.rights.get(&self.token_alias_id).unwrap_or(no_rights);
        let owner_access = self.get_access(owner_rights, &self.message.permission);

        if self.colleagues.is_empty() {
            return Self::evaluate(&owner_access, self.message.operation, AclOwnerShip::Owner);
        }

        self.colleagues
            .iter()
            .map(|c| match c.status {
                ColleagueStatus::Colleague => AclOwnerShip::Colleague,
                ColleagueStatus::Suspended => AclOwnerShip::Suspended,
                ColleagueStatus::Deleted => AclOwnerShip::Deleted,
   
            })
            .all(|o| Self::evaluate(&owner_access, self.message.operation, o))
    }
}
