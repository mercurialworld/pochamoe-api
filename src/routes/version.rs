use axum::{
    debug_handler, extract::{FromRequestParts,  path::ErrorKind, rejection::PathRejection}, http::{StatusCode, request::Parts}, response::{IntoResponse, Response}
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use validator::{Validate, ValidationError};
use regex::Regex;
use std::sync::LazyLock;

static RE_BS_VERSION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[0-9]+\.[0-9]+\.[0-9]+(?:_[0-9]+)?$").unwrap()
});

#[derive(Debug, Deserialize, Validate)]
pub struct ModParams {
    #[validate(custom(function = "validate_mod", message = "Must be valid mod name"))]
    pub mod_name: String,
    #[validate(regex(path = *RE_BS_VERSION, message = "Must be valid Beat Saber version"))]
    pub bs_version: String
}

#[debug_handler]
// [TODO] do not hardcode this
pub async fn handler(Path(params): Path<ModParams>) -> Result<&'static str, VersionError> {
    match params.validate() {
        Ok(_) => Ok("0.6.7.0"),
        Err(e) => Err(VersionError(e.into())),
    }
}

fn validate_mod(mod_name: &str) -> Result<(), ValidationError> {
    tracing::debug!(mod_name);
    // [TODO] do not hardcode this
    if mod_name.eq_ignore_ascii_case("dumbrequestmanager") {
        return Ok(());
    }

    Err(ValidationError::new("Invalid mod name"))
}

pub struct VersionError(anyhow::Error);

impl IntoResponse for VersionError {
    fn into_response(self) -> Response {
        (
            StatusCode::BAD_REQUEST,
            format!("{}", self.0),
        )
            .into_response()
    }
}

pub struct Path<T>(T);

impl<S, T> FromRequestParts<S> for Path<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = (StatusCode, axum::Json<PathError>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Path::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                let (status, body) = match rejection {
                    PathRejection::FailedToDeserializePathParams(inner) => {
                        let mut status = StatusCode::BAD_REQUEST;

                        let kind = inner.into_kind();
                        let body = match &kind {
                            ErrorKind::WrongNumberOfParameters { .. } => PathError {
                                message: kind.to_string(),
                                location: None,
                            },

                            ErrorKind::ParseErrorAtKey { key, .. } => PathError {
                                message: kind.to_string(),
                                location: Some(key.clone()),
                            },

                            ErrorKind::ParseErrorAtIndex { index, .. } => PathError {
                                message: kind.to_string(),
                                location: Some(index.to_string()),
                            },

                            ErrorKind::ParseError { .. } => PathError {
                                message: kind.to_string(),
                                location: None,
                            },

                            ErrorKind::InvalidUtf8InPathParam { key } => PathError {
                                message: kind.to_string(),
                                location: Some(key.clone()),
                            },

                            ErrorKind::UnsupportedType { .. } => {
                                status = StatusCode::INTERNAL_SERVER_ERROR;
                                PathError {
                                    message: kind.to_string(),
                                    location: None,
                                }
                            }

                            ErrorKind::Message(msg) => PathError {
                                message: msg.clone(),
                                location: None,
                            },

                            _ => PathError {
                                message: format!("Unhandled deserialization error: {kind}"),
                                location: None,
                            },
                        };

                        (status, body)
                    }
                    PathRejection::MissingPathParams(error) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        PathError {
                            message: error.to_string(),
                            location: None,
                        },
                    ),
                    _ => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        PathError {
                            message: format!("Unhandled path rejection: {rejection}"),
                            location: None,
                        },
                    ),
                };

                Err((status, axum::Json(body)))
            }
        }
    }
}

#[derive(Serialize)]
pub struct PathError {
    message: String,
    location: Option<String>,
}