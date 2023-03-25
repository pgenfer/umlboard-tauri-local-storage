use std::marker::PhantomData;

use polodb_core::Database;

use crate::{repository::Repository, entity::Entity};

pub struct PoloRepository<'a, TData> {
    db: &'a Database,
    phantom: PhantomData<TData>
}

impl <'a, TData> PoloRepository<'a, TData> {
    pub fn new(db: &'a Database) -> Self { Self { db, phantom: PhantomData } }
}

impl <'a, TData> Repository<TData> for PoloRepository<'a, TData> {
    fn query_all(&self) -> Vec<Entity<TData>> {
        todo!()
    }

    fn insert(&self, data: TData) -> Entity<TData> {
        todo!()
    }

    fn edit(&self, id: u64, data: TData) -> Option<Entity<TData>> {
        todo!()
    }

    fn query_by_id(&self, id: u64) -> Option<Entity<TData>> {
        todo!()
    }
}