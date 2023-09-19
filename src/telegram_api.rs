use crate::settings::TelegramSettings;

// Define a function to send a message using the Telegram Bot API
pub async fn send_message(telegram_settings: &TelegramSettings, env_info: &str, text: &str) {
    // Set the API endpoint and parameters
    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        telegram_settings.api_key
    );
    let params = [
        ("chat_id", telegram_settings.chat_id.to_string()),
        (
            "message_thread_id",
            telegram_settings.message_thread_id.to_string(),
        ),
        ("text", format!("EnvInfo:{env_info}. {text}")),
    ];

    // Create a client and send a POST request to the API

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let response = client.post(&url).form(&params).send().await;

    println!("{:?}", response);

    // Parse the JSON response
    //let telegram_response: TelegramResponse = response.json().await?;

    // Return the telegram response
}
