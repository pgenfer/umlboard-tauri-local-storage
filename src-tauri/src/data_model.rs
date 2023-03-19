
use bonsaidb::{core::{schema::{Collection, SerializedCollection}, Error, document::CollectionDocument}, local::{Database, config::{StorageConfiguration, Builder}}};
use serde::{Serialize, Deserialize};

use polodb_core::{self, bson::{doc, Document, self}};

use crate::{classifier::{Classifier, ClassifierEntity}, value_objects::Point};




/*
Both, PoloDB and BonsaiDB store the id separately which makes it a bit cumbersome
to implememnt a single repository for it. Instead, we could have different data types
1. one that contains only the data, used to store in the document
2. another one that additionally contains the id, added by the repository.
Rust supports merging structs, but does it also support deriving one struct from another?
We could implement an additional ID trait with two methods, get and set
*/


// pub fn test() {
//     let data = Classifier{
//         name: "test".to_string(),
//         position: Point{x: 0.0, y: 0.0},
//         custom_dimension: None
//     };
//     let test = ClassifierEntity{
//         id: "123".to_string(),
//         content: data
//     };
// }

// pub fn save_classifier(db: &Database) -> Result<CollectionDocument<Classifier>, Error> {
//     let classifier = Classifier {
//         custom_dimension: None,
//         name: "Class 1".to_string(),
//         position: Point {x: 0.0, y: 0.0}
//     }.push_into(db)?;
//     return Ok(classifier);
// }

// pub fn change_name(id: u64, new_name: String, db: &Database) -> Result<CollectionDocument<Classifier>, Error> {
//     let mut classifier = Classifier::get(id, db).unwrap().unwrap();
//     classifier.contents.name = new_name;
//     classifier.update(db).unwrap();
//     return Ok(classifier);
// }

// pub fn save_classifier_polo(db: &polodb_core::Database) {
//     let collection = db.collection("classifiers");
//     let result = collection.insert_one(Classifier {
//         custom_dimension: None,
//         name: "Class 1".to_string(),
//         position: Point {x: 0.0, y: 0.0}
//     }).unwrap();
//     print!("{}", result.inserted_id);
// }

// pub fn get_classifiers(db: &polodb_core::Database) {
//     let collection = db.collection::<Document>("classifiers");
//     let classifiers = collection.find_many(doc! {}).unwrap();
//     print!("{:?}", classifiers[0].get_object_id("_id"));
//     let document = classifiers[0].to_owned();
//     let classifier = bson::from_document::<Classifier>(document).unwrap();
//     print!("{}", classifier.name);
//     print!("{:?}", classifiers);
// }