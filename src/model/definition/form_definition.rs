use super::field_definition::{FieldDefinition, IFieldDefinition};
use crate::controller::input::InputSource;
use crate::error::Errors;
use crate::model::response::form_response::FormResponse;
use std::collections::HashMap;
use uuid::Uuid;

pub struct FormDefinition {
    pub id: Uuid,
    pub name: String,
    pub fields: Vec<FieldDefinition>,
    pub roles: Vec<String>,
}

pub enum ParsedValue {
    Text(String),
}

impl FormDefinition {
    pub fn new(name: String, fields: Vec<FieldDefinition>, roles: Vec<String>) -> FormDefinition {
        FormDefinition {
            id: Uuid::new_v4(),
            name,
            fields,
            roles,
        }
    }

    pub fn parse<T>(
        &self,
        input_source: &dyn InputSource,
    ) -> Result<FormResponse, Errors::ParseError> {
        let mut map: HashMap<Uuid, Option<ParsedValue>> = HashMap::new();
        for field in self.fields {
            match field {
                FieldDefinition::ShortText(field) => {
                    match input_source.get_value(&field.get_id()) {
                        Some(value) => {
                            map.insert(
                                field.get_id(),
                                Some(ParsedValue::Text(field.parse(value)?)),
                            );
                        }
                        None => {
                            map.insert(field.get_id(), None);
                        }
                    }
                }
            }
        }
        Ok(FormResponse::new(self.id, map))
    }
}
