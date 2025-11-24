use axum::{
    Form, extract::{Query, State}, http::StatusCode, response::Html
};
use mime_web::types::{AppState, FlashData, Params, PostResponse, get_flash_cookie, post_response};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, PaginatorTrait, QueryOrder};
use tower_cookies::Cookies;

pub async fn create_user(
    State(state): State<AppState>,
    mut cookies: Cookies,
    form: Form<crate::entities::user::Model>,
) -> Result<PostResponse, (StatusCode, String)> {
    let form = form.0;

    let data = FlashData {
        kind: "success".to_owned(),
        message: "User successfully added".to_owned(),
    };

    crate::entities::user::ActiveModel {
        username: Set(form.username),
        password: Set(form.password),
        enabled: Set(form.enabled),
        ..Default::default()
    }
    .insert(&state.database_connection)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", e)))?;

    Ok(post_response(&mut cookies, data, "/user".to_owned()))
}
