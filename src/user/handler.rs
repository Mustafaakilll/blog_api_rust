use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Extension, Json,
};
use hyper::StatusCode;
use jsonwebtoken::{encode, EncodingKey, Header};
use rand_core::OsRng;
use serde_json::json;
use sqlx::Row;

use crate::{posts::model::Post, state::AppState, user::model::User};

use super::model::{LoginUser, RegisterUser, TokenClaims, UserPostResponse};

pub async fn register_user_handler(
    State(state): State<AppState>,
    Json(data): Json<RegisterUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_exists: Option<bool> =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
            .bind(&data.email)
            .fetch_one(&state.db)
            .await
            .map_err(|e| {
                dbg!(&e);

                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": format!("Database error: {}", e),
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            })?;

    if let Some(true) = user_exists {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "User with that email already exists",
        });
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password =
        match argon2::Argon2::default().hash_password(data.password.as_bytes(), &salt) {
            Ok(hash) => hash,
            Err(err) => {
                dbg!(err);
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": format!("User with that email already exists,"),
                });
                return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
            }
        };

    let hashed_password = hashed_password.to_string();

    let user = sqlx::query_as!(
        User,
        "insert into users (name,username,email,password) values($1,$2,$3,$4) returning *",
        data.name,
        data.username,
        data.email,
        hashed_password,
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let user_response =
        serde_json::json!({"status": "success","data": serde_json::json!({ "user": user })});

    Ok(Json(user_response))
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(data): Json<LoginUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user = sqlx::query_as!(User, "select * from users where email = $1", data.email)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Database Error: {}",e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?
        .ok_or_else(|| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("invalid email or password")
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let is_valid = match PasswordHash::new(&user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(data.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Invalid email or password"),
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims = TokenClaims {
        sub: user.id,
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(env!("JWT_SECRET").as_ref()),
    )
    .map_err(|_e| {});

    let token = token.unwrap();

    return Ok(Json(
        json!({"status": "success", "token": token,"user":user}),
    ));
}

pub async fn get_me_handler(
    Extension(data): Extension<User>,
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user = data.filter_user_record();
    let posts = sqlx::query_as::<_, Post>("select * from posts where author_id=$1")
        .bind(user.id)
        .fetch_all(&app_state.db)
        .await
        .map_err(|e| {
            dbg!(&e);

            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Database error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    return Ok(Json(
        json!({"status": "success", "user": user, "posts": posts}),
    ));
}

pub async fn logout_handler() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let response = Response::new(json!({"status": "success"}).to_string());
    return Ok(Json(json!({"status": "success", "user": response.body()})));
}
