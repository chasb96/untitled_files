use axum::{response::{IntoResponse, Response}, Json};
use axum_extra::protobuf::Protobuf;
use prost::Message;
use serde::Serialize;

use crate::host::axum::JsonOrProtobuf;

impl<T> IntoResponse for JsonOrProtobuf<T> 
where
    T: Serialize + Message + Default
{
    fn into_response(self) -> Response {
        match self {
            JsonOrProtobuf::Protobuf(p) => Protobuf(p).into_response(),
            JsonOrProtobuf::Json(j) => Json(j).into_response(),
        }
    }
}