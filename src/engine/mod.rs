use crate::authorize::Authorizer;
use crate::error::Errors;
use crate::model::definition::field_definition::FieldDefinition;
use crate::model::definition::form_definition::FormDefinition;
use crate::model::response::form_response::FormResponse;
use crate::storage::FormStorage;
use uuid::Uuid;

pub trait FormEngine<T> {
    fn get_authorizer(&self) -> dyn Authorizer<T>;
    fn get_storage(&self) -> dyn FormStorage;

    async fn create_form(
        &self,
        t: &T,
        name: String,
        fields: Vec<FieldDefinition>,
        roles: Vec<String>,
    ) -> Result<FormDefinition, FormEngineError> {
        let form_definition = FormDefinition::new(name, fields, roles);

        if self.get_authorizer().can_create_form(t, &form_definition) {
            match self.get_storage().create_form(&form_definition).await {
                Ok(_) => Ok(form_definition),
                Err(e) => Err(FormEngineError::StorageError(e)),
            }
        } else {
            Err(FormEngineError::AuthorizationError(
                "Authorization failed.".to_string(),
            ))
        }
    }

    async fn load_form(&self, t: &T, id: &Uuid) -> Result<FormDefinition, FormEngineError> {
        let form_definition = self
            .get_storage()
            .load_form(&id)
            .await
            .map_err(|err| FormEngineError::StorageError(err))?;
        if self.get_authorizer().can_read_form(t, &form_definition) {
            Ok(form_definition)
        } else {
            Err(FormEngineError::AuthorizationError(
                "Authorization failed.".to_string(),
            ))
        }
    }

    async fn load_all_forms(&self, t: &T) -> Result<Vec<FormDefinition>, FormEngineError> {
        if self.get_authorizer().can_read_all_forms(t) {
            match self.get_storage().load_all_forms().await {
                Ok(vec) => Ok(vec),
                Err(e) => Err(FormEngineError::StorageError(e)),
            }
        } else {
            Err(FormEngineError::AuthorizationError(
                "Authorization failed.".to_string(),
            ))
        }
    }

    async fn load_forms_by_role(
        &self,
        t: &T,
        role: &str,
    ) -> Result<Vec<FormDefinition>, FormEngineError> {
        if self.get_authorizer().can_read_forms(t, role) {
            match self.get_storage().load_forms_by_role(role).await {
                Ok(vec) => Ok(vec),
                Err(e) => Err(FormEngineError::StorageError(e)),
            }
        } else {
            Err(FormEngineError::AuthorizationError(
                "Authorization failed.".to_string(),
            ))
        }
    }

    async fn update_form(&self, t: &T, form: &FormDefinition) -> Result<(), FormEngineError> {
        if self.get_authorizer().can_update_form(t, form) {
            match self.get_storage().update_form(form).await {
                Ok(_) => Ok(()),
                Err(e) => Err(FormEngineError::StorageError(e)),
            }
        } else {
            Err(FormEngineError::AuthorizationError(
                "Authorization failed.".to_string(),
            ))
        }
    }

    async fn delete_form(&self, t: &T, id: &Uuid) -> Result<(), FormEngineError> {
        if self.get_authorizer().can_delete_form(t, id) {
            match self.get_storage().delete_form(id).await {
                Ok(_) => Ok(()),
                Err(e) => Err(FormEngineError::StorageError(e)),
            }
        } else {
            Err(FormEngineError::AuthorizationError(
                "Authorization failed.".to_string(),
            ))
        }
    }

    async fn create_response(
        &self,
        t: &T,
        form_response: &FormResponse,
    ) -> Result<(), FormEngineError> {
        if self.get_authorizer().can_create_response(t, form_response) {
            match self.get_storage().create_response(&form_response).await {
                Ok(_) => Ok(()),
                Err(e) => Err(FormEngineError::StorageError(e)),
            }
        } else {
            Err(FormEngineError::AuthorizationError(
                "Authorization failed.".to_string(),
            ))
        }
    }

    async fn load_response(&self, t: &T, id: &Uuid) -> Result<FormResponse, FormEngineError> {
        let form_response = self
            .get_storage()
            .load_response(id)
            .await
            .map_err(|err| FormEngineError::StorageError(err))?;

        if self.get_authorizer().can_read_response(t, &form_response) {
            Ok(form_response)
        } else {
            Err(FormEngineError::AuthorizationError(
                "Authorization failed.".to_string(),
            ))
        }
    }

    async fn edit_response(
        &self,
        t: &T,
        form_response: &FormResponse,
    ) -> Result<(), FormEngineError> {
        if self.get_authorizer().can_update_response(t, form_response) {
            match self.get_storage().edit_response(form_response).await {
                Ok(_) => Ok(()),
                Err(e) => Err(FormEngineError::StorageError(e)),
            }
        } else {
            Err(FormEngineError::AuthorizationError(
                "Authorization failed.".to_string(),
            ))
        }
    }

    async fn delete_response(&self, t: &T, id: &Uuid) -> Result<(), FormEngineError> {
        if self.get_authorizer().can_delete_form(t, id) {
            match self.get_storage().delete_response(id).await {
                Ok(_) => Ok(()),
                Err(e) => Err(FormEngineError::StorageError(e)),
            }
        } else {
            Err(FormEngineError::AuthorizationError(
                "Authorization failed.".to_string(),
            ))
        }
    }
}
pub enum FormEngineError {
    StorageError(Errors::StorageError),
    AuthorizationError(Errors::AuthorizationError),
}
