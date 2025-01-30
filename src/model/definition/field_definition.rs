use crate::error::ParseError;
use uuid::Uuid;

pub enum FieldDefinition {
    ShortText(FieldDefinitionShortText),
}

pub trait IFieldDefinition<T> {
    fn is_required(&self) -> bool;
    fn get_id(&self) -> Uuid;
    fn get_name(&self) -> String;
    fn parse(&self, value: Vec<u8>) -> Result<T, ParseError>;
}

pub struct FieldDefinitionShortText {
    id: Uuid,
    name: String,
    is_required: bool,
}

impl FieldDefinitionShortText {
    fn new(name: String, is_required: bool) -> FieldDefinitionShortText {
        FieldDefinitionShortText {
            id: Uuid::new_v4(),
            name,
            is_required,
        }
    }
}

impl IFieldDefinition<String> for FieldDefinitionShortText {
    fn is_required(&self) -> bool {
        self.is_required
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn parse(&self, value: Vec<u8>) -> Result<String, ParseError> {
        String::from_utf8(value).map_err(|_| ParseError {
            message: "Input string is not valid utf-8".to_string(),
        })
    }
}
