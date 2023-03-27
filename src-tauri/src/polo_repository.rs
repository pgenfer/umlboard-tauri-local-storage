use std::{marker::PhantomData, borrow::Borrow};

use polodb_core::{Database, bson::{Document, doc, bson, self}};
use serde::{de::DeserializeOwned, Serialize};

use crate::{repository::Repository};

pub struct PoloRepository<'a, TEntity> {
    db: &'a Database,
    phantom: PhantomData<TEntity>
}

impl <'a, TEntity> PoloRepository<'a, TEntity> {
    pub fn new(db: &'a Database) -> Self { Self { db, phantom: PhantomData } }
}

impl <'a, TEntity> Repository<TEntity> for PoloRepository<'a, TEntity> 
    where TEntity: DeserializeOwned + Serialize {
    fn query_all(&self) -> Vec<TEntity> {
        let collection = self.db.collection::<Document>("classifiers");
        let documents = collection.find_many(doc! {}).unwrap();
        let entities = documents.into_iter().map(|d| bson::from_document::<TEntity>(d).unwrap()).collect();
        entities
    }

    fn insert(&self, data: TEntity) -> TEntity {
        let collection = self.db.collection::<Document>("classifiers");
        let document = bson::to_document(&data).unwrap();
        collection.insert_one(document).unwrap();
        data
    }

    fn edit(&self, id: &str, data: TEntity) -> Option<TEntity> {
        let collection = self.db.collection::<Document>("classifiers");
        let mut session = self.db.start_session().unwrap();
        session.start_transaction(None).unwrap();

        let mut bson_document = bson::to_document(&data).unwrap();
        bson_document.remove("_id");
        
        let result = collection.update_one_with_session(
            doc! {"_id": id},
            doc! { "$set": bson_document},
            &mut session
        ).unwrap();
        print!("{}", result.modified_count);
        session.commit_transaction().unwrap();
        let result_document = self.query_by_id(id).unwrap();
        
        Some(result_document)
    }

    fn query_by_id(&self, id: &str) -> Option<TEntity> {
        let collection = self.db.collection::<Document>("classifiers");
        let document = collection.find_one(doc! {"_id": id}).unwrap().unwrap();
        Some(bson::from_document::<TEntity>(document).unwrap())
    }
}