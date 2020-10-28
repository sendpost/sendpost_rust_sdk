# \AccountvalidateApi

All URIs are relative to *https://api.sendpost.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**validate_router_validate_email_bulk**](AccountvalidateApi.md#validate_router_validate_email_bulk) | **Post** /account/validate/bulk | 
[**validate_router_validate_email_list**](AccountvalidateApi.md#validate_router_validate_email_list) | **Post** /account/validate/ | 


# **validate_router_validate_email_bulk**
> ::models::ModelsBulkResponse validate_router_validate_email_bulk(fileinput, x_account_api_key)


Validate Emails In File Asynchronously

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **fileinput** | **File**| CSV whose emails need to be validated | 
  **x_account_api_key** | **String**| Account API Key | 

### Return type

[**::models::ModelsBulkResponse**](models.BulkResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: multipart/form-data
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **validate_router_validate_email_list**
> ::models::ModelsCleanedList validate_router_validate_email_list(x_account_api_key, body)


Validate Email List Synchronously

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **body** | [**ModelsEmailList**](ModelsEmailList.md)| The email list to be sent for being validated | 

### Return type

[**::models::ModelsCleanedList**](models.CleanedList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

