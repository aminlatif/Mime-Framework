use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Html,
};
use mime_web::types::{AppState, FlashData, Params, get_flash_cookie};
use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder};
use tower_cookies::Cookies;

pub async fn new_user(
    State(state): State<AppState>,
    Query(params): Query<Params>,
    cookies: Cookies,
) -> Result<Html<String>, (StatusCode, String)> {
    let ctx = tera::Context::new();

    let body = state
        .templates
        .render("security::user/new", &ctx)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Template error: {}", e)))?;

    Ok(Html(body))
}
