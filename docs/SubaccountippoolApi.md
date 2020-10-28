# \SubaccountippoolApi

All URIs are relative to *https://api.sendpost.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**i_p_pool_router_count**](SubaccountippoolApi.md#i_p_pool_router_count) | **Get** /subaccount/ippool/count | 
[**i_p_pool_router_create**](SubaccountippoolApi.md#i_p_pool_router_create) | **Post** /subaccount/ippool/ | 
[**i_p_pool_router_delete**](SubaccountippoolApi.md#i_p_pool_router_delete) | **Delete** /subaccount/ippool/{ippoolid} | 
[**i_p_pool_router_get**](SubaccountippoolApi.md#i_p_pool_router_get) | **Get** /subaccount/ippool/{ippoolid} | 
[**i_p_pool_router_get_all**](SubaccountippoolApi.md#i_p_pool_router_get_all) | **Get** /subaccount/ippool/ | 
[**i_p_pool_router_update**](SubaccountippoolApi.md#i_p_pool_router_update) | **Put** /subaccount/ippool/{ippoolid} | 


# **i_p_pool_router_count**
> ::models::ModelsCountStat i_p_pool_router_count(x_sub_account_api_key)


Count Total IPPools

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

# **i_p_pool_router_create**
> ::models::ModelsIpPool i_p_pool_router_create(x_sub_account_api_key, body)


Create IPPool

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_sub_account_api_key** | **String**| Sub-Account API Key | 
  **body** | [**ModelsEipPool**](ModelsEipPool.md)| The IPPool content | 

### Return type

[**::models::ModelsIpPool**](models.IPPool.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **i_p_pool_router_delete**
> ::models::ModelsDeleteResponse i_p_pool_router_delete(x_sub_account_api_key, ippoolid)


Delete IPPool

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_sub_account_api_key** | **String**| Sub-Account API Key | 
  **ippoolid** | **i64**| The IPPoolId you want to delete | 

### Return type

[**::models::ModelsDeleteResponse**](models.DeleteResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **i_p_pool_router_get**
> ::models::ModelsIpPool i_p_pool_router_get(x_sub_account_api_key, ippoolid)


Find IPPool by IPPoolId

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_sub_account_api_key** | **String**| Sub-Account API Key | 
  **ippoolid** | **i64**| the IPPoolId you want to get | 

### Return type

[**::models::ModelsIpPool**](models.IPPool.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **i_p_pool_router_get_all**
> Vec<::models::ModelsIpPool> i_p_pool_router_get_all(x_sub_account_api_key, optional)


Get All IPPools

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

[**Vec<::models::ModelsIpPool>**](models.IPPool.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **i_p_pool_router_update**
> ::models::ModelsIpPool i_p_pool_router_update(x_sub_account_api_key, ippoolid, body)


Update IPPool

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_sub_account_api_key** | **String**| Sub-Account API Key | 
  **ippoolid** | **i64**| The IPPoolId you want to update | 
  **body** | [**ModelsEipPool**](ModelsEipPool.md)| The body | 

### Return type

[**::models::ModelsIpPool**](models.IPPool.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

