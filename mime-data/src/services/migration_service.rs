pub trait MigrationService {
    async fn run_migrations(&self, params: Vec<String>);
}
