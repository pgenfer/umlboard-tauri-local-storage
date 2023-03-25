use crate::entity::Entity;

pub trait Repository<TData> {
    fn query_all(&self) -> Vec<Entity<TData>>;
    fn query_by_id(&self, id: u64) -> Option<Entity<TData>>;
    fn insert(&self, data: TData) -> Entity<TData>;
    fn edit(&self, id: u64, data: TData) -> Option<Entity<TData>>;
}