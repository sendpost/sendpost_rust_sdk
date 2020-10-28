/* 
 * SendPost API
 *
 * SendPost API to send transactional emails reliably
 *
 * OpenAPI spec version: 1.0.0
 * Contact: hello@sendx.io
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelsValidation {
  #[serde(rename = "created")]
  created: Option<i64>,
  #[serde(rename = "email")]
  email: Option<String>,
  #[serde(rename = "error")]
  error: Option<String>,
  #[serde(rename = "id")]
  id: Option<i64>,
  #[serde(rename = "reason")]
  reason: Option<::models::ModelsValidationReason>
}

impl ModelsValidation {
  pub fn new() -> ModelsValidation {
    ModelsValidation {
      created: None,
      email: None,
      error: None,
      id: None,
      reason: None
    }
  }

  pub fn set_created(&mut self, created: i64) {
    self.created = Some(created);
  }

  pub fn with_created(mut self, created: i64) -> ModelsValidation {
    self.created = Some(created);
    self
  }

  pub fn created(&self) -> Option<&i64> {
    self.created.as_ref()
  }

  pub fn reset_created(&mut self) {
    self.created = None;
  }

  pub fn set_email(&mut self, email: String) {
    self.email = Some(email);
  }

  pub fn with_email(mut self, email: String) -> ModelsValidation {
    self.email = Some(email);
    self
  }

  pub fn email(&self) -> Option<&String> {
    self.email.as_ref()
  }

  pub fn reset_email(&mut self) {
    self.email = None;
  }

  pub fn set_error(&mut self, error: String) {
    self.error = Some(error);
  }

  pub fn with_error(mut self, error: String) -> ModelsValidation {
    self.error = Some(error);
    self
  }

  pub fn error(&self) -> Option<&String> {
    self.error.as_ref()
  }

  pub fn reset_error(&mut self) {
    self.error = None;
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i64) -> ModelsValidation {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i64> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_reason(&mut self, reason: ::models::ModelsValidationReason) {
    self.reason = Some(reason);
  }

  pub fn with_reason(mut self, reason: ::models::ModelsValidationReason) -> ModelsValidation {
    self.reason = Some(reason);
    self
  }

  pub fn reason(&self) -> Option<&::models::ModelsValidationReason> {
    self.reason.as_ref()
  }

  pub fn reset_reason(&mut self) {
    self.reason = None;
  }

}



