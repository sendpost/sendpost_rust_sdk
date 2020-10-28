# \SubaccountdomainApi

All URIs are relative to *https://api.sendpost.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**domain_router_count**](SubaccountdomainApi.md#domain_router_count) | **Get** /subaccount/domain/count | 
[**domain_router_create**](SubaccountdomainApi.md#domain_router_create) | **Post** /subaccount/domain/ | 
[**domain_router_delete**](SubaccountdomainApi.md#domain_router_delete) | **Delete** /subaccount/domain/{domainId} | 
[**domain_router_get**](SubaccountdomainApi.md#domain_router_get) | **Get** /subaccount/domain/{domainId} | 
[**domain_router_get_all**](SubaccountdomainApi.md#domain_router_get_all) | **Get** /subaccount/domain/ | 
[**domain_router_update**](SubaccountdomainApi.md#domain_router_update) | **Put** /subaccount/domain/{domainId} | 
[**domain_router_verify**](SubaccountdomainApi.md#domain_router_verify) | **Post** /subaccount/domain/{domainId}/verify | 


# **domain_router_count**
> ::models::ModelsCountStat domain_router_count(x_sub_account_api_key)


Count Total Domains

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

# **domain_router_create**
> ::models::ModelsDomain domain_router_create(x_sub_account_api_key, body)


Create Domain

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_sub_account_api_key** | **String**| Sub-Account API Key | 
  **body** | [**ModelsEDomain**](ModelsEDomain.md)| The Domain content | 

### Return type

[**::models::ModelsDomain**](models.Domain.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **domain_router_delete**
> ::models::ModelsDeleteResponse domain_router_delete(x_sub_account_api_key, domain_id)


Delete Domain

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_sub_account_api_key** | **String**| Sub-Account API Key | 
  **domain_id** | **i64**| The DomainId you want to delete | 

### Return type

[**::models::ModelsDeleteResponse**](models.DeleteResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **domain_router_get**
> ::models::ModelsDomain domain_router_get(x_sub_account_api_key, domain_id)


Find Domain by DomainId

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_sub_account_api_key** | **String**| Sub-Account API Key | 
  **domain_id** | **i64**| the DomainId you want to get | 

### Return type

[**::models::ModelsDomain**](models.Domain.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **domain_router_get_all**
> Vec<::models::ModelsDomain> domain_router_get_all(x_sub_account_api_key, optional)


Get All Domains

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
 **search** | **String**| search term | 

### Return type

[**Vec<::models::ModelsDomain>**](models.Domain.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **domain_router_update**
> ::models::ModelsDomain domain_router_update(x_sub_account_api_key, domain_id, body)


Update Domain

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_sub_account_api_key** | **String**| Sub-Account API Key | 
  **domain_id** | **i64**| The DomainId you want to update | 
  **body** | [**ModelsEDomain**](ModelsEDomain.md)| The body | 

### Return type

[**::models::ModelsDomain**](models.Domain.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **domain_router_verify**
> ::models::ModelsDomain domain_router_verify(x_sub_account_api_key, domain_id)


Verify Domain By Domain Id

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_sub_account_api_key** | **String**| Sub-Account API Key | 
  **domain_id** | **i64**| the DomainId you want to get | 

### Return type

[**::models::ModelsDomain**](models.Domain.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

