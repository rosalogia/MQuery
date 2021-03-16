extern crate chrono;
use super::schema::messages;
use chrono::NaiveDateTime;

#[derive(Queryable, Insertable)]
#[table_name = "messages"]
pub struct StoredMessage {
    pub id: i64,
    pub author: String,
    pub author_id: String,
    pub content: String,
    pub attachment: Option<String>,
    pub channel: String,
    pub channel_id: String,
    pub time_posted: NaiveDateTime,
    pub mentions: Option<String>,
    pub reactions: Option<String>,
}

// #[derive(Insertable)]
// #[table_name = "messages"]
// pub struct NewMessage<'a> {
//     pub author_id: &'a str,
//     pub content: &'a str,
//     pub channel_id: &'a str,
//     pub time_posted: &'a NaiveDateTime
// }
