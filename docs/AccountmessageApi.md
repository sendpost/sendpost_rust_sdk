# \AccountmessageApi

All URIs are relative to *https://api.sendpost.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**message_router_get**](AccountmessageApi.md#message_router_get) | **Get** /account/message/{messageId} | 
[**message_router_get_all_events_for_a_message_id**](AccountmessageApi.md#message_router_get_all_events_for_a_message_id) | **Get** /account/message/{messageId}/events | 
[**message_router_get_all_events_for_a_message_id_from_a_node**](AccountmessageApi.md#message_router_get_all_events_for_a_message_id_from_a_node) | **Get** /account/message/node/{messageId}/events | 
[**message_router_get_message_from_node**](AccountmessageApi.md#message_router_get_message_from_node) | **Get** /account/message/node/{messageId} | 


# **message_router_get**
> ::models::ModelsQEmailMessage message_router_get(x_account_api_key, message_id)


Find Message By Id

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Sub-Account API Key | 
  **message_id** | **String**| the messageId that you want to retrieve | 

### Return type

[**::models::ModelsQEmailMessage**](models.QEmailMessage.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **message_router_get_all_events_for_a_message_id**
> Vec<::models::ModelsQEvent> message_router_get_all_events_for_a_message_id(x_account_api_key, message_id)


Find all events associated with a message id

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **message_id** | **String**| the messageId that you want to retrieve | 

### Return type

[**Vec<::models::ModelsQEvent>**](models.QEvent.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **message_router_get_all_events_for_a_message_id_from_a_node**
> Vec<::models::ModelsQEvent> message_router_get_all_events_for_a_message_id_from_a_node(x_account_api_key, message_id)


Find all message events associated with a message id from a specific node

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **message_id** | **String**| the messageId that you want to retrieve | 

### Return type

[**Vec<::models::ModelsQEvent>**](models.QEvent.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **message_router_get_message_from_node**
> ::models::ModelsQEmailMessage message_router_get_message_from_node(x_account_api_key, message_id)


Find Message from node by specific Id

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **message_id** | **String**| the messageId that you want to retrieve | 

### Return type

[**::models::ModelsQEmailMessage**](models.QEmailMessage.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

