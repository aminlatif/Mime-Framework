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

    let username = form.username.clone();

    let result =crate::entities::user::ActiveModel {
        username: Set(form.username),
        password: Set(form.password),
        enabled: Set(form.enabled),
        ..Default::default()
    }.insert(&state.database_connection)
    .await;

    if result.is_err() {
        let data = FlashData {
            kind: "error".to_owned(),
            message: format!("User creation failed: {}", result.err().unwrap()).to_owned(),
        };

        return Ok(post_response(&mut cookies, data, "/user/new".to_owned()))
    }

    // .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", e)))?;

    let data = FlashData {
        kind: "success".to_owned(),
        message: format!("User \"{}\" successfully created.", username).to_owned(),
    };

    Ok(post_response(&mut cookies, data, "/user".to_owned()))
}
