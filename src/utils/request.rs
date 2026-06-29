use axum::{
    Json, extract::{FromRequest, FromRequestParts, Path, Query, Request, rejection::JsonRejection}, http::request::Parts,
};
use serde::de::DeserializeOwned;
use crate::utils::response::*;

// Digunakan untuk format response jika request tidak sesuai
pub struct ValidatedJson<T>(pub T);
pub struct ValidatedPath<T>(pub T);
pub struct ValidatedQuery<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = ApiError;
    
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let uri = req.uri().clone();
        match Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                let error_message = rejection.to_string(); 
                Err(AppError::BadRequest(format!("Input Validation Error: {}", error_message)).with_path(&uri))
            }
        }
    }
}

impl<T, S> FromRequestParts<S> for ValidatedPath<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let uri = parts.uri.clone();

        match Path::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                // Catch error default Axum
                let error_msg = rejection.to_string();

                Err(
                    AppError::BadRequest(format!("URL Path Error: {}", error_msg))
                    .with_path(&uri)
                )
            }
        }
    }
}

impl<T, S> FromRequestParts<S> for ValidatedQuery<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = ApiError; 

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let uri = parts.uri.clone();

        match Query::<T>::from_request_parts(parts, state).await {
            Ok(Query(value)) => Ok(ValidatedQuery(value)),
            Err(rejection) => {
                let error_msg = rejection.to_string();

                Err(
                    AppError::BadRequest(format!("Query Validation Error: {}", error_msg)).with_path(&uri)
                )
            }
        }
    }
}