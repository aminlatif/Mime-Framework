pub trait WebService {
    async fn start(&mut self) -> anyhow::Result<()>;
}