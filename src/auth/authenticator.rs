// use async_trait::async_trait;
// use axum::{

//     extract::{FromRequestParts},
//     http::request::Parts,
// };
// use axum::http::StatusCode;

// /// Simple User extractor
// #[derive(Debug, Clone)]
// pub struct AuthenticatedUser {
//     pub user_id: String,
// }

// #[async_trait]
// impl<S> FromRequestParts<S> for AuthenticatedUser
// where
//     S: Send + Sync,
// {
//     async fn from_request_parts(
//         parts: &mut Parts,
//         _state: &S,
//     ) -> Result<Self, StatusCode> {
//         if let Some(user_id) = parts.headers.get("x-user-id") {
//             if let Ok(user_id) = user_id.to_str() {
//                 return Ok(AuthenticatedUser {
//                     user_id: user_id.to_string(),
//                 });
//             }
//         }
//         Err(StatusCode::UNAUTHORIZED)
//     }
    
//     #[doc = " If the extractor fails it\'ll use this \"rejection\" type. A rejection is"]
//     #[doc = " a kind of error that can be converted into a response."]
//     type Rejection;
// }
