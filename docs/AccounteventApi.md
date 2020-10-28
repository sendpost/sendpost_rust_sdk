# \AccounteventApi

All URIs are relative to *https://api.sendpost.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**event_router_count_all_events_from_a_account_for_a_given_time_range**](AccounteventApi.md#event_router_count_all_events_from_a_account_for_a_given_time_range) | **Get** /account/event/count | 
[**event_router_count_all_events_from_a_node_of_a_sub_account_for_a_given_time_range**](AccounteventApi.md#event_router_count_all_events_from_a_node_of_a_sub_account_for_a_given_time_range) | **Get** /account/event/node/count | 
[**event_router_get**](AccounteventApi.md#event_router_get) | **Get** /account/event/{eventId} | 
[**event_router_get_all_event_timestamp_keys_of_a_sub_account_from_a_specific_node_for_a_given_time_range**](AccounteventApi.md#event_router_get_all_event_timestamp_keys_of_a_sub_account_from_a_specific_node_for_a_given_time_range) | **Get** /account/event/node/timestampkeys | 
[**event_router_get_all_events_of_a_account_from_a_specific_node**](AccounteventApi.md#event_router_get_all_events_of_a_account_from_a_specific_node) | **Post** /account/event/node | 
[**event_router_get_event_in_node**](AccounteventApi.md#event_router_get_event_in_node) | **Get** /account/event/node/{eventId} | 


# **event_router_count_all_events_from_a_account_for_a_given_time_range**
> ::models::ModelsCountStat event_router_count_all_events_from_a_account_for_a_given_time_range(x_account_api_key, optional)


Count all events from a account for a given time-range

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **x_account_api_key** | **String**| Account API Key | 
 **search** | **String**| search term | 
 **_type** | **String**| search type | 
 **from** | **String**| from date | 
 **to** | **String**| to date | 
 **source** | **String**| data source from which to get timestamp keys subaccount or ip | 
 **source_id** | **String**| source id from which to get timestamp keys subaccount or ip | 

### Return type

[**::models::ModelsCountStat**](models.CountStat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **event_router_count_all_events_from_a_node_of_a_sub_account_for_a_given_time_range**
> ::models::ModelsCountStat event_router_count_all_events_from_a_node_of_a_sub_account_for_a_given_time_range(x_account_api_key, optional)


Count all events from a node of a sub-account for a given time-range

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **x_account_api_key** | **String**| Account API Key | 
 **search** | **String**| search term | 
 **_type** | **String**| search type | 
 **from** | **String**| from date | 
 **to** | **String**| to date | 
 **source** | **String**| data source from which to get timestamp keys subaccount or ip | 
 **source_id** | **String**| source id from which to get timestamp keys subaccount or ip | 

### Return type

[**::models::ModelsCountStat**](models.CountStat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **event_router_get**
> ::models::ModelsQEvent event_router_get(x_account_api_key, event_id)


Find Event By Id

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **event_id** | **String**| the eventId that you want to retrieve | 

### Return type

[**::models::ModelsQEvent**](models.QEvent.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **event_router_get_all_event_timestamp_keys_of_a_sub_account_from_a_specific_node_for_a_given_time_range**
> Vec<::models::ModelsQEvent> event_router_get_all_event_timestamp_keys_of_a_sub_account_from_a_specific_node_for_a_given_time_range(x_account_api_key, optional)


Find all events of a sub-account from a specific node for a give time-range

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **x_account_api_key** | **String**| Account API Key | 
 **search** | **String**| search term | 
 **_type** | **String**| search type | 
 **from** | **String**| from date | 
 **to** | **String**| to date | 
 **source** | **String**| data source from which to get timestamp keys subaccount or ip | 
 **source_id** | **String**| source id from which to get timestamp keys subaccount or ip | 

### Return type

[**Vec<::models::ModelsQEvent>**](models.QEvent.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **event_router_get_all_events_of_a_account_from_a_specific_node**
> Vec<::models::ModelsQEvent> event_router_get_all_events_of_a_account_from_a_specific_node(x_account_api_key)


Find all events of a account from a specific node

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 

### Return type

[**Vec<::models::ModelsQEvent>**](models.QEvent.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **event_router_get_event_in_node**
> ::models::ModelsQEvent event_router_get_event_in_node(x_account_api_key, event_id)


Find Event From Node by id

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **event_id** | **String**| the eventId that you want to retrieve | 

### Return type

[**::models::ModelsQEvent**](models.QEvent.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

