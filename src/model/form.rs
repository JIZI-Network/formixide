use crate::controller::input::InputSource;
use crate::error::ParseError;
use crate::model::field::{Field, IField};

struct Form {
    name: String,
    fields: Vec<Field>,
}

pub enum ParsedValue {
    Text(String),
}

impl Form {
    pub fn new(name: String, fields: Vec<Field>) -> Form {
        Form { name, fields }
    }

    fn parse<T>(
        &self,
        input_source: &dyn InputSource,
    ) -> Result<Vec<Option<ParsedValue>>, ParseError> {
        let mut vector: Vec<Option<ParsedValue>> = Vec::new();
        for field in self.fields {
            match field {
                Field::ShortText(field) => match input_source.get_value(&*field.get_name()) {
                    Some(value) => vector.push(Some(ParsedValue::Text(field.parse(value)?))),
                    None => vector.push(None),
                },
            }
        }
        Ok(vector)
    }
}
