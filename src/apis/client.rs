use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  accountevent_api: Box<::apis::AccounteventApi>,
  accountintegration_api: Box<::apis::AccountintegrationApi>,
  accountip_api: Box<::apis::AccountipApi>,
  accountippool_api: Box<::apis::AccountippoolApi>,
  accountipstat_api: Box<::apis::AccountipstatApi>,
  accountlabel_api: Box<::apis::AccountlabelApi>,
  accountmember_api: Box<::apis::AccountmemberApi>,
  accountmessage_api: Box<::apis::AccountmessageApi>,
  accountrecipient_api: Box<::apis::AccountrecipientApi>,
  accountsmtpstat_api: Box<::apis::AccountsmtpstatApi>,
  accountstat_api: Box<::apis::AccountstatApi>,
  accountsubaccount_api: Box<::apis::AccountsubaccountApi>,
  accountvalidation_api: Box<::apis::AccountvalidationApi>,
  accountwebhook_api: Box<::apis::AccountwebhookApi>,
  smtp_api: Box<::apis::SmtpApi>,
  subaccountdomain_api: Box<::apis::SubaccountdomainApi>,
  subaccountemail_api: Box<::apis::SubaccountemailApi>,
  subaccountippool_api: Box<::apis::SubaccountippoolApi>,
  subaccountsender_api: Box<::apis::SubaccountsenderApi>,
  subaccountstat_api: Box<::apis::SubaccountstatApi>,
  subaccountsuppression_api: Box<::apis::SubaccountsuppressionApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      accountevent_api: Box::new(::apis::AccounteventApiClient::new(rc.clone())),
      accountintegration_api: Box::new(::apis::AccountintegrationApiClient::new(rc.clone())),
      accountip_api: Box::new(::apis::AccountipApiClient::new(rc.clone())),
      accountippool_api: Box::new(::apis::AccountippoolApiClient::new(rc.clone())),
      accountipstat_api: Box::new(::apis::AccountipstatApiClient::new(rc.clone())),
      accountlabel_api: Box::new(::apis::AccountlabelApiClient::new(rc.clone())),
      accountmember_api: Box::new(::apis::AccountmemberApiClient::new(rc.clone())),
      accountmessage_api: Box::new(::apis::AccountmessageApiClient::new(rc.clone())),
      accountrecipient_api: Box::new(::apis::AccountrecipientApiClient::new(rc.clone())),
      accountsmtpstat_api: Box::new(::apis::AccountsmtpstatApiClient::new(rc.clone())),
      accountstat_api: Box::new(::apis::AccountstatApiClient::new(rc.clone())),
      accountsubaccount_api: Box::new(::apis::AccountsubaccountApiClient::new(rc.clone())),
      accountvalidation_api: Box::new(::apis::AccountvalidationApiClient::new(rc.clone())),
      accountwebhook_api: Box::new(::apis::AccountwebhookApiClient::new(rc.clone())),
      smtp_api: Box::new(::apis::SmtpApiClient::new(rc.clone())),
      subaccountdomain_api: Box::new(::apis::SubaccountdomainApiClient::new(rc.clone())),
      subaccountemail_api: Box::new(::apis::SubaccountemailApiClient::new(rc.clone())),
      subaccountippool_api: Box::new(::apis::SubaccountippoolApiClient::new(rc.clone())),
      subaccountsender_api: Box::new(::apis::SubaccountsenderApiClient::new(rc.clone())),
      subaccountstat_api: Box::new(::apis::SubaccountstatApiClient::new(rc.clone())),
      subaccountsuppression_api: Box::new(::apis::SubaccountsuppressionApiClient::new(rc.clone())),
    }
  }

  pub fn accountevent_api(&self) -> &::apis::AccounteventApi{
    self.accountevent_api.as_ref()
  }

  pub fn accountintegration_api(&self) -> &::apis::AccountintegrationApi{
    self.accountintegration_api.as_ref()
  }

  pub fn accountip_api(&self) -> &::apis::AccountipApi{
    self.accountip_api.as_ref()
  }

  pub fn accountippool_api(&self) -> &::apis::AccountippoolApi{
    self.accountippool_api.as_ref()
  }

  pub fn accountipstat_api(&self) -> &::apis::AccountipstatApi{
    self.accountipstat_api.as_ref()
  }

  pub fn accountlabel_api(&self) -> &::apis::AccountlabelApi{
    self.accountlabel_api.as_ref()
  }

  pub fn accountmember_api(&self) -> &::apis::AccountmemberApi{
    self.accountmember_api.as_ref()
  }

  pub fn accountmessage_api(&self) -> &::apis::AccountmessageApi{
    self.accountmessage_api.as_ref()
  }

  pub fn accountrecipient_api(&self) -> &::apis::AccountrecipientApi{
    self.accountrecipient_api.as_ref()
  }

  pub fn accountsmtpstat_api(&self) -> &::apis::AccountsmtpstatApi{
    self.accountsmtpstat_api.as_ref()
  }

  pub fn accountstat_api(&self) -> &::apis::AccountstatApi{
    self.accountstat_api.as_ref()
  }

  pub fn accountsubaccount_api(&self) -> &::apis::AccountsubaccountApi{
    self.accountsubaccount_api.as_ref()
  }

  pub fn accountvalidation_api(&self) -> &::apis::AccountvalidationApi{
    self.accountvalidation_api.as_ref()
  }

  pub fn accountwebhook_api(&self) -> &::apis::AccountwebhookApi{
    self.accountwebhook_api.as_ref()
  }

  pub fn smtp_api(&self) -> &::apis::SmtpApi{
    self.smtp_api.as_ref()
  }

  pub fn subaccountdomain_api(&self) -> &::apis::SubaccountdomainApi{
    self.subaccountdomain_api.as_ref()
  }

  pub fn subaccountemail_api(&self) -> &::apis::SubaccountemailApi{
    self.subaccountemail_api.as_ref()
  }

  pub fn subaccountippool_api(&self) -> &::apis::SubaccountippoolApi{
    self.subaccountippool_api.as_ref()
  }

  pub fn subaccountsender_api(&self) -> &::apis::SubaccountsenderApi{
    self.subaccountsender_api.as_ref()
  }

  pub fn subaccountstat_api(&self) -> &::apis::SubaccountstatApi{
    self.subaccountstat_api.as_ref()
  }

  pub fn subaccountsuppression_api(&self) -> &::apis::SubaccountsuppressionApi{
    self.subaccountsuppression_api.as_ref()
  }


}
