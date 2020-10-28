# \SubaccountstatApi

All URIs are relative to *https://api.sendpost.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sub_account_stat_router_get_all_aggregate_sub_account_stats**](SubaccountstatApi.md#sub_account_stat_router_get_all_aggregate_sub_account_stats) | **Get** /subaccount/stat/aggregate | 
[**sub_account_stat_router_get_all_aggregate_sub_account_stats_by_group**](SubaccountstatApi.md#sub_account_stat_router_get_all_aggregate_sub_account_stats_by_group) | **Get** /subaccount/stat/aggregate/group | 
[**sub_account_stat_router_get_all_aggregated_group_stats_for_a_sub_account**](SubaccountstatApi.md#sub_account_stat_router_get_all_aggregated_group_stats_for_a_sub_account) | **Get** /subaccount/stat/aggregate/groups | 
[**sub_account_stat_router_get_all_aggregated_ip_stats_for_a_sub_account**](SubaccountstatApi.md#sub_account_stat_router_get_all_aggregated_ip_stats_for_a_sub_account) | **Get** /subaccount/stat/aggregate/ips | 
[**sub_account_stat_router_get_all_aggregated_provider_stats_for_a_specific_ip_of_a_sub_account**](SubaccountstatApi.md#sub_account_stat_router_get_all_aggregated_provider_stats_for_a_specific_ip_of_a_sub_account) | **Get** /subaccount/stat/aggregate/ip/{ipid}/providers | 
[**sub_account_stat_router_get_all_aggregated_provider_stats_for_a_sub_account**](SubaccountstatApi.md#sub_account_stat_router_get_all_aggregated_provider_stats_for_a_sub_account) | **Get** /subaccount/stat/aggregate/providers | 
[**sub_account_stat_router_get_all_sub_account_stats**](SubaccountstatApi.md#sub_account_stat_router_get_all_sub_account_stats) | **Get** /subaccount/stat/ | 
[**sub_account_stat_router_get_all_sub_account_stats_by_group**](SubaccountstatApi.md#sub_account_stat_router_get_all_sub_account_stats_by_group) | **Get** /subaccount/stat/group | 


# **sub_account_stat_router_get_all_aggregate_sub_account_stats**
> ::models::ModelsStat sub_account_stat_router_get_all_aggregate_sub_account_stats(x_sub_account_api_key, optional)


Get All Aggregate Sub-Account Stats

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

# **sub_account_stat_router_get_all_aggregate_sub_account_stats_by_group**
> ::models::ModelsStat sub_account_stat_router_get_all_aggregate_sub_account_stats_by_group(x_sub_account_api_key, group, optional)


Get All Aggregate Sub-Account Stats by Group

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_sub_account_api_key** | **String**| Sub-Account API Key | 
  **group** | **String**| the group whose stats you want | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **x_sub_account_api_key** | **String**| Sub-Account API Key | 
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

# **sub_account_stat_router_get_all_aggregated_group_stats_for_a_sub_account**
> Vec<::models::ModelsAgStat> sub_account_stat_router_get_all_aggregated_group_stats_for_a_sub_account(x_sub_account_api_key, optional)


Get All Aggregated Group Stats for a Sub-Account

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
 **from** | **String**| from date | 
 **to** | **String**| to date | 

### Return type

[**Vec<::models::ModelsAgStat>**](models.AGStat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **sub_account_stat_router_get_all_aggregated_ip_stats_for_a_sub_account**
> Vec<::models::ModelsAipStat> sub_account_stat_router_get_all_aggregated_ip_stats_for_a_sub_account(x_sub_account_api_key, optional)


Get All Aggregated IP Stats for a Sub-Account

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
 **from** | **String**| from date | 
 **to** | **String**| to date | 

### Return type

[**Vec<::models::ModelsAipStat>**](models.AIPStat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **sub_account_stat_router_get_all_aggregated_provider_stats_for_a_specific_ip_of_a_sub_account**
> Vec<::models::ModelsPipStat> sub_account_stat_router_get_all_aggregated_provider_stats_for_a_specific_ip_of_a_sub_account(x_sub_account_api_key, ipid, optional)


Get All Aggregated Provider Stats for a Specific IP of a Sub-Account

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_sub_account_api_key** | **String**| Sub-Account API Key | 
  **ipid** | **i64**| the IPId you want to get | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **x_sub_account_api_key** | **String**| Sub-Account API Key | 
 **ipid** | **i64**| the IPId you want to get | 
 **from** | **String**| from date | 
 **to** | **String**| to date | 

### Return type

[**Vec<::models::ModelsPipStat>**](models.PIPStat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **sub_account_stat_router_get_all_aggregated_provider_stats_for_a_sub_account**
> Vec<::models::ModelsPipStat> sub_account_stat_router_get_all_aggregated_provider_stats_for_a_sub_account(x_sub_account_api_key, optional)


Get All Aggregated Provider Stats for a Sub-Account

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
 **from** | **String**| from date | 
 **to** | **String**| to date | 

### Return type

[**Vec<::models::ModelsPipStat>**](models.PIPStat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **sub_account_stat_router_get_all_sub_account_stats**
> Vec<::models::ModelsRStat> sub_account_stat_router_get_all_sub_account_stats(x_sub_account_api_key, optional)


Get All Sub-Account Stats

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

# **sub_account_stat_router_get_all_sub_account_stats_by_group**
> Vec<::models::ModelsRStat> sub_account_stat_router_get_all_sub_account_stats_by_group(x_sub_account_api_key, group, optional)


Get All Sub-Account Stats by Group

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_sub_account_api_key** | **String**| Sub-Account API Key | 
  **group** | **String**| the tag whose stats you want | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **x_sub_account_api_key** | **String**| Sub-Account API Key | 
 **group** | **String**| the tag whose stats you want | 
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

