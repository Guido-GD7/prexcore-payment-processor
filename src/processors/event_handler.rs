use async_trait::async_trait;

#[async_trait]
pub trait EventHandler<E>: Send + Sync {
    async fn handle(&self, event: E);
}
