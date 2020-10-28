# \AccountalertApi

All URIs are relative to *https://api.sendpost.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**alert_router_count**](AccountalertApi.md#alert_router_count) | **Get** /account/alert/count | 
[**alert_router_create_alert**](AccountalertApi.md#alert_router_create_alert) | **Post** /account/alert/ | 
[**alert_router_get_all**](AccountalertApi.md#alert_router_get_all) | **Get** /account/alert/ | 
[**alert_router_update**](AccountalertApi.md#alert_router_update) | **Put** /account/alert/{alertid} | 


# **alert_router_count**
> ::models::ModelsCountStat alert_router_count(x_account_api_key, optional)


Count Total Alerts for account

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

# **alert_router_create_alert**
> ::models::ModelsAlert alert_router_create_alert(x_account_api_key, body)


create an alert

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **body** | [**ModelsAlertRequest**](ModelsAlertRequest.md)| The List to br sent | 

### Return type

[**::models::ModelsAlert**](models.Alert.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **alert_router_get_all**
> Vec<::models::ModelsDetailedAlert> alert_router_get_all(x_account_api_key, optional)


Get All Alerts

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

[**Vec<::models::ModelsDetailedAlert>**](models.DetailedAlert.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **alert_router_update**
> ::models::Alert alert_router_update(x_account_api_key, alertid, body)


Update an Alert

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **alertid** | **i64**| The alert you want to update | 
  **body** | [**Alert**](Alert.md)| The alert  Settings | 

### Return type

[**::models::Alert**](.alert.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

