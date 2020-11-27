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
pub struct ModelsEditorTokenResponse {
  #[serde(rename = ".expires")]
  _expires: Option<String>,
  #[serde(rename = ".issued")]
  _issued: Option<String>,
  #[serde(rename = "access_token")]
  access_token: Option<String>,
  #[serde(rename = "as:client_id")]
  asclient_id: Option<String>,
  #[serde(rename = "as:region")]
  asregion: Option<String>,
  #[serde(rename = "expires_in")]
  expires_in: Option<i32>,
  #[serde(rename = "refresh_token")]
  refresh_token: Option<String>,
  #[serde(rename = "token_type")]
  token_type: Option<String>,
  #[serde(rename = "userName")]
  user_name: Option<String>
}

impl ModelsEditorTokenResponse {
  pub fn new() -> ModelsEditorTokenResponse {
    ModelsEditorTokenResponse {
      _expires: None,
      _issued: None,
      access_token: None,
      asclient_id: None,
      asregion: None,
      expires_in: None,
      refresh_token: None,
      token_type: None,
      user_name: None
    }
  }

  pub fn set__expires(&mut self, _expires: String) {
    self._expires = Some(_expires);
  }

  pub fn with__expires(mut self, _expires: String) -> ModelsEditorTokenResponse {
    self._expires = Some(_expires);
    self
  }

  pub fn _expires(&self) -> Option<&String> {
    self._expires.as_ref()
  }

  pub fn reset__expires(&mut self) {
    self._expires = None;
  }

  pub fn set__issued(&mut self, _issued: String) {
    self._issued = Some(_issued);
  }

  pub fn with__issued(mut self, _issued: String) -> ModelsEditorTokenResponse {
    self._issued = Some(_issued);
    self
  }

  pub fn _issued(&self) -> Option<&String> {
    self._issued.as_ref()
  }

  pub fn reset__issued(&mut self) {
    self._issued = None;
  }

  pub fn set_access_token(&mut self, access_token: String) {
    self.access_token = Some(access_token);
  }

  pub fn with_access_token(mut self, access_token: String) -> ModelsEditorTokenResponse {
    self.access_token = Some(access_token);
    self
  }

  pub fn access_token(&self) -> Option<&String> {
    self.access_token.as_ref()
  }

  pub fn reset_access_token(&mut self) {
    self.access_token = None;
  }

  pub fn set_asclient_id(&mut self, asclient_id: String) {
    self.asclient_id = Some(asclient_id);
  }

  pub fn with_asclient_id(mut self, asclient_id: String) -> ModelsEditorTokenResponse {
    self.asclient_id = Some(asclient_id);
    self
  }

  pub fn asclient_id(&self) -> Option<&String> {
    self.asclient_id.as_ref()
  }

  pub fn reset_asclient_id(&mut self) {
    self.asclient_id = None;
  }

  pub fn set_asregion(&mut self, asregion: String) {
    self.asregion = Some(asregion);
  }

  pub fn with_asregion(mut self, asregion: String) -> ModelsEditorTokenResponse {
    self.asregion = Some(asregion);
    self
  }

  pub fn asregion(&self) -> Option<&String> {
    self.asregion.as_ref()
  }

  pub fn reset_asregion(&mut self) {
    self.asregion = None;
  }

  pub fn set_expires_in(&mut self, expires_in: i32) {
    self.expires_in = Some(expires_in);
  }

  pub fn with_expires_in(mut self, expires_in: i32) -> ModelsEditorTokenResponse {
    self.expires_in = Some(expires_in);
    self
  }

  pub fn expires_in(&self) -> Option<&i32> {
    self.expires_in.as_ref()
  }

  pub fn reset_expires_in(&mut self) {
    self.expires_in = None;
  }

  pub fn set_refresh_token(&mut self, refresh_token: String) {
    self.refresh_token = Some(refresh_token);
  }

  pub fn with_refresh_token(mut self, refresh_token: String) -> ModelsEditorTokenResponse {
    self.refresh_token = Some(refresh_token);
    self
  }

  pub fn refresh_token(&self) -> Option<&String> {
    self.refresh_token.as_ref()
  }

  pub fn reset_refresh_token(&mut self) {
    self.refresh_token = None;
  }

  pub fn set_token_type(&mut self, token_type: String) {
    self.token_type = Some(token_type);
  }

  pub fn with_token_type(mut self, token_type: String) -> ModelsEditorTokenResponse {
    self.token_type = Some(token_type);
    self
  }

  pub fn token_type(&self) -> Option<&String> {
    self.token_type.as_ref()
  }

  pub fn reset_token_type(&mut self) {
    self.token_type = None;
  }

  pub fn set_user_name(&mut self, user_name: String) {
    self.user_name = Some(user_name);
  }

  pub fn with_user_name(mut self, user_name: String) -> ModelsEditorTokenResponse {
    self.user_name = Some(user_name);
    self
  }

  pub fn user_name(&self) -> Option<&String> {
    self.user_name.as_ref()
  }

  pub fn reset_user_name(&mut self) {
    self.user_name = None;
  }

}



