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
pub struct ModelsEInvitation {
  #[serde(rename = "fromEmail")]
  from_email: Option<String>,
  #[serde(rename = "toEmail")]
  to_email: Option<String>
}

impl ModelsEInvitation {
  pub fn new() -> ModelsEInvitation {
    ModelsEInvitation {
      from_email: None,
      to_email: None
    }
  }

  pub fn set_from_email(&mut self, from_email: String) {
    self.from_email = Some(from_email);
  }

  pub fn with_from_email(mut self, from_email: String) -> ModelsEInvitation {
    self.from_email = Some(from_email);
    self
  }

  pub fn from_email(&self) -> Option<&String> {
    self.from_email.as_ref()
  }

  pub fn reset_from_email(&mut self) {
    self.from_email = None;
  }

  pub fn set_to_email(&mut self, to_email: String) {
    self.to_email = Some(to_email);
  }

  pub fn with_to_email(mut self, to_email: String) -> ModelsEInvitation {
    self.to_email = Some(to_email);
    self
  }

  pub fn to_email(&self) -> Option<&String> {
    self.to_email.as_ref()
  }

  pub fn reset_to_email(&mut self) {
    self.to_email = None;
  }

}



