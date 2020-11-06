# \AccountvalidationApi

All URIs are relative to *https://api.sendpost.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**validate_router_validate_email_bulk**](AccountvalidationApi.md#validate_router_validate_email_bulk) | **Post** /account/validation/bulk | 
[**validation_router_count**](AccountvalidationApi.md#validation_router_count) | **Get** /account/validation/count | 
[**validation_router_delete_validation**](AccountvalidationApi.md#validation_router_delete_validation) | **Delete** /account/validation/ | 
[**validation_router_get_all**](AccountvalidationApi.md#validation_router_get_all) | **Get** /account/validation/ | 
[**validation_router_validate_email_list**](AccountvalidationApi.md#validation_router_validate_email_list) | **Post** /account/validation/ | 


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

# **validation_router_count**
> ::models::ModelsCountStat validation_router_count(x_account_api_key)


Count Total Validations

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 

### Return type

[**::models::ModelsCountStat**](models.CountStat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **validation_router_delete_validation**
> ::models::ModelsValidation validation_router_delete_validation(x_account_api_key, body)


Delete a specific validation

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **body** | [**ModelsEValidation**](ModelsEValidation.md)| List of emails to be deleted from validation | 

### Return type

[**::models::ModelsValidation**](models.Validation.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **validation_router_get_all**
> Vec<::models::ModelsValidation> validation_router_get_all(x_account_api_key, optional)


Get all validation

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
 **search** | **String**| search | 

### Return type

[**Vec<::models::ModelsValidation>**](models.Validation.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **validation_router_validate_email_list**
> ::models::ModelsCleanedList validation_router_validate_email_list(x_account_api_key, body)


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

