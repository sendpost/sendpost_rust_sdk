# \AccounttagApi

All URIs are relative to *https://api.sendpost.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tag_router_create**](AccounttagApi.md#tag_router_create) | **Post** /account/tag/ | 
[**tag_router_delete**](AccounttagApi.md#tag_router_delete) | **Delete** /account/tag/{tagid} | 
[**tag_router_get_all**](AccounttagApi.md#tag_router_get_all) | **Get** /account/tag/ | 


# **tag_router_create**
> ::models::ModelsTag tag_router_create(x_account_api_key, body)


Create Tag

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **body** | [**ModelsTag**](ModelsTag.md)| The Tag content | 

### Return type

[**::models::ModelsTag**](models.Tag.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **tag_router_delete**
> ::models::ModelsDeleteResponse tag_router_delete(x_account_api_key, tagid)


Delete Tag

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **tagid** | **i64**| The AccountTagId you want to delete | 

### Return type

[**::models::ModelsDeleteResponse**](models.DeleteResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **tag_router_get_all**
> Vec<::models::ModelsTag> tag_router_get_all(x_account_api_key)


Get All Tags

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 

### Return type

[**Vec<::models::ModelsTag>**](models.Tag.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

