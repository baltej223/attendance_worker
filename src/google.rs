use std::collections::HashMap;

pub async fn submit_google_form(
    attendance: crate::seri::AttendanceLink,
    fbzx_token: String,
) -> Result<crate::request::ResponseStruct, reqwest::Error> {
    attendance.link.clone().push_str("/formResponse");
    let submit_link = format!("{}/formResponse", attendance.link);
    println!("The submit link is {submit_link}");
    let mut form_data = HashMap::new();
    for q in attendance.questions {
        form_data.insert(format!("{}", q.question), q.answer);
    }
    form_data.insert("fvv".into(), "1".into());
    form_data.insert("pageHistory".into(), "0".into());
    form_data.insert("fbzx".into(), fbzx_token);
    println!("The form data is : {:?}", form_data);
    crate::request::send_google_post_request(submit_link, form_data).await
}
