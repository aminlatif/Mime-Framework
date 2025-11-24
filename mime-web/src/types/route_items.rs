use axum::routing::MethodRouter;

#[derive(Clone)]
pub struct RouteItem<S>
where
    S: Clone + Send + Sync + 'static,
{
    pub path: String,
    pub method_router: MethodRouter<S>,
}

impl<S> RouteItem<S>
where
    S: Clone + Send + Sync + 'static,
{
    pub fn new(path: &str, method_router: MethodRouter<S>) -> Self {
        Self {
            path: path.to_string(),
            method_router,
        }
    }
}

pub struct RouteItems<S>
where
    S: Clone + Send + Sync + 'static,
{
    pub items: Vec<RouteItem<S>>,
}

impl<S> RouteItems<S>
where
    S: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn single(item: RouteItem<S>) -> Self {
        Self { items: vec![item] }
    }

    pub fn add_route_item(&mut self, item: RouteItem<S>) {
        self.items.push(item);
    }

    pub fn add(&mut self, path: &str, method_router: MethodRouter<S>) {
        self.add_route_item(RouteItem::new(path, method_router));
    }
}
