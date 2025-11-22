use std::sync::{Arc, RwLock};

use axum::{http::StatusCode, routing::{get, get_service}};
use mime_core::services::ConfigurationService;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use tracing::{debug, info};

use crate::{routes::dashboard, services::WebService, types::AppState};

pub struct DefaultWebService {
    configuration_service: Arc<RwLock<dyn ConfigurationService>>,
}

impl DefaultWebService {
    pub fn new(configuration_service: Arc<RwLock<dyn ConfigurationService>>) -> Self {
        Self {
            configuration_service,
        }
    }
}

impl WebService for DefaultWebService {
    async fn start(&self) -> anyhow::Result<()>{
        debug!("running mime web...");

        let host = self.configuration_service
            .read()
            .unwrap()
            .get("host")
            .unwrap_or("127.0.0.1".to_string());
        let port = self.configuration_service
            .read()
            .unwrap()
            .get("port")
            .unwrap_or("8000".to_string());

        let server_url = format!("{host}:{port}");

        info!("Server URL: {server_url}");

        debug!("Initializing templates...");
        let templates = tera::Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))
            .expect("Tera initialization failed");

        let state = AppState { templates };

        let app = axum::Router::new()
            .route("/", get(dashboard))
            .nest_service(
                "/static",
                get_service(ServeDir::new(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/static"
                )))
                .handle_error(|error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {error}"),
                    )
                }),
            )
            .layer(CookieManagerLayer::new())
            .with_state(state);

        let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
        axum::serve(listener, app).await?;

        Ok(())
    }
}
