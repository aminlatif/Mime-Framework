use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Html,
};
use mime_web::types::{AppState, FlashData, Params, get_flash_cookie};
use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder};
use tower_cookies::Cookies;

pub async fn list_users(
    State(state): State<AppState>,
    Query(params): Query<Params>,
    cookies: Cookies,
) -> Result<Html<String>, (StatusCode, String)> {
    let page = params.page.unwrap_or(1);
    let items_per_page = params.items_per_page.unwrap_or(100);

    let paginator = crate::entities::user::Entity::find()
        .order_by_asc(crate::entities::user::Column::Username)
        .paginate(&state.database_connection, items_per_page);

    let num_pages = paginator
        .num_pages()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", e)))?;

    let users = paginator
        .fetch_page(page - 1)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", e)))?;

    let mut ctx = tera::Context::new();

    ctx.insert("page_title", "Users");

    ctx.insert("users", &users);
    ctx.insert("page", &page);
    ctx.insert("posts_per_page", &items_per_page);
    ctx.insert("num_pages", &num_pages);

    // let data = FlashData {
    //     kind: "success".to_owned(),
    //     message: "Post successfully added".to_owned(),
    // };
    // ctx.insert("flash", &data);

    if let Some(value) = get_flash_cookie::<FlashData>(&cookies) {
        ctx.insert("flash", &value);
    }


    let body = state
        .templates
        .render("security::user/index", &ctx)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Template error: {}", e)))?;

    Ok(Html(body))
}
