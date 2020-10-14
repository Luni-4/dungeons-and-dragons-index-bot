use crate::langs_strs::SUPPORTED_LANGS;

macro_rules! check_missing_argument {
    ($input_vec: ident, $lang: expr) => {
        if $input_vec.len() == 1 {
            return Self::Help(Some($lang.to_owned()));
        }
    };
}

#[inline(always)]
fn read_first_char(s: &str) -> char {
    s.chars().nth(0).unwrap().to_ascii_lowercase()
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
        let splits: Vec<&str> = text.splitn(2, ' ').collect();
        match splits[0] {
            "/eng" => {
                check_missing_argument!(splits, "eng");
                Self::Eng(splits[1].to_string())
            }
            "/ita" => {
                check_missing_argument!(splits, "ita");
                Self::Ita(splits[1].to_string())
            }
            "/list" => {
                check_missing_argument!(splits, "eng");
                let list_splits: Vec<&str> =
                    splits[1].splitn(2, ' ').collect();
                if list_splits.len() == 1 && list_splits[0].len() == 1 {
                    Self::List(read_first_char(&list_splits[0]), None)
                } else if list_splits.len() == 2
                    && list_splits[0].len() == 1
                    && SUPPORTED_LANGS.contains(&list_splits[1])
                {
                    Self::List(
                        read_first_char(&list_splits[0]),
                        Some(list_splits[1].to_owned()),
                    )
                } else if list_splits.len() == 2
                    && SUPPORTED_LANGS.contains(&list_splits[1])
                {
                    Self::Help(Some(list_splits[1].to_owned()))
                } else {
                    Self::Help(Some("eng".to_owned()))
                }
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
