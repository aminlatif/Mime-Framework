use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Html,
};
use mime_web::types::AppState;
use tower_cookies::Cookies;
use tracing::debug;

pub async fn list_users(
    state: State<AppState>,
    Query(params): Query<()>,
    cookies: Cookies,
) -> Result<Html<String>, (StatusCode, String)> {
    let page = 1;
    let items_per_page = 100;

    let mut ctx = tera::Context::new();

    ctx.insert("page_title", "Users");

    // let template = concat!(env!("CARGO_MANIFEST_DIR"), "/templates/user_list.html.tera");

    // debug!("Template: {}", template);

    // let body: Result<String, (StatusCode, &str)> = state
    // let body = state
    //     .templates
    //     .render("security::user/index", &ctx)
    //     .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, format!("Template error").to_string().as_str()))?;
    // .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"));
    // .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Template error: {}", e)))?;
    let body = state
        .templates
        .render("security::user/index", &ctx)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Template error: {}", e)))?;

    Ok(Html(body))

    // let (posts, num_pages) = QueryService::find_posts_in_page(&state.conn, page, posts_per_page)
    //     .await
    //     .expect("Cannot find posts in page");

    // let mut ctx = tera::Context::new();
    // ctx.insert("posts", &posts);
    // ctx.insert("page", &page);
    // ctx.insert("posts_per_page", &posts_per_page);
    // ctx.insert("num_pages", &num_pages);

    // if let Some(value) = get_flash_cookie::<FlashData>(&cookies) {
    //     ctx.insert("flash", &value);
    // }

    // let body = state
    //     .templates
    //     .render("index.html.tera", &ctx)
    //     .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    // Ok(Html(body))
}
