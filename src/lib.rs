pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::StoredMessage;
use serenity::model::channel::*;
use serenity::client::Context;
use serenity::http::Http;

/// Attempts to identify an attachment from its file extension
fn identify_attachment(attachment: &Attachment) -> String {
    let a: Vec<&str> = attachment.filename.split('.').collect();
    if a.len() == 1 {
        String::from("file")
    } else {
        String::from(match *a.last().unwrap() {
            "png" | "jpg" | "jpeg" | "gif" | "webp" | "svg" | "bmp" => "image",
            "webm"| "mpg" | "mp2"  | "mpeg"| "mpe"  | "mpv" | "mp4" | "m4p" | "avi" |
            "mov" | "qt" => "video",
            "m4a" | "mp3" | "flac" | "wav" | "aac" => "sound",
            "pdf" => "pdf",
            _ => "file",
        })
    }
}

/// Establishes and returns a connection to a PostgreSQL database
/// using the connection string in the .env file if `None` is given.
/// Otherwise, used the connection string in `Some`.
///
/// # Panics
/// 
/// If the connection can't be established, the program will panic.
pub fn establish_connection(conn_string: Option<&str>) -> PgConnection {
    dotenv().ok();

    let db_url = match conn_string {
        Some(string) => String::from(string),
        None => env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    };

    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}

/// Stores an incoming message in the database to which `conn` is connected.
/// Function is asynchronous in order to fetch channel name, which must be
/// awaited. If successful, returns a `usize` containing the amount of records
/// successfully inserted.
///
/// # Errors
///
/// If the insertion fails, returns a `diesel::result::Error`
pub async fn store_message(conn: PgConnection, ctx: &Context, msg: Message) -> QueryResult<usize> {
    use schema::messages;
    
    let token = env::var("DISCORD_TOKEN").expect("Bot Token must be set");
    
    let http = Http::new_with_token(&token);
    let bot_id = http.get_current_user().await.unwrap().id;

    if msg.author.id.0 == bot_id.0 {
        return Ok(0)
    }

    let new_message = StoredMessage {
        id: msg.id.0 as i64,
        author: msg.author.name,
        author_id: msg.author.id.0.to_string(),
        content: msg.content,
        attachment: if msg.attachments.len() == 0 {
            None
        } else {
            let filenames: String = msg
                .attachments
                .iter()
                .map(|a| identify_attachment(a))
                .collect::<Vec<String>>()
                .join(",");
            Some(filenames)
        },
        channel: msg.channel_id.name(&ctx).await.unwrap(),
        channel_id: msg.channel_id.0.to_string(),
        time_posted: msg.timestamp.naive_utc(),
        mentions: if msg.mentions.len() == 0 {
            None
        } else {
            let usernames: String = msg
                .mentions
                .iter()
                .map(|u| u.id.to_string())
                .collect::<Vec<String>>()
                .join(",");
            Some(usernames)
        },
        reactions: None
    };

    diesel::insert_into(messages::table)
        .values(&new_message)
        .execute(&conn)
}
