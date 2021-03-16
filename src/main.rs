use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
//    CommandResult,
    macros::{
//        command,
        group
    }
};

use dotenv::dotenv;
use std::env;

#[group]
// #[commands(ping)]
struct General;

struct Handler;

use mquery::*;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, m: Message) {
        // Establish a connection and store the incoming message
        // each time one is sent.
        let connection = establish_connection();
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
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

// #[command]
// async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
//     msg.reply(ctx, "Pong!").await?;

//     Ok(())
// }
