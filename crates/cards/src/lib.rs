use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Validate)]
pub struct Card {
    #[garde(length(min = 1))]
    pub id: String,
    #[garde(length(min = 1))]
    pub title: String,
    #[garde(length(min = 1))]
    pub body: String,
}

impl Card {
    #[must_use]
    pub fn new(id: impl Into<String>, title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            body: body.into(),
        }
    }
}
