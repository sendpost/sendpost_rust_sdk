# \AccountsubaccountApi

All URIs are relative to *https://api.sendpost.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sub_account_router_count**](AccountsubaccountApi.md#sub_account_router_count) | **Get** /account/subaccount/count | 
[**sub_account_router_create**](AccountsubaccountApi.md#sub_account_router_create) | **Post** /account/subaccount/ | 
[**sub_account_router_delete**](AccountsubaccountApi.md#sub_account_router_delete) | **Delete** /account/subaccount/{subAccountId} | 
[**sub_account_router_get**](AccountsubaccountApi.md#sub_account_router_get) | **Get** /account/subaccount/{subAccountId} | 
[**sub_account_router_get_all**](AccountsubaccountApi.md#sub_account_router_get_all) | **Get** /account/subaccount/ | 
[**sub_account_router_update**](AccountsubaccountApi.md#sub_account_router_update) | **Put** /account/subaccount/{subAccountId} | 


# **sub_account_router_count**
> ::models::ModelsCountStat sub_account_router_count(x_account_api_key, optional)


Count Total Subaccounts

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
 **filter_by** | **String**| filterBy | 
 **filter_value** | **i64**| filterValue | 
 **search** | **String**| search term | 

### Return type

[**::models::ModelsCountStat**](models.CountStat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **sub_account_router_create**
> ::models::ModelsSubAccount sub_account_router_create(x_account_api_key, body)


Create SubAccount

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **body** | [**ModelsESubAccount**](ModelsESubAccount.md)| The SubAccount content | 

### Return type

[**::models::ModelsSubAccount**](models.SubAccount.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **sub_account_router_delete**
> ::models::ModelsDeleteResponse sub_account_router_delete(x_account_api_key, sub_account_id)


Delete SubAccount

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **sub_account_id** | **i64**| The SubAccountId you want to delete | 

### Return type

[**::models::ModelsDeleteResponse**](models.DeleteResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **sub_account_router_get**
> ::models::ModelsSubAccount sub_account_router_get(x_account_api_key, sub_account_id)


Find SubAccount by SubAccountId

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **sub_account_id** | **i64**| the SubAccountId you want to get | 

### Return type

[**::models::ModelsSubAccount**](models.SubAccount.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **sub_account_router_get_all**
> Vec<::models::ModelsSubAccount> sub_account_router_get_all(x_account_api_key, optional)


Get All SubAccounts

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
 **filter_by** | **String**| filterBy | 
 **filter_value** | **i64**| filterValue | 
 **search** | **String**| search term | 

### Return type

[**Vec<::models::ModelsSubAccount>**](models.SubAccount.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **sub_account_router_update**
> ::models::ModelsSubAccount sub_account_router_update(x_account_api_key, sub_account_id, body)


Update SubAccount

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **sub_account_id** | **i64**| The SubAccountId you want to update | 
  **body** | [**ModelsESubAccount**](ModelsESubAccount.md)| The body | 

### Return type

[**::models::ModelsSubAccount**](models.SubAccount.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

