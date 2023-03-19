use bonsaidb::core::schema::Collection;
use serde::{Serialize, Deserialize};

use crate::{value_objects::{Point, Dimension}, entity::Entity};


/// Data representing a single UML classifier (like a class, interface etc).
/// Leaking abstraction here as it also contains persistence information (name of collection)
#[derive(Debug, Serialize, Deserialize, Collection, Default)]
#[collection(name="classifiers", primary_key = u64)]
pub struct Classifier {
    pub name: String,
    pub position: Point,
    pub custom_dimension: Option<Dimension>
}

/// shortcut for an entity holding classifier data
pub type ClassifierEntity = Entity<Classifier>;