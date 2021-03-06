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
pub struct ModelsAccount {
  #[serde(rename = "apiKey")]
  api_key: Option<String>,
  #[serde(rename = "created")]
  created: Option<i64>,
  #[serde(rename = "currentEmailServiceProvider")]
  current_email_service_provider: Option<String>,
  #[serde(rename = "id")]
  id: Option<i64>,
  #[serde(rename = "industry")]
  industry: Option<String>,
  #[serde(rename = "isCanceled")]
  is_canceled: Option<bool>,
  #[serde(rename = "isLastPaymentFailed")]
  is_last_payment_failed: Option<bool>,
  #[serde(rename = "isUpgraded")]
  is_upgraded: Option<bool>,
  #[serde(rename = "lockThreshold")]
  lock_threshold: Option<i64>,
  #[serde(rename = "locked")]
  locked: Option<bool>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "onboardCFinished")]
  onboard_c_finished: Option<bool>,
  #[serde(rename = "onboardQAnswered")]
  onboard_q_answered: Option<bool>,
  #[serde(rename = "sendingVolumePerMonth")]
  sending_volume_per_month: Option<String>,
  #[serde(rename = "url")]
  url: Option<String>
}

impl ModelsAccount {
  pub fn new() -> ModelsAccount {
    ModelsAccount {
      api_key: None,
      created: None,
      current_email_service_provider: None,
      id: None,
      industry: None,
      is_canceled: None,
      is_last_payment_failed: None,
      is_upgraded: None,
      lock_threshold: None,
      locked: None,
      name: None,
      onboard_c_finished: None,
      onboard_q_answered: None,
      sending_volume_per_month: None,
      url: None
    }
  }

  pub fn set_api_key(&mut self, api_key: String) {
    self.api_key = Some(api_key);
  }

  pub fn with_api_key(mut self, api_key: String) -> ModelsAccount {
    self.api_key = Some(api_key);
    self
  }

  pub fn api_key(&self) -> Option<&String> {
    self.api_key.as_ref()
  }

  pub fn reset_api_key(&mut self) {
    self.api_key = None;
  }

  pub fn set_created(&mut self, created: i64) {
    self.created = Some(created);
  }

  pub fn with_created(mut self, created: i64) -> ModelsAccount {
    self.created = Some(created);
    self
  }

  pub fn created(&self) -> Option<&i64> {
    self.created.as_ref()
  }

  pub fn reset_created(&mut self) {
    self.created = None;
  }

  pub fn set_current_email_service_provider(&mut self, current_email_service_provider: String) {
    self.current_email_service_provider = Some(current_email_service_provider);
  }

  pub fn with_current_email_service_provider(mut self, current_email_service_provider: String) -> ModelsAccount {
    self.current_email_service_provider = Some(current_email_service_provider);
    self
  }

  pub fn current_email_service_provider(&self) -> Option<&String> {
    self.current_email_service_provider.as_ref()
  }

  pub fn reset_current_email_service_provider(&mut self) {
    self.current_email_service_provider = None;
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i64) -> ModelsAccount {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i64> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_industry(&mut self, industry: String) {
    self.industry = Some(industry);
  }

  pub fn with_industry(mut self, industry: String) -> ModelsAccount {
    self.industry = Some(industry);
    self
  }

  pub fn industry(&self) -> Option<&String> {
    self.industry.as_ref()
  }

  pub fn reset_industry(&mut self) {
    self.industry = None;
  }

  pub fn set_is_canceled(&mut self, is_canceled: bool) {
    self.is_canceled = Some(is_canceled);
  }

  pub fn with_is_canceled(mut self, is_canceled: bool) -> ModelsAccount {
    self.is_canceled = Some(is_canceled);
    self
  }

  pub fn is_canceled(&self) -> Option<&bool> {
    self.is_canceled.as_ref()
  }

  pub fn reset_is_canceled(&mut self) {
    self.is_canceled = None;
  }

  pub fn set_is_last_payment_failed(&mut self, is_last_payment_failed: bool) {
    self.is_last_payment_failed = Some(is_last_payment_failed);
  }

  pub fn with_is_last_payment_failed(mut self, is_last_payment_failed: bool) -> ModelsAccount {
    self.is_last_payment_failed = Some(is_last_payment_failed);
    self
  }

  pub fn is_last_payment_failed(&self) -> Option<&bool> {
    self.is_last_payment_failed.as_ref()
  }

  pub fn reset_is_last_payment_failed(&mut self) {
    self.is_last_payment_failed = None;
  }

  pub fn set_is_upgraded(&mut self, is_upgraded: bool) {
    self.is_upgraded = Some(is_upgraded);
  }

  pub fn with_is_upgraded(mut self, is_upgraded: bool) -> ModelsAccount {
    self.is_upgraded = Some(is_upgraded);
    self
  }

  pub fn is_upgraded(&self) -> Option<&bool> {
    self.is_upgraded.as_ref()
  }

  pub fn reset_is_upgraded(&mut self) {
    self.is_upgraded = None;
  }

  pub fn set_lock_threshold(&mut self, lock_threshold: i64) {
    self.lock_threshold = Some(lock_threshold);
  }

  pub fn with_lock_threshold(mut self, lock_threshold: i64) -> ModelsAccount {
    self.lock_threshold = Some(lock_threshold);
    self
  }

  pub fn lock_threshold(&self) -> Option<&i64> {
    self.lock_threshold.as_ref()
  }

  pub fn reset_lock_threshold(&mut self) {
    self.lock_threshold = None;
  }

  pub fn set_locked(&mut self, locked: bool) {
    self.locked = Some(locked);
  }

  pub fn with_locked(mut self, locked: bool) -> ModelsAccount {
    self.locked = Some(locked);
    self
  }

  pub fn locked(&self) -> Option<&bool> {
    self.locked.as_ref()
  }

  pub fn reset_locked(&mut self) {
    self.locked = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> ModelsAccount {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_onboard_c_finished(&mut self, onboard_c_finished: bool) {
    self.onboard_c_finished = Some(onboard_c_finished);
  }

  pub fn with_onboard_c_finished(mut self, onboard_c_finished: bool) -> ModelsAccount {
    self.onboard_c_finished = Some(onboard_c_finished);
    self
  }

  pub fn onboard_c_finished(&self) -> Option<&bool> {
    self.onboard_c_finished.as_ref()
  }

  pub fn reset_onboard_c_finished(&mut self) {
    self.onboard_c_finished = None;
  }

  pub fn set_onboard_q_answered(&mut self, onboard_q_answered: bool) {
    self.onboard_q_answered = Some(onboard_q_answered);
  }

  pub fn with_onboard_q_answered(mut self, onboard_q_answered: bool) -> ModelsAccount {
    self.onboard_q_answered = Some(onboard_q_answered);
    self
  }

  pub fn onboard_q_answered(&self) -> Option<&bool> {
    self.onboard_q_answered.as_ref()
  }

  pub fn reset_onboard_q_answered(&mut self) {
    self.onboard_q_answered = None;
  }

  pub fn set_sending_volume_per_month(&mut self, sending_volume_per_month: String) {
    self.sending_volume_per_month = Some(sending_volume_per_month);
  }

  pub fn with_sending_volume_per_month(mut self, sending_volume_per_month: String) -> ModelsAccount {
    self.sending_volume_per_month = Some(sending_volume_per_month);
    self
  }

  pub fn sending_volume_per_month(&self) -> Option<&String> {
    self.sending_volume_per_month.as_ref()
  }

  pub fn reset_sending_volume_per_month(&mut self) {
    self.sending_volume_per_month = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> ModelsAccount {
    self.url = Some(url);
    self
  }

  pub fn url(&self) -> Option<&String> {
    self.url.as_ref()
  }

  pub fn reset_url(&mut self) {
    self.url = None;
  }

}



