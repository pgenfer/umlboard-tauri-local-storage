use std::marker::PhantomData;
use async_trait::async_trait;
use bonsaidb::{core::{schema::{Collection, SerializedCollection}}, local::{AsyncDatabase}};
use crate::{repository::Repository};

pub struct BonsaiRepository<'a, TData> {
    db: &'a AsyncDatabase,
    /// we need this field, otherwise the compiler complains that the type parameter is not used
    /// see here: https://doc.rust-lang.org/std/marker/struct.PhantomData.html
    phantom: PhantomData<TData>
}

impl <'a, TData> BonsaiRepository<'a, TData> 
    where TData: SerializedCollection<Contents = TData> + 
        Collection<PrimaryKey = String> + 
        'static + 
        std::marker::Unpin {

    pub fn new(db: &'a AsyncDatabase) -> Self { Self { db, phantom: PhantomData } }
}

#[async_trait]
impl <'a, TData> Repository<TData> for BonsaiRepository<'a, TData> 
    where TData: SerializedCollection<Contents = TData> + 
    Collection<PrimaryKey = String> + 'static  + std::marker::Unpin {
    
    async fn query_all(&self) -> Vec<TData> {
        let result_documents = TData::all_async(self.db).await.unwrap();
        let result_entities: Vec<_> = result_documents.into_iter().map(|f| f.contents).collect();
        result_entities
    }

    async fn insert(&self, data: TData, id: &str) -> TData {
        let new_document = data.push_into_async(self.db).await.unwrap();
        new_document.contents
    }

    async fn edit(&self, id: &str, data: TData) -> TData {
        let updated_document = TData::overwrite_async(id, data, self.db).await.unwrap();
        updated_document.contents
    }

    async fn query_by_id(&self, id: &str) -> Option<TData> {
        let document = TData::get_async(id, self.db).await.unwrap().unwrap();
        Some(document.contents)
    }
}

