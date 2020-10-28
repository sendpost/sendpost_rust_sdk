# \AccountmemberApi

All URIs are relative to *https://api.sendpost.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**member_router_delete**](AccountmemberApi.md#member_router_delete) | **Delete** /account/member/{memberId} | 
[**member_router_get**](AccountmemberApi.md#member_router_get) | **Get** /account/member/{memberId} | 
[**member_router_get_all**](AccountmemberApi.md#member_router_get_all) | **Get** /account/member/ | 


# **member_router_delete**
> ::models::ModelsDeleteResponse member_router_delete(x_account_api_key, member_id)


Delete Member

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **member_id** | **i64**| The MemberId you want to delete | 

### Return type

[**::models::ModelsDeleteResponse**](models.DeleteResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **member_router_get**
> ::models::ModelsMember member_router_get(x_account_api_key, member_id)


Find Member by MemberId

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **member_id** | **i64**| the MemberId you want to get | 

### Return type

[**::models::ModelsMember**](models.Member.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **member_router_get_all**
> Vec<::models::ModelsMember> member_router_get_all(x_account_api_key)


Get All Members

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 

### Return type

[**Vec<::models::ModelsMember>**](models.Member.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

