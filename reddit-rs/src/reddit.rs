use serde_json::Value;

#[derive(Clone, Debug)]
pub struct Reddit {
    pub id: String,
    pub selftext: String,
    pub parent_id: Option<String>,
}

impl Reddit {
    pub fn from_comment(json: Value) -> Option<Self> {
        if let Some(body) = json.get("body") {
            // if the string is [deleted] or [removed], return None
            let body = body.as_str().unwrap();
            if body == "[deleted]" || body == "[removed]" {
                return None;
            }
            let id = json
                .get("name")
                .or_else(|| json.get("id"))
                .unwrap_or_else(|| {
                    eprintln!("Error: both 'id' and 'name' are missing. JSON: {:?}", json);
                    panic!("Neither 'id' nor 'name' found in JSON");
                })
                .as_str()
                .unwrap_or_else(|| {
                    eprintln!("Error: value is not a string. JSON: {:?}", json);
                    panic!("Neither 'id' nor 'name' is a string");
                });

            let parent_id = json.get("parent_id").unwrap().as_str().unwrap();
            let comment = Reddit {
                id: id.to_string(),
                selftext: body.to_string(),
                parent_id: Some(parent_id.to_string()),
            };
            Some(comment)
        } else {
            dbg!("No 'body' field found");
            None
        }
    }
    pub fn from_thread(json: Value) -> Option<Self> {
        if let Some(num_comments) = json.get("num_comments") {
            let num_comments = num_comments.as_i64().unwrap();

            if num_comments == 0 {
                return None;
            }

            let id = json
                .get("name")
                .or_else(|| json.get("id"))
                .unwrap_or_else(|| {
                    eprintln!("Error: both 'id' and 'name' are missing. JSON: {:?}", json);
                    panic!("Neither 'id' nor 'name' found in JSON");
                })
                .as_str()
                .unwrap_or_else(|| {
                    eprintln!("Error: value is not a string. JSON: {:?}", json);
                    panic!("Neither 'id' nor 'name' is a string");
                });
            let selftext = json.get("selftext").unwrap().as_str().unwrap().to_string();

            if selftext.len() < 10 {
                return None;
            }

            let thread = Reddit {
                id: id.to_string(),
                selftext,
                parent_id: None,
            };

            Some(thread)
        } else {
            dbg!("No 'num_comments' field found");
            None
        }
    }
}
