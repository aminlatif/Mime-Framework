pub trait DatasourceService {
    fn get_connection_string(&self) -> &String;

    fn get_connection_options(&self) -> &sea_orm::ConnectOptions;

    fn get_connection(&self) -> &sea_orm::DatabaseConnection;
}
