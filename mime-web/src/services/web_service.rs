pub trait WebService {
    async fn start(&self) -> anyhow::Result<()>;
}