pub mod services;

// use axum::{
//     Router,
//     extract::{Form, Path, Query, State},
//     http::StatusCode,
//     response::Html,
//     routing::{get, get_service, post},
// };
// use serde::Deserialize;
// use std::sync::{Arc, RwLock};
// use tower_cookies::{CookieManagerLayer, Cookies};
// use tower_http::services::ServeDir;
// use tracing::{debug, info};

// use app_state::AppState;
// use mime_core::ConfigurationManager;

// pub mod app_state;

// pub async fn main(
//     configuration_manager: Arc<RwLock<dyn ConfigurationManager>>,
// ) -> anyhow::Result<()> {
//     debug!("running mime web...");

//     let host = configuration_manager
//         .read()
//         .unwrap()
//         .get("host")
//         .unwrap_or("127.0.0.1".to_string());
//     let port = configuration_manager
//         .read()
//         .unwrap()
//         .get("port")
//         .unwrap_or("8000".to_string());

//     let server_url = format!("{host}:{port}");

//     info!("Server URL: {server_url}");

//     debug!("Initializing templates...");
//     let templates = tera::Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))
//         .expect("Tera initialization failed");

//     let state = AppState { templates };

//     let app = axum::Router::new()
//         .route("/", get(index_page))
//         .nest_service(
//             "/static",
//             get_service(ServeDir::new(concat!(
//                 env!("CARGO_MANIFEST_DIR"),
//                 "/static"
//             )))
//             .handle_error(|error| async move {
//                 (
//                     StatusCode::INTERNAL_SERVER_ERROR,
//                     format!("Unhandled internal error: {error}"),
//                 )
//             }),
//         )
//         .layer(CookieManagerLayer::new())
//         .with_state(state);

//     let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
//     axum::serve(listener, app).await?;

//     Ok(())
// }

// #[derive(Deserialize)]
// struct Params {
//     page: Option<u64>,
//     posts_per_page: Option<u64>,
// }

// async fn index_page(
//     state: State<AppState>,
//     Query(params): Query<Params>,
//     cookies: Cookies,
// ) -> Result<Html<String>, (StatusCode, &'static str)> {
//     // let page = params.page.unwrap_or(1);
//     // let posts_per_page = params.posts_per_page.unwrap_or(5);

//     // let (posts, num_pages) = QueryService::find_posts_in_page(&state.conn, page, posts_per_page)
//     //     .await
//     //     .expect("Cannot find posts in page");

//     // let mut ctx = tera::Context::new();
//     // ctx.insert("posts", &posts);
//     // ctx.insert("page", &page);
//     // ctx.insert("posts_per_page", &posts_per_page);
//     // ctx.insert("num_pages", &num_pages);

//     // if let Some(value) = get_flash_cookie::<FlashData>(&cookies) {
//     //     ctx.insert("flash", &value);
//     // }

//     // let body = state
//     //     .templates
//     //     .render("index.html.tera", &ctx)
//     //     .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

//     let mut ctx = tera::Context::new();

//     let body = state
//         .templates
//         .render("index.html.tera", &ctx)
//         .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

//     Ok(Html(body))
// }

