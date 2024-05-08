use log::error;

pub enum JsonOrProtobuf<T> {
    Protobuf(T),
    Json(T),
}

impl<T> JsonOrProtobuf<T> {
    pub fn new(body: T, content_type: &str) -> Self {
        match content_type {
            "application/octet-stream" => Self::Protobuf(body),
            "application/json" => Self::Json(body),
            _ => {
                error!("Attempted to serialize invalid Content-Type {}", content_type);
                panic!("Attempted to serialize invalid Content-Type {}", content_type)
            },
        }
    }

    pub fn decompose(self) -> (T, String) {
        match self {
            JsonOrProtobuf::Protobuf(body) => (body, "application/octet-stream".to_string()),
            JsonOrProtobuf::Json(body) => (body, "application/json".to_string()),
        }
    }
}

impl<T> From<(T, String)> for JsonOrProtobuf<T> {
    fn from(value: (T, String)) -> Self {
        Self::new(value.0, &value.1)
    }
}

impl<T> Into<(T, String)> for JsonOrProtobuf<T> {
    fn into(self) -> (T, String) {
        self.decompose()
    }
}