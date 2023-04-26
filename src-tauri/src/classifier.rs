use bonsaidb::core::schema::{Collection};
use serde::{Serialize, Deserialize};

use crate::{value_objects::{Point, Dimension}};

// TODO: remove the entity and auto generate the id every time we add an item to the database.
// The logic should be implemented in the repository, but only if no id is set already.
// Check how polo handles id



#[derive(Debug, Serialize, Deserialize, Default, Collection)]
// macros required by BonsaiDB to
// specify collection name custom primary key
#[collection(
    name="classifiers", 
    primary_key = String, 
    natural_id = |classifier: &Classifier| Some(classifier._id.clone())
)]

// problem that we have is that BonsaiDB stores ids in header, while surrealDB stores id separately.
// therefore, we manage the identify ourselves via the _id field.
pub struct Classifier {
    pub _id: String,
    pub name: String,
    pub position: Point,
    pub is_interface: bool,
    pub custom_dimension: Option<Dimension>
}