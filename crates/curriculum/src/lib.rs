use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Validate)]
pub struct Lesson {
    #[garde(length(min = 1))]
    pub id: String,
    #[garde(length(min = 1))]
    pub title: String,
    #[garde(length(min = 1))]
    pub markdown: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Validate)]
pub struct Curriculum {
    #[garde(length(min = 1))]
    pub id: String,
    #[garde(length(min = 1))]
    pub title: String,
    #[garde(dive)]
    pub lessons: Vec<Lesson>,
}

#[must_use]
pub fn starter_curriculum() -> Curriculum {
    Curriculum {
        id: "rust-wasm-foundations".to_owned(),
        title: "Rust WASM Foundations".to_owned(),
        lessons: vec![Lesson {
            id: "hello-rs-dean".to_owned(),
            title: "Hello rs-dean".to_owned(),
            markdown: "A tiny first lesson for the scaffold.".to_owned(),
        }],
    }
}
