use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

use dotenv::dotenv;
use std::env;
use diesel::*;
use models::*;

#[group]
#[commands(query)]
struct General;

struct Handler;

use mquery::*;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, m: Message) {
        // Establish a connection and store the incoming message
        // each time one is sent.
        let connection = establish_connection(None);
        match store_message(connection, &ctx, m).await {
            Ok(_) => (),
            Err(e) => println!("Storing message failed with {:?}", e),
        }
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(">>= ")) // set the bot's prefix to ">>="
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Bot Token must be set");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");


    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn query(ctx: &Context, msg: &Message) -> CommandResult {
    let conn_string = env::var("SEARCH_CONN").expect("Must create a read-only database role and provide a connection string for it");
    let db_conn = establish_connection(Some(&conn_string));

    let query_string = format!("SELECT * FROM messages {}", &msg.content[10..]);

    // println!("Running query: {}", query_string);

    let results: Result<Vec<StoredMessage>, diesel::result::Error> = sql_query(query_string)
        .load(&db_conn);

    /*
    if let Ok(res) = &results {
        println!("Got results: {:?}", res);
    } else {
        println!("Error");
    }
    */


    msg.reply(ctx, match results {
        Ok(res) => {
            res
                .iter()
                .map(|r| format!("Author: {}\nContent: {}\nSent At: {}\n\n", r.author, r.content, r.time_posted))
                .collect::<Vec<String>>()
                .join("\n")
        },
        Err(e) => String::from(format!("{:?}", e)),
    }).await?;

    Ok(())
}
