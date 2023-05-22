use log::info;
use mempools_api::api::{
    ConfirmDestinationRegistrationRequest, ConfirmDestinationRegistrationResponse,
    DestinationRegistrationProposal, InitiateDestinationRegistrationRequest,
    InitiateDestinationRegistrationResponse, UserDestination,
};

use util::service_registry::{
    ConfirmationCodeNotification, DestinationServiceInterface, Notification, ServiceRegistery,
    UserDestinationFilter,
};

use util::Result;

use self::storage::DestinationStorage;

pub mod storage;

#[derive(Clone)]
pub struct DestinationService {
    registery: ServiceRegistery,
    store: Box<dyn DestinationStorage>,
}

impl DestinationService {
    pub fn new<S: DestinationStorage>(registery: ServiceRegistery, store: S) -> Self {
        Self {
            registery,
            store: Box::new(store),
        }
    }
}

#[tonic::async_trait]
impl DestinationServiceInterface for DestinationService {
    async fn initiate_destination_registration(
        &self,
        request: &InitiateDestinationRegistrationRequest,
    ) -> Result<InitiateDestinationRegistrationResponse> {
        let proposal = self
            .store
            .create_destination_registration_proposal(request)
            .await?;

        self.registery
            .get_services()
            .await?
            .notification_service
            .send_notification(Notification::ConfirmationCodeNotification(
                ConfirmationCodeNotification {
                    code: proposal.confirmation_code,
                    destination: proposal
                        .destination
                        .as_ref()
                        .ok_or("could not find dest")?
                        .destination
                        .as_ref()
                        .ok_or("could not find dest")?
                        .clone(),
                },
            ))
            .await?;

        info!("sent registration code related to proposal {}", proposal.id);

        Ok(InitiateDestinationRegistrationResponse {
            destination_registration_proposal_id: proposal.id,
        })
    }

    async fn confirm_destination_registration(
        &self,
        request: &ConfirmDestinationRegistrationRequest,
    ) -> Result<ConfirmDestinationRegistrationResponse> {
        let proposal = self
            .store
            .get_destination_registration_proposal_by_id(
                request.destination_registration_proposal_id.clone(),
            )
            .await?;

        if proposal.confirmation_code != request.confirmation_code {
            return Err("invalid confirmation code".into());
        }

        let user_dest = self.store.create_user_destination(&proposal).await?;

        Ok(ConfirmDestinationRegistrationResponse {
            destination: Some(user_dest),
        })
    }

    async fn get_user_destinations(
        &self,
        filter: UserDestinationFilter,
        page: Option<u64>,
    ) -> Result<Vec<UserDestination>> {
        let user_dests = self.store.get_user_destinations(filter, page).await?;
        Ok(user_dests)
    }

    async fn update_user_destination(&self, dest: UserDestination) -> Result<UserDestination> {
        self.store.update_user_destination(dest).await
    }

    async fn delete_user_destination(&self, id: i32) -> Result<()> {
        self.store.delete_user_destination(id).await
    }

    async fn get_destination_registration_proposal_by_id(
        &self,
        id: String,
    ) -> Result<DestinationRegistrationProposal> {
        let drp = self
            .store
            .get_destination_registration_proposal_by_id(id)
            .await?;
        Ok(drp)
    }
}
