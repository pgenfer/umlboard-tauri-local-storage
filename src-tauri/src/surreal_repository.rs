use std::{marker::PhantomData};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use surrealdb::{Surreal, engine::local::Db};

use crate::{repository::Repository};

pub struct SurrealRepository<TData> {
    db: Box<Surreal<Db>>,
    phantom: PhantomData<TData>,
    table_name: &'static str
}

impl <'a, TData> SurrealRepository<TData> {
    pub fn new(db: Box<Surreal<Db>>, table_name: &'static str) -> Self { Self { 
        db, 
        phantom: PhantomData, 
        table_name 
    }}
}

#[async_trait]
impl <TData> Repository<TData> for SurrealRepository<TData> 
where TData: std::marker::Sync + std::marker::Send + DeserializeOwned + Serialize {
    
    async fn query_all(&self) -> Vec<TData> {
        let entities: Vec<TData> = self.db.select(self.table_name).await.unwrap();
        entities
    }

    async fn insert(&self, data: TData, id: &str) -> TData {
        let created = self.db.create((self.table_name, id)).content(data).await.unwrap();
        created
    }

    async fn edit(&self, id: &str, data: TData) -> TData {
        let updated = self.db.update((self.table_name, id)).content(data).await.unwrap();
        updated
    }

    async fn query_by_id(&self, id: &str) -> Option<TData> {
        let entity = self.db.select((self.table_name, id)).await.unwrap();
        entity
    }
}

