use std::marker::PhantomData;

use bonsaidb::{core::{schema::{Collection, SerializedCollection}}, local::Database};

use crate::{repository::Repository};



pub struct BonsaiRepository<'a, TData> {
    db: &'a Database,
    /// we need this field, otherwise the compiler complains that the type parameter is not used
    /// see here: https://doc.rust-lang.org/std/marker/struct.PhantomData.html
    phantom: PhantomData<TData>
}

impl <'a, TData> BonsaiRepository<'a, TData> {
    pub fn new(db: &'a Database) -> Self { Self { db, phantom: PhantomData } }
}

/// Bonsai repository implements the repository trait by using the bonsai database.
/// Every method converts the CollectionDocument into an Entity object.
/// To make this conversion possible, we need to constraint the type parameter.
/// Note that the PK is constrained to a string, as we did in the Classifier definition
impl <'a, TData> Repository<TData> for BonsaiRepository<'a, TData> 
    where TData: SerializedCollection<Contents = TData> + Collection<PrimaryKey = String> + 'static {
    
    fn query_all(&self) -> Vec<TData> {
        let result_documents = TData::all(self.db).query().unwrap();
        let result_entities: Vec<_> = result_documents.into_iter().map(|f| f.contents).collect();
        result_entities
    }

    fn insert(&self, data: TData) -> TData {
        let new_document = data.push_into(self.db).unwrap();
        new_document.contents
    }

    fn edit(&self, id: &str, data: TData) -> Option<TData> {
        // overwrite creates new document if not there, so it always returns a document
        // maybe we should change our API?
        let updated_document = TData::overwrite(id, data, self.db).unwrap();
        Some(updated_document.contents)
    }

    fn query_by_id(&self, id: &str) -> Option<TData> {
        let document = TData::get(id, self.db).unwrap().unwrap();
        Some(document.contents)
    }
}

