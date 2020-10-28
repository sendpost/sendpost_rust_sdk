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
pub struct ModelsFrom {
  #[serde(rename = "email")]
  email: Option<String>,
  #[serde(rename = "name")]
  name: Option<String>
}

impl ModelsFrom {
  pub fn new() -> ModelsFrom {
    ModelsFrom {
      email: None,
      name: None
    }
  }

  pub fn set_email(&mut self, email: String) {
    self.email = Some(email);
  }

  pub fn with_email(mut self, email: String) -> ModelsFrom {
    self.email = Some(email);
    self
  }

  pub fn email(&self) -> Option<&String> {
    self.email.as_ref()
  }

  pub fn reset_email(&mut self) {
    self.email = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> ModelsFrom {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

}



