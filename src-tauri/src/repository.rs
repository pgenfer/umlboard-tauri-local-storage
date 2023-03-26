use crate::entity::Entity;

pub trait Repository<TData> {
    fn query_all(&self) -> Vec<Entity<TData>>;
    fn query_by_id(&self, id: &str) -> Option<Entity<TData>>;
    fn insert(&self, data: TData) -> Entity<TData>;
    fn edit(&self, id: &str, data: TData) -> Option<Entity<TData>>;
}