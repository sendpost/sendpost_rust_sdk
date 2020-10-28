/* 
 * SendPost API
 *
 * SendPost API to send transactional emails reliably
 *
 * OpenAPI spec version: 1.0.0
 * Contact: hello@sendx.io
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::rc::Rc;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::collections::HashMap;

use hyper;
use serde_json;
use futures;
use futures::{Future, Stream};

use hyper::header::UserAgent;

use super::{Error, configuration};

pub struct SubaccountdomainApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> SubaccountdomainApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> SubaccountdomainApiClient<C> {
        SubaccountdomainApiClient {
            configuration: configuration,
        }
    }
}

pub trait SubaccountdomainApi {
    fn domain_router_count(&self, x_sub_account_api_key: &str) -> Box<Future<Item = ::models::ModelsCountStat, Error = Error<serde_json::Value>>>;
    fn domain_router_create(&self, x_sub_account_api_key: &str, body: ::models::ModelsEDomain) -> Box<Future<Item = ::models::ModelsDomain, Error = Error<serde_json::Value>>>;
    fn domain_router_delete(&self, x_sub_account_api_key: &str, domain_id: i64) -> Box<Future<Item = ::models::ModelsDeleteResponse, Error = Error<serde_json::Value>>>;
    fn domain_router_get(&self, x_sub_account_api_key: &str, domain_id: i64) -> Box<Future<Item = ::models::ModelsDomain, Error = Error<serde_json::Value>>>;
    fn domain_router_get_all(&self, x_sub_account_api_key: &str, offset: i64, limit: i64, search: &str) -> Box<Future<Item = Vec<::models::ModelsDomain>, Error = Error<serde_json::Value>>>;
    fn domain_router_update(&self, x_sub_account_api_key: &str, domain_id: i64, body: ::models::ModelsEDomain) -> Box<Future<Item = ::models::ModelsDomain, Error = Error<serde_json::Value>>>;
    fn domain_router_verify(&self, x_sub_account_api_key: &str, domain_id: i64) -> Box<Future<Item = ::models::ModelsDomain, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>SubaccountdomainApi for SubaccountdomainApiClient<C> {
    fn domain_router_count(&self, x_sub_account_api_key: &str) -> Box<Future<Item = ::models::ModelsCountStat, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/subaccount/domain/count?{}", configuration.base_path, query_string);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        {
            let mut headers = req.headers_mut();
            headers.set_raw("X-SubAccount-ApiKey", x_sub_account_api_key);
        }



        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::ModelsCountStat, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn domain_router_create(&self, x_sub_account_api_key: &str, body: ::models::ModelsEDomain) -> Box<Future<Item = ::models::ModelsDomain, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/subaccount/domain/?{}", configuration.base_path, query_string);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        {
            let mut headers = req.headers_mut();
            headers.set_raw("X-SubAccount-ApiKey", x_sub_account_api_key);
        }


        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::ModelsDomain, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn domain_router_delete(&self, x_sub_account_api_key: &str, domain_id: i64) -> Box<Future<Item = ::models::ModelsDeleteResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/subaccount/domain/{domainId}?{}", configuration.base_path, query_string, domainId=domain_id);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        {
            let mut headers = req.headers_mut();
            headers.set_raw("X-SubAccount-ApiKey", x_sub_account_api_key);
        }



        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::ModelsDeleteResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn domain_router_get(&self, x_sub_account_api_key: &str, domain_id: i64) -> Box<Future<Item = ::models::ModelsDomain, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/subaccount/domain/{domainId}?{}", configuration.base_path, query_string, domainId=domain_id);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        {
            let mut headers = req.headers_mut();
            headers.set_raw("X-SubAccount-ApiKey", x_sub_account_api_key);
        }



        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::ModelsDomain, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn domain_router_get_all(&self, x_sub_account_api_key: &str, offset: i64, limit: i64, search: &str) -> Box<Future<Item = Vec<::models::ModelsDomain>, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("offset", &offset.to_string());
            query.append_pair("limit", &limit.to_string());
            query.append_pair("search", &search.to_string());
            query.finish()
        };
        let uri_str = format!("{}/subaccount/domain/?{}", configuration.base_path, query_string);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        {
            let mut headers = req.headers_mut();
            headers.set_raw("X-SubAccount-ApiKey", x_sub_account_api_key);
        }



        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<Vec<::models::ModelsDomain>, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn domain_router_update(&self, x_sub_account_api_key: &str, domain_id: i64, body: ::models::ModelsEDomain) -> Box<Future<Item = ::models::ModelsDomain, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Put;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/subaccount/domain/{domainId}?{}", configuration.base_path, query_string, domainId=domain_id);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        {
            let mut headers = req.headers_mut();
            headers.set_raw("X-SubAccount-ApiKey", x_sub_account_api_key);
        }


        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::ModelsDomain, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn domain_router_verify(&self, x_sub_account_api_key: &str, domain_id: i64) -> Box<Future<Item = ::models::ModelsDomain, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/subaccount/domain/{domainId}/verify?{}", configuration.base_path, query_string, domainId=domain_id);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        {
            let mut headers = req.headers_mut();
            headers.set_raw("X-SubAccount-ApiKey", x_sub_account_api_key);
        }



        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::ModelsDomain, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

}
