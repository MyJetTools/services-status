//pub URL: &'static str = "https://api.telegram.org/bot{api_key}/sendMessage?chat_id={chat_id}&text={text}";

use rust_extensions::StrOrString;
use teloxide::{
    requests::{Request, Requester},
    types::{ChatId, Recipient},
    Bot,
};

use crate::settings::TelegramSettings;

pub async fn send_message_to_telegram(
    telegram_settings: &TelegramSettings,
    message: StrOrString<'static>,
) {
    let bot = Bot::new(telegram_settings.api_key.as_str());

    let chat_id = ChatId(telegram_settings.chat_id);
    let request = bot.send_message(Recipient::Id(chat_id), message.as_str());

    let result = request.send().await;

    println!("Send telegram result: {:?}", result);
}
