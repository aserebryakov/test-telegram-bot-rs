use frankenstein::AsyncTelegramApi;
use frankenstein::GetUpdatesParams;
use frankenstein::Message;
use frankenstein::SendMessageParams;
use frankenstein::AsyncApi;
use frankenstein::UpdateContent;
use frankenstein::reqwest::Client;
use std::env;

static BASE_API_URL: &str = "https://api.telegram.org/bot";

#[tokio::main]
async fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api_url = format!("{}{}", BASE_API_URL, token);
    // let api = AsyncApi::new_with_client(Client::builder().danger_accept_invalid_certs(true).build().unwrap(), api_url);
    let update_params_builder = GetUpdatesParams::builder();
    let mut update_params = update_params_builder.clone().build();

    let client= Client::builder().danger_accept_invalid_certs(true).build().unwrap();
    let api = AsyncApi::builder().api_url(api_url).client(client).build();

    loop {
        let result = api.get_updates(&update_params).await;

        println!("result: {:?}", result);

        match result {
            Ok(response) => {
                for update in response.result {
                    if let UpdateContent::Message(message) = update.content {
                        let api_clone = api.clone();

                        tokio::spawn(async move {
                            process_message(message, api_clone).await;
                        });

                        update_params = update_params_builder
                            .clone()
                            .offset(update.update_id + 1)
                            .build();
                    }
                }
            }
            Err(error) => {
                println!("Failed to get updates: {:?}", error);
            }
        }
    }
}

async fn process_message(message: Message, api: AsyncApi) {
    let send_message_params = SendMessageParams::builder()
        .chat_id(message.chat.id)
        .text(format!("hello, you wrote \'{}\'", message.text.unwrap())).build();

    if let Err(err) = api.send_message(&send_message_params).await {
        println!("Failed to send message: {:?}", err);
    }
}
