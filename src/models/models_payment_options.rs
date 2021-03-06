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
pub struct ModelsPaymentOptions {
  #[serde(rename = "customerId")]
  customer_id: Option<String>,
  #[serde(rename = "paymentMethodId")]
  payment_method_id: Option<String>,
  #[serde(rename = "priceId")]
  price_id: Option<String>
}

impl ModelsPaymentOptions {
  pub fn new() -> ModelsPaymentOptions {
    ModelsPaymentOptions {
      customer_id: None,
      payment_method_id: None,
      price_id: None
    }
  }

  pub fn set_customer_id(&mut self, customer_id: String) {
    self.customer_id = Some(customer_id);
  }

  pub fn with_customer_id(mut self, customer_id: String) -> ModelsPaymentOptions {
    self.customer_id = Some(customer_id);
    self
  }

  pub fn customer_id(&self) -> Option<&String> {
    self.customer_id.as_ref()
  }

  pub fn reset_customer_id(&mut self) {
    self.customer_id = None;
  }

  pub fn set_payment_method_id(&mut self, payment_method_id: String) {
    self.payment_method_id = Some(payment_method_id);
  }

  pub fn with_payment_method_id(mut self, payment_method_id: String) -> ModelsPaymentOptions {
    self.payment_method_id = Some(payment_method_id);
    self
  }

  pub fn payment_method_id(&self) -> Option<&String> {
    self.payment_method_id.as_ref()
  }

  pub fn reset_payment_method_id(&mut self) {
    self.payment_method_id = None;
  }

  pub fn set_price_id(&mut self, price_id: String) {
    self.price_id = Some(price_id);
  }

  pub fn with_price_id(mut self, price_id: String) -> ModelsPaymentOptions {
    self.price_id = Some(price_id);
    self
  }

  pub fn price_id(&self) -> Option<&String> {
    self.price_id.as_ref()
  }

  pub fn reset_price_id(&mut self) {
    self.price_id = None;
  }

}



