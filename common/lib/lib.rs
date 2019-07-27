use serde_derive::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Hello(pub String);

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Weight(pub f32);
