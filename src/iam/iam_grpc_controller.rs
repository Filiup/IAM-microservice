use poem_grpc::{Request, Response, Status};
poem_grpc::include_proto!("iam");

pub struct IamGrpcController;

impl IamService for IamGrpcController {
    async fn get_access_rights(
        &self,
        _request: Request<AclRequestRpc>,
    ) -> Result<Response<AclResponseRpc>, Status> {
        todo!()
    }
}
