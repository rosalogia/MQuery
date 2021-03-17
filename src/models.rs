extern crate chrono;
use super::schema::messages;
use chrono::NaiveDateTime;
use diesel::*;

#[derive(Debug, Queryable, Insertable, QueryableByName)]
#[table_name = "messages"]
/// Represents a Discord message stored in the database
pub struct StoredMessage {
    /// Message ID stored as 64-bit integer (BIGSERIAL)
    pub id: i64,
    /// Author's username
    pub author: String,
    /// Author's User ID as a String
    pub author_id: String,
    /// Message contents
    pub content: String,
    /// ONLY stores a comma separated list of attachment **types**.
    /// This field is unconcerned with the attachments themselves. The purpose
    /// is to filter searches according to what kind of attachments they contain.
    /// Stores `None` if the list is empty.
    pub attachment: Option<String>,
    /// Channel name
    pub channel: String,
    /// Channel ID as a String
    pub channel_id: String,
    /// Message Timestamp as `chrono::NaiveDateTime`
    pub time_posted: NaiveDateTime,
    /// Mentions are stored in a comma separated list of User IDs.
    /// If there are none, `None` is stored.
    pub mentions: Option<String>,
    /// Reactions will always be `None` on insertion.
    pub reactions: Option<String>,
}
