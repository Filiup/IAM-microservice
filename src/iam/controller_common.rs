use super::{iam_driver::IamDriver, iam_model::AclMessage, iam_service::IamService};
use std::sync::Arc;

pub async fn get_permissions(
    acl_messages: Vec<AclMessage>,
    token_client_alias_id: i32,
    group_id: i32,
) -> Result<Vec<AclMessage>, sqlx::Error> {
    let mut handles = Vec::new();

    let driver = IamDriver::new().await;

    let all_colleagues = Arc::new(driver.load_colleagues(token_client_alias_id).await?);
    let rights = Arc::new(driver.load_access_rights(group_id).await?);

    for mut acl_message in acl_messages {
        let colleagues = Arc::clone(&all_colleagues);
        let rights = Arc::clone(&rights);

        let handle = tokio::task::spawn_blocking(move || {
            let colleagues_message = colleagues
                .iter()
                .filter(|&c| acl_message.client_alias_ids.contains(&c.client_alias_id))
                .copied()
                .collect::<Vec<_>>();

            let service = IamService {
                token_alias_id: token_client_alias_id,
                colleagues: colleagues_message,
                message: &acl_message,
                rights,
            };

            let group_allowed = service.is_group_allowed();
            let colleague_allowed = service.are_colleagues_allowed();
            let allowed = service.is_allowed();

            acl_message.allowed = Some(group_allowed && colleague_allowed && allowed);
            Ok(acl_message)
        });

        handles.push(handle);
    }

    let mut result = Vec::new();
    for handle in handles {
        let acl_message: Result<AclMessage, sqlx::Error> = handle.await.unwrap();
        match acl_message {
            Ok(message) => result.push(message),
            Err(err) => return Err(err),
        }
    }

    Ok(result)
}
