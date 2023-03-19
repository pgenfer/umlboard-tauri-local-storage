use std::marker::PhantomData;

use bonsaidb::{core::{schema::{Collection, SerializedCollection}}, local::Database};

use crate::{repository::Repository, entity::Entity};

pub struct BonsaiRepository<TData> {
    db: Database,
    /// we need this field, otherwise the compiler complains that the type parameter is not used
    /// see here: https://doc.rust-lang.org/std/marker/struct.PhantomData.html
    phantom: PhantomData<TData>
}

impl <TData> BonsaiRepository<TData> {
    pub fn new(db: Database) -> Self { Self { db, phantom: PhantomData } }
}

/// Bonsai repository implements the repository trait by using the bonsai database.
/// Every method converts the CollectionDocument into an Entity object.
/// To make this conversion possible, we need to constraint the type parameter.
/// Note that the PK is constrained to a string, as we did in the Classifier definition
impl <TData> Repository<TData> for BonsaiRepository<TData> 
    where TData: SerializedCollection<Contents = TData> + Collection<PrimaryKey = String> {
    
    fn query_all(&self) -> Vec<Entity<TData>> {
        let result_documents = TData::all(&self.db).query().unwrap();
        let result_entities: Vec<_> = result_documents.iter().map(|f| Entity::<TData>{
            id: f.header.id, 
            content: f.contents
        }).collect();
        result_entities
    }
}

