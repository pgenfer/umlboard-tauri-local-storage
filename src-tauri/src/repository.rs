use crate::entity::Entity;

pub trait Repository<TData> {
    fn query_all(&self) -> Vec<Entity<TData>>;
    fn insert(&self, data: TData) -> Entity<TData>;
}