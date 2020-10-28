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
pub struct ModelsEWebhook {
  #[serde(rename = "clicked")]
  clicked: Option<bool>,
  #[serde(rename = "delivered")]
  delivered: Option<bool>,
  #[serde(rename = "dropped")]
  dropped: Option<bool>,
  #[serde(rename = "enabled")]
  enabled: Option<bool>,
  #[serde(rename = "hardBounced")]
  hard_bounced: Option<bool>,
  #[serde(rename = "opened")]
  opened: Option<bool>,
  #[serde(rename = "processed")]
  processed: Option<bool>,
  #[serde(rename = "softBounced")]
  soft_bounced: Option<bool>,
  #[serde(rename = "spam")]
  spam: Option<bool>,
  #[serde(rename = "unsubscribed")]
  unsubscribed: Option<bool>,
  #[serde(rename = "url")]
  url: Option<String>
}

impl ModelsEWebhook {
  pub fn new() -> ModelsEWebhook {
    ModelsEWebhook {
      clicked: None,
      delivered: None,
      dropped: None,
      enabled: None,
      hard_bounced: None,
      opened: None,
      processed: None,
      soft_bounced: None,
      spam: None,
      unsubscribed: None,
      url: None
    }
  }

  pub fn set_clicked(&mut self, clicked: bool) {
    self.clicked = Some(clicked);
  }

  pub fn with_clicked(mut self, clicked: bool) -> ModelsEWebhook {
    self.clicked = Some(clicked);
    self
  }

  pub fn clicked(&self) -> Option<&bool> {
    self.clicked.as_ref()
  }

  pub fn reset_clicked(&mut self) {
    self.clicked = None;
  }

  pub fn set_delivered(&mut self, delivered: bool) {
    self.delivered = Some(delivered);
  }

  pub fn with_delivered(mut self, delivered: bool) -> ModelsEWebhook {
    self.delivered = Some(delivered);
    self
  }

  pub fn delivered(&self) -> Option<&bool> {
    self.delivered.as_ref()
  }

  pub fn reset_delivered(&mut self) {
    self.delivered = None;
  }

  pub fn set_dropped(&mut self, dropped: bool) {
    self.dropped = Some(dropped);
  }

  pub fn with_dropped(mut self, dropped: bool) -> ModelsEWebhook {
    self.dropped = Some(dropped);
    self
  }

  pub fn dropped(&self) -> Option<&bool> {
    self.dropped.as_ref()
  }

  pub fn reset_dropped(&mut self) {
    self.dropped = None;
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.enabled = Some(enabled);
  }

  pub fn with_enabled(mut self, enabled: bool) -> ModelsEWebhook {
    self.enabled = Some(enabled);
    self
  }

  pub fn enabled(&self) -> Option<&bool> {
    self.enabled.as_ref()
  }

  pub fn reset_enabled(&mut self) {
    self.enabled = None;
  }

  pub fn set_hard_bounced(&mut self, hard_bounced: bool) {
    self.hard_bounced = Some(hard_bounced);
  }

  pub fn with_hard_bounced(mut self, hard_bounced: bool) -> ModelsEWebhook {
    self.hard_bounced = Some(hard_bounced);
    self
  }

  pub fn hard_bounced(&self) -> Option<&bool> {
    self.hard_bounced.as_ref()
  }

  pub fn reset_hard_bounced(&mut self) {
    self.hard_bounced = None;
  }

  pub fn set_opened(&mut self, opened: bool) {
    self.opened = Some(opened);
  }

  pub fn with_opened(mut self, opened: bool) -> ModelsEWebhook {
    self.opened = Some(opened);
    self
  }

  pub fn opened(&self) -> Option<&bool> {
    self.opened.as_ref()
  }

  pub fn reset_opened(&mut self) {
    self.opened = None;
  }

  pub fn set_processed(&mut self, processed: bool) {
    self.processed = Some(processed);
  }

  pub fn with_processed(mut self, processed: bool) -> ModelsEWebhook {
    self.processed = Some(processed);
    self
  }

  pub fn processed(&self) -> Option<&bool> {
    self.processed.as_ref()
  }

  pub fn reset_processed(&mut self) {
    self.processed = None;
  }

  pub fn set_soft_bounced(&mut self, soft_bounced: bool) {
    self.soft_bounced = Some(soft_bounced);
  }

  pub fn with_soft_bounced(mut self, soft_bounced: bool) -> ModelsEWebhook {
    self.soft_bounced = Some(soft_bounced);
    self
  }

  pub fn soft_bounced(&self) -> Option<&bool> {
    self.soft_bounced.as_ref()
  }

  pub fn reset_soft_bounced(&mut self) {
    self.soft_bounced = None;
  }

  pub fn set_spam(&mut self, spam: bool) {
    self.spam = Some(spam);
  }

  pub fn with_spam(mut self, spam: bool) -> ModelsEWebhook {
    self.spam = Some(spam);
    self
  }

  pub fn spam(&self) -> Option<&bool> {
    self.spam.as_ref()
  }

  pub fn reset_spam(&mut self) {
    self.spam = None;
  }

  pub fn set_unsubscribed(&mut self, unsubscribed: bool) {
    self.unsubscribed = Some(unsubscribed);
  }

  pub fn with_unsubscribed(mut self, unsubscribed: bool) -> ModelsEWebhook {
    self.unsubscribed = Some(unsubscribed);
    self
  }

  pub fn unsubscribed(&self) -> Option<&bool> {
    self.unsubscribed.as_ref()
  }

  pub fn reset_unsubscribed(&mut self) {
    self.unsubscribed = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> ModelsEWebhook {
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



