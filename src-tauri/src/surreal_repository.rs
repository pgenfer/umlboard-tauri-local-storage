use std::{marker::PhantomData};
use async_trait::async_trait;
use bonsaidb::core::schema::view::Serialized;
use serde::{de::DeserializeOwned, Serialize};
use surrealdb::{Surreal, engine::local::Db};

use crate::{repository::Repository};



pub struct SurrealRepository<'a, TData> {
    db: &'a Surreal<Db>,
    /// we need this field, otherwise the compiler complains that the type parameter is not used
    /// see here: https://doc.rust-lang.org/std/marker/struct.PhantomData.html
    phantom: PhantomData<TData>
}

impl <'a, TData> SurrealRepository<'a, TData> {
    pub fn new(db: &'a Surreal<Db>) -> Self { Self { db, phantom: PhantomData } }
}

#[async_trait]
impl <'a, TData> Repository<TData> for SurrealRepository<'a, TData> 
where TData: std::marker::Sync + std::marker::Send + DeserializeOwned + Serialize {
    
    async fn query_all(&self) -> Vec<TData> {
        let type_name = std::any::type_name::<TData>(); // either this here or use a string in the new method, maybe second approach is better?
        let entities: Vec<TData> = self.db.select("classifiers").await.unwrap();
        entities
    }

    async fn insert(&self, data: TData, id: &str) -> TData {
        let created = self.db.create(("classifiers", id)).content(data).await.unwrap();
        created
    }

    async fn edit(&self, id: &str, data: TData) -> Option<TData> {
        let updated = self.db.update(("classifiers", id)).content(data).await.unwrap();
        updated
    }

    async fn query_by_id(&self, id: &str) -> Option<TData> {
        let entity = self.db.select(("classifiers", id)).await.unwrap();
        entity
    }
}

