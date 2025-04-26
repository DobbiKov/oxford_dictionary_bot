use oxford_dictionary_lib::search_dictionary;
use teloxide::{
    adaptors::DefaultParseMode, dispatching::dialogue::GetChatId, prelude::*, types::User,
    RequestError,
};

type HandlerResult = anyhow::Result<()>;

#[tokio::main]
async fn main() {
    println!("Startign bot");

    let bot = teloxide::Bot::from_env();

    let handler = Update::filter_message()
        .inspect(|u: Update| {
            eprintln!("{u:#?}");
        })
        .branch(
            Update::filter_message()
                .filter(|m: Message| m.text().map_or(false, |t| t.starts_with("/start")))
                .endpoint(start_command_handler),
        )
        .branch(Message::filter_text().endpoint(usual_text_handler));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    //teloxide::repl(bot, |bot: Bot, msg: Message| async move { Ok(()) }).await;
}

async fn start_command_handler(bot: teloxide::Bot, msg: Message) -> ResponseResult<()> {
    let msg_to_send = "";

    bot.send_message(msg.chat_id().unwrap(), msg_to_send).await;
    Ok(())
}

async fn usual_text_handler(bot: teloxide::Bot, msg: Message) -> ResponseResult<()> {
    let msg_txt = msg.text();
    if let Some(txt) = msg_txt {
        let word = txt.to_owned();
        match search_dictionary(&word).await {
            Ok(search_res) => match search_res {
                oxford_dictionary_lib::ParseLinkResult::ResultList(vec_r) => {
                    let mut txt_to_send =
                        "This word is not found, but there are possible variants:".to_string();
                    vec_r.iter().for_each(|el| {
                        let mut temp_str = el.to_owned();
                        temp_str.push_str("\\n");
                        txt_to_send.push_str(&temp_str);
                    });

                    bot.send_message(msg.chat_id().unwrap(), txt_to_send).await;
                }
                oxford_dictionary_lib::ParseLinkResult::MeaningsList(vec_r) => {
                    let mut txt_to_send =
                        "This word is not found, but there are possible variants:".to_string();
                    vec_r.iter().for_each(|el| {
                        let mut temp_str = el.to_owned();
                        temp_str.push_str("\\n");
                        txt_to_send.push_str(&temp_str)
                    });

                    bot.send_message(msg.chat_id().unwrap(), txt_to_send).await;
                }
                oxford_dictionary_lib::ParseLinkResult::None => {
                    bot.send_message(msg.chat_id().unwrap(), "No variants found for your word!");
                }
            },
            Err(e) => {
                bot.send_message(msg.chat_id().unwrap(), "Unexpected error!")
                    .await;
            }
        }
    }
    Ok(())
}
