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
pub struct UaparserUserAgent {
  #[serde(rename = "Family")]
  family: Option<String>,
  #[serde(rename = "Major")]
  major: Option<String>,
  #[serde(rename = "Minor")]
  minor: Option<String>,
  #[serde(rename = "Patch")]
  patch: Option<String>
}

impl UaparserUserAgent {
  pub fn new() -> UaparserUserAgent {
    UaparserUserAgent {
      family: None,
      major: None,
      minor: None,
      patch: None
    }
  }

  pub fn set_family(&mut self, family: String) {
    self.family = Some(family);
  }

  pub fn with_family(mut self, family: String) -> UaparserUserAgent {
    self.family = Some(family);
    self
  }

  pub fn family(&self) -> Option<&String> {
    self.family.as_ref()
  }

  pub fn reset_family(&mut self) {
    self.family = None;
  }

  pub fn set_major(&mut self, major: String) {
    self.major = Some(major);
  }

  pub fn with_major(mut self, major: String) -> UaparserUserAgent {
    self.major = Some(major);
    self
  }

  pub fn major(&self) -> Option<&String> {
    self.major.as_ref()
  }

  pub fn reset_major(&mut self) {
    self.major = None;
  }

  pub fn set_minor(&mut self, minor: String) {
    self.minor = Some(minor);
  }

  pub fn with_minor(mut self, minor: String) -> UaparserUserAgent {
    self.minor = Some(minor);
    self
  }

  pub fn minor(&self) -> Option<&String> {
    self.minor.as_ref()
  }

  pub fn reset_minor(&mut self) {
    self.minor = None;
  }

  pub fn set_patch(&mut self, patch: String) {
    self.patch = Some(patch);
  }

  pub fn with_patch(mut self, patch: String) -> UaparserUserAgent {
    self.patch = Some(patch);
    self
  }

  pub fn patch(&self) -> Option<&String> {
    self.patch.as_ref()
  }

  pub fn reset_patch(&mut self) {
    self.patch = None;
  }

}



