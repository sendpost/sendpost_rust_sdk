use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

use super::models::*;

mod accountevent_api;
pub use self::accountevent_api::{ AccounteventApi, AccounteventApiClient };
mod accountintegration_api;
pub use self::accountintegration_api::{ AccountintegrationApi, AccountintegrationApiClient };
mod accountip_api;
pub use self::accountip_api::{ AccountipApi, AccountipApiClient };
mod accountippool_api;
pub use self::accountippool_api::{ AccountippoolApi, AccountippoolApiClient };
mod accountipstat_api;
pub use self::accountipstat_api::{ AccountipstatApi, AccountipstatApiClient };
mod accountlabel_api;
pub use self::accountlabel_api::{ AccountlabelApi, AccountlabelApiClient };
mod accountmember_api;
pub use self::accountmember_api::{ AccountmemberApi, AccountmemberApiClient };
mod accountmessage_api;
pub use self::accountmessage_api::{ AccountmessageApi, AccountmessageApiClient };
mod accountrecipient_api;
pub use self::accountrecipient_api::{ AccountrecipientApi, AccountrecipientApiClient };
mod accountsmtpstat_api;
pub use self::accountsmtpstat_api::{ AccountsmtpstatApi, AccountsmtpstatApiClient };
mod accountstat_api;
pub use self::accountstat_api::{ AccountstatApi, AccountstatApiClient };
mod accountsubaccount_api;
pub use self::accountsubaccount_api::{ AccountsubaccountApi, AccountsubaccountApiClient };
mod accountvalidation_api;
pub use self::accountvalidation_api::{ AccountvalidationApi, AccountvalidationApiClient };
mod accountwebhook_api;
pub use self::accountwebhook_api::{ AccountwebhookApi, AccountwebhookApiClient };
mod smtp_api;
pub use self::smtp_api::{ SmtpApi, SmtpApiClient };
mod subaccountdomain_api;
pub use self::subaccountdomain_api::{ SubaccountdomainApi, SubaccountdomainApiClient };
mod subaccountemail_api;
pub use self::subaccountemail_api::{ SubaccountemailApi, SubaccountemailApiClient };
mod subaccountippool_api;
pub use self::subaccountippool_api::{ SubaccountippoolApi, SubaccountippoolApiClient };
mod subaccountsender_api;
pub use self::subaccountsender_api::{ SubaccountsenderApi, SubaccountsenderApiClient };
mod subaccountstat_api;
pub use self::subaccountstat_api::{ SubaccountstatApi, SubaccountstatApiClient };
mod subaccountsuppression_api;
pub use self::subaccountsuppression_api::{ SubaccountsuppressionApi, SubaccountsuppressionApiClient };

pub mod configuration;
pub mod client;
