# \AccountipApi

All URIs are relative to *https://api.sendpost.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**i_p_router_allocate_ip**](AccountipApi.md#i_p_router_allocate_ip) | **Post** /account/ip/allocate | 
[**i_p_router_count**](AccountipApi.md#i_p_router_count) | **Get** /account/ip/count | 
[**i_p_router_delete**](AccountipApi.md#i_p_router_delete) | **Delete** /account/ip/{ipid} | 
[**i_p_router_get**](AccountipApi.md#i_p_router_get) | **Get** /account/ip/{ipid} | 
[**i_p_router_get_all**](AccountipApi.md#i_p_router_get_all) | **Get** /account/ip/ | 
[**i_p_router_update**](AccountipApi.md#i_p_router_update) | **Put** /account/ip/{ipid} | 


# **i_p_router_allocate_ip**
> Vec<::models::ModelsIp> i_p_router_allocate_ip(x_account_api_key)


Allocate IP To Account

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 

### Return type

[**Vec<::models::ModelsIp>**](models.IP.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **i_p_router_count**
> ::models::ModelsCountStat i_p_router_count(x_account_api_key, optional)


Count Total AccountIPs

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

### Return type

[**::models::ModelsCountStat**](models.CountStat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **i_p_router_delete**
> ::models::ModelsDeleteResponse i_p_router_delete(x_account_api_key, ipid)


Delete IP

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **ipid** | **i64**| The IPId you want to delete | 

### Return type

[**::models::ModelsDeleteResponse**](models.DeleteResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **i_p_router_get**
> ::models::ModelsIp i_p_router_get(x_account_api_key, ipid)


Find IP by IPId

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **ipid** | **i64**| the IPId you want to get | 

### Return type

[**::models::ModelsIp**](models.IP.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **i_p_router_get_all**
> Vec<::models::ModelsIp> i_p_router_get_all(x_account_api_key, optional)


Get All IPs

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

[**Vec<::models::ModelsIp>**](models.IP.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **i_p_router_update**
> ::models::ModelsIp i_p_router_update(x_account_api_key, ipid, body)


Update an IP

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **ipid** | **i64**| The IP you want to update | 
  **body** | [**ModelsIip**](ModelsIip.md)| The IP Email Provider Settings | 

### Return type

[**::models::ModelsIp**](models.IP.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

