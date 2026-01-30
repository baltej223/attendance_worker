use futures_util::TryStreamExt;
use mongodb::{Client, Collection, Database, bson::doc};
use std::env;

pub async fn connect() -> Result<Database, mongodb::error::Error> {
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI env var not found!");
    let client = Client::with_uri_str(mongo_uri).await?;
    let db_name = env::var("DATABASE_NAME").expect("The DATABASE_NAME environment var not found!");
    let db = client.database(&db_name);
    println!("Mongo ready");
    Ok(db)
}

// pub async fn fetch_users() -> Result<Vec<User>, mongodb::error::Error> {
//     let db = connect().await?;
//
//     let collection: Collection<User> = db.collection("users");
//
//     let mut cursor = collection.find(None, None).await?;
//     let mut users: Vec<User> = Vec::new();
//     while let Some(user) = cursor.try_next().await? {
//         users.push(user);
//     }
//
//     Ok(users)
// }
//

pub async fn get_current_attendances() -> Result<Vec<crate::seri::Attendance>, mongodb::error::Error>
{
    let db = connect().await?;
    let collection = db.collection::<crate::seri::Attendance>("attendances");

    let current_time = String::from("9:20"); // crate::time::get_current_time_hhmm();

    let filter = doc! {
        "time": &current_time
    };

    let mut cursor = collection.find(filter, None).await?;

    let mut attendances = Vec::new();
    while let Some(attendance) = cursor.try_next().await? {
        attendances.push(attendance);
    }

    Ok(attendances)
}
