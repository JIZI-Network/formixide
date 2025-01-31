use crate::error::StorageError;
use crate::model::definition::field_definition::FieldDefinition;
use crate::model::definition::form_definition::FormDefinition;
use crate::model::response::form_response::FormResponse;
use crate::storage::FormStorage;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait FormEngine<T: Sync> {
    fn get_storage(&self) -> &impl FormStorage;

    async fn create_form(
        &self,
        name: String,
        fields: Vec<FieldDefinition>,
        roles: Vec<String>,
    ) -> Result<FormDefinition, StorageError> {
        let form_definition = FormDefinition::new(name, fields, roles);

        match self.get_storage().create_form(&form_definition).await {
            Ok(_) => Ok(form_definition),
            Err(e) => Err(e),
        }
    }

    async fn load_form(&self, id: &Uuid) -> Result<FormDefinition, StorageError> {
        let form_definition = self.get_storage().load_form(&id).await.map_err(|err| err)?;
        Ok(form_definition)
    }

    async fn load_all_forms(&self) -> Result<Vec<FormDefinition>, StorageError> {
        match self.get_storage().load_all_forms().await {
            Ok(vec) => Ok(vec),
            Err(e) => Err(e),
        }
    }

    async fn load_forms_by_role(
        &self,
        role: &str,
    ) -> Result<Vec<FormDefinition>, StorageError> {
        match self.get_storage().load_forms_by_role(role).await {
            Ok(vec) => Ok(vec),
            Err(e) => Err(e),
        }
    }

    async fn update_form(&self, form: &FormDefinition) -> Result<(), StorageError> {
        match self.get_storage().update_form(form).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    async fn delete_form(&self, id: &Uuid) -> Result<(), StorageError> {
        match self.get_storage().delete_form(id).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    async fn create_response(
        &self,
        form_response: &FormResponse,
    ) -> Result<(), StorageError> {
        match self.get_storage().create_response(&form_response).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    async fn load_response(&self, id: &Uuid) -> Result<FormResponse, StorageError> {
        let form_response = self
            .get_storage()
            .load_response(id)
            .await
            .map_err(|err| err)?;

        Ok(form_response)
    }

    async fn edit_response(&self, form_response: &FormResponse) -> Result<(), StorageError> {
        match self.get_storage().edit_response(form_response).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    async fn delete_response(&self, id: &Uuid) -> Result<(), StorageError> {
        match self.get_storage().delete_response(id).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
