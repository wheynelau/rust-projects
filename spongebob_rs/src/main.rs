use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting sarcastic spongebob bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_message(msg.chat.id, spongebob_case(&msg.text().unwrap_or_default()))
        .await?;
        Ok(())
    })
    .await;
}

fn spongebob_case(input: &str) -> String {
    input.chars().enumerate().map(|(i, c)| {
        if i % 2 == 0 {
            c.to_ascii_uppercase()
        } else {
            c.to_ascii_lowercase()
        }
    }).collect()
}
