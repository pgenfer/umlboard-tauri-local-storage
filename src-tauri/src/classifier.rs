use bonsaidb::core::schema::{Collection};
use serde::{Serialize, Deserialize};

use crate::{value_objects::{Point, Dimension}, entity::Entity};

// TODO: remove the entity and auto generate the id every time we add an item to the database.
// The logic should be implemented in the repository, but only if no id is set already.
// Check how polo handles id


/// Data representing a single UML classifier (like a class, interface etc).
/// Leaking abstraction here as it also contains persistence information (name of collection)
#[derive(Debug, Serialize, Deserialize, Default, Collection)]
#[collection(name="classifiers", primary_key = String, natural_id = |classifier: &Classifier| Some(&classifier.id))]
pub struct Classifier {
    pub _id: String,
    pub name: String,
    pub position: Point,
    pub isInterface: bool,
    pub custom_dimension: Option<Dimension>
}

/// shortcut for an entity holding classifier data
pub type ClassifierEntity = Entity<Classifier>;