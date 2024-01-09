use regex::Regex;
use serenity::{async_trait, Client};
use serenity::client::Context;
use serenity::framework::StandardFramework;
use serenity::model::channel::Message;
use serenity::prelude::{EventHandler, GatewayIntents};

const DISCORD_TOKEN: &str = "DISCORD_TOKEN";
const FX_TWITTER_HOST: &str = "https://fxtwitter.com";
const X_LINK_REGEX: &str = r"https?:\/\/x[.]com\/(?<path>\w+\/status\/\d+(?:\?[a-zA-Z0-9_=]*)?)";
const TWITTER_LINK_REGEX: &str = r"https?:\/\/twitter[.]com\/(?<path>\w+\/status\/\d+(?:\?[a-zA-Z0-9_=]*)?)";

struct FXHandler {
    x_regex: Regex,
    twitter_regex: Regex,
}

impl FXHandler {
    pub fn new() -> Self {
        let x_regex = Regex::new(X_LINK_REGEX).unwrap();
        let twitter_regex = Regex::new(TWITTER_LINK_REGEX).unwrap();

        Self {
            x_regex,
            twitter_regex,
        }
    }

    fn contains_x_or_twitter_link(&self, message: &str) -> bool {
        self.x_regex.is_match(message)
            || self.twitter_regex.is_match(message)
    }

    fn correct_x_with_fx(&self, message: &str) -> Option<String> {
        if !self.contains_x_or_twitter_link(message) {
            return None;
        }

        let message = self
            .twitter_regex
            .replace_all(message, format!("{FX_TWITTER_HOST}/$path"));

        let message = self
            .x_regex
            .replace_all(&message, format!("{FX_TWITTER_HOST}/$path"));

        Some(message.to_string())
    }
}

#[async_trait]
impl EventHandler for FXHandler {
    async fn message(&self, ctx: Context, message: Message) {
        if message.is_own(&ctx.cache) {
            // we don't want to process the bot's own messages
            return;
        }

        let Some(corrected_message) = self.correct_x_with_fx(&message.content) else {
            return;
        };

        let bot_message = format!(
            "<@{user}> says: \n\n{message}",
            user = message.author.id,
            message = corrected_message
        );

        let delete_message_result = message
            .delete(&ctx.http)
            .await;

        if let Err(err) = delete_message_result {
            println!("Error deleting origin message: {err:#?}");
            // if we failed to delete the old message, let's bail out now
            return;
        }

        let send_message_result = message
            .channel_id
            .say(&ctx.http, bot_message)
            .await;

        if let Err(err) = send_message_result {
            println!("Error sending message: {err:#?}");
        }
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var(DISCORD_TOKEN)
        .expect("Missing discord token");

    let intents = GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGES;

    let mut client = Client::builder(token, intents)
        .event_handler(FXHandler::new())
        .framework(StandardFramework::new())
        .await
        .expect("Failed to create bot client");

    if let Err(e) = client.start().await {
        println!("client error: {e:#?}");
    }
}