# \AccountsmtpstatApi

All URIs are relative to *https://api.sendpost.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**s_mtp_stat_router_get_all_aggregate_ip_provider_smtp_stats**](AccountsmtpstatApi.md#s_mtp_stat_router_get_all_aggregate_ip_provider_smtp_stats) | **Get** /account/smtp/stat/ip/{ipid}/provider/{pname}/aggregate | 
[**s_mtp_stat_router_get_all_aggregate_ip_smtp_stats**](AccountsmtpstatApi.md#s_mtp_stat_router_get_all_aggregate_ip_smtp_stats) | **Get** /account/smtp/stat/ip/{ipid}/aggregate | 
[**s_mtp_stat_router_get_all_aggregate_ip_smtp_stats_for_sub_account**](AccountsmtpstatApi.md#s_mtp_stat_router_get_all_aggregate_ip_smtp_stats_for_sub_account) | **Get** /account/smtp/stat/ip/{ipid}/subaccount/{sid}/aggregate | 
[**s_mtp_stat_router_get_all_aggregate_sub_account_provider_smtp_stats**](AccountsmtpstatApi.md#s_mtp_stat_router_get_all_aggregate_sub_account_provider_smtp_stats) | **Get** /account/smtp/stat/subaccount/{sid}/provider/{pname}/aggregate | 
[**s_mtp_stat_router_get_all_aggregate_sub_account_smtp_stats**](AccountsmtpstatApi.md#s_mtp_stat_router_get_all_aggregate_sub_account_smtp_stats) | **Get** /account/smtp/stat/subaccount/{sid}/aggregate | 
[**s_mtp_stat_router_get_all_aggregate_sub_account_smtp_stats_for_ip**](AccountsmtpstatApi.md#s_mtp_stat_router_get_all_aggregate_sub_account_smtp_stats_for_ip) | **Get** /account/smtp/stat/subaccount/{sid}/ip/{ipid}/aggregate | 


# **s_mtp_stat_router_get_all_aggregate_ip_provider_smtp_stats**
> Vec<::models::ModelsSmtpStat> s_mtp_stat_router_get_all_aggregate_ip_provider_smtp_stats(x_account_api_key, ipid, pname, optional)


Get All Aggregate IP Provider SMTP Stats

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **ipid** | **i64**| the IP ID you want to get | 
  **pname** | **String**| the provider name | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **x_account_api_key** | **String**| Account API Key | 
 **ipid** | **i64**| the IP ID you want to get | 
 **pname** | **String**| the provider name | 
 **from** | **String**| from date | 
 **to** | **String**| to date | 

### Return type

[**Vec<::models::ModelsSmtpStat>**](models.SMTPStat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **s_mtp_stat_router_get_all_aggregate_ip_smtp_stats**
> Vec<::models::ModelsSmtpStat> s_mtp_stat_router_get_all_aggregate_ip_smtp_stats(x_account_api_key, ipid, optional)


Get All Aggregate IP SMTP Stats

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

[**Vec<::models::ModelsSmtpStat>**](models.SMTPStat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **s_mtp_stat_router_get_all_aggregate_ip_smtp_stats_for_sub_account**
> Vec<::models::ModelsSmtpStat> s_mtp_stat_router_get_all_aggregate_ip_smtp_stats_for_sub_account(x_account_api_key, ipid, sid, optional)


Get All Aggregate IP SMTP Stats For SubAccount

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **ipid** | **i64**| the IP ID you want to get | 
  **sid** | **i64**| the SubAccount ID you want to get | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **x_account_api_key** | **String**| Account API Key | 
 **ipid** | **i64**| the IP ID you want to get | 
 **sid** | **i64**| the SubAccount ID you want to get | 
 **from** | **String**| from date | 
 **to** | **String**| to date | 

### Return type

[**Vec<::models::ModelsSmtpStat>**](models.SMTPStat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **s_mtp_stat_router_get_all_aggregate_sub_account_provider_smtp_stats**
> Vec<::models::ModelsSmtpStat> s_mtp_stat_router_get_all_aggregate_sub_account_provider_smtp_stats(x_account_api_key, sid, pname, optional)


Get All Aggregate SubAccount Provider SMTP Stats

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **sid** | **i64**| the SubAccount ID you want to get | 
  **pname** | **String**| the provider name | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **x_account_api_key** | **String**| Account API Key | 
 **sid** | **i64**| the SubAccount ID you want to get | 
 **pname** | **String**| the provider name | 
 **from** | **String**| from date | 
 **to** | **String**| to date | 

### Return type

[**Vec<::models::ModelsSmtpStat>**](models.SMTPStat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **s_mtp_stat_router_get_all_aggregate_sub_account_smtp_stats**
> Vec<::models::ModelsSmtpStat> s_mtp_stat_router_get_all_aggregate_sub_account_smtp_stats(x_account_api_key, sid, optional)


Get All Aggregate SubAccount SMTP Stats

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **sid** | **i64**| the Sub-Account ID you want to get | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **x_account_api_key** | **String**| Account API Key | 
 **sid** | **i64**| the Sub-Account ID you want to get | 
 **from** | **String**| from date | 
 **to** | **String**| to date | 

### Return type

[**Vec<::models::ModelsSmtpStat>**](models.SMTPStat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **s_mtp_stat_router_get_all_aggregate_sub_account_smtp_stats_for_ip**
> Vec<::models::ModelsSmtpStat> s_mtp_stat_router_get_all_aggregate_sub_account_smtp_stats_for_ip(x_account_api_key, sid, ipid, optional)


Get All Aggregate SubAccount SMTP Stats For IP

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **sid** | **i64**| the Sub-Account ID you want to get | 
  **ipid** | **i64**| the IP  ID you want to get | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **x_account_api_key** | **String**| Account API Key | 
 **sid** | **i64**| the Sub-Account ID you want to get | 
 **ipid** | **i64**| the IP  ID you want to get | 
 **from** | **String**| from date | 
 **to** | **String**| to date | 

### Return type

[**Vec<::models::ModelsSmtpStat>**](models.SMTPStat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

