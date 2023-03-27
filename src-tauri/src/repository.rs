
pub trait Repository<TData> {
    fn query_all(&self) -> Vec<TData>;
    fn query_by_id(&self, id: &str) -> Option<TData>;
    fn insert(&self, data: TData) -> TData;
    fn edit(&self, id: &str, data: TData) -> Option<TData>;
}