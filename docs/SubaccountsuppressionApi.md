# \SubaccountsuppressionApi

All URIs are relative to *https://api.sendpost.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**suppression_router_count**](SubaccountsuppressionApi.md#suppression_router_count) | **Get** /subaccount/suppression/count | 
[**suppression_router_create_suppressions**](SubaccountsuppressionApi.md#suppression_router_create_suppressions) | **Post** /subaccount/suppression/ | 
[**suppression_router_create_suppressions_in_suppression_filter**](SubaccountsuppressionApi.md#suppression_router_create_suppressions_in_suppression_filter) | **Post** /subaccount/suppression/filter | 
[**suppression_router_delete_suppression**](SubaccountsuppressionApi.md#suppression_router_delete_suppression) | **Delete** /subaccount/suppression/ | 
[**suppression_router_delete_suppressions_in_suppression_filter**](SubaccountsuppressionApi.md#suppression_router_delete_suppressions_in_suppression_filter) | **Delete** /subaccount/suppression/filter | 
[**suppression_router_get_all_suppressions**](SubaccountsuppressionApi.md#suppression_router_get_all_suppressions) | **Get** /subaccount/suppression/ | 


# **suppression_router_count**
> ::models::ModelsCountStat suppression_router_count(x_sub_account_api_key)


Count Total Suppressions

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

# **suppression_router_create_suppressions**
> ::models::ModelsSuppression suppression_router_create_suppressions(x_sub_account_api_key, body)


Add Email Addresses To Suppression List

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_sub_account_api_key** | **String**| Sub-Account API Key | 
  **body** | [**ModelsRSuppression**](ModelsRSuppression.md)| Suppression content | 

### Return type

[**::models::ModelsSuppression**](models.Suppression.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **suppression_router_create_suppressions_in_suppression_filter**
> suppression_router_create_suppressions_in_suppression_filter(body)


Add Email Addresses To Suppression Filter

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**ModelsSuppression**](ModelsSuppression.md)| Add suppressions to suppression filter | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **suppression_router_delete_suppression**
> ::models::ModelsSuppression suppression_router_delete_suppression(x_sub_account_api_key, body)


Delete specific emails which are in suppression list

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_sub_account_api_key** | **String**| Sub-Account API Key | 
  **body** | [**ModelsRdSuppression**](ModelsRdSuppression.md)| Suppression content | 

### Return type

[**::models::ModelsSuppression**](models.Suppression.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **suppression_router_delete_suppressions_in_suppression_filter**
> suppression_router_delete_suppressions_in_suppression_filter(body)


Delete specific emails which are in suppression list

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**ModelsSuppression**](ModelsSuppression.md)| Suppression content | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **suppression_router_get_all_suppressions**
> Vec<::models::ModelsSuppression> suppression_router_get_all_suppressions(x_sub_account_api_key, optional)


Get all suppressions

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
 **search** | **String**| search | 

### Return type

[**Vec<::models::ModelsSuppression>**](models.Suppression.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

