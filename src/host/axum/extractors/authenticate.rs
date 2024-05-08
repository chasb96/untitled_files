use std::{ops::Deref, sync::OnceLock};
use auth::client::{AuthClient, VerifyTokenRequest};
use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};

use crate::host::util::or_status_code::{OrInternalServerError, OrStatusCode};

static AUTH_CLIENT: OnceLock<AuthClient> = OnceLock::new();

pub struct AuthenticateUser {
    pub id: i32,
}

pub struct AuthenticateExtractor(pub AuthenticateUser);

impl Deref for AuthenticateExtractor {
    type Target = AuthenticateUser;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for AuthenticateExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(parts: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        let (scheme, token) = parts.headers
            .get("Authorization")
            .or_status_code(StatusCode::UNAUTHORIZED)?
            .to_str()
            .or_status_code(StatusCode::UNAUTHORIZED)?
            .split_once(' ')
            .or_status_code(StatusCode::BAD_REQUEST)?;

        if scheme.to_ascii_uppercase() != "BEARER" {
            return Err(StatusCode::BAD_REQUEST);
        }

        let auth_client = AUTH_CLIENT.get_or_init(AuthClient::default);

        let request = VerifyTokenRequest {
            token: token.to_string(),
        };

        let response = auth_client.verify_token(request)
            .await
            .or_internal_server_error()?;

        Ok(AuthenticateExtractor(
            AuthenticateUser { 
                id: response.user_id 
            }
        ))
    }
}