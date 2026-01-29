use serde::Deserialize;
// use std::collections::HashMap;
//
// #[derive(Debug, Deserialize, Serialize)]
// pub struct Job {
//     #[serde(rename = "_id")]
//     id: mongodb::bson::oid::ObjectId,
//
//     form_url: String,
//
//     fields: HashMap<String, String>,
//
//     submit_at: mongodb::bson::DateTime,
//
//     status: String,
// }
//

#[derive(Debug, Deserialize)]
pub struct User {
    email: String,
    password: String,
    cookie: String,
}
