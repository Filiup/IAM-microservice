use poem_grpc::{Request, Response, Status};

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

impl IamService for IamGrpcController {
    async fn get_access_rights(
        &self,
        _request: Request<AclRequestRpc>,
    ) -> Result<Response<AclResponseRpc>, Status> {
        todo!()
    }
}
