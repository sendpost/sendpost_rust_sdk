# \AccountintegrationApi

All URIs are relative to *https://api.sendpost.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_integration_router_count**](AccountintegrationApi.md#account_integration_router_count) | **Get** /account/integration/count | 
[**account_integration_router_create**](AccountintegrationApi.md#account_integration_router_create) | **Post** /account/integration/{itype} | 
[**account_integration_router_delete**](AccountintegrationApi.md#account_integration_router_delete) | **Delete** /account/integration/{itype} | 
[**account_integration_router_disable_glockapps_ip_monitoring**](AccountintegrationApi.md#account_integration_router_disable_glockapps_ip_monitoring) | **Delete** /account/integration/glockapps/monitor/{ipid} | 
[**account_integration_router_enable_glockapps_ip_monitoring**](AccountintegrationApi.md#account_integration_router_enable_glockapps_ip_monitoring) | **Post** /account/integration/glockapps/monitor/{ipid} | 
[**account_integration_router_get_all**](AccountintegrationApi.md#account_integration_router_get_all) | **Get** /account/integration/ | 
[**account_integration_router_get_monitored_ip_stats**](AccountintegrationApi.md#account_integration_router_get_monitored_ip_stats) | **Get** /account/integration/glockapps/monitor/stat/{ipid} | 
[**account_integration_router_update**](AccountintegrationApi.md#account_integration_router_update) | **Put** /account/integration/{itype} | 


# **account_integration_router_count**
> ::models::ModelsCountStat account_integration_router_count(x_account_api_key)


Count Total AccountIntegrations

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

# **account_integration_router_create**
> ::models::ModelsIntegration account_integration_router_create(x_account_api_key, itype, body)


Create Integration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **itype** | **String**| The integration type you want to create | 
  **body** | [**ModelsEIntegration**](ModelsEIntegration.md)| The Integration content | 

### Return type

[**::models::ModelsIntegration**](models.Integration.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **account_integration_router_delete**
> ::models::ModelsDeleteResponse account_integration_router_delete(x_account_api_key, itype)


Delete Integration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **itype** | **String**| The integration type you want to update | 

### Return type

[**::models::ModelsDeleteResponse**](models.DeleteResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **account_integration_router_disable_glockapps_ip_monitoring**
> ::models::ModelsDeleteResponse account_integration_router_disable_glockapps_ip_monitoring(x_account_api_key, ipid)


Disable IP Monitoring for a single IP

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **ipid** | **i64**| the IPId you want to disable monitoring for | 

### Return type

[**::models::ModelsDeleteResponse**](models.DeleteResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **account_integration_router_enable_glockapps_ip_monitoring**
> ::models::ModelsResponse account_integration_router_enable_glockapps_ip_monitoring(x_account_api_key, ipid)


Enable IP Monitoring for a single IP

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **ipid** | **i64**| the IPId you want to enable monitoring for | 

### Return type

[**::models::ModelsResponse**](models.Response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **account_integration_router_get_all**
> Vec<::models::ModelsIntegration> account_integration_router_get_all(x_account_api_key)


Get All Integrations

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 

### Return type

[**Vec<::models::ModelsIntegration>**](models.Integration.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **account_integration_router_get_monitored_ip_stats**
> Vec<::models::ModelsRGlockappsMonitorStat> account_integration_router_get_monitored_ip_stats(x_account_api_key, ipid, optional)


Get Monitored IP Stats

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **ipid** | **i64**| the IPId for which you want monitored stats | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **x_account_api_key** | **String**| Account API Key | 
 **ipid** | **i64**| the IPId for which you want monitored stats | 
 **from** | **String**| from date | 
 **to** | **String**| to date | 

### Return type

[**Vec<::models::ModelsRGlockappsMonitorStat>**](models.RGlockappsMonitorStat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **account_integration_router_update**
> ::models::ModelsIntegration account_integration_router_update(x_account_api_key, itype, body)


Update Integration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **itype** | **String**| The integration type you want to update | 
  **body** | [**ModelsEIntegration**](ModelsEIntegration.md)| The Integration content | 

### Return type

[**::models::ModelsIntegration**](models.Integration.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

