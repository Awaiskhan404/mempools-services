use crate::GatewayService;
use mempools_api::api::{
    gateway_public_server::GatewayPublic, SubmitCognitoCodeGrantRequest,
    SubmitCognitoCodeGrantResponse,
};
use request_validation::Validateable;
use tonic::{Request, Response, Status};
use util::ToGrpcResult;

#[tonic::async_trait]
impl GatewayPublic for GatewayService {
    async fn submit_cognito_code_grant(
        &self,
        request: Request<SubmitCognitoCodeGrantRequest>,
    ) -> Result<Response<SubmitCognitoCodeGrantResponse>, Status> {
        request
            .validate(self.registery.clone())
            .await
            .to_grpc_result()?;

        Ok(Response::new(
            self.registery
                .get_services()
                .await
                .to_grpc_result()?
                .auth_service
                .submit_cognito_code_grant(request.get_ref().clone())
                .await
                .to_grpc_result()?,
        ))
    }
}
