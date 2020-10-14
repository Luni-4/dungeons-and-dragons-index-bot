use crate::langs_strs::SUPPORTED_LANGS;

macro_rules! missing_argument {
    ($input_vec: ident, $lang: expr) => {
        if $input_vec.len() == 1 {
            return Self::Help(Some($lang.to_owned()));
        }
    };
}

pub enum Command {
    Eng(String),
    Ita(String),
    Help(Option<String>),
    Unknown,
}

impl Command {
    pub fn analyze_command(text: &str) -> Self {
        let splits: Vec<&str> = text.splitn(2, ' ').collect();
        match splits[0] {
            "/eng" => {
                missing_argument!(splits, "eng");
                Self::Eng(splits[1].to_string())
            }
            "/ita" => {
                missing_argument!(splits, "ita");
                Self::Ita(splits[1].to_string())
            }
            "/help" => Self::Help(
                if splits.len() == 2 && SUPPORTED_LANGS.contains(&splits[1]) {
                    Some(splits[1].to_owned())
                } else if splits.len() == 1 {
                    Some("eng".to_owned())
                } else {
                    None
                },
            ),
            _ => Self::Unknown,
        }
    }
}
