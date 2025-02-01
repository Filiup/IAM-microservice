use std::collections::HashMap;

use super::{
    iam_driver::IamDriver,
    iam_model::{
        AccessRightEntity, AclAction, AclMessage, AclOwnerShip, ColleageStatus, ColleagueEntity,
    },
};

#[derive(Clone)]
pub struct IamService {
    pub driver: IamDriver,
}

impl IamService {
    pub fn new(driver: IamDriver) -> Self {
        Self { driver }
    }

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

    pub async fn is_group_allowed(
        &self,
        acl_message: &AclMessage,
        group_access_rights: &[HashMap<String, AccessRightEntity>],
    ) -> bool {
        group_access_rights.iter().any(|rights| {
            let access = self.get_access(rights, &acl_message.permission);
            Self::evaluate(&access, AclAction::ACCESS, AclOwnerShip::Feature)
        })
    }

    pub async fn are_colleagues_allowed(
        &self,
        acl_message: &AclMessage,
        colleagues_message: &[ColleagueEntity],
    ) -> Result<bool, sqlx::Error> {
        for &colleague in colleagues_message {
            let rights = self
                .driver
                .load_access_rights(colleague.client_alias_id)
                .await?;

            let access = self.get_access(&rights, &acl_message.permission);
            let colleague_allowed =
                Self::evaluate(&access, AclAction::ACCESS, AclOwnerShip::Feature);
            if !colleague_allowed {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub fn is_allowed(
        &self,
        acl_message: &AclMessage,
        access_right_entity: &AccessRightEntity,
        colleagues_message: &[ColleagueEntity],
    ) -> bool {
        if colleagues_message.is_empty() {
            return Self::evaluate(
                access_right_entity,
                acl_message.operation,
                AclOwnerShip::Owner,
            );
        }

        colleagues_message
            .into_iter()
            .map(|c| match c.status {
                ColleageStatus::Colleague => AclOwnerShip::Colleague,
                ColleageStatus::Suspended => AclOwnerShip::Suspended,
                ColleageStatus::Deleted => AclOwnerShip::Deleted,
                ColleageStatus::None => AclOwnerShip::Owner,
            })
            .all(|o| Self::evaluate(access_right_entity, acl_message.operation, o))
    }
}
