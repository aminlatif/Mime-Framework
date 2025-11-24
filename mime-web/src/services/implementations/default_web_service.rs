use std::{
    sync::{Arc, RwLock},
};

use axum::{http::StatusCode, routing::get_service};
use mime_core::services::ConfigurationService;
use mime_data::services::DatasourceService;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use tracing::{debug, info};

use crate::{
    helpers::load_template_files, services::WebService, types::{AppState, RouteItems, TemplatePath}
};

pub struct DefaultWebService {
    configuration_service: Arc<RwLock<dyn ConfigurationService>>,
    datasource_service: Arc<RwLock<dyn DatasourceService + Send + Sync>>,
    route_items_list: Vec<RouteItems<AppState>>,
    templates_list_list: Vec<Vec<TemplatePath>>,
    router: axum::Router<AppState>,
    templates: tera::Tera,
}

impl DefaultWebService {
    pub fn new(
        configuration_service: Arc<RwLock<dyn ConfigurationService>>,
        datasource_service: Arc<RwLock<dyn DatasourceService + Send + Sync>>,
        route_items_list: Vec<RouteItems<AppState>>,
        templates_list_list: Vec<Vec<TemplatePath>>,
    ) -> Self {
        let router = axum::Router::new();
        let templates = tera::Tera::default();

        Self {
            configuration_service,
            datasource_service,
            route_items_list,
            templates_list_list,
            router,
            templates,
        }
    }

    async fn initialize_routes(&mut self) -> anyhow::Result<()> {
        debug!("Initializing routes...");
        for route_items in &self.route_items_list {
            for route_item in &route_items.items {
                debug!(" * adding route: {}", route_item.path.as_str());
                self.router = self
                    .router
                    .clone()
                    .route(route_item.path.as_str(), route_item.method_router.clone());
            }
        }

        debug!("Routes initialized.");

        Ok(())
    }

    async fn initialize_templates(&mut self) -> anyhow::Result<()> {
        debug!("Initializing templates...");

        self.templates = load_template_files(&self.templates_list_list, self.templates.clone()).await?;

        Ok(())
    }
}

impl WebService for DefaultWebService {
    async fn start(&mut self) -> anyhow::Result<()> {
        debug!("running mime web...");

        self.initialize_routes().await?;

        self.initialize_templates().await?;

        let host = self
            .configuration_service
            .read()
            .unwrap()
            .get("host")
            .unwrap_or("127.0.0.1".to_string());
        let port = self
            .configuration_service
            .read()
            .unwrap()
            .get("port")
            .unwrap_or("8000".to_string());

        let server_url = format!("{host}:{port}");

        info!("Server URL: {server_url}");

        let state = AppState {
            templates: self.templates.clone(),
            database_connection: self.datasource_service.read().unwrap().get_connection().clone(),
        };

        let app = self
            .router
            .clone()
            .nest_service(
                "/static",
                get_service(ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static"))).handle_error(
                    |error| async move {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        )
                    },
                ),
            )
            .layer(CookieManagerLayer::new())
            .with_state(state);

        let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
        axum::serve(listener, app).await?;

        Ok(())
    }
}
