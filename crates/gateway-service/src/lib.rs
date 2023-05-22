use mempools_api::api::{
    gateway_server::Gateway, ConfirmDestinationRegistrationRequest,
    ConfirmDestinationRegistrationResponse, CreateAlertRequest, CreateAlertResponse,
    DeleteAlertRequest, DeleteAlertResponse, DeleteUserDestinationRequest,
    DeleteUserDestinationResponse, GetAlertsRequest, GetAlertsResponse, GetChainsRequest,
    GetChainsResponse, GetNotificationsRequest, GetNotificationsResponse,
    GetUserDestinationsRequest, GetUserDestinationsResponse,
    InitiateDestinationRegistrationRequest, InitiateDestinationRegistrationResponse,
    UpdateAlertRequest, UpdateAlertResponse, UpdateUserDestinationRequest,
    UpdateUserDestinationResponse,
};

use request_validation::Validateable;
use tonic::{Request, Response, Status};
use util::{
    service_registry::{AlertFilter, NotificationFilter, ServiceRegistery, UserDestinationFilter},
    ToGrpcResult,
};

pub mod admin;
pub mod public;

#[derive(Clone)]
pub struct GatewayService {
    registery: ServiceRegistery,
}

impl GatewayService {
    pub fn new(registery: ServiceRegistery) -> Self {
        Self { registery }
    }
}

#[tonic::async_trait]
impl Gateway for GatewayService {
    async fn initiate_destination_registration(
        &self,
        request: Request<InitiateDestinationRegistrationRequest>,
    ) -> Result<Response<InitiateDestinationRegistrationResponse>, Status> {
        request
            .validate(self.registery.clone())
            .await
            .to_grpc_result()?;

        let resp = self
            .registery
            .get_services()
            .await
            .to_grpc_result()?
            .destination_service
            .initiate_destination_registration(request.get_ref())
            .await
            .to_grpc_result()?;

        Ok(Response::new(resp))
    }
    async fn confirm_destination_registration(
        &self,
        request: Request<ConfirmDestinationRegistrationRequest>,
    ) -> Result<Response<ConfirmDestinationRegistrationResponse>, Status> {
        request
            .validate(self.registery.clone())
            .await
            .to_grpc_result()?;

        let resp = self
            .registery
            .get_services()
            .await
            .to_grpc_result()?
            .destination_service
            .confirm_destination_registration(request.get_ref())
            .await
            .to_grpc_result()?;

        Ok(Response::new(resp))
    }

    async fn get_user_destinations(
        &self,
        request: Request<GetUserDestinationsRequest>,
    ) -> Result<Response<GetUserDestinationsResponse>, Status> {
        request
            .validate(self.registery.clone())
            .await
            .to_grpc_result()?;

        let req = request.get_ref().clone();
        let user_destinations = self
            .registery
            .get_services()
            .await
            .to_grpc_result()?
            .destination_service
            .get_user_destinations(
                UserDestinationFilter {
                    user_id: Some(req.user_id),
                    ..Default::default()
                },
                Some(req.page),
            )
            .await
            .to_grpc_result()?;

        Ok(Response::new(GetUserDestinationsResponse {
            user_destinations,
        }))
    }

    async fn update_user_destination(
        &self,
        request: Request<UpdateUserDestinationRequest>,
    ) -> Result<Response<UpdateUserDestinationResponse>, Status> {
        request
            .validate(self.registery.clone())
            .await
            .to_grpc_result()?;

        let req = request.get_ref().clone();
        let dest = self
            .registery
            .get_services()
            .await
            .to_grpc_result()?
            .destination_service
            .update_user_destination(
                req.destination
                    .ok_or("destination not provided")
                    .to_grpc_result()?,
            )
            .await
            .to_grpc_result()?;

        Ok(Response::new(UpdateUserDestinationResponse {
            destination: Some(dest),
        }))
    }
    async fn delete_user_destination(
        &self,
        request: Request<DeleteUserDestinationRequest>,
    ) -> Result<Response<DeleteUserDestinationResponse>, Status> {
        request
            .validate(self.registery.clone())
            .await
            .to_grpc_result()?;

        let req = request.get_ref().clone();
        self.registery
            .get_services()
            .await
            .to_grpc_result()?
            .destination_service
            .delete_user_destination(req.id.parse::<i32>().to_grpc_result()?)
            .await
            .to_grpc_result()?;

        Ok(Response::new(DeleteUserDestinationResponse {}))
    }

    async fn create_alert(
        &self,
        request: Request<CreateAlertRequest>,
    ) -> Result<Response<CreateAlertResponse>, Status> {
        request
            .validate(self.registery.clone())
            .await
            .to_grpc_result()?;

        let alert_svc = self
            .registery
            .get_services()
            .await
            .to_grpc_result()?
            .alert_service;

        let alert = alert_svc
            .create_alert(request.get_ref())
            .await
            .to_grpc_result()?;

        Ok(Response::new(CreateAlertResponse { alert: Some(alert) }))
    }

    async fn get_alerts(
        &self,
        request: Request<GetAlertsRequest>,
    ) -> Result<Response<GetAlertsResponse>, Status> {
        request
            .validate(self.registery.clone())
            .await
            .to_grpc_result()?;

        let req = request.get_ref().clone();
        let registery = self.registery.get_services().await.to_grpc_result()?;
        let alert_svc = registery.alert_service;

        let alerts = alert_svc
            .get_alerts(
                AlertFilter {
                    user_id: Some(req.user_id),
                    ..Default::default()
                },
                req.page,
            )
            .await
            .to_grpc_result()?;

        Ok(Response::new(GetAlertsResponse { alerts }))
    }
    async fn update_alert(
        &self,
        request: Request<UpdateAlertRequest>,
    ) -> Result<Response<UpdateAlertResponse>, Status> {
        request
            .validate(self.registery.clone())
            .await
            .to_grpc_result()?;

        let req = request.get_ref().clone();
        let registery = self.registery.get_services().await.to_grpc_result()?;
        let alert_svc = registery.alert_service;

        let alert = alert_svc
            .update_alert(
                req.alert
                    .ok_or("could not find alert in request")
                    .to_grpc_result()?,
            )
            .await
            .to_grpc_result()?;

        Ok(Response::new(UpdateAlertResponse { alert: Some(alert) }))
    }
    async fn delete_alert(
        &self,
        request: Request<DeleteAlertRequest>,
    ) -> Result<Response<DeleteAlertResponse>, Status> {
        request
            .validate(self.registery.clone())
            .await
            .to_grpc_result()?;

        let req = request.get_ref().clone();
        let registery = self.registery.get_services().await.to_grpc_result()?;
        let alert_svc = registery.alert_service;

        alert_svc
            .delete_alert(req.alert_id.parse::<i32>().to_grpc_result()?)
            .await
            .to_grpc_result()?;

        Ok(Response::new(DeleteAlertResponse {}))
    }

    async fn get_notifications(
        &self,
        request: Request<GetNotificationsRequest>,
    ) -> Result<Response<GetNotificationsResponse>, Status> {
        request
            .validate(self.registery.clone())
            .await
            .to_grpc_result()?;

        let req = request.get_ref().clone();
        let registery = self.registery.get_services().await.to_grpc_result()?;
        let notification_svc = registery.notification_service;

        let notifications = notification_svc
            .get_notifications(
                NotificationFilter {
                    alert_id: req.alert_id.parse().ok(),
                    ..Default::default()
                },
                req.page,
            )
            .await
            .to_grpc_result()?;

        Ok(Response::new(GetNotificationsResponse { notifications }))
    }
    async fn get_chains(
        &self,
        request: Request<GetChainsRequest>,
    ) -> Result<Response<GetChainsResponse>, Status> {
        request
            .validate(self.registery.clone())
            .await
            .to_grpc_result()?;

        let registery = self.registery.get_services().await.to_grpc_result()?;
        let chain_svc = registery.chain_service;
        Ok(Response::new(
            chain_svc.get_chains().await.to_grpc_result()?,
        ))
    }
}
