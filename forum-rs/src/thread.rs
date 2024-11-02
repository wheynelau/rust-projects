use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct JsonStruct {
    id: String,
    is_thread: String,
    pagetext: String,
    parent_post_id: String,
    root_post_id: String,
}
#[derive(Clone, Debug, Default)]
pub struct Post {
    pub id: String,
    pub is_thread: bool,
    pub pagetext: String,
    pub parent_post_id: String,
    pub root_post_id: String,
}

impl Post {
    pub fn placeholder(id: String) -> Self {
        Post {
            id: id.clone(),
            is_thread: true,
            pagetext: "".to_string(),
            parent_post_id: id.clone(),
            root_post_id: id,
        }
    }
    pub fn from_json_struct(json: JsonStruct) -> Option<Self> {
        Some(Post {
            id: json.id,
            is_thread: json.is_thread == "Y",
            pagetext: json.pagetext,
            parent_post_id: json.parent_post_id,
            root_post_id: json.root_post_id,
        })
    }
}
