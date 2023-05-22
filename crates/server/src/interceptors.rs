use auth_service::config::CognitoConfig;
use base64::Engine;
use jsonwebtokens_cognito::KeySet;
use tonic::service::Interceptor;
use util::Result;
use util::{CognitoClaims, GetClaims, ToGrpcResult, ToResult};

#[derive(Clone)]
pub struct AdminInterceptor {
    admins: Vec<String>,
}

impl AdminInterceptor {
    pub fn new(admins: Vec<String>) -> Self {
        Self { admins }
    }

    pub fn is_admin(&self, claims: CognitoClaims) -> bool {
        if claims.email_verified && self.admins.contains(&claims.email) {
            return true;
        }

        false
    }
}

impl Interceptor for AdminInterceptor {
    fn call(
        &mut self,
        request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Request<()>, tonic::Status> {
        let claims = request.get_claims().to_grpc_result()?;

        if !self.is_admin(claims) {
            return Err(tonic::Status::permission_denied(
                "only admins can access this endpoint",
            ));
        }

        Ok(request)
    }
}

#[derive(Clone)]
pub struct AuthInterceptor {
    config: CognitoConfig,
    cognito_jwks: jsonwebtokens_cognito::KeySet,
}

impl AuthInterceptor {
    pub async fn new(config: CognitoConfig) -> Result<Self> {
        let keyset = KeySet::new(
            config.aws_region.clone(),
            config.cognito_user_pool_id.clone(),
        )
        .to_result()?;
        keyset.prefetch_jwks().await.to_result()?;

        Ok(Self {
            config,
            cognito_jwks: keyset,
        })
    }
}

impl Interceptor for AuthInterceptor {
    fn call(
        &mut self,
        request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Request<()>, tonic::Status> {
        let (_, bearer_token) = request
            .metadata()
            .get("Authorization")
            .ok_or("could not find Authorization header to validate jwt")
            .to_grpc_result()?
            .to_str()
            .to_grpc_result()?
            .split_once(' ')
            .ok_or("incorrect authorization header format - must be in bearer format")
            .to_grpc_result()?;
        let verifier = self
            .cognito_jwks
            .new_id_token_verifier(&[self.config.cognito_client_id.as_str()])
            .build()
            .to_grpc_result()?;
        let _ = self
            .cognito_jwks
            .try_verify(bearer_token, &verifier)
            .to_result()
            .to_grpc_result()?;

        Ok(request)
    }
}

#[derive(Clone)]
pub struct MockIdTokenSetter {
    default_user_id: String,
}

impl MockIdTokenSetter {
    pub fn new(default_user_id: impl ToString) -> Self {
        Self {
            default_user_id: default_user_id.to_string(),
        }
    }
}

impl Interceptor for MockIdTokenSetter {
    fn call(
        &mut self,
        mut request: tonic::Request<()>,
    ) -> core::result::Result<tonic::Request<()>, tonic::Status> {
        let claims = CognitoClaims {
            sub: self.default_user_id.clone(),
            ..Default::default()
        };

        if !request.metadata().contains_key("authorization") {
            let json_claims = serde_json::to_string(&claims).to_grpc_result()?;
            let b64_claims = base64::prelude::BASE64_URL_SAFE_NO_PAD.encode(json_claims);
            let mock_id_token = format!("mock_header.{}.mock_signature", b64_claims);

            // DONT MAKE THIS caps tonic bug - https://github.com/hyperium/tonic/issues/343
            request.metadata_mut().insert(
                "authorization",
                format!("Bearer {}", mock_id_token)
                    .as_str()
                    .parse()
                    .to_grpc_result()?,
            );
        }

        Ok(request)
    }
}
