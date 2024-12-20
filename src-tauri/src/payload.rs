use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    pub name: Option<String>,
    pub status: Option<String>,
    pub state: Option<String>,
    pub ports: Option<Vec<String>>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub repo_tag: String,
    pub size: i64
}