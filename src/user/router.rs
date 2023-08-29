use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::state::AppState;

use super::{
    handler::{get_profile_handler, login_handler, logout_handler, register_user_handler},
    middleware::auth_middleware,
};

pub struct UserRouter;

impl UserRouter {
    pub fn new_router(state: AppState) -> Router {
        return Router::new()
            .route("/login", post(login_handler))
            .route(
                "/me",
                get(get_profile_handler).route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware,
                )),
            )
            .route("/register", post(register_user_handler))
            .route("/logout", post(logout_handler))
            .with_state(state.clone());
    }
}
