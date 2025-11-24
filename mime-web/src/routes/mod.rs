mod dashboard;

use axum::routing::get;
pub use dashboard::dashboard;

use crate::types::{AppState, RouteItems};

pub fn get_routes() -> RouteItems<AppState> {
    let mut route_items = RouteItems::new();
    route_items.add("/", get(dashboard));

    route_items
}
