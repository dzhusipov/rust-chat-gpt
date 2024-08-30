use libs::libs::{get_chatgpt_response, replace_code_markers};
use teloxide::prelude::*;
use teloxide::types::ParseMode;


use std::env;
use dotenv::dotenv;

mod models;
mod libs;

#[tokio::main]
async fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    dotenv().ok();
    // Load environment variables

    // let openai_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set")

    // Initialize the Telegram bot
    let bot = Bot::from_env();

    // Define the handler function
    teloxide::repl(bot, move |bot: Bot, message: Message| async move {
        let openai_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
        
        if let Some(text) = message.text() {
            match get_chatgpt_response(&openai_key, text).await {
                Ok(response) => {
                    // Use bot to send a message
                    // Escape the response for MarkdownV2
                    let escaped_response = replace_code_markers(&response);
                    // log::info!("{}", escaped_response);
                    let formatted_code = format!("{}", escaped_response);
                    // log::info!("{}", formatted_code);
                    bot.send_message(message.chat.id, formatted_code)
                        .parse_mode(ParseMode::Html)
                        .send()
                        .await?;
                }
                Err(err) => {
                    // Use bot to send an error message
                    bot.send_message(message.chat.id, format!("Error: {:?}", err))
                        .send()
                        .await?;
                }
            }
        }
        respond(())
    })
    .await;
}