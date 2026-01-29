use crate::seri::User;
use futures_util::stream::TryStreamExt;
// use mongodb::bson::doc;
use mongodb::{Client, Collection, Database};
use std::env;

pub async fn connect() -> Result<Database, mongodb::error::Error> {
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI env var not found!");
    let client = Client::with_uri_str(mongo_uri).await?;
    let db_name = env::var("DATABASE_NAME").expect("The DATABASE_NAME environment var not found!");
    let db = client.database(&db_name);
    // let collection = db.collection::<crate::seri::Job>("jobs");
    println!("Mongo ready");
    Ok(db)
}

pub async fn fetch_users() -> Result<Vec<User>, mongodb::error::Error> {
    let db = connect().await?;

    let collection: Collection<User> = db.collection("users");

    let mut cursor = collection.find(None, None).await?;

    let mut users: Vec<User> = Vec::new();

    while let Some(user) = cursor.try_next().await? {
        users.push(user);
    }

    Ok(users)
}
