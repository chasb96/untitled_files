use axum::{async_trait, extract::{FromRequest, Request}, http::{header::CONTENT_TYPE, StatusCode}, Json, RequestExt};
use axum_extra::protobuf::Protobuf;

use crate::host::{axum::JsonOrProtobuf, util::or_status_code::OrBadRequest};

#[async_trait]
impl<'a, T, S> FromRequest<S> for JsonOrProtobuf<T> 
where
    T: 'static,
    Json<T>: FromRequest<()>,
    Protobuf<T>: FromRequest<()>,
    S: Send + Sync
{
    type Rejection = StatusCode;

    async fn from_request(request: Request, _: &S) -> Result<Self, Self::Rejection> {
        let content_type = request
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|content_type| content_type.to_str().ok());

        match content_type {
            Some("application/octet-stream") => {
                let Protobuf(payload) = request
                    .extract::<Protobuf<T>,_>()
                    .await
                    .or_bad_request()?;

                Ok(Self::Protobuf(payload))
            },
            Some("application/json") => {
                let Json(payload) = request
                    .extract::<Json<T>, _>()
                    .await
                    .or_bad_request()?;

                Ok(Self::Json(payload))
            },
            _ => Err(StatusCode::BAD_REQUEST),
        }
    }
}