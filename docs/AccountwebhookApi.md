# \AccountwebhookApi

All URIs are relative to *https://api.sendpost.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_webhook_router_count**](AccountwebhookApi.md#account_webhook_router_count) | **Get** /account/webhook/count | 
[**account_webhook_router_create**](AccountwebhookApi.md#account_webhook_router_create) | **Post** /account/webhook/ | 
[**account_webhook_router_delete**](AccountwebhookApi.md#account_webhook_router_delete) | **Delete** /account/webhook/{webhookId} | 
[**account_webhook_router_get**](AccountwebhookApi.md#account_webhook_router_get) | **Get** /account/webhook/{webhookId} | 
[**account_webhook_router_get_all**](AccountwebhookApi.md#account_webhook_router_get_all) | **Get** /account/webhook/ | 
[**account_webhook_router_update**](AccountwebhookApi.md#account_webhook_router_update) | **Put** /account/webhook/{webhookId} | 


# **account_webhook_router_count**
> ::models::ModelsCountStat account_webhook_router_count(x_account_api_key)


Count Total AccountWebhooks

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 

### Return type

[**::models::ModelsCountStat**](models.CountStat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **account_webhook_router_create**
> ::models::ModelsAccountWebhook account_webhook_router_create(x_account_api_key, body)


Create AccountWebhook

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **body** | [**ModelsEWebhook**](ModelsEWebhook.md)| The AccountWebhook content | 

### Return type

[**::models::ModelsAccountWebhook**](models.AccountWebhook.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **account_webhook_router_delete**
> ::models::ModelsDeleteResponse account_webhook_router_delete(x_account_api_key, webhook_id)


Delete AccountWebhook

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **webhook_id** | **i64**| The AccountWebhookId you want to delete | 

### Return type

[**::models::ModelsDeleteResponse**](models.DeleteResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **account_webhook_router_get**
> ::models::ModelsAccountWebhook account_webhook_router_get(x_account_api_key, webhook_id)


Find AccountWebhook by AccountWebhookId

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **webhook_id** | **i64**| the AccountWebhookId you want to get | 

### Return type

[**::models::ModelsAccountWebhook**](models.AccountWebhook.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **account_webhook_router_get_all**
> Vec<::models::ModelsAccountWebhook> account_webhook_router_get_all(x_account_api_key, optional)


Get All AccountWebhooks

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
 **offset** | **i64**| offset | 
 **limit** | **i64**| limit | 
 **search** | **String**| search | 

### Return type

[**Vec<::models::ModelsAccountWebhook>**](models.AccountWebhook.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **account_webhook_router_update**
> ::models::ModelsAccountWebhook account_webhook_router_update(x_account_api_key, webhook_id, body)


Update AccountWebhook

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **webhook_id** | **i64**| The AccountWebhookId you want to update | 
  **body** | [**ModelsEWebhook**](ModelsEWebhook.md)| The body | 

### Return type

[**::models::ModelsAccountWebhook**](models.AccountWebhook.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

