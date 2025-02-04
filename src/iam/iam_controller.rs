use super::controller_common;
use super::iam_model::AclRequest;
use super::iam_model::AclResponse;
use crate::{common::security::jwt_guard::JwtGuard, tags::IamApiTags};
use poem_openapi::{
    payload::Json,
    types::{ParseFromJSON, ToJSON},
    ApiResponse, Object, OpenApi,
};

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
        let response =
            controller_common::get_permissions(request.0.rights, user.caid, user.gid).await;

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
