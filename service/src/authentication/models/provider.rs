use async_trait::async_trait;

/// Trait representing any provider that can authenticate a user with an external system
#[async_trait]
pub trait Provider {}
