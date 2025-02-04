use super::iam_model::AclResponse;
use super::{iam_driver::IamDriver, iam_model::AclMessage};
use super::{iam_model::AclRequest, iam_service::IamService};
use crate::{common::security::jwt_guard::JwtGuard, tags::IamApiTags};
use poem_openapi::{
    payload::Json,
    types::{ParseFromJSON, ToJSON},
    ApiResponse, Object, OpenApi,
};
use std::sync::Arc;

#[derive(Object)]
struct ApiError {
    code: u32,
    message: String,
    details: Option<String>,
}

#[derive(ApiResponse)]
enum Response<T: ParseFromJSON + ToJSON + Send + Sync> {
    #[oai(status = 200)]
    Ok(Json<T>),

    #[oai(status = 500)]
    SqlError(Json<ApiError>),
}

pub struct IamController;

#[OpenApi(prefix_path = "rights", tag = "IamApiTags::Acl")]
impl IamController {
    #[oai(path = "/", method = "post")]
    async fn get_access_rights(
        &self,
        request: Json<AclRequest>,
        guard: JwtGuard,
    ) -> Response<AclResponse> {
        let user = guard.get_user();
        let response = get_permissions(request.0.rights, user.caid, user.gid).await;

        match response {
            Ok(acl_messages) => Response::Ok(Json(AclResponse {
                rights: acl_messages,
            })),
            Err(err) => Response::SqlError(Json(ApiError {
                code: 500,
                message: "Failed to fetch data from database".to_string(),
                details: Some(err.to_string()),
            })),
        }
    }
}

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
