use crate::error::ParseError;
pub enum Field {
    ShortText(FieldShortText),
}

pub trait IField<T> {
    fn get_name(&self) -> String;
    fn parse(&self, value: Vec<u8>) -> Result<T, ParseError>;
}

struct FieldShortText {
    name: String,
}

impl IField<String> for FieldShortText {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn parse(&self, value: Vec<u8>) -> Result<String, ParseError> {
        String::from_utf8(value).map_err(|err| ParseError {
            message: "Input string is not valid utf-8".to_string(),
        })
    }
}
