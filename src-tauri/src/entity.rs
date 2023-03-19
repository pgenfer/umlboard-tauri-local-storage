/// merges id provided by the persistence layer with the domain information
/// extracted from the database. Normally, this struct should be created manually
/// but instead will be returned by the repository
pub struct Entity<T> {
    pub id: String,
    content: T
}