use crate::model::definition::form_definition::FormDefinition;
use crate::model::response::form_response::FormResponse;
use uuid::Uuid;

pub trait Authorizer<T> {
    fn can_create_form(&self, t: &T, form_definition: &FormDefinition) -> bool;
    fn can_read_form(&self, t: &T, form_definition: &FormDefinition) -> bool;

    fn can_read_all_forms(&self, t: &T) -> bool;

    fn can_read_forms_by_role(&self, t: &T, role: &str) -> bool;

    fn can_update_form(&self, t: &T, form_definition: &FormDefinition) -> bool;

    fn can_delete_form(&self, t: &T, id: &Uuid) -> bool;

    fn can_create_response(&self, t: &T, form_definition: &FormResponse) -> bool;

    fn can_read_response(&self, t: &T, form_definition: &FormResponse) -> bool;

    fn can_update_response(&self, t: &T, form_definition: &FormResponse) -> bool;

    fn can_delete_response(&self, t: &T, form_definition: &FormResponse) -> bool;
}
