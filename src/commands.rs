use crate::langs_strs::SUPPORTED_LANGS;

macro_rules! check_missing_argument {
    ($argument: ident, $lang: expr) => {
        if $argument.is_empty() {
            return Self::Help(Some($lang.to_owned()));
        }
    };
}

#[inline(always)]
fn read_first_char(s: &str) -> char {
    s.chars().next().unwrap().to_ascii_lowercase()
}

pub enum Command {
    Eng(String),
    Ita(String),
    List(char, Option<String>),
    Help(Option<String>),
    Unknown,
}

impl Command {
    #[inline(always)]
    pub fn analyze_command(text: &str) -> Self {
        let (command, argument) =
            text.split_at(text.find(' ').unwrap_or_else(|| text.len()));
        let argument = argument.trim_start();
        match command {
            "/eng" => {
                check_missing_argument!(argument, "eng");
                Self::Eng(argument.to_owned())
            }
            "/ita" => {
                check_missing_argument!(argument, "ita");
                Self::Ita(argument.to_owned())
            }
            "/list" => {
                check_missing_argument!(argument, "eng");
                let (letter_str, lang) = argument.split_at(
                    argument.find(' ').unwrap_or_else(|| argument.len()),
                );
                let lang = lang.trim_start();
                if lang.is_empty() && letter_str.len() == 1 {
                    Self::List(read_first_char(letter_str), None)
                } else if letter_str.len() == 1
                    && SUPPORTED_LANGS.contains(&lang)
                {
                    Self::List(
                        read_first_char(letter_str),
                        Some(lang.to_owned()),
                    )
                } else if SUPPORTED_LANGS.contains(&lang) {
                    Self::Help(Some(lang.to_owned()))
                } else {
                    Self::Help(Some("eng".to_owned()))
                }
            }
            "/help" => Self::Help(if argument.is_empty() {
                Some("eng".to_owned())
            } else if SUPPORTED_LANGS.contains(&argument) {
                Some(argument.to_owned())
            } else {
                None
            }),
            _ => Self::Unknown,
        }
    }
}
