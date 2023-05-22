use mempools_api::api::{
    destination::Destination, ConfirmDestinationRegistrationRequest, CreateAlertRequest,
    CreateChainRequest, DeleteAlertRequest, DeleteUserDestinationRequest, GetAlertsRequest,
    GetChainsRequest, GetNotificationsRequest, GetUserDestinationsRequest,
    InitiateDestinationRegistrationRequest, SubmitCognitoCodeGrantRequest, UpdateAlertRequest,
    UpdateChainRequest, UpdateUserDestinationRequest,
};
use tonic::Request;
use util::{
    service_registry::{ServiceRegistery, UserDestinationFilter},
    GetClaims, Result,
};

#[tonic::async_trait]
pub trait Validateable {
    async fn validate(&self, registery: ServiceRegistery) -> Result<()>;
}

#[tonic::async_trait]
impl Validateable for Request<InitiateDestinationRegistrationRequest> {
    async fn validate(&self, registery: ServiceRegistery) -> Result<()> {
        let request = self;
        let req = request.get_ref();
        let claims = request.get_claims()?;
        let registery = registery.get_services().await?;
        let dest_svc = registery.destination_service;

        if claims.sub != req.user_id {
            return Err("cannot register destinations for other users".into());
        }

        match req
            .destination
            .as_ref()
            .ok_or("could not find dest in req")?
            .destination
            .as_ref()
            .ok_or("could not find dest in req")?
        {
            Destination::Email(email) => {
                if !email_address::EmailAddress::is_valid(email) {
                    return Err("invalid email address in destination".into());
                }
            }
            Destination::Webhook(webhook) => {
                url::Url::parse(webhook)?;
            }
            Destination::Telegram(_) => {}
            Destination::DiscordWebhook(_) => {}
        }

        // check if destination already exists
        let users_dests = dest_svc
            .get_user_destinations(
                UserDestinationFilter {
                    user_id: Some(req.user_id.clone()),
                    ..Default::default()
                },
                None,
            )
            .await?;
        for user_dest in users_dests {
            if req.destination == user_dest.destination {
                return Err("user destination already exists".into());
            }
        }

        Ok(())
    }
}

#[tonic::async_trait]
impl Validateable for Request<ConfirmDestinationRegistrationRequest> {
    async fn validate(&self, registery: ServiceRegistery) -> Result<()> {
        let registery = registery.get_services().await?;
        let dest_svc = registery.destination_service;
        let req = self.get_ref();
        let claims = self.get_claims()?;

        let proposal = dest_svc
            .get_destination_registration_proposal_by_id(
                req.destination_registration_proposal_id.clone(),
            )
            .await?;
        if claims.sub != proposal.user_id {
            return Err("cannot confirm destination for another user".into());
        }

        // check if destination already exists
        let users_dests = dest_svc
            .get_user_destinations(
                UserDestinationFilter {
                    user_id: Some(proposal.user_id),
                    ..Default::default()
                },
                None,
            )
            .await?;
        for user_dest in users_dests {
            if proposal.destination == user_dest.destination {
                return Err("user destination already exists".into());
            }
        }

        Ok(())
    }
}

#[tonic::async_trait]
impl Validateable for Request<GetUserDestinationsRequest> {
    async fn validate(&self, _registery: ServiceRegistery) -> Result<()> {
        Ok(())
    }
}

#[tonic::async_trait]
impl Validateable for Request<UpdateUserDestinationRequest> {
    async fn validate(&self, _registery: ServiceRegistery) -> Result<()> {
        Ok(())
    }
}

#[tonic::async_trait]
impl Validateable for Request<DeleteUserDestinationRequest> {
    async fn validate(&self, _registery: ServiceRegistery) -> Result<()> {
        Ok(())
    }
}

#[tonic::async_trait]
impl Validateable for Request<CreateAlertRequest> {
    async fn validate(&self, registery: ServiceRegistery) -> Result<()> {
        let req = self.get_ref();
        let claims = self.get_claims()?;
        let registery = registery.get_services().await?;
        let dest_svc = registery.destination_service;

        if req.destination_ids.is_empty() {
            return Err("alert needs to have atleast 1 destination".into());
        }

        if req.chain_id.is_empty() {
            return Err("alert needs to specify chain".into());
        }

        if claims.sub != req.user_id {
            return Err("cannot create alert for other users".into());
        }

        for dest_id in &req.destination_ids {
            let dest = dest_svc
                .get_user_destinations(
                    UserDestinationFilter {
                        id: Some(dest_id.clone()),
                        ..Default::default()
                    },
                    None,
                )
                .await?
                .first()
                .ok_or("could not find dest")?
                .clone();
            if claims.sub != dest.user_id {
                return Err("destinations should belong to the current user".into());
            }
        }

        Ok(())
    }
}

#[tonic::async_trait]
impl Validateable for Request<GetAlertsRequest> {
    async fn validate(&self, _registery: ServiceRegistery) -> Result<()> {
        Ok(())
    }
}

#[tonic::async_trait]
impl Validateable for Request<UpdateAlertRequest> {
    async fn validate(&self, _registery: ServiceRegistery) -> Result<()> {
        Ok(())
    }
}

#[tonic::async_trait]
impl Validateable for Request<DeleteAlertRequest> {
    async fn validate(&self, _registery: ServiceRegistery) -> Result<()> {
        Ok(())
    }
}

#[tonic::async_trait]
impl Validateable for Request<GetNotificationsRequest> {
    async fn validate(&self, _registery: ServiceRegistery) -> Result<()> {
        Ok(())
    }
}

#[tonic::async_trait]
impl Validateable for Request<GetChainsRequest> {
    async fn validate(&self, _registery: ServiceRegistery) -> Result<()> {
        Ok(())
    }
}

#[tonic::async_trait]
impl Validateable for Request<SubmitCognitoCodeGrantRequest> {
    async fn validate(&self, _registery: ServiceRegistery) -> Result<()> {
        Ok(())
    }
}

#[tonic::async_trait]
impl Validateable for Request<CreateChainRequest> {
    async fn validate(&self, _registery: ServiceRegistery) -> Result<()> {
        Ok(())
    }
}

#[tonic::async_trait]
impl Validateable for Request<UpdateChainRequest> {
    async fn validate(&self, _registery: ServiceRegistery) -> Result<()> {
        Ok(())
    }
}
