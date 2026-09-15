use async_trait::async_trait;

/// Fresh-bearer provider: the relay re-reads it on every (re)dial so an expired access
/// token is never reused after a refresh. `None` = signed out (host relay idles quietly).
#[async_trait]
pub trait TokenSource: Send + Sync + 'static {
    async fn token(&self) -> Option<String>;

    /// Changes whenever credentials become available or are replaced. Long-lived
    /// supervisors use this to retry immediately instead of waiting for backoff.
    fn subscribe(&self) -> Option<tokio::sync::watch::Receiver<u64>> {
        None
    }
}

/// A fixed token (dev mode / tests).
pub struct StaticToken(pub String);

#[async_trait]
impl TokenSource for StaticToken {
    async fn token(&self) -> Option<String> {
        Some(self.0.clone())
    }
}
