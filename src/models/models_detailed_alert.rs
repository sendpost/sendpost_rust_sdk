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
pub struct ModelsDetailedAlert {
  #[serde(rename = "active")]
  active: Option<bool>,
  #[serde(rename = "entityType")]
  entity_type: Option<String>,
  #[serde(rename = "entityValue")]
  entity_value: Option<String>,
  #[serde(rename = "id")]
  id: Option<i64>,
  #[serde(rename = "label")]
  label: Option<::models::ModelsAlertLabel>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "parameter")]
  parameter: Option<i64>,
  #[serde(rename = "provider")]
  provider: Option<String>,
  #[serde(rename = "threshold")]
  threshold: Option<String>
}

impl ModelsDetailedAlert {
  pub fn new() -> ModelsDetailedAlert {
    ModelsDetailedAlert {
      active: None,
      entity_type: None,
      entity_value: None,
      id: None,
      label: None,
      name: None,
      parameter: None,
      provider: None,
      threshold: None
    }
  }

  pub fn set_active(&mut self, active: bool) {
    self.active = Some(active);
  }

  pub fn with_active(mut self, active: bool) -> ModelsDetailedAlert {
    self.active = Some(active);
    self
  }

  pub fn active(&self) -> Option<&bool> {
    self.active.as_ref()
  }

  pub fn reset_active(&mut self) {
    self.active = None;
  }

  pub fn set_entity_type(&mut self, entity_type: String) {
    self.entity_type = Some(entity_type);
  }

  pub fn with_entity_type(mut self, entity_type: String) -> ModelsDetailedAlert {
    self.entity_type = Some(entity_type);
    self
  }

  pub fn entity_type(&self) -> Option<&String> {
    self.entity_type.as_ref()
  }

  pub fn reset_entity_type(&mut self) {
    self.entity_type = None;
  }

  pub fn set_entity_value(&mut self, entity_value: String) {
    self.entity_value = Some(entity_value);
  }

  pub fn with_entity_value(mut self, entity_value: String) -> ModelsDetailedAlert {
    self.entity_value = Some(entity_value);
    self
  }

  pub fn entity_value(&self) -> Option<&String> {
    self.entity_value.as_ref()
  }

  pub fn reset_entity_value(&mut self) {
    self.entity_value = None;
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i64) -> ModelsDetailedAlert {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i64> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_label(&mut self, label: ::models::ModelsAlertLabel) {
    self.label = Some(label);
  }

  pub fn with_label(mut self, label: ::models::ModelsAlertLabel) -> ModelsDetailedAlert {
    self.label = Some(label);
    self
  }

  pub fn label(&self) -> Option<&::models::ModelsAlertLabel> {
    self.label.as_ref()
  }

  pub fn reset_label(&mut self) {
    self.label = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> ModelsDetailedAlert {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_parameter(&mut self, parameter: i64) {
    self.parameter = Some(parameter);
  }

  pub fn with_parameter(mut self, parameter: i64) -> ModelsDetailedAlert {
    self.parameter = Some(parameter);
    self
  }

  pub fn parameter(&self) -> Option<&i64> {
    self.parameter.as_ref()
  }

  pub fn reset_parameter(&mut self) {
    self.parameter = None;
  }

  pub fn set_provider(&mut self, provider: String) {
    self.provider = Some(provider);
  }

  pub fn with_provider(mut self, provider: String) -> ModelsDetailedAlert {
    self.provider = Some(provider);
    self
  }

  pub fn provider(&self) -> Option<&String> {
    self.provider.as_ref()
  }

  pub fn reset_provider(&mut self) {
    self.provider = None;
  }

  pub fn set_threshold(&mut self, threshold: String) {
    self.threshold = Some(threshold);
  }

  pub fn with_threshold(mut self, threshold: String) -> ModelsDetailedAlert {
    self.threshold = Some(threshold);
    self
  }

  pub fn threshold(&self) -> Option<&String> {
    self.threshold.as_ref()
  }

  pub fn reset_threshold(&mut self) {
    self.threshold = None;
  }

}



