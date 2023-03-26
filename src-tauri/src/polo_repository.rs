use std::marker::PhantomData;

use polodb_core::{Database, bson::{Document, doc, self}};
use serde::{de::DeserializeOwned, Serialize};

use crate::{repository::Repository, entity::Entity};

pub struct PoloRepository<'a, TData> {
    db: &'a Database,
    phantom: PhantomData<TData>
}

impl <'a, TData> PoloRepository<'a, TData> {
    pub fn new(db: &'a Database) -> Self { Self { db, phantom: PhantomData } }
}

impl <'a, TData> Repository<TData> for PoloRepository<'a, TData> 
    where TData: DeserializeOwned + Serialize {
    fn query_all(&self) -> Vec<Entity<TData>> {
        let collection = self.db.collection::<Document>("classifiers");
        let classifier_documents = collection.find_many(doc! {}).unwrap();
        let classifiers = classifier_documents.into_iter().map(|d| Entity::<TData>{
            id: d.get_object_id("_id").unwrap().to_string(),
            content: bson::from_document::<TData>(d).unwrap()
        }).collect();
        return classifiers;
    }

    fn insert(&self, data: TData) -> Entity<TData> {
        todo!()
    }

    fn edit(&self, id: &str, data: TData) -> Option<Entity<TData>> {
        todo!()
    }

    fn query_by_id(&self, id: &str) -> Option<Entity<TData>> {
        todo!()
    }
}