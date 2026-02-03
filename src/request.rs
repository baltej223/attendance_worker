use reqwest::Client;
use std::collections::HashMap;

pub struct RequestStruct {
    pub url: String,
    pub headers: HashMap<String, String>,
}

pub struct ResponseStruct {
    pub body: String,
    pub url: String,
    pub response_code: u16,
}

pub async fn send_request(req: RequestStruct) -> Result<ResponseStruct, reqwest::Error> {
    let client = Client::new();

    let mut request = client.get(&req.url);

    for (key, value) in req.headers {
        request = request.header(&key, &value);
    }

    let response = request.send().await?;

    let status = response.status().as_u16();
    let final_url = response.url().to_string();
    let body = response.text().await?;

    Ok(ResponseStruct {
        body,
        url: final_url,
        response_code: status,
    })
}

pub async fn send_google_post_request(
    url: String,
    form_data: HashMap<String, String>,
    // headers: HashMap<String, String>
) -> Result<ResponseStruct, reqwest::Error> {
    let client = Client::new();

    let request = client.post(&url);

    // for (key, value) in headers {
    //     request = request.header(&key, &value);
    // }

    let response = request.form(&form_data).send().await?;

    let status = response.status().as_u16();
    let final_url = response.url().to_string();
    let body = response.text().await?;

    Ok(ResponseStruct {
        body,
        url: final_url,
        response_code: status,
    })
}
