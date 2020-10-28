# \AccountipstatApi

All URIs are relative to *https://api.sendpost.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**i_p_stat_router_get_all_aggregate_ip_stats**](AccountipstatApi.md#i_p_stat_router_get_all_aggregate_ip_stats) | **Get** /account/ip/stat/{ipid}/aggregate | 
[**i_p_stat_router_get_all_aggregate_ip_stats_by_group**](AccountipstatApi.md#i_p_stat_router_get_all_aggregate_ip_stats_by_group) | **Get** /account/ip/stat/{ipid}/aggregate/provider | 
[**i_p_stat_router_get_all_aggregated_provider_stats_for_a_ip**](AccountipstatApi.md#i_p_stat_router_get_all_aggregated_provider_stats_for_a_ip) | **Get** /account/ip/stat/{ipid}/aggregate/providers | 
[**i_p_stat_router_get_all_aggregated_provider_stats_for_a_specific_sub_account_of_a_ip**](AccountipstatApi.md#i_p_stat_router_get_all_aggregated_provider_stats_for_a_specific_sub_account_of_a_ip) | **Get** /account/ip/stat/{ipid}/aggregate/sid/{sid}/providers | 
[**i_p_stat_router_get_all_aggregated_sub_account_stats_for_an_ip**](AccountipstatApi.md#i_p_stat_router_get_all_aggregated_sub_account_stats_for_an_ip) | **Get** /account/ip/stat/{ipid}/aggregate/subaccounts | 
[**i_p_stat_router_get_all_ip_stats**](AccountipstatApi.md#i_p_stat_router_get_all_ip_stats) | **Get** /account/ip/stat/{ipid} | 
[**i_p_stat_router_get_all_ip_stats_by_group**](AccountipstatApi.md#i_p_stat_router_get_all_ip_stats_by_group) | **Get** /account/ip/stat/{ipid}/provider | 


# **i_p_stat_router_get_all_aggregate_ip_stats**
> ::models::ModelsStat i_p_stat_router_get_all_aggregate_ip_stats(x_account_api_key, ipid, optional)


Get All Aggregate Stats

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **ipid** | **i64**| the IPId you want to get | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **x_account_api_key** | **String**| Account API Key | 
 **ipid** | **i64**| the IPId you want to get | 
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

# **i_p_stat_router_get_all_aggregate_ip_stats_by_group**
> ::models::ModelsStat i_p_stat_router_get_all_aggregate_ip_stats_by_group(x_account_api_key, ipid, provider, optional)


Get All Aggregate Stats by Group

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **ipid** | **i64**| the IPId you want to get | 
  **provider** | **String**| the group whose stats you want | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **x_account_api_key** | **String**| Account API Key | 
 **ipid** | **i64**| the IPId you want to get | 
 **provider** | **String**| the group whose stats you want | 
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

# **i_p_stat_router_get_all_aggregated_provider_stats_for_a_ip**
> Vec<::models::ModelsPipStat> i_p_stat_router_get_all_aggregated_provider_stats_for_a_ip(x_account_api_key, ipid, optional)


Get All Aggregated Provider Stats for a IP

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **ipid** | **i64**| the IPId you want to get | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **x_account_api_key** | **String**| Account API Key | 
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

# **i_p_stat_router_get_all_aggregated_provider_stats_for_a_specific_sub_account_of_a_ip**
> Vec<::models::ModelsPipStat> i_p_stat_router_get_all_aggregated_provider_stats_for_a_specific_sub_account_of_a_ip(x_account_api_key, ipid, sid, optional)


Get All Aggregated Provider Stats for a Specific Sub-Account of a IP

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **ipid** | **i64**| the IPId you want to get | 
  **sid** | **i64**| the Sub Account Id you want to get | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **x_account_api_key** | **String**| Account API Key | 
 **ipid** | **i64**| the IPId you want to get | 
 **sid** | **i64**| the Sub Account Id you want to get | 
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

# **i_p_stat_router_get_all_aggregated_sub_account_stats_for_an_ip**
> Vec<::models::ModelsSipStat> i_p_stat_router_get_all_aggregated_sub_account_stats_for_an_ip(x_account_api_key, ipid, optional)


Get All Aggregated Sub-Account Stats for an IP

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **ipid** | **i64**| the IPId you want to get | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **x_account_api_key** | **String**| Account API Key | 
 **ipid** | **i64**| the IPId you want to get | 
 **from** | **String**| from date | 
 **to** | **String**| to date | 

### Return type

[**Vec<::models::ModelsSipStat>**](models.SIPStat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **i_p_stat_router_get_all_ip_stats**
> Vec<::models::ModelsRipStat> i_p_stat_router_get_all_ip_stats(x_account_api_key, ipid, optional)


Get All IP Stats

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **ipid** | **i64**| the IPId you want to get | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **x_account_api_key** | **String**| Account API Key | 
 **ipid** | **i64**| the IPId you want to get | 
 **from** | **String**| from date | 
 **to** | **String**| to date | 

### Return type

[**Vec<::models::ModelsRipStat>**](models.RIPStat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **i_p_stat_router_get_all_ip_stats_by_group**
> Vec<::models::ModelsRipStat> i_p_stat_router_get_all_ip_stats_by_group(ipid, x_account_api_key, provider, optional)


Get All IP Stats by Group

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **ipid** | **i64**| the IPId you want to get | 
  **x_account_api_key** | **String**| Account API Key | 
  **provider** | **String**| the provider whose stats you want | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ipid** | **i64**| the IPId you want to get | 
 **x_account_api_key** | **String**| Account API Key | 
 **provider** | **String**| the provider whose stats you want | 
 **from** | **String**| from date | 
 **to** | **String**| to date | 

### Return type

[**Vec<::models::ModelsRipStat>**](models.RIPStat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

