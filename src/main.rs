use std::collections::HashMap;

#[allow(unused_imports)]
use dotenvy::dotenv;

mod database;
mod google;
mod request;
mod scrape;
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

    let attendances = database::get_current_attendances().await?;

    for atten in attendances {
        let attendance_links = atten.links;
        for link_ in attendance_links {
            if !time::compare_day(link_.day.clone()) {
                continue;
            }
            let current_time = time::get_current_time_hhmm();
            println!("Current time is : {current_time}");
            let mut headers = HashMap::new();
            headers.insert("User-Agent".to_string(), "Firefox".to_string());
            let mut req_struc = request::RequestStruct {
                url: link_.link.clone(),
                headers,
            };
            // Now add the /viewform in the end, and send the get request. request
            req_struc.url.push_str("/viewform");
            let output = request::send_request(req_struc).await?;
            // println!("Get form: {}", output.body);
            let fbzx_token = scrape::extract_fbzx(&output.body)
                .ok_or("FBZX token not found in the damm html")
                .unwrap();
            // println!("fbzx token is {}", fbzx_token);
            let response = google::submit_google_form(link_, fbzx_token).await?;
            println!(
                "response from google submit: \nstatus code:{}\n",
                response.response_code
            );
        }
    }

    Ok(())
}
