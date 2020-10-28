# \AccountlabelApi

All URIs are relative to *https://api.sendpost.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**label_router_count**](AccountlabelApi.md#label_router_count) | **Get** /account/label/count | 
[**label_router_create**](AccountlabelApi.md#label_router_create) | **Post** /account/label/ | 
[**label_router_delete**](AccountlabelApi.md#label_router_delete) | **Delete** /account/label/{labelId} | 
[**label_router_get**](AccountlabelApi.md#label_router_get) | **Get** /account/label/{labelId} | 
[**label_router_get_all**](AccountlabelApi.md#label_router_get_all) | **Get** /account/label/ | 
[**label_router_update**](AccountlabelApi.md#label_router_update) | **Put** /account/label/{labelId} | 


# **label_router_count**
> ::models::ModelsCountStat label_router_count(x_account_api_key)


Count Total Labels

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

# **label_router_create**
> ::models::ModelsLabel label_router_create(x_account_api_key, body)


Create Label

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **body** | [**ModelsLabel**](ModelsLabel.md)| The Label content | 

### Return type

[**::models::ModelsLabel**](models.Label.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **label_router_delete**
> ::models::ModelsDeleteResponse label_router_delete(x_account_api_key, label_id)


Delete Label

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **label_id** | **i64**| The LabelId you want to delete | 

### Return type

[**::models::ModelsDeleteResponse**](models.DeleteResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **label_router_get**
> ::models::ModelsLabel label_router_get(x_account_api_key, label_id)


Find Label by LabelId

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **label_id** | **i64**| the LabelId you want to get | 

### Return type

[**::models::ModelsLabel**](models.Label.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **label_router_get_all**
> Vec<::models::ModelsLabel> label_router_get_all(x_account_api_key)


Get All Labels

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 

### Return type

[**Vec<::models::ModelsLabel>**](models.Label.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **label_router_update**
> ::models::ModelsLabel label_router_update(x_account_api_key, label_id, body)


Update Label

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **label_id** | **i64**| The LabelId you want to update | 
  **body** | [**ModelsLabel**](ModelsLabel.md)| The body | 

### Return type

[**::models::ModelsLabel**](models.Label.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

