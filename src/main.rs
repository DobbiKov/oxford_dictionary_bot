use oxford_dictionary_lib::{search_dictionary, ParseLinkResult};
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
    let msg_to_send =
        "Welcome to the Oxford Dictionary Bot!\n Write me any word to learn it's meaning!";

    bot.send_message(msg.chat_id().unwrap(), msg_to_send).await;
    Ok(())
}

async fn usual_text_handler(bot: teloxide::Bot, msg: Message) -> ResponseResult<()> {
    let msg_txt = msg.text();
    if let Some(txt) = msg_txt {
        let word = txt.to_owned();
        match search_dictionary_wrapper(&word).await {
            Ok(search_res) => match search_res {
                oxford_dictionary_lib::ParseLinkResult::ResultList(vec_r) => {
                    let mut txt_to_send =
                        "This word is not found, but there are possible variants:\n".to_string();
                    vec_r.iter().for_each(|el| {
                        let mut temp_str = format!("{}\n", el);
                        txt_to_send.push_str(&temp_str);
                    });

                    bot.send_message(msg.chat_id().unwrap(), txt_to_send).await;
                }
                oxford_dictionary_lib::ParseLinkResult::MeaningsList(vec_r) => {
                    let mut txt_to_send = "The word is found, here's a meaning list:\n".to_string();
                    vec_r.iter().for_each(|el| {
                        let mut temp_str = el.to_owned();
                        temp_str.push_str("\n");
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

async fn search_dictionary_wrapper(
    word: &str,
) -> Result<ParseLinkResult, Box<dyn std::error::Error + Send + Sync>> {
    let word = word.to_string();
    tokio::task::spawn_blocking(move || {
        // This code can use non-Send types safely!
        let r = futures::executor::block_on(search_dictionary(&word));
        r
    })
    .await
    .unwrap()
}
