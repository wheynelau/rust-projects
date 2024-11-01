use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokenizers::models::unigram::Node;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct JsonStruct {
    id: String,
    is_thread: String,
    pagetext: String,
    parent_post_id: String,
    root_post_id: String,
}
#[derive(Clone, Debug)]
pub struct Post {
    pub id: String,
    pub is_thread: bool,
    pub pagetext: String,
    pub parent_post_id: String,
    pub root_post_id: String,
}

impl Post {
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