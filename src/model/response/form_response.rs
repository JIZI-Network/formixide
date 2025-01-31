use crate::controller::input::InputSource;
use crate::error::ParseError;
use crate::model::definition::form_definition::{FormDefinition, ParsedValue};
use std::collections::HashMap;
use uuid::Uuid;

pub struct FormResponse {
    id: Option<Uuid>,
    form_id: Uuid,
    map: HashMap<Uuid, Option<ParsedValue>>,
}

impl FormResponse {
    pub(crate) fn new(form_id: Uuid, map: HashMap<Uuid, Option<ParsedValue>>) -> FormResponse {
        FormResponse {
            id: None,
            form_id,
            map,
        }
    }

    pub fn from(
        form_definition: FormDefinition,
        input_source: &impl InputSource,
    ) -> Result<FormResponse, ParseError> {
        form_definition.parse(input_source)
    }
}
