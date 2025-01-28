use crate::error::Errors;
use crate::model::definition::form_definition::FormDefinition;
use crate::model::response::form_response::FormResponse;
use uuid::Uuid;

pub trait FormStorage {
    async fn init(&self) -> Result<(), Errors::StorageError()>;
    async fn create_form(&self, form: &FormDefinition) -> Result<(), Errors::StorageError>;
    async fn load_form(&self, id: &Uuid) -> Result<FormDefinition, Errors::StorageError>;
    async fn load_all_forms(&self) -> Result<Vec<FormDefinition>, Errors::StorageError>;
    async fn load_forms_by_role(
        &self,
        role_name: &str,
    ) -> Result<Vec<FormDefinition>, Errors::StorageError>;
    async fn update_form(&self, form: &FormDefinition) -> Result<(), Errors::StorageError>;
    async fn delete_form(&self, id: &Uuid) -> Result<(), Errors::StorageError>;
    async fn create_response(&self, response: &FormResponse) -> Result<(), Errors::StorageError>;
    async fn load_response(&self, id: &Uuid) -> Result<FormResponse, Errors::StorageError>;
    async fn edit_response(&self, response: &FormResponse) -> Result<(), Errors::StorageError>;
    async fn delete_response(&self, id: &Uuid) -> Result<(), Errors::StorageError>;
}
