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

pub struct AccounteventApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> AccounteventApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> AccounteventApiClient<C> {
        AccounteventApiClient {
            configuration: configuration,
        }
    }
}

pub trait AccounteventApi {
    fn event_router_count_all_events_from_a_account_for_a_given_time_range(&self, x_account_api_key: &str, search: &str, _type: &str, from: &str, to: &str, source: &str, source_id: &str) -> Box<Future<Item = ::models::ModelsCountStat, Error = Error<serde_json::Value>>>;
    fn event_router_count_all_events_from_a_node_of_a_sub_account_for_a_given_time_range(&self, x_account_api_key: &str, search: &str, _type: &str, from: &str, to: &str, source: &str, source_id: &str) -> Box<Future<Item = ::models::ModelsCountStat, Error = Error<serde_json::Value>>>;
    fn event_router_get(&self, x_account_api_key: &str, event_id: &str) -> Box<Future<Item = ::models::ModelsQEvent, Error = Error<serde_json::Value>>>;
    fn event_router_get_all_event_timestamp_keys_of_a_sub_account_from_a_specific_node_for_a_given_time_range(&self, x_account_api_key: &str, search: &str, _type: &str, from: &str, to: &str, source: &str, source_id: &str) -> Box<Future<Item = Vec<::models::ModelsQEvent>, Error = Error<serde_json::Value>>>;
    fn event_router_get_all_events_of_a_account_from_a_specific_node(&self, x_account_api_key: &str) -> Box<Future<Item = Vec<::models::ModelsQEvent>, Error = Error<serde_json::Value>>>;
    fn event_router_get_event_in_node(&self, x_account_api_key: &str, event_id: &str) -> Box<Future<Item = ::models::ModelsQEvent, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>AccounteventApi for AccounteventApiClient<C> {
    fn event_router_count_all_events_from_a_account_for_a_given_time_range(&self, x_account_api_key: &str, search: &str, _type: &str, from: &str, to: &str, source: &str, source_id: &str) -> Box<Future<Item = ::models::ModelsCountStat, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("search", &search.to_string());
            query.append_pair("type", &_type.to_string());
            query.append_pair("from", &from.to_string());
            query.append_pair("to", &to.to_string());
            query.append_pair("source", &source.to_string());
            query.append_pair("sourceId", &source_id.to_string());
            query.finish()
        };
        let uri_str = format!("{}/account/event/count?{}", configuration.base_path, query_string);

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
            headers.set_raw("X-Account-ApiKey", x_account_api_key);
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

    fn event_router_count_all_events_from_a_node_of_a_sub_account_for_a_given_time_range(&self, x_account_api_key: &str, search: &str, _type: &str, from: &str, to: &str, source: &str, source_id: &str) -> Box<Future<Item = ::models::ModelsCountStat, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("search", &search.to_string());
            query.append_pair("type", &_type.to_string());
            query.append_pair("from", &from.to_string());
            query.append_pair("to", &to.to_string());
            query.append_pair("source", &source.to_string());
            query.append_pair("sourceId", &source_id.to_string());
            query.finish()
        };
        let uri_str = format!("{}/account/event/node/count?{}", configuration.base_path, query_string);

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
            headers.set_raw("X-Account-ApiKey", x_account_api_key);
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

    fn event_router_get(&self, x_account_api_key: &str, event_id: &str) -> Box<Future<Item = ::models::ModelsQEvent, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/account/event/{eventId}?{}", configuration.base_path, query_string, eventId=event_id);

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
            headers.set_raw("X-Account-ApiKey", x_account_api_key);
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
                let parsed: Result<::models::ModelsQEvent, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn event_router_get_all_event_timestamp_keys_of_a_sub_account_from_a_specific_node_for_a_given_time_range(&self, x_account_api_key: &str, search: &str, _type: &str, from: &str, to: &str, source: &str, source_id: &str) -> Box<Future<Item = Vec<::models::ModelsQEvent>, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("search", &search.to_string());
            query.append_pair("type", &_type.to_string());
            query.append_pair("from", &from.to_string());
            query.append_pair("to", &to.to_string());
            query.append_pair("source", &source.to_string());
            query.append_pair("sourceId", &source_id.to_string());
            query.finish()
        };
        let uri_str = format!("{}/account/event/node/timestampkeys?{}", configuration.base_path, query_string);

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
            headers.set_raw("X-Account-ApiKey", x_account_api_key);
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
                let parsed: Result<Vec<::models::ModelsQEvent>, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn event_router_get_all_events_of_a_account_from_a_specific_node(&self, x_account_api_key: &str) -> Box<Future<Item = Vec<::models::ModelsQEvent>, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/account/event/node?{}", configuration.base_path, query_string);

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
            headers.set_raw("X-Account-ApiKey", x_account_api_key);
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
                let parsed: Result<Vec<::models::ModelsQEvent>, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn event_router_get_event_in_node(&self, x_account_api_key: &str, event_id: &str) -> Box<Future<Item = ::models::ModelsQEvent, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/account/event/node/{eventId}?{}", configuration.base_path, query_string, eventId=event_id);

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
            headers.set_raw("X-Account-ApiKey", x_account_api_key);
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
                let parsed: Result<::models::ModelsQEvent, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

}
