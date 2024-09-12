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

    // Initialize the Telegram bot
    let bot = Bot::from_env();

    // Define the handler function
    teloxide::repl(bot, move |bot: Bot, message: Message| async move {
        let allowed_ids = vec![151137540, -1001641510706];
        let chat_id = message.chat.id.0;

        if !allowed_ids.contains(&chat_id) {
            log::info!("Chat ID: {}", chat_id);

            bot.send_message(message.chat.id, "Sorry, you can't use this bot")
                .send()
                .await?;
            return Ok(());
        }

        if let Some(text) = message.text() {
            // Check if the message starts with "/gpt"
            if text.starts_with("/gpt") {
                let openai_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
                
                // Extract the command argument after "/gpt"
                let query = text.trim_start_matches("/gpt").trim();

                if !query.is_empty() {
                    match get_chatgpt_response(&openai_key, query).await {
                        Ok(response) => {
                            let escaped_response = replace_code_markers(&response);
                            let formatted_code = format!("{}", escaped_response);

                            bot.send_message(message.chat.id, formatted_code)
                                .parse_mode(ParseMode::Html)
                                .send()
                                .await?;
                        }
                        Err(err) => {
                            bot.send_message(message.chat.id, format!("Error: {:?}", err))
                                .send()
                                .await?;
                        }
                    }
                } else {
                    bot.send_message(message.chat.id, "Please provide a query after the /gpt command.")
                        .send()
                        .await?;
                }
            }
        }
        respond(())
    })
    .await;
}