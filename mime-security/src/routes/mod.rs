use axum::routing::get;
use mime_web::types::{AppState, RouteItems};

mod create_user;
mod edit_user;
mod list_users;
mod new_user;

pub use create_user::create_user;
pub use edit_user::edit_user;
pub use list_users::list_users;
pub use new_user::new_user;

pub fn get_routes() -> RouteItems<AppState> {
    let mut route_items = RouteItems::new();
    route_items.add("/user", get(list_users).post(create_user));
    route_items.add("/user/new", get(new_user));
    route_items.add("/user/{username}", get(edit_user));

    route_items
}
