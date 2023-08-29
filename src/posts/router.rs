use axum::{middleware, routing, Router};

use crate::{state::AppState, user::middleware::auth_middleware};

use super::handler::{
    create_posts_handler, get_my_posts_handler, get_post_by_id_handler, get_posts_handler,
};

pub struct PostRouter;

impl PostRouter {
    pub fn new_router(state: AppState) -> Router {
        return Router::new()
            .route("/", routing::get(get_posts_handler))
            .route(
                "/",
                routing::post(create_posts_handler).route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware,
                )),
            )
            .route(
                "/:id",
                routing::post(get_my_posts_handler).route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware,
                )),
            )
            .route("/:id", routing::get(get_post_by_id_handler))
            .with_state(state);
    }
}
