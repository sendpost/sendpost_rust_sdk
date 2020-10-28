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
pub struct ModelsAipStat {
  #[serde(rename = "ipid")]
  ipid: Option<i64>,
  #[serde(rename = "publicIP")]
  public_ip: Option<String>,
  #[serde(rename = "stat")]
  stat: Option<::models::ModelsIpStat>
}

impl ModelsAipStat {
  pub fn new() -> ModelsAipStat {
    ModelsAipStat {
      ipid: None,
      public_ip: None,
      stat: None
    }
  }

  pub fn set_ipid(&mut self, ipid: i64) {
    self.ipid = Some(ipid);
  }

  pub fn with_ipid(mut self, ipid: i64) -> ModelsAipStat {
    self.ipid = Some(ipid);
    self
  }

  pub fn ipid(&self) -> Option<&i64> {
    self.ipid.as_ref()
  }

  pub fn reset_ipid(&mut self) {
    self.ipid = None;
  }

  pub fn set_public_ip(&mut self, public_ip: String) {
    self.public_ip = Some(public_ip);
  }

  pub fn with_public_ip(mut self, public_ip: String) -> ModelsAipStat {
    self.public_ip = Some(public_ip);
    self
  }

  pub fn public_ip(&self) -> Option<&String> {
    self.public_ip.as_ref()
  }

  pub fn reset_public_ip(&mut self) {
    self.public_ip = None;
  }

  pub fn set_stat(&mut self, stat: ::models::ModelsIpStat) {
    self.stat = Some(stat);
  }

  pub fn with_stat(mut self, stat: ::models::ModelsIpStat) -> ModelsAipStat {
    self.stat = Some(stat);
    self
  }

  pub fn stat(&self) -> Option<&::models::ModelsIpStat> {
    self.stat.as_ref()
  }

  pub fn reset_stat(&mut self) {
    self.stat = None;
  }

}


