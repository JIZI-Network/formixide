use crate::error::StorageError;
use crate::model::definition::form_definition::FormDefinition;
use crate::model::response::form_response::FormResponse;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait FormStorage {
    async fn init(&self) -> Result<(), StorageError>;
    async fn create_form(&self, form: &FormDefinition) -> Result<(), StorageError>;
    async fn load_form(&self, id: &Uuid) -> Result<FormDefinition, StorageError>;
    async fn load_all_forms(&self) -> Result<Vec<FormDefinition>, StorageError>;
    async fn load_forms_by_role(
        &self,
        role_name: &str,
    ) -> Result<Vec<FormDefinition>, StorageError>;
    async fn update_form(&self, form: &FormDefinition) -> Result<(), StorageError>;
    async fn delete_form(&self, id: &Uuid) -> Result<(), StorageError>;
    async fn create_response(&self, response: &FormResponse) -> Result<(), StorageError>;
    async fn load_response(&self, id: &Uuid) -> Result<FormResponse, StorageError>;
    async fn edit_response(&self, response: &FormResponse) -> Result<(), StorageError>;
    async fn delete_response(&self, id: &Uuid) -> Result<(), StorageError>;
}
