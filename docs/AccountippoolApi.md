# \AccountippoolApi

All URIs are relative to *https://api.sendpost.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_ip_pool_router_count**](AccountippoolApi.md#account_ip_pool_router_count) | **Get** /account/ippool/count | 
[**account_ip_pool_router_create**](AccountippoolApi.md#account_ip_pool_router_create) | **Post** /account/ippool/ | 
[**account_ip_pool_router_delete**](AccountippoolApi.md#account_ip_pool_router_delete) | **Delete** /account/ippool/{ippoolid} | 
[**account_ip_pool_router_get**](AccountippoolApi.md#account_ip_pool_router_get) | **Get** /account/ippool/{ippoolid} | 
[**account_ip_pool_router_get_all**](AccountippoolApi.md#account_ip_pool_router_get_all) | **Get** /account/ippool/ | 
[**account_ip_pool_router_update**](AccountippoolApi.md#account_ip_pool_router_update) | **Put** /account/ippool/{ippoolid} | 


# **account_ip_pool_router_count**
> ::models::ModelsCountStat account_ip_pool_router_count(x_account_api_key)


Count Total AccountIPPools

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

# **account_ip_pool_router_create**
> ::models::ModelsAccountIpPool account_ip_pool_router_create(x_account_api_key, body)


Create AccountIPPool

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **body** | [**ModelsEipPool**](ModelsEipPool.md)| The AccountIPPool content | 

### Return type

[**::models::ModelsAccountIpPool**](models.AccountIPPool.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **account_ip_pool_router_delete**
> ::models::ModelsDeleteResponse account_ip_pool_router_delete(x_account_api_key, ippoolid)


Delete AccountIPPool

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **ippoolid** | **i64**| The AccountIPPoolId you want to delete | 

### Return type

[**::models::ModelsDeleteResponse**](models.DeleteResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **account_ip_pool_router_get**
> ::models::ModelsAccountIpPool account_ip_pool_router_get(x_account_api_key, ippoolid)


Find AccountIPPool by AccountIPPoolId

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **ippoolid** | **i64**| the AccountIPPoolId you want to get | 

### Return type

[**::models::ModelsAccountIpPool**](models.AccountIPPool.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **account_ip_pool_router_get_all**
> Vec<::models::ModelsAccountIpPool> account_ip_pool_router_get_all(x_account_api_key, optional)


Get All AccountIPPools

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
 **search** | **String**| search term | 

### Return type

[**Vec<::models::ModelsAccountIpPool>**](models.AccountIPPool.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **account_ip_pool_router_update**
> ::models::ModelsAccountIpPool account_ip_pool_router_update(x_account_api_key, ippoolid, body)


Update AccountIPPool

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **ippoolid** | **i64**| The AccountIPPoolId you want to update | 
  **body** | [**ModelsEipPool**](ModelsEipPool.md)| The body | 

### Return type

[**::models::ModelsAccountIpPool**](models.AccountIPPool.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

