mod items;
mod langs_strs;

use std::{convert::Infallible, env, net::SocketAddr};

use teloxide::dispatching::update_listeners;
use teloxide::prelude::*;
use teloxide::types::ParseMode;
use teloxide::utils::command::{BotCommand, ParseError};
use teloxide::utils::markdown::{code_inline, escape};
use teloxide::BotBuilder;

use tokio::sync::mpsc;
use warp::Filter;

use reqwest::StatusCode;

use items::{ENG_MAP, ITA_MAP};
use langs_strs::{ENG_STRS, ITA_STRS};

fn split_list(input: String) -> Result<(String, String), ParseError> {
    let input = input.to_lowercase();
    let (letter_str, lang) =
        input.split_at(input.find(' ').unwrap_or_else(|| input.len()));
    let lang = lang.trim_start();
    if lang.is_empty() && letter_str.len() == 1 {
        Ok((letter_str.to_owned(), "eng".to_owned()))
    } else if letter_str.len() == 1 {
        Ok((letter_str.to_owned(), lang.to_owned()))
    } else {
        Err(ParseError::Custom(
            "Error parsing the list command".to_owned().into(),
        ))
    }
}

#[derive(BotCommand)]
#[command(rename = "lowercase")]
enum Command {
    Help(String),
    Eng(String),
    Ita(String),
    #[command(parse_with = "split_list")]
    List(String, String),
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
    cx: UpdateWithCx<Message>,
    item: &str,
    map: &phf::Map<&'static str, phf::Set<&'static str>>,
    langs_strs: &phf::Map<&'static str, &'static str>,
) -> ResponseResult<()> {
    if item.is_empty() {
        return Ok(());
    }

    let map_value = map.get(item.to_lowercase().as_str());

    let output_str = if let Some(set) = map_value {
        let output_heading =
            langs_strs["results"].to_owned() + &code_inline(&item) + "\n\n";
        output_heading
            + &escape(
                &set.iter().map(|s| &**s).collect::<Vec<&str>>().join(""),
            )
    } else {
        code_inline(&item) + langs_strs["heading"]
    };

    cx.answer_str(output_str).await?;

    Ok(())
}

async fn run_help(
    cx: UpdateWithCx<Message>,
    lang: &str,
) -> ResponseResult<()> {
    let helper = if let "ita" = lang {
        ITA_STRS["help"]
    } else {
        ENG_STRS["help"]
    };

    cx.answer_str(helper).await?;

    Ok(())
}

async fn run_list(
    cx: UpdateWithCx<Message>,
    letter_str: &str,
    lang: &str,
) -> ResponseResult<()> {
    let letter = match letter_str.chars().next() {
        Some(l) => l,
        None => return Ok(()),
    };
    let items_str = match lang {
        "ita" => {
            ITA_STRS["list"].to_owned()
                + &code_inline(&letter_str.to_uppercase())
                + "\n\n"
                + &filter_keys(&ITA_MAP, &ITA_STRS, letter)
        }
        "eng" | "" => {
            ENG_STRS["list"].to_owned()
                + &code_inline(&letter_str.to_uppercase())
                + "\n\n"
                + &filter_keys(&ENG_MAP, &ENG_STRS, letter)
        }
        _ => return Ok(()),
    };

    cx.answer_str(items_str).await?;

    Ok(())
}

async fn handle_rejection(
    error: warp::Rejection,
) -> Result<impl warp::Reply, Infallible> {
    log::error!("Cannot process the request due to: {:?}", error);
    Ok(StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn webhook<'a>(
    bot: Bot,
) -> impl update_listeners::UpdateListener<Infallible> {
    // Heroku defines auto defines a port value
    let teloxide_token = env::var("TELOXIDE_TOKEN")
        .expect("TELOXIDE_TOKEN env variable missing");
    let port: u16 = env::var("PORT")
        .expect("PORT env variable missing")
        .parse()
        .expect("PORT value to be integer");
    // Heroku host example: "heroku-dungeons-and-dragons-index-bot.herokuapp.com"
    let host = env::var("HOST").expect("have HOST env variable");
    let path = format!("bot{}", teloxide_token);
    let url = format!("https://{}/{}", host, path);

    bot.set_webhook(url)
        .send()
        .await
        .expect("Cannot setup a webhook");

    let (tx, rx) = mpsc::unbounded_channel();

    let server = warp::post()
        .and(warp::path(path))
        .and(warp::body::json())
        .map(move |json: serde_json::Value| {
            let try_parse = match serde_json::from_str(&json.to_string()) {
                Ok(update) => Ok(update),
                Err(error) => {
                    log::error!(
                        "Cannot parse an update.\nError: {:?}\nValue: {}\n\
                       This is a bug in teloxide, please open an issue here: \
                       https://github.com/teloxide/teloxide/issues.",
                        error,
                        json
                    );
                    Err(error)
                }
            };
            if let Ok(update) = try_parse {
                tx.send(Ok(update))
                    .expect("Cannot send an incoming update from the webhook")
            }

            StatusCode::OK
        })
        .recover(handle_rejection);

    let serve = warp::serve(server);

    let address = format!("0.0.0.0:{}", port);
    tokio::spawn(serve.run(address.parse::<SocketAddr>().unwrap()));
    rx
}

async fn answer(
    cx: UpdateWithCx<Message>,
    command: Command,
) -> ResponseResult<()> {
    match command {
        Command::Help(ref lang) => run_help(cx, lang).await?,
        Command::Eng(ref item) => {
            run_item(cx, item, &ENG_MAP, &ENG_STRS).await?
        }
        Command::Ita(ref item) => {
            run_item(cx, item, &ITA_MAP, &ITA_STRS).await?
        }
        Command::List(ref letter, ref lang) => {
            run_list(cx, letter, lang).await?
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting dungeons-and-dragons-index-bot...");

    let bot = BotBuilder::new().parse_mode(ParseMode::MarkdownV2).build();

    let bot_name = "dungeons-and-dragons-index-bot";
    let cloned_bot = bot.clone();
    teloxide::commands_repl_with_listener(
        bot,
        bot_name,
        answer,
        webhook(cloned_bot).await,
    )
    .await;
}
