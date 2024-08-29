# Rust Telegram ChatGPT bot

This project is a Telegram bot that interacts with OpenAI's GPT models to generate code snippets and format them correctly using HTML markup for Telegram. It supports various programming languages and allows code snippets to be displayed with correct formatting in chat messages.

## Features

- Generates code snippets using OpenAI's GPT models.
- Formats code blocks with correct HTML tags for Telegram display.
- Converts Markdown formatting to HTML, such as bold text using `**bold**` to `<b>bold</b>`.
- Supports syntax highlighting by specifying languages (e.g., `objective-c`, `cpp`, etc.).

## Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)
- [Telegram Bot API Token](https://core.telegram.org/bots#6-botfather)
- OpenAI API Key

## Setup

1. **Clone the Repository**

   ```bash
   git clone https://github.com/yourusername/telegram-code-formatter-bot.git
   cd telegram-code-formatter-bot

2. **Set Up Environment Variables**
Create a .env file in the root directory with the following content:
   ```bash
   TELEGRAM_BOT_TOKEN=your_telegram_bot_token
   OPENAI_API_KEY=your_openai_api_key
   ```
Replace your_telegram_bot_token with your Telegram Bot API token and your_openai_api_key with your OpenAI API key.

3. **Install Dependencies**
Use Cargo to install the necessary dependencies:
   ```bash
   cargo build
   ```
4.	**Configure Logging**
Ensure you have a logging configuration file config/log4rs.yaml set up for logging purposes. Here’s a basic setup example:
   ```yaml
   refresh_rate: 30 seconds
   appenders:
    stdout:
      kind: console
  
   root:
     level: info
     appenders:
       - stdout
   ```

## Running the Bot

To run the bot, use the following command:
    ```bash
    cargo run
    ```
The bot will start and begin listening for messages on Telegram. It will respond to any text message by querying OpenAI’s API, formatting the response, and sending it back in a formatted manner.

How It Works

	1.	Message Handling: The bot listens for incoming text messages.
	2.	OpenAI GPT Integration: When a message is received, the bot sends the content to OpenAI’s API for generating responses.
	3.	Code Formatting: The bot identifies code snippets and formats them using HTML tags for proper display on Telegram.
	4.	Special Formatting: It also converts Markdown syntax like **bold** into HTML <b>bold</b> for bold text.

## Running with Docker

You can also build and run the bot using Docker to simplify deployment.
Building the Docker Image

In the project directory, run:
   ```bash
   docker run --rm --env-file .env telegram-code-formatter-bot
   ```
Dockerfile Explanation

	•	Build Stage: The Dockerfile uses a multi-stage build to create the application with Alpine Linux as the base image. It installs Rust, builds the application, and prepares it for deployment.
	•	Runtime Stage: The runtime image is based on Alpine Linux, which copies the built binary and necessary configuration files from the build stage.
	•	Environment Setup: It uses environment variables from the .env file for configuring the bot.
	•	Command to Run: The container runs the compiled binary to start the bot.

Contributing

Feel free to open issues or submit pull requests for improvements and bug fixes. Contributions are welcome!
