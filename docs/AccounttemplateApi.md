# \AccounttemplateApi

All URIs are relative to *https://api.sendpost.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_template_router_create**](AccounttemplateApi.md#account_template_router_create) | **Post** /account/template/ | 
[**account_template_router_delete**](AccounttemplateApi.md#account_template_router_delete) | **Delete** /account/template/{templateid} | 
[**account_template_router_get**](AccounttemplateApi.md#account_template_router_get) | **Get** /account/template/{templateid} | 
[**account_template_router_get_all**](AccounttemplateApi.md#account_template_router_get_all) | **Get** /account/template/ | 
[**account_template_router_update**](AccounttemplateApi.md#account_template_router_update) | **Put** /account/template/{templateid} | 


# **account_template_router_create**
> ::models::ModelsAccountTemplate account_template_router_create(x_account_api_key, body)


Create a new template

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **body** | [**ModelsAccountTemplate**](ModelsAccountTemplate.md)| The AccountTemplate content | 

### Return type

[**::models::ModelsAccountTemplate**](models.AccountTemplate.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **account_template_router_delete**
> ::models::ModelsDeleteResponse account_template_router_delete(x_account_api_key, templateid)


Delete AccountTemplate

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **templateid** | **i64**| The id of the template you want to delete | 

### Return type

[**::models::ModelsDeleteResponse**](models.DeleteResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **account_template_router_get**
> ::models::ModelsAccountTemplate account_template_router_get(x_account_api_key, templateid)


Get single template

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **templateid** | **i64**| ID of the template required | 

### Return type

[**::models::ModelsAccountTemplate**](models.AccountTemplate.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **account_template_router_get_all**
> Vec<::models::ModelsAccountTemplate> account_template_router_get_all(x_account_api_key)


Get all templates

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 

### Return type

[**Vec<::models::ModelsAccountTemplate>**](models.AccountTemplate.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **account_template_router_update**
> ::models::ModelsAccountTemplate account_template_router_update(x_account_api_key, templateid, body)


update template

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **templateid** | **i64**| The id of the template you want to update | 
  **body** | [**ModelsAccountTemplate**](ModelsAccountTemplate.md)| The template content | 

### Return type

[**::models::ModelsAccountTemplate**](models.AccountTemplate.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

