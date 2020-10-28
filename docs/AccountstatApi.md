# \AccountstatApi

All URIs are relative to *https://api.sendpost.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_stat_router_get_all_account_stats**](AccountstatApi.md#account_stat_router_get_all_account_stats) | **Get** /account/stat/ | 
[**account_stat_router_get_all_account_stats_by_group**](AccountstatApi.md#account_stat_router_get_all_account_stats_by_group) | **Get** /account/stat/group | 
[**account_stat_router_get_all_aggregate_account_stats**](AccountstatApi.md#account_stat_router_get_all_aggregate_account_stats) | **Get** /account/stat/aggregate | 
[**account_stat_router_get_all_aggregate_account_stats_by_group**](AccountstatApi.md#account_stat_router_get_all_aggregate_account_stats_by_group) | **Get** /account/stat/aggregate/group | 


# **account_stat_router_get_all_account_stats**
> Vec<::models::ModelsRStat> account_stat_router_get_all_account_stats(x_account_api_key, optional)


Get All Account Stats

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Sub-Account API Key | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **x_account_api_key** | **String**| Sub-Account API Key | 
 **from** | **String**| from date | 
 **to** | **String**| to date | 

### Return type

[**Vec<::models::ModelsRStat>**](models.RStat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **account_stat_router_get_all_account_stats_by_group**
> Vec<::models::ModelsRStat> account_stat_router_get_all_account_stats_by_group(x_account_api_key, group, optional)


Get All Account Stats by Group

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Sub-Account API Key | 
  **group** | **String**| the group whose stats you want | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **x_account_api_key** | **String**| Sub-Account API Key | 
 **group** | **String**| the group whose stats you want | 
 **from** | **String**| from date | 
 **to** | **String**| to date | 

### Return type

[**Vec<::models::ModelsRStat>**](models.RStat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **account_stat_router_get_all_aggregate_account_stats**
> ::models::ModelsStat account_stat_router_get_all_aggregate_account_stats(x_account_api_key, optional)


Get All Aggregate Stats

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Sub-Account API Key | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **x_account_api_key** | **String**| Sub-Account API Key | 
 **from** | **String**| from date | 
 **to** | **String**| to date | 

### Return type

[**::models::ModelsStat**](models.Stat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **account_stat_router_get_all_aggregate_account_stats_by_group**
> ::models::ModelsStat account_stat_router_get_all_aggregate_account_stats_by_group(x_account_api_key, group, optional)


Get All Aggregate Stats by Group

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Sub-Account API Key | 
  **group** | **String**| the group whose stats you want | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **x_account_api_key** | **String**| Sub-Account API Key | 
 **group** | **String**| the group whose stats you want | 
 **from** | **String**| from date | 
 **to** | **String**| to date | 

### Return type

[**::models::ModelsStat**](models.Stat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

