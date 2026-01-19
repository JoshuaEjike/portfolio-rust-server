use async_trait::async_trait;

#[async_trait]
pub trait JwtService: Send + Sync {
    fn generate(&self, user_id: &str) -> String;
    fn verify(&self, token: &str) -> Option<String>;
}
