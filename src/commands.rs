use crate::langs_strs::{ENG_STRS, ITA_STRS, SUPPORTED_LANGS};

macro_rules! input_error {
    ($input_vec: ident, $langs_strs: ident, $right_command: ident) => {
        if $input_vec.len() == 1 {
            Self::Error(
                "`".to_owned()
                    + &$input_vec[0].to_owned()
                    + "` "
                    + $langs_strs["error"],
            )
        } else {
            Self::$right_command($input_vec[1].to_string())
        }
    };
}

pub enum Command {
    Eng(String),
    Ita(String),
    Help(Option<String>),
    Error(String),
    Unknown,
}

impl Command {
    #[inline(always)]
    pub fn analyze_command(text: &str) -> Self {
        let splits: Vec<&str> = text.splitn(2, ' ').collect();
        match splits[0] {
            "/eng" => input_error!(splits, ENG_STRS, Eng),
            "/ita" => input_error!(splits, ITA_STRS, Ita),
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
