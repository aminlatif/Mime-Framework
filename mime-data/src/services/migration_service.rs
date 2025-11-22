pub trait MigrationService {
    fn run_migration(&self, params: Vec<String>) -> &String;
}
