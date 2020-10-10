mod eng_items;
mod ita_items;

use std::env;

use futures::StreamExt;
use telegram_bot::prelude::*;
use telegram_bot::{Api, Error, Message, MessageKind, ParseMode, Update, UpdateKind};

use eng_items::ENG_MAP;
use ita_items::ITA_MAP;

macro_rules! input_error {
    ($input_vec: ident, $right_command: ident) => {
        if $input_vec.len() == 1 {
            Self::Error(
                "`".to_owned()
                    + &$input_vec[0].to_owned()
                    + "` needs an `ITEM` argument\n\n \
                       Run the `/help` command for major details",
            )
        } else {
            Self::$right_command($input_vec[1].to_string())
        }
    };
}

macro_rules! parse_markdown {
    ($message_api: expr) => {
        $message_api.parse_mode(ParseMode::Markdown)
    };
}

enum Command {
    Eng(String),
    Ita(String),
    Error(String),
    Help,
    Unknown,
}

impl Command {
    #[inline(always)]
    pub fn analyze_command(text: &str) -> Self {
        let splits: Vec<&str> = text.splitn(2, ' ').collect();
        match splits[0] {
            "/eng" => input_error!(splits, Eng),
            "/ita" => input_error!(splits, Ita),
            "/help" => Self::Help,
            _ => Self::Unknown,
        }
    }
}

async fn run_item(
    api: Api,
    message: Message,
    input: &str,
    map: &phf::Map<&'static str, phf::Set<&'static str>>,
) -> Result<(), Error> {
    if !map.contains_key(input.to_lowercase().as_str()) {
        api.send(parse_markdown!(message.text_reply(
            "`".to_owned()
                + &input.to_owned()
                + "` is not a valid word or not contained in our dictionary"
        )))
        .await?;
    } else {
        let output_heading = "Results for the `".to_owned() + &input.to_owned() + "` item:\n\n";
        let output_str = output_heading
            + &map[input.to_lowercase().as_str()]
                .iter()
                .map(|s| &**s)
                .collect::<Vec<&str>>()
                .join("");
        api.send(parse_markdown!(message.text_reply(output_str)))
            .await?;
    }

    Ok(())
}

async fn run_command_error(api: Api, message: Message, error: &str) -> Result<(), Error> {
    api.send(parse_markdown!(message.text_reply(error))).await?;

    Ok(())
}

async fn run_help(api: Api, message: Message) -> Result<(), Error> {
    let helper = "This bot retrieves the pages of some chosen items from \
Dungeons and Dragons version 5.0 books.

It implements the following commands:

/eng `ITEM` --> Retrieves the pages of an `ITEM` passed as input using \
the English books
/ita `ITEM` --> Retrieves the pages of an `ITEM` passed as input using \
the Italian books
/help --> Prints this helper message";

    api.send(parse_markdown!(message.text_reply(helper)))
        .await?;

    Ok(())
}

async fn run_command(api: Api, message: Message) -> Result<(), Error> {
    if let MessageKind::Text { ref data, .. } = message.kind {
        let command = Command::analyze_command(data.as_str());
        match command {
            Command::Eng(ref input) => run_item(api, message, input, &ENG_MAP).await?,
            Command::Ita(ref input) => run_item(api, message, input, &ITA_MAP).await?,
            Command::Error(ref error) => run_command_error(api, message, error).await?,
            Command::Help => run_help(api, message).await?,
            _ => (),
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");

    let api = Api::new(token);
    let mut stream = api.stream();

    while let Some(update) = stream.next().await {
        match update {
            Ok(Update {
                kind: UpdateKind::Message(message),
                id: _,
            }) => {
                run_command(api.clone(), message).await?;
            }
            Ok(update_kind) => {
                dbg!(
                    "Received a non-supported kind of update = {:?}",
                    update_kind
                );
            }
            Err(err) => {
                dbg!("Update error = {}", err);
            }
        }
    }

    Ok(())
}