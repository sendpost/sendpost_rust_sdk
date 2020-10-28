# \SubaccountsenderApi

All URIs are relative to *https://api.sendpost.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sender_router_count**](SubaccountsenderApi.md#sender_router_count) | **Get** /subaccount/sender/count | 
[**sender_router_create**](SubaccountsenderApi.md#sender_router_create) | **Post** /subaccount/sender/ | 
[**sender_router_delete**](SubaccountsenderApi.md#sender_router_delete) | **Delete** /subaccount/sender/{senderId} | 
[**sender_router_get**](SubaccountsenderApi.md#sender_router_get) | **Get** /subaccount/sender/{senderId} | 
[**sender_router_get_all**](SubaccountsenderApi.md#sender_router_get_all) | **Get** /subaccount/sender/ | 
[**sender_router_update**](SubaccountsenderApi.md#sender_router_update) | **Put** /subaccount/sender/{senderId} | 


# **sender_router_count**
> ::models::ModelsCountStat sender_router_count(x_sub_account_api_key)


Count Total Senders

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_sub_account_api_key** | **String**| Sub-Account API Key | 

### Return type

[**::models::ModelsCountStat**](models.CountStat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **sender_router_create**
> ::models::ModelsSender sender_router_create(x_sub_account_api_key, body)


Create Sender

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_sub_account_api_key** | **String**| Sub-Account API Key | 
  **body** | [**ModelsESender**](ModelsESender.md)| The Sender content | 

### Return type

[**::models::ModelsSender**](models.Sender.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **sender_router_delete**
> ::models::ModelsDeleteResponse sender_router_delete(x_sub_account_api_key, sender_id)


Delete Sender

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_sub_account_api_key** | **String**| Sub-Account API Key | 
  **sender_id** | **i64**| The SenderId you want to delete | 

### Return type

[**::models::ModelsDeleteResponse**](models.DeleteResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **sender_router_get**
> ::models::ModelsSender sender_router_get(x_sub_account_api_key, sender_id)


Find Sender by SenderId

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_sub_account_api_key** | **String**| Sub-Account API Key | 
  **sender_id** | **i64**| the SenderId you want to get | 

### Return type

[**::models::ModelsSender**](models.Sender.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **sender_router_get_all**
> Vec<::models::ModelsSender> sender_router_get_all(x_sub_account_api_key, optional)


Get All Senders

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_sub_account_api_key** | **String**| Sub-Account API Key | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **x_sub_account_api_key** | **String**| Sub-Account API Key | 
 **offset** | **i64**| offset | 
 **limit** | **i64**| limit | 
 **search** | **String**| search term | 

### Return type

[**Vec<::models::ModelsSender>**](models.Sender.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **sender_router_update**
> ::models::ModelsSender sender_router_update(x_sub_account_api_key, sender_id, body)


Update Sender

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_sub_account_api_key** | **String**| Sub-Account API Key | 
  **sender_id** | **i64**| The SenderId you want to update | 
  **body** | [**ModelsESender**](ModelsESender.md)| The body | 

### Return type

[**::models::ModelsSender**](models.Sender.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

