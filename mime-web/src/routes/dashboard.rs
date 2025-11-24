use axum::{extract::{Query, State}, http::StatusCode, response::Html};
use tower_cookies::Cookies;

use crate::types::AppState;

pub async fn dashboard(
    state: State<AppState>,
    Query(params): Query<()>,
    cookies: Cookies,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    // let page = params.page.unwrap_or(1);
    // let posts_per_page = params.posts_per_page.unwrap_or(5);

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

    let mut ctx = tera::Context::new();

    let body = state
        .templates
        .render("dashboard", &ctx)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    Ok(Html(body))
}