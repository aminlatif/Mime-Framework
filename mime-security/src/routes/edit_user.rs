use axum::{
    Form, extract::{Query, State, Path}, http::StatusCode, response::Html
};
use mime_web::types::{AppState, FlashData, Params, PostResponse, get_flash_cookie, post_response};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use tower_cookies::Cookies;

pub async fn edit_user(
    state: State<AppState>,
    Path(username): Path<String>,
) -> Result<Html<String>, (StatusCode, String)> {
    let user = crate::entities::user::Entity::find()
        .filter(crate::entities::user::Column::Username.eq(username))
        .one(&state.database_connection)
        .await.unwrap();

    let user = user.unwrap();

    // let post: post::Model = QueryService::find_post_by_id(&state.conn, id)
    //     .await
    //     .expect("could not find post")
    //     .unwrap_or_else(|| panic!("could not find post with id {id}"));

    let mut ctx = tera::Context::new();
    ctx.insert("user", &user);
    ctx.insert("page_title", "Edit User");

    let body = state
        .templates
        .render("security::user/edit", &ctx)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Template error: {}", e)))?;

    Ok(Html(body))
}