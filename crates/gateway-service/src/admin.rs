use crate::GatewayService;

use mempools_api::api::{
    gateway_admin_server::GatewayAdmin, CreateChainRequest, CreateChainResponse,
    UpdateChainRequest, UpdateChainResponse,
};

use request_validation::Validateable;
use tonic::{Request, Response, Status};
use util::ToGrpcResult;

#[tonic::async_trait]
impl GatewayAdmin for GatewayService {
    async fn create_chain(
        &self,
        request: Request<CreateChainRequest>,
    ) -> Result<Response<CreateChainResponse>, Status> {
        request
            .validate(self.registery.clone())
            .await
            .to_grpc_result()?;

        let registery = self.registery.get_services().await.to_grpc_result()?;
        let chain_svc = registery.chain_service;
        Ok(Response::new(
            chain_svc
                .create_chain(request.get_ref())
                .await
                .to_grpc_result()?,
        ))
    }
    async fn update_chain(
        &self,
        request: Request<UpdateChainRequest>,
    ) -> Result<Response<UpdateChainResponse>, Status> {
        request
            .validate(self.registery.clone())
            .await
            .to_grpc_result()?;

        let registery = self.registery.get_services().await.to_grpc_result()?;
        let chain_svc = registery.chain_service;
        Ok(Response::new(
            chain_svc
                .update_chain(request.get_ref())
                .await
                .to_grpc_result()?,
        ))
    }
}
