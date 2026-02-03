// use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};
//
// #[derive(Debug, Serialize, Deserialize)]
// pub enum HttpMethod {
//     Get,
//     Post,
//     Put,
//     Delete,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub enum JobStatus {
//     Pending,
//     Running,
//     Success,
//     Failed,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Job {
//     #[serde(rename = "_id")]
//     pub id: ObjectId,
//     pub http_url: String,
//     pub run_at: DateTime,
//     /// When the job actually ran (None if not yet executed)
//     pub executed_at: Option<DateTime>,
//     pub method: HttpMethod,
//     /// Data to send (query params for GET, body for POST later)
//     pub data: Option<HashMap<String, String>>,
//     pub status: JobStatus,
//     /// HTTP response code if request reached server
//     pub response_code: Option<i32>,
//     /// Error message if request failed before response
//     pub last_error: Option<String>,
//     pub attempts: u32,
//     pub max_attempts: u32,
// }

#[derive(Debug, Deserialize)]
pub struct User {
    pub email: String,
    pub password: String,
    pub cookie: String,
}

#[derive(Debug, Deserialize)]
pub struct Question {
    pub question: String,
    pub answer: String,
}

#[derive(Debug, Deserialize)]
pub struct AttendanceLink {
    pub link: String,
    pub day: String,
    pub email: String,
    pub questions: Vec<Question>,
    pub done: bool,
}

#[derive(Debug, Deserialize)]
pub struct Attendance {
    pub time: String,
    pub links: Vec<AttendanceLink>,
}
