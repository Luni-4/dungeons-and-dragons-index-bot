mod commands;
mod items;
mod langs_strs;

use std::env;

use futures::StreamExt;
use telegram_bot::prelude::*;
use telegram_bot::{Api, Error, Message, MessageKind, ParseMode, UpdateKind};

use commands::Command;
use items::{ENG_MAP, ITA_MAP};
use langs_strs::{ENG_STRS, ITA_STRS};

macro_rules! parse_markdown {
    ($message_api: expr) => {
        $message_api.parse_mode(ParseMode::Markdown)
    };
}

#[inline(always)]
fn filter_keys(
    map: &phf::Map<&'static str, phf::Set<&'static str>>,
    langs_strs: &phf::Map<&'static str, &'static str>,
    letter: char,
) -> String {
    let items_strs = map
        .keys()
        .filter(|s| s.starts_with(letter))
        .map(|s| &**s)
        .collect::<Vec<&str>>()
        .join("\n");

    if items_strs.is_empty() {
        langs_strs["empty"].to_owned()
    } else {
        items_strs
    }
}

async fn run_item(
    api: Api,
    message: Message,
    input: &str,
    map: &phf::Map<&'static str, phf::Set<&'static str>>,
    langs_strs: &phf::Map<&'static str, &'static str>,
) -> Result<(), Error> {
    let map_value = map.get(input.to_lowercase().as_str());

    if let Some(set) = map_value {
        let output_heading = langs_strs["results"].to_owned()
            + &" `".to_owned()
            + &input.to_owned()
            + "`\n\n";
        let output_str = output_heading
            + &set.iter().map(|s| &**s).collect::<Vec<&str>>().join("");
        api.send(parse_markdown!(message.text_reply(output_str)))
            .await?;
    } else {
        api.send(parse_markdown!(message.text_reply(
            "`".to_owned() + &input.to_owned() + "` " + langs_strs["heading"]
        )))
        .await?;
    }

    Ok(())
}

async fn run_help(
    api: Api,
    message: Message,
    lang: Option<&str>,
) -> Result<(), Error> {
    let helper = match lang {
        Some("eng") => ENG_STRS["help"],
        Some("ita") => ITA_STRS["help"],
        Some(_) | None => return Ok(()),
    };

    api.send(parse_markdown!(message.text_reply(helper)))
        .await?;

    Ok(())
}

async fn run_list(
    api: Api,
    message: Message,
    letter: char,
    lang: Option<&str>,
) -> Result<(), Error> {
    let letter_str =
        " `".to_owned() + &letter.to_string().to_uppercase() + "`\n\n";
    let items_str = match (letter, lang) {
        (letter, None) => {
            ENG_STRS["list"].to_owned()
                + &letter_str
                + &filter_keys(&ENG_MAP, &ENG_STRS, letter)
        }
        (letter, Some("ita")) => {
            ITA_STRS["list"].to_owned()
                + &letter_str
                + &filter_keys(&ITA_MAP, &ITA_STRS, letter)
        }
        (_, _) => return Ok(()),
    };

    api.send(parse_markdown!(message.text_reply(items_str)))
        .await?;

    Ok(())
}

async fn run_command(api: Api, message: Message) -> Result<(), Error> {
    if let MessageKind::Text { ref data, .. } = message.kind {
        let command = Command::analyze_command(data.as_str());
        match command {
            Command::Eng(ref input) => {
                run_item(api, message, input, &ENG_MAP, &ENG_STRS).await?
            }
            Command::Ita(ref input) => {
                run_item(api, message, input, &ITA_MAP, &ITA_STRS).await?
            }
            Command::List(letter, ref lang) => {
                run_list(api, message, letter, lang.as_deref()).await?
            }
            Command::Help(ref lang) => {
                run_help(api, message, lang.as_deref()).await?
            }
            _ => (),
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token =
        env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");

    let api = Api::new(token);
    let mut stream = api.stream();

    while let Some(update) = stream.next().await {
        match update {
            Ok(update) => {
                if let UpdateKind::Message(message) = update.kind {
                    run_command(api.clone(), message).await?;
                }
            }
            Err(err) => {
                dbg!("Update error = {}", err);
            }
        }
    }

    Ok(())
}
