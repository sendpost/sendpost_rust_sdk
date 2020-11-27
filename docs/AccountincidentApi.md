# \AccountincidentApi

All URIs are relative to *https://api.sendpost.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**incident_router_add**](AccountincidentApi.md#incident_router_add) | **Post** /account/incident/{incidentId}/comment | 
[**incident_router_count**](AccountincidentApi.md#incident_router_count) | **Get** /account/incident/count | 
[**incident_router_create**](AccountincidentApi.md#incident_router_create) | **Post** /account/incident/ | 
[**incident_router_get_all**](AccountincidentApi.md#incident_router_get_all) | **Get** /account/incident/ | 
[**incident_router_get_all_comments**](AccountincidentApi.md#incident_router_get_all_comments) | **Get** /account/incident/{incidentId}/comment | 
[**incident_router_get_incident**](AccountincidentApi.md#incident_router_get_incident) | **Get** /account/incident/{incidentId} | 
[**incident_router_update**](AccountincidentApi.md#incident_router_update) | **Put** /account/incident/{incidentId} | 


# **incident_router_add**
> ::models::ModelsComment incident_router_add(x_account_api_key, incident_id, body)


Add comment to Incident

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **incident_id** | **i64**| the incident id | 
  **body** | [**ModelsEComment**](ModelsEComment.md)| The Comment content | 

### Return type

[**::models::ModelsComment**](models.Comment.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **incident_router_count**
> ::models::ModelsCountStat incident_router_count(x_account_api_key, optional)


Count Total Incidents

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
 **status** | **i64**| status | 
 **tag** | **i64**| status | 
 **search** | **String**| search term | 

### Return type

[**::models::ModelsCountStat**](models.CountStat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **incident_router_create**
> ::models::ModelsIncident incident_router_create(x_account_api_key, body)


Create Incident

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **body** | [**ModelsEIncident**](ModelsEIncident.md)| The Incident content | 

### Return type

[**::models::ModelsIncident**](models.Incident.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **incident_router_get_all**
> Vec<::models::ModelsIncident> incident_router_get_all(x_account_api_key, optional)


Get All Incidents

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
 **status** | **i64**| status | 
 **tag** | **i64**| status | 

### Return type

[**Vec<::models::ModelsIncident>**](models.Incident.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **incident_router_get_all_comments**
> Vec<::models::ModelsComment> incident_router_get_all_comments(x_account_api_key, incident_id)


Get All Comments Associated with Incident

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **incident_id** | **i64**| the IncidentId you want to get comments for | 

### Return type

[**Vec<::models::ModelsComment>**](models.Comment.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **incident_router_get_incident**
> ::models::ModelsIncident incident_router_get_incident(x_account_api_key, incident_id)


Find Incident by incidentId

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **incident_id** | **i64**| the IncidentId you want to get | 

### Return type

[**::models::ModelsIncident**](models.Incident.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **incident_router_update**
> ::models::ModelsIncident incident_router_update(x_account_api_key, incident_id, body)


Update Incident

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **incident_id** | **i64**| the Incident Id you want to update | 
  **body** | [**ModelsEIncident**](ModelsEIncident.md)| The Incident content | 

### Return type

[**::models::ModelsIncident**](models.Incident.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

