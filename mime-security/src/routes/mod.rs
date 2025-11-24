use axum::routing::get;
use mime_web::types::{AppState, RouteItems};

mod list_users;

pub use list_users::list_users;

pub fn get_routes() -> RouteItems<AppState> {
    let mut route_items = RouteItems::new();
    route_items.add("/user", get(list_users));

    route_items
}
