#[allow(unused_imports)]
use dotenvy::dotenv;

use crate::seri::Attendance;
// use reqwest::Client;

mod database;
mod seri;
mod time;

#[derive(Debug)]
enum ErrHandler {
    Req(reqwest::Error),
    Mongo(mongodb::error::Error),
}

impl From<reqwest::Error> for ErrHandler {
    fn from(err: reqwest::Error) -> Self {
        ErrHandler::Req(err)
    }
}

impl From<mongodb::error::Error> for ErrHandler {
    fn from(err: mongodb::error::Error) -> Self {
        ErrHandler::Mongo(err)
    }
}

#[tokio::main]
async fn main() -> Result<(), ErrHandler> {
    dotenv().ok();

    // let client = Client::new();
    //
    // let response = client
    //     .get("https://httpbin.org/get")
    //     .header("User-Agent", "rust-client")
    //     .send()
    //     .await?;
    //
    // let body = response.text().await?;
    // println!("{}", body);
    // database::connect().await?;

    // let users: Vec<seri::User> = database::fetch_users().await.unwrap();
    // for user in users {
    //     println!("{:?}", user);
    // }

    // let (time, _): (std::time::Duration, std::time::SystemTime) = time::get_time();
    // println!("{:?}", time);

    let attendances = database::get_current_attendances().await?;
    println!("{}", attendances.len());

    for atten in attendances {
        println!("nrje");
        println!("{:?}", atten);
    }

    Ok(())
}
