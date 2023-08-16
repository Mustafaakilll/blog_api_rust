use axum::{extract::State, middleware::Next, response::IntoResponse, Json};
use hyper::{header, Request, StatusCode};

use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{state::AppState, user::model::User};

use super::model::TokenClaims;

#[derive(Debug, serde::Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub async fn auth_middleware<B>(
    State(state): State<AppState>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    // INFO: Get token from headers
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        });

    // INFO: Check token is not empty
    let token = token.ok_or_else(|| {
        let error = ErrorResponse {
            message: String::from("You have to login to access this site"),
            status: "fail",
        };
        (StatusCode::UNAUTHORIZED, Json(error))
    })?;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(state.config.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|err| {
        let err = ErrorResponse {
            status: "fail",
            message: err.to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(err))
    })?
    .claims;

    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", claims.sub,)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            let json_error = ErrorResponse {
                status: "fail",
                message: format!("Error fetching user from database: {}", e),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_error))
        })?;

    let user = user.ok_or_else(|| {
        let json_error = ErrorResponse {
            status: "fail",
            message: "The user belonging to this token no longer exists".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}
