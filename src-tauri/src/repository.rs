use std::future::Future;

use async_trait::async_trait;


// Trait describing the common behavior of 
// a repository. TEntity is the type of
// domain entity handled by this repository.
#[async_trait]
pub trait Repository<TEntity> {
    async fn query_all(&self) -> Vec<TEntity>;
    async fn query_by_id(&self, id: &str) -> Option<TEntity>;
    async fn insert(&self, data: TEntity) -> TEntity;
    async fn edit(&self, id: &str, data: TEntity) -> Option<TEntity>;
}