use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{
        header::{
            HeaderValue,
            USER_AGENT,
        },
        request::Parts,
        StatusCode,
    },
};

/// Extractor that gets the `User-Agent` header from the request.
pub struct UserAgent(pub HeaderValue);

#[async_trait]
impl<S> FromRequestParts<S> for UserAgent
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if let Some(user_agent) = parts.headers.get(USER_AGENT) {
            Ok(UserAgent(user_agent.clone()))
        } else {
            Err((StatusCode::BAD_REQUEST, "`User-Agent` header is missing"))
        }
    }
}
