use poem_grpc::{Code, Request, Response, Status};

use crate::{common::security::jwt_decoder, iam::controller_common};

use super::iam_model::{AclAction, AclMessage};
poem_grpc::include_proto!("iam");
pub struct IamGrpcController;

impl Into<AclAction> for AclActionRpc {
    fn into(self) -> AclAction {
        match self {
            AclActionRpc::Access => AclAction::ACCESS,
            AclActionRpc::Delete => AclAction::DELETE,
            AclActionRpc::Edit => AclAction::EDIT,
            AclActionRpc::Read => AclAction::READ,
        }
    }
}

impl Into<AclActionRpc> for AclAction {
    fn into(self) -> AclActionRpc {
        match self {
            AclAction::ACCESS => AclActionRpc::Access,
            AclAction::DELETE => AclActionRpc::Delete,
            AclAction::READ => AclActionRpc::Read,
            AclAction::EDIT => AclActionRpc::Edit,
        }
    }
}

impl Into<AclMessage> for AclMessageRpc {
    fn into(self) -> AclMessage {
        AclMessage {
            allowed: self.allowed,
            client_alias_ids: self.client_alias_ids.clone(),
            operation: self.operation().into(),
            permission: self.permission,
        }
    }
}

impl Into<AclMessageRpc> for AclMessage {
    fn into(self) -> AclMessageRpc {
        let operation_rpc: AclActionRpc = self.operation.into();

        AclMessageRpc {
            allowed: self.allowed,
            client_alias_ids: self.client_alias_ids,
            operation: operation_rpc.into(),
            permission: self.permission,
        }
    }
}

impl IamService for IamGrpcController {
    async fn get_access_rights(
        &self,
        request: Request<AclRequestRpc>,
    ) -> Result<Response<AclResponseRpc>, Status> {
        let public_key_base64 = request.data::<String>().unwrap();
        let public_key = jwt_decoder::decode_key(public_key_base64).unwrap();

        let authorization_header = request.metadata().get("authorization").ok_or(
            Status::new(Code::Unauthenticated)
                .with_message("You must provide authorization header with JWT token."),
        )?;

        let user = jwt_decoder::decode_jwt(authorization_header, &public_key).ok_or(
            Status::new(Code::Unauthenticated).with_message("Failed to decode provided JWT."),
        )?;

        let rights_rpc = request.into_inner().rights;
        let rights: Vec<AclMessage> = rights_rpc.into_iter().map(|m| m.into()).collect();

        let result = controller_common::get_permissions(rights, user.caid, user.gid)
            .await
            .map_err(|e| Status::new(Code::Internal).with_message(e.to_string()))?;

        let result_rpc: Vec<AclMessageRpc> = result.into_iter().map(|m| m.into()).collect();
        Ok(Response::new(AclResponseRpc { rights: result_rpc }))
    }
}
